use bevy::prelude::*;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

/// Atomic quaternion representation using AtomicU32
#[derive(Default)]
pub struct AtomicQuat {
    pub x: AtomicU32,
    pub y: AtomicU32,
    pub z: AtomicU32,
    pub w: AtomicU32,
}

impl AtomicQuat {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store(&self, quat: Quat) {
        self.x.store(quat.x.to_bits(), Ordering::Relaxed);
        self.y.store(quat.y.to_bits(), Ordering::Relaxed);
        self.z.store(quat.z.to_bits(), Ordering::Relaxed);
        self.w.store(quat.w.to_bits(), Ordering::Relaxed);
    }

    pub fn load(&self) -> Quat {
        let x = f32::from_bits(self.x.load(Ordering::Relaxed));
        let y = f32::from_bits(self.y.load(Ordering::Relaxed));
        let z = f32::from_bits(self.z.load(Ordering::Relaxed));
        let w = f32::from_bits(self.w.load(Ordering::Relaxed));

        Quat::from_xyzw(x, y, z, w)
    }
}

#[derive(Resource)]
pub struct RotationResource(pub Arc<AtomicQuat>);
