use std::sync::mpsc::Sender;

use crate::{AppEvent, KbtError};

pub trait KeyBackend {
    fn subscribe(&self, sender: Sender<AppEvent>) -> Result<(), KbtError>;
}
