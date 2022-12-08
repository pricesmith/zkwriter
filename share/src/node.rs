use libp2p::{identity, PeerId};
use std::error::Error;

pub fn node() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    Ok(())
}










// use libp2p::swarm::{Swarm, SwarmEvent, Behavior};
// use async_std::io;

// pub fn create_source_node() {

// }

// pub fn create_target_node() {

// }

// pub fn connect_to_target_node() {

// }

// pub fn count_source_node_peers() {

// }

// pub fn count_target_node_peers() {

// }

// pub fn print_node_id() {

// }

// pub fn print_node_addresses() {

// }

// pub fn init_and_connect() {

// }

// //

// pub fn run_target_node() {

// }

// pub fn run_source_node() {

// }