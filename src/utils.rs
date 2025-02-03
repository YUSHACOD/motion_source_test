use bevy::prelude::{EulerRot, Quat};

pub fn parse_quat(input: &str) -> Result<Option<Quat>, &'static str> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 3 {
        return Err("Input does not contain exactly three coordinates");
    }

    let x = parts[0]
        .trim()
        .parse::<f32>()
        .map_err(|_| "Invalid float for X")?;

    let y = parts[1]
        .trim()
        .parse::<f32>()
        .map_err(|_| "Invalid float for Y")?;

    let z = parts[2]
        .trim()
        .parse::<f32>()
        .map_err(|_| "Invalid float for Z")?;

    // This shit is the key to get proper orientation of this shit
    let result = Quat::from_euler(EulerRot::YXZ, -x, -y, -z);
    ///////////////////////////////////////////////////////////////
    Ok(Some(result))
}
