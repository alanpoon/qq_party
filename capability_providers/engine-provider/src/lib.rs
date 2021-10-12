#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use chrono::prelude::*;

mod store;

use store::MatchStore;

extern crate wasmdome_domain as domain;
extern crate wasmdome_protocol as protocol;

use codec::capabilities::{
    CapabilityDescriptor, CapabilityProvider, Dispatcher, NullDispatcher, OperationDirection,
    OP_GET_CAPABILITY_DESCRIPTOR,
};
use codec::core::{CapabilityConfiguration, OP_BIND_ACTOR, OP_REMOVE_ACTOR};
use codec::{deserialize, serialize};
use protocol::commands::*;
use protocol::{events::ArenaEvent, OP_TAKE_TURN};

use domain::state::MatchState;
use protocol::MechInfo;
use std::error::Error;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

const SYSTEM_ACTOR: &str = "system";
const CAPABILITY_ID: &str = "wasmdome:engine";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REVISION: u32 = 1;

const LATTICE_HOST_KEY: &str = "LATTICE_HOST"; // env var name
const DEFAULT_LATTICE_HOST: &str = "127.0.0.1"; // default mode is anonymous via loopback
const LATTICE_CREDSFILE_KEY: &str = "LATTICE_CREDS_FILE";
const TURN_DELAY_MILLIS_KEY: &str = "TURN_DELAY_MILLIS";
const TURN_DELAY_MILLIS_DEFAULT: u64 = 0;

const PROVIDER_QUEUE: &str = "wasmdome-provider"; // Queue subscription ID

#[cfg(not(feature = "static_plugin"))]
capability_provider!(WasmdomeEngineProvider, WasmdomeEngineProvider::new);

pub struct WasmdomeEngineProvider {
    nc: Arc<nats::Connection>,
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
    store: Arc<RwLock<MatchStore>>,
    turn_delay_millis: u64,
}

impl Default for WasmdomeEngineProvider {
    fn default() -> Self {
        let _ = env_logger::builder().format_module_path(false).try_init();
        let td = get_turn_delay();
        info!("Using turn delay of {}ms", td);

        WasmdomeEngineProvider {
            dispatcher: Arc::new(RwLock::new(Box::new(NullDispatcher::new()))),
            store: Arc::new(RwLock::new(MatchStore::new())),
            nc: Arc::new(get_connection()),
            turn_delay_millis: get_turn_delay(),
        }
    }
}

fn get_turn_delay() -> u64 {
    match std::env::var(TURN_DELAY_MILLIS_KEY) {
        Ok(val) => {
            if val.is_empty() {
                TURN_DELAY_MILLIS_DEFAULT
            } else {
                val.parse().unwrap_or(TURN_DELAY_MILLIS_DEFAULT)
            }
        }
        Err(_) => TURN_DELAY_MILLIS_DEFAULT,
    }
}

fn get_connection() -> nats::Connection {
    let host = get_env(LATTICE_HOST_KEY, DEFAULT_LATTICE_HOST);
    let mut opts = if let Some(creds) = get_credsfile() {
        nats::Options::with_credentials(creds)
    } else {
        nats::Options::new()
    };
    opts = opts.with_name("waSCC Lattice");
    opts.connect(&host).unwrap()
}

fn get_credsfile() -> Option<String> {
    std::env::var(LATTICE_CREDSFILE_KEY).ok()
}

fn get_env(var: &str, default: &str) -> String {
    match std::env::var(var) {
        Ok(val) => {
            if val.is_empty() {
                default.to_string()
            } else {
                val.to_string()
            }
        }
        Err(_) => default.to_string(),
    }
}

impl WasmdomeEngineProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(&self, config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        // Handle actor binding metadata here...
        // This is typically where you would establish a
        // client or connection to a resource on behalf of
        // an actor
        let mi = mechinfo_from_hashmap(&config.module, config.values);
        self.store
            .write()
            .unwrap()
            .add_bound_actor(&config.module, mi.clone())?;

        self.nc
            .publish(
                &protocol::events::events_subject(None),
                serde_json::to_string(&ArenaEvent::MechConnected {
                    name: mi.name.to_string(),
                    avatar: mi.avatar.to_string(),
                    team: mi.team.to_string(),
                    actor: config.module.to_string(),
                    time: Utc::now(),
                })
                .unwrap(),
            )
            .unwrap();

        Ok(vec![])
    }

    fn deconfigure(&self, config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        // Handle removal of resources claimed by an actor here

        self.store
            .write()
            .unwrap()
            .remove_bound_actor(&config.module)?;

        publish_disconnect_event(self.nc.clone(), &config.module);
        Ok(vec![])
    }

    // Capability providers must provide a descriptor to the host containing metadata and a list of supported operations
    fn get_descriptor(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(serialize(
            CapabilityDescriptor::builder()
                .id(CAPABILITY_ID)
                .name("Assembly Mechs: Beyond Wasmdome Game Engine Provider")
                .long_description("A capability provider that exposes the core of the game engine")
                .version(VERSION)
                .revision(REVISION)
                .with_operation(
                    OP_TAKE_TURN,
                    OperationDirection::ToActor,
                    "Send to the actor to obtain the actor's action sequence for the given turn",
                ) // TODO: make the operation descriptors match your real interface
                .build(),
        )?)
    }
}

fn remove_noshows(orig: &Vec<String>, healthy: &Vec<String>) -> Vec<String> {
    let mut filtered = orig.clone();
    filtered.retain(|s| healthy.contains(s));
    filtered
}

impl CapabilityProvider for WasmdomeEngineProvider {
    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;
        let td = self.turn_delay_millis;

        spawn_health_check(self.nc.clone(), self.dispatcher.clone(), self.store.clone());
        let (nc, dp, sto) = (self.nc.clone(), self.dispatcher.clone(), self.store.clone());
        let _h = self
            .nc
            .queue_subscribe(&protocol::commands::arena_control_subject(), PROVIDER_QUEUE)?
            .with_handler(move |msg| {
                let ac: ArenaControlCommand = serde_json::from_slice(&msg.data).unwrap();
                handle_control_command(ac, nc.clone(), dp.clone(), sto.clone(), msg.reply, td);
                Ok(())
            });

        Ok(())
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => self.configure(deserialize(msg)?),
            OP_REMOVE_ACTOR if actor == SYSTEM_ACTOR => self.deconfigure(deserialize(msg)?),
            OP_GET_CAPABILITY_DESCRIPTOR if actor == SYSTEM_ACTOR => self.get_descriptor(),
            //  OP_START_MATCH => self.start_match(deserialize(msg)?),
            _ => Err("bad dispatch".into()),
        }
    }
}

fn handle_control_command(
    ac: ArenaControlCommand,
    nc: Arc<nats::Connection>,
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
    store: Arc<RwLock<MatchStore>>,
    reply: Option<String>,
    turn_delay_millis: u64,
) {
    use ArenaControlCommand::*;
    match ac {
        StartMatch(cm) => match start_match(cm, store, dispatcher, nc.clone(), turn_delay_millis) {
            Ok(_) => {
                if let Some(s) = reply {
                    let _ = nc.publish(&s, b"OK");
                }
            }
            Err(e) => {
                error!("Failed to start match: {}", e);
            }
        },
        QueryMechs => match reply {
            Some(s) => {
                let mut lock = store.write().unwrap();
                let resp = MechQueryResponse {
                    mechs: lock.bound_actors().unwrap().clone(),
                };
                nc.publish(&s, &serde_json::to_vec(&resp).unwrap()).unwrap();
            }
            None => {}
        },
    };
}

fn start_match(
    createmsg: CreateMatch,
    store: Arc<RwLock<MatchStore>>,
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
    nc: Arc<nats::Connection>,
    turn_delay_millis: u64,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // ensure that right before we start the match, we're confident all mechs have responded to a health request
    perform_health_check(store.clone(), dispatcher.clone(), nc.clone());
    use domain::MatchParameters;

    let current_mech_ids: Vec<String> = store
        .write()
        .unwrap()
        .bound_actors()?
        .iter()
        .map(|a| a.id.to_string())
        .collect();
    let params = MatchParameters::new(
        createmsg.match_id.clone(),
        createmsg.board_width,
        createmsg.board_height,
        createmsg.max_turns,
        createmsg.aps_per_turn,
        remove_noshows(&createmsg.actors, &current_mech_ids), // use this instead of the match params list because this one's filtered by healthy
    );
    let mut state = MatchState::new_with_parameters(params.clone());
    state = spawn_mechs(nc.clone(), state, store.write().unwrap().bound_actors()?);
    store
        .write()
        .unwrap()
        .save_match_state(&createmsg.match_id, state)?;

    nc.publish(
        &protocol::events::events_subject(None),
        &serde_json::to_string(&ArenaEvent::MatchStarted {
            match_id: createmsg.match_id.clone(),
            actors: params.actors.clone(),
            board_height: params.height,
            board_width: params.width,
            start_time: Utc::now(),
        })?,
    )?;
    nc.flush()?;
    std::thread::sleep(Duration::from_millis(turn_delay_millis));

    manage_match(
        nc.clone(),
        dispatcher.clone(),
        store.clone(),
        createmsg.actors.clone(),
        createmsg.match_id.clone(),
        turn_delay_millis,
    );
    Ok(vec![])
}

fn mechinfo_from_hashmap(actor: &str, hm: HashMap<String, String>) -> MechInfo {
    use wascc_codec::core::{CONFIG_WASCC_CLAIMS_NAME, CONFIG_WASCC_CLAIMS_TAGS};
    let name: String = hm
        .get(CONFIG_WASCC_CLAIMS_NAME)
        .unwrap_or(&"Anonymous Mech".to_string())
        .to_string();
    let tags: String = hm
        .get(CONFIG_WASCC_CLAIMS_TAGS)
        .unwrap_or(&"".to_string())
        .to_string();
    let tvec: Vec<String> = tags.to_string().split(",").map(|t| t.to_string()).collect();
    MechInfo {
        id: actor.to_string(),
        avatar: get_avatar(&tvec),
        name: name.to_string(),
        team: get_team(&tvec),
    }
}

fn get_team(tags: &Vec<String>) -> String {
    if tags.contains(&"npc".to_string()) {
        "boylur".to_string()
    } else {
        "earth".to_string()
    }
}

fn get_avatar(tags: &Vec<String>) -> String {
    match tags.iter().find(|t| t.starts_with("avatar-")) {
        Some(t) => t.replace("avatar-", ""),
        None => "none".to_string(),
    }
}

mod game_logic;
use game_logic::*;
