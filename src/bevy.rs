#[cfg(feature = "bevy")]
use bevy_math::{Quat, Vec3};
#[cfg(feature = "bevy")]
use bevy_transform::components::Transform as BevyTransform;
#[cfg(feature = "bevy")]
use crate::transform::Transform as DollyTransform;
#[cfg(feature = "bevy")]
use crate::handedness::RightHanded;

#[cfg(feature = "bevy")]
impl From<DollyTransform<RightHanded>> for BevyTransform {
    fn from(dolly_transform: DollyTransform<RightHanded>) -> Self {
        BevyTransform {
            translation: Vec3::new(
                dolly_transform.position.x,
                dolly_transform.position.y,
                dolly_transform.position.z,
            ),
            rotation: Quat::from_xyzw(
                dolly_transform.rotation.v.x,
                dolly_transform.rotation.v.y,
                dolly_transform.rotation.v.z,
                dolly_transform.rotation.s,
            ),
            scale: Vec3::ONE,
        }
    }
}