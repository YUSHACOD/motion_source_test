use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use crate::rotation::AtomicQuat;
use bevy::prelude::*;

pub fn parse_quat(input_buffer: &[u8; 12]) -> Result<Quat, &'static str> {

    let x = f32::from_be_bytes(input_buffer[0..4].try_into().unwrap());
    let y = f32::from_be_bytes(input_buffer[4..8].try_into().unwrap());
    let z = f32::from_be_bytes(input_buffer[8..12].try_into().unwrap());

    // This shit is the key to get proper orientation of this shit
    let result = Quat::from_euler(EulerRot::YXZ, -x, -y, -z);
    ///////////////////////////////////////////////////////////////

    Ok(result)
}

pub fn start_udp_listener(rotation_atomic: Arc<AtomicQuat>, sockt_addr: String) {
    let socket = UdpSocket::bind(sockt_addr).expect("Failed to bind UDP socket");
    socket
        .set_read_timeout(Some(Duration::from_millis(1)))
        .expect("Failed to set read timeout");

    let mut buffer = [0u8; 12];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((_size, _src_addr)) => {
                if let Ok(quat) = parse_quat(&buffer) {
                    rotation_atomic.store(quat); // Store latest rotation atomically
                }
            }
            Err(_e) => {
                // Optional: Uncomment for debugging
                // println!("UDP read timeout");
            }
        }
        thread::sleep(Duration::from_millis(1)); // 1000 Hz polling
    }
}
