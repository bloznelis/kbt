mod backend;
mod key;
mod linux;

use backend::KeyBackend;
use key::Key;
use linux::X11;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let receiver = X11.subscribe()?;

    loop {
        let key = receiver.recv()?;
        println!("received {:?}", key)
    }
}
