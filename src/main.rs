use bevy::prelude::*;
// use std::net::UdpSocket;

mod utils;

// struct UdpListener {
//     socket: UdpSocket,
//     buffer: [u8; 1024],
// }

fn main() {
    // let mut udp_listener = UdpListener {
    //     socket: UdpSocket::bind("192.168.1.4:42069").unwrap(),
    //     buffer: [0u8; 1024],
    // };

    // Main shit do not fuck with this thing /////////
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        // .add_systems(Update, rot_player)
        // .add_systems(Update, move |event: EventWriter<RotEvent>| {
        //     rot_event(event, &mut udp_listener);
        // })
        // .add_event::<RotEvent>()
        .run();
    //////////////////////////////////////////////////
}

#[derive(Component)]
struct Player;

// #[derive(Event)]
// struct RotEvent(Option<Quat>);

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
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.5, 1.0))),
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
        Transform::from_xyz(0.0, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::ZERO),
    ));
}

// fn rot_event(mut event_writer: EventWriter<RotEvent>, udp_listener: &mut UdpListener) {
//     match udp_listener.socket.recv_from(&mut udp_listener.buffer) {
//         Ok((size, _src_addr)) => {
//             let message = String::from_utf8_lossy(&udp_listener.buffer[..size]);
//
//             let quat = utils::parse_quat(&message).unwrap_or(None);
//             event_writer.send(RotEvent(quat));
//         }
//         Err(e) => {
//             eprintln!("❌ Receive error: {}", e);
//         }
//     }
// }

// fn rot_player(
//     mut event_reader: EventReader<RotEvent>,
//     mut transforms: Query<&mut Transform, With<Player>>) {
//     for mut transform in transforms.iter_mut() {
//         for quat in event_reader.read() {
//             match quat.0 {
//                 Some(rot) => transform.rotation = rot,
//                 None => (),
//             }
//         }
//     }
// }

const MOVE_SPEED: f32 = 0.1;
const ROT_SPEED: f32 = 0.05;
fn move_player(
    mut transforms: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut rot = Quat::IDENTITY;

        // translation
        if keys.pressed(KeyCode::KeyW) {
            direction.z -= 1.0
        };
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0
        };
        if keys.pressed(KeyCode::KeyS) {
            direction.z += 1.0
        };
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0
        };
        if keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0
        };
        if keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0
        };

        // rotation x
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

        // transform.rotation.x += rot.x;
        // transform.rotation.y += rot.y;
        if rot != Quat::IDENTITY {
            println!("{:?}", transform.rotation);
            transform.rotate_x(rot.x * ROT_SPEED);
            transform.rotate_y(rot.y * ROT_SPEED);
            transform.rotate_z(rot.z * ROT_SPEED);
        }
    }
}
