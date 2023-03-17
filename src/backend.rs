use std::sync::mpsc::Receiver;

use crate::Key;

pub trait KeyBackend {
    fn subscribe(&self) -> Result<Receiver<Key>, Box<dyn std::error::Error>>;
}
