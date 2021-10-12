use domain::state::MatchState;
use std::collections::HashMap;

const REDIS_URL_KEY: &str = "WASMDOME_ENGINE_REDIS_URL";

extern crate redis;
use crate::MechInfo;
use redis::Commands;

pub(crate) struct MatchStore {
    matches: Option<HashMap<String, MatchState>>,
    bound_actors: HashMap<String, MechInfo>,
    client: Option<redis::Client>,
}

impl Default for MatchStore {
    fn default() -> MatchStore {
        MatchStore {
            matches: Some(HashMap::new()),
            client: None,
            bound_actors: HashMap::new(),
        }
    }
}

impl MatchStore {
    pub fn new() -> MatchStore {
        if let Ok(url) = std::env::var(REDIS_URL_KEY) {
            info!("Wasmdome Engine Provider Using Redis: {}", url);
            MatchStore::from_redis_url(&url)
        } else {
            info!("Wasmdome Engine Provider Using In-Memory Data");
            MatchStore::default()
        }
    }

    fn from_redis_url(url: &str) -> MatchStore {
        MatchStore {
            matches: None,
            client: Some(redis::Client::open(url).unwrap()),
            bound_actors: HashMap::new(),
        }
    }

    pub fn add_bound_actor(
        &mut self,
        actor: &str,
        mechinfo: MechInfo,
    ) -> Result<(), Box<dyn ::std::error::Error>> {
        if self.client.is_none() {
            let _ = self.bound_actors.insert(actor.to_string(), mechinfo);
            Ok(())
        } else {
            let k = actors_key();
            let _: u32 = self.client.as_mut().unwrap().sadd(k, actor)?;
            let mi = mechinfo_key(actor);
            self.client
                .as_mut()
                .unwrap()
                .set(mi, &serde_json::to_string(&mechinfo)?)
                .map(|_: ()| ())
                .map_err(|e| e.into())
        }
    }

    pub fn bound_actors(&mut self) -> Result<Vec<MechInfo>, Box<dyn ::std::error::Error>> {
        if self.client.is_none() {
            let ba: Vec<MechInfo> = self.bound_actors.values().cloned().collect();
            Ok(ba)
        } else {
            let mut mis = Vec::new();
            let k = actors_key();
            let actors: Vec<String> = self.client.as_mut().unwrap().smembers(k)?;
            for actor in actors {
                let mi = mechinfo_key(&actor);
                let s: String = self.client.as_mut().unwrap().get(mi)?;
                let mechinfo: MechInfo = serde_json::from_str(&s)?;
                mis.push(mechinfo);
            }
            Ok(mis)
        }
    }

    pub fn remove_bound_actor(&mut self, actor: &str) -> Result<(), Box<dyn ::std::error::Error>> {
        if self.client.is_none() {
            let _ = self.bound_actors.remove(actor);
            Ok(())
        } else {
            let mi = mechinfo_key(actor);
            let _: bool = self.client.as_mut().unwrap().del(&mi)?;
            let k = actors_key();
            self.client
                .as_mut()
                .unwrap()
                .srem(k, actor.to_string())
                .map(|_: ()| ())
                .map_err(|e| e.into())
        }
    }

    pub fn save_match_state(
        &mut self,
        match_id: &str,
        state: domain::state::MatchState,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error>> {
        if self.matches.is_some() {
            let _existed = self
                .matches
                .as_mut()
                .unwrap()
                .insert(match_id.to_string(), state);
            Ok(())
        } else {
            self.client
                .as_ref()
                .unwrap()
                .get_connection()?
                .set(&match_key(match_id), serde_json::to_string(&state)?)
                .map(|_: ()| ())
                .map_err(|e| e.into())
        }
    }

    pub fn get_match_state(
        &self,
        match_id: &str,
    ) -> ::std::result::Result<domain::state::MatchState, Box<dyn ::std::error::Error>> {
        if self.matches.is_some() {
            self.matches
                .as_ref()
                .unwrap()
                .get(match_id)
                .cloned()
                .ok_or("No such match".into())
        } else {
            let s: String = self
                .client
                .as_ref()
                .unwrap()
                .get_connection()?
                .get(&match_key(match_id))?;
            Ok(serde_json::from_str(&s)?)
        }
    }
}

fn match_key(match_id: &str) -> String {
    format!("wasmdome:matches:{}:state", match_id)
}

fn actors_key() -> String {
    "wasmdome:actors".to_string()
}

fn mechinfo_key(pk: &str) -> String {
    format!("wasmdome:actor:{}", pk)
}

#[cfg(test)]
mod test {
    use wasmdome_protocol::commands::{ArenaControlCommand, CreateMatch};

    #[test]
    fn singleton_enum_serialize() {
        // This test is basically just here so that we can provide a reference for how the
        // Phoenix/Elixir website needs to send arena control messages

        let query = ArenaControlCommand::QueryMechs;
        let s = serde_json::to_string(&query).unwrap();
        assert_eq!("\"QueryMechs\"", s);

        let sm = ArenaControlCommand::StartMatch(CreateMatch {
            actors: Vec::new(),
            aps_per_turn: 4,
            match_id: "test".to_string(),
            board_height: 10,
            board_width: 20,
            max_turns: 100,
        });
        let s2 = serde_json::to_string(&sm).unwrap();
        assert_eq!(
            "{\"StartMatch\":{\"match_id\":\"test\",\"actors\":[],\"board_height\":10,\"board_width\":20,\"max_turns\":100,\"aps_per_turn\":4}}", 
            s2);
    }
}
