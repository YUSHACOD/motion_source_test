use bevy::prelude::{EulerRot, Quat};

pub fn parse_quat(input_buffer: &[u8; 12]) -> Result<Quat, &'static str> {

    let x = f32::from_be_bytes(input_buffer[0..4].try_into().unwrap());
    let y = f32::from_be_bytes(input_buffer[4..8].try_into().unwrap());
    let z = f32::from_be_bytes(input_buffer[8..12].try_into().unwrap());

    // This shit is the key to get proper orientation of this shit
    let result = Quat::from_euler(EulerRot::YXZ, -x, -y, -z);
    ///////////////////////////////////////////////////////////////

    Ok(result)
}
