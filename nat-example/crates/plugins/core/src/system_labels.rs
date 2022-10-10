use bevy::prelude::*;
use derivative::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum DeskSystem {
    UpdateStatesToLatest,
    Shell,
    HandleOperations,
    PrePhysics,
}

#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
pub enum ShellSystem {
    Add,
    UpdateComponent,
    UpdateWidget,
    Render,
    HandleEvents,
}

#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
pub enum ProtocolSystem {
    ReceiveEvents,
    HandleEvents,
    SendCommands,
}

#[derive(Derivative)]
#[derivative(
    PartialEq(bound = ""),
    Eq(bound = ""),
    Debug(bound = ""),
    Hash(bound = ""),
    Clone(bound = "")
)]
#[derive(SystemLabel)]
pub enum EventHandlerSystem {
    Before,
    Handle,
    After,
 //   _Phantom(std::convert::Infallible, std::marker::PhantomData<T>),
}

// impl<T: Send + Sync + 'static> SystemLabel for EventHandlerSystem<T> {
//     fn dyn_clone(&self) -> Box<dyn SystemLabel> {
//         Box::new(self.clone())
        
//     }
//     fn as_str(&self) -> &'static str {
//         let s = self.0.to_string();
//         Box::leak(s.into_boxed_str())
//     }
// }

