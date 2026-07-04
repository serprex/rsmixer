use std::any::Any;

use super::actor::ActorItem;
use crate::prelude::*;

pub type BoxedMessage = Box<dyn Any + Send + Sync + 'static>;

pub enum SystemMessage {
    RegisterActor(ActorItem),
    StopActor(&'static str),
    StartActor(&'static str),
    SendMsg(&'static str, BoxedMessage),
    RestartActor(&'static str),
    ActorTaskFinished(&'static str, Option<Result<()>>),
    Shutdown,
    // Broadcast(BoxedMessage),
}

pub struct Shutdown {}
