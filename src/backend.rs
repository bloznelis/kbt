use std::sync::mpsc::Sender;

use crate::AppEvent;

pub trait KeyBackend {
    fn subscribe(&self, sender: Sender<AppEvent>) -> Result<(), Box<dyn std::error::Error>>;
}
