extern crate silk;

use silk::historian::Historian;
use silk::log::{verify_slice, Entry, Sha256Hash};
use silk::event::{generate_keypair, get_pubkey, sign_claim_data, Event};
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::SendError;

fn create_log(hist: &Historian<Sha256Hash>) -> Result<(), SendError<Event<Sha256Hash>>> {
    sleep(Duration::from_millis(15));
    let data = Sha256Hash::default();
    let keypair = generate_keypair();
    let event0 = Event::new_claim(get_pubkey(&keypair), data, sign_claim_data(&data, &keypair));
    hist.sender.send(event0)?;
    sleep(Duration::from_millis(10));
    Ok(())
}

fn main() {
    let seed = Sha256Hash::default();
    let hist = Historian::new(&seed, Some(10));
    create_log(&hist).expect("send error");
    drop(hist.sender);
    let entries: Vec<Entry<Sha256Hash>> = hist.receiver.iter().collect();
    for entry in &entries {
        println!("{:?}", entry);
    }
    // Proof-of-History: Verify the historian learned about the events
    // in the same order they appear in the vector.
    assert!(verify_slice(&entries, &seed));
}