use bevy::prelude::*;
use std::net::UdpSocket;
use std::process;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};
use std::time::Duration;
use std::thread;

mod utils;

/// Atomic quaternion representation using AtomicU32
#[derive(Default)]
struct AtomicQuat {
    x: AtomicU32,
    y: AtomicU32,
    z: AtomicU32,
    w: AtomicU32,
}

impl AtomicQuat {
    fn new() -> Self {
        Self::default()
    }

    fn store(&self, quat: Quat) {
        self.x.store(quat.x.to_bits(), Ordering::Relaxed);
        self.y.store(quat.y.to_bits(), Ordering::Relaxed);
        self.z.store(quat.z.to_bits(), Ordering::Relaxed);
        self.w.store(quat.w.to_bits(), Ordering::Relaxed);
    }

    fn load(&self) -> Quat {
        let x = f32::from_bits(self.x.load(Ordering::Relaxed));
        let y = f32::from_bits(self.y.load(Ordering::Relaxed));
        let z = f32::from_bits(self.z.load(Ordering::Relaxed));
        let w = f32::from_bits(self.w.load(Ordering::Relaxed));

        Quat::from_xyzw(x, y, z, w)
    }
}

#[derive(Resource)]
struct RotationResource(Arc<AtomicQuat>);

fn main() {
    // Parsing arguments for IP and port
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <local_network_ip> <port>", args[0]);
        process::exit(1);
    }

    // Setting up UDP socket
    let sockt_addr = format!("{}:{}", args[1], args[2]);
    let socket = UdpSocket::bind(sockt_addr.as_str()).expect("Failed to bind UDP socket");
    socket
        .set_read_timeout(Some(Duration::from_millis(1)))
        .expect("Failed to set read timeout");

    // Create a shared atomic quaternion
    let rotation_atomic = Arc::new(AtomicQuat::new());

    // Start UDP listener in an async tokio runtime
    let rotation_data_clone = rotation_atomic.clone();
    thread::spawn(move || udp_rot_listener(rotation_data_clone, socket));

    // Bevy App
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RotationResource(rotation_atomic))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(40.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // cuboid
    commands.spawn((
        Player,
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(9.0, 4.5, 0.0).looking_at(Vec3::ZERO, Vec3::ZERO),
    ));
}

/// Asynchronous UDP listener that updates the atomic quaternion
fn udp_rot_listener(rotation_atomic: Arc<AtomicQuat>, socket: UdpSocket) {
    let mut buffer = [0u8; 12];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((_size, _src_addr)) => {
                if let Ok(quat) = utils::parse_quat(&buffer) {
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

const MOVE_SPEED: f32 = 0.1;
const ROT_SPEED: f32 = 0.05;

fn move_player(
    mut transforms: Query<&mut Transform, With<Player>>,
    rotation_resource: Res<RotationResource>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut rot = Quat::IDENTITY;

        // Translation movement
        if keys.pressed(KeyCode::KeyW) {
            direction.x -= 1.0
        };
        if keys.pressed(KeyCode::KeyA) {
            direction.z += 1.0
        };
        if keys.pressed(KeyCode::KeyS) {
            direction.x += 1.0
        };
        if keys.pressed(KeyCode::KeyD) {
            direction.z -= 1.0
        };
        if keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0
        };
        if keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0
        };

        // Rotation using numpad
        if keys.pressed(KeyCode::Numpad7) {
            rot.x += 1.0;
        };
        if keys.pressed(KeyCode::Numpad4) {
            rot.x -= 1.0;
        };
        if keys.pressed(KeyCode::Numpad8) {
            rot.y += 1.0;
        };
        if keys.pressed(KeyCode::Numpad5) {
            rot.y -= 1.0;
        };
        if keys.pressed(KeyCode::Numpad9) {
            rot.z += 1.0;
        };
        if keys.pressed(KeyCode::Numpad6) {
            rot.z -= 1.0;
        };

        if 0.0 < direction.length() {
            transform.translation += MOVE_SPEED * direction.normalize();
        }

        if rot != Quat::IDENTITY {
            println!("{:?}", transform.rotation);
            transform.rotate_x(rot.x * ROT_SPEED);
            transform.rotate_y(rot.y * ROT_SPEED);
            transform.rotate_z(rot.z * ROT_SPEED);
        }

        // Apply the latest rotation
        transform.rotation = rotation_resource.0.load();
    }
}
