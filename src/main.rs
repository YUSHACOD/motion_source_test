mod setup;
mod rotation;
mod udp_listener;
mod input_handler;

use bevy::prelude::*;
use std::process;

use setup::setup;
use input_handler::handle_input;
use rotation::{AtomicQuat, RotationResource};
use udp_listener::start_udp_listener;

#[derive(Component)]
pub struct Player;

fn main() {
    // Parsing arguments for IP and port
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <local_network_ip> <port>", args[0]);
        process::exit(1);
    }

    // Setting up UDP socket
    let sockt_addr = format!("{}:{}", args[1], args[2]);

    // Create a shared atomic quaternion
    let rotation_atomic = std::sync::Arc::new(AtomicQuat::new());

    // Start UDP listener in a separate thread
    let rotation_data_clone = rotation_atomic.clone();
    std::thread::spawn(move || start_udp_listener(rotation_data_clone, sockt_addr));

    // Bevy App
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RotationResource(rotation_atomic))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}
