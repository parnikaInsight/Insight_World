use crate::math::Vect;
use bevy::{prelude::*, reflect::FromReflect};
use rapier::prelude::{
    Isometry, LockedAxes as RapierLockedAxes, RigidBodyActivation, RigidBodyHandle, RigidBodyType,
};

/// The Rapier handle of a rigid-body that was inserted to the physics scene.
#[derive(Copy, Clone, Debug, Component)]
pub struct RapierRigidBodyHandle(pub RigidBodyHandle);

/// A rigid-body.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub enum RigidBody {
    /// A `RigidBody::Dynamic` body can be affected by all external forces.
    Dynamic,
    /// A `RigidBody::Fixed` body cannot be affected by external forces.
    Fixed,
    /// A `RigidBody::KinematicPositionBased` body cannot be affected by any external forces but can be controlled
    /// by the user at the position level while keeping realistic one-way interaction with dynamic bodies.
    ///
    /// One-way interaction means that a kinematic body can push a dynamic body, but a kinematic body
    /// cannot be pushed by anything. In other words, the trajectory of a kinematic body can only be
    /// modified by the user and is independent from any contact or joint it is involved in.
    KinematicPositionBased,
    /// A `RigidBody::KinematicVelocityBased` body cannot be affected by any external forces but can be controlled
    /// by the user at the velocity level while keeping realistic one-way interaction with dynamic bodies.
    ///
    /// One-way interaction means that a kinematic body can push a dynamic body, but a kinematic body
    /// cannot be pushed by anything. In other words, the trajectory of a kinematic body can only be
    /// modified by the user and is independent from any contact or joint it is involved in.
    KinematicVelocityBased,
}

impl Default for RigidBody {
    fn default() -> Self {
        RigidBody::Dynamic
    }
}

impl From<RigidBody> for RigidBodyType {
    fn from(rigid_body: RigidBody) -> RigidBodyType {
        match rigid_body {
            RigidBody::Dynamic => RigidBodyType::Dynamic,
            RigidBody::Fixed => RigidBodyType::Fixed,
            RigidBody::KinematicPositionBased => RigidBodyType::KinematicPositionBased,
            RigidBody::KinematicVelocityBased => RigidBodyType::KinematicVelocityBased,
        }
    }
}

/// The velocity of a rigid-body.
///
/// Use this component to control and/or read the velocity of a dynamic or kinematic rigid-body.
/// If this component isn’t present, a dynamic rigid-body will still be able to move (you will just
/// not be able to read/modify its velocity).
#[derive(Copy, Clone, Debug, Default, PartialEq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct Velocity {
    /// The linear velocity of the rigid-body.
    pub linvel: Vect,
    /// The angular velocity of the rigid-body.
    #[cfg(feature = "dim2")]
    pub angvel: f32,
    /// The angular velocity of the rigid-body.
    #[cfg(feature = "dim3")]
    pub angvel: Vect,
}

impl Velocity {
    /// Initialize a velocity set to zero.
    pub fn zero() -> Self {
        Self::default()
    }

    /// Initialize a velocity with the given linear velocity, and an angular velocity of zero.
    pub fn linear(linvel: Vect) -> Self {
        Self {
            linvel,
            ..Self::default()
        }
    }

    /// Initialize a velocity with the given angular velocity, and a linear velocity of zero.
    #[cfg(feature = "dim2")]
    pub fn angular(angvel: f32) -> Self {
        Self {
            angvel,
            ..Self::default()
        }
    }

    /// Initialize a velocity with the given angular velocity, and a linear velocity of zero.
    #[cfg(feature = "dim3")]
    pub fn angular(angvel: Vect) -> Self {
        Self {
            angvel,
            ..Self::default()
        }
    }
}

/// Mass-properties of a rigid-body, added to the contributions of its attached colliders.
#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub enum AdditionalMassProperties {
    /// This mass will be added to the rigid-body. The rigid-body’s total
    /// angular inertia tensor (obtained from its attached colliders) will
    /// be scaled accordingly.
    Mass(f32),
    /// These mass properties will be added to the rigid-body.
    MassProperties(MassProperties),
}

impl Default for AdditionalMassProperties {
    fn default() -> Self {
        Self::MassProperties(MassProperties::default())
    }
}

/// Center-of-mass, mass, and angular inertia.
///
/// When this is used as a component, this lets you read the total mass properties of
/// a rigid-body (including the colliders contribution). Modifying this component won’t
/// affect the mass-properties of the rigid-body (the attached colliders’ `ColliderMassProperties`
/// and the `AdditionalMassProperties` should be modified instead).
#[derive(Copy, Clone, Debug, Default, PartialEq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct ReadMassProperties(pub MassProperties);

/// Center-of-mass, mass, and angular inertia.
///
/// This cannot be used as a component. Use the components `ReadMassProperties` to read a rigid-body’s
/// mass-properties or `AdditionalMassProperties` to set its additional mass-properties.
#[derive(Copy, Clone, Debug, Default, PartialEq, Reflect, FromReflect)]
#[reflect(PartialEq)]
pub struct MassProperties {
    /// The center of mass of a rigid-body expressed in its local-space.
    pub local_center_of_mass: Vect,
    /// The mass of a rigid-body.
    pub mass: f32,
    /// The principal angular inertia of the rigid-body.
    #[cfg(feature = "dim2")]
    pub principal_inertia: f32,
    /// The principal vectors of the local angular inertia tensor of the rigid-body.
    #[cfg(feature = "dim3")]
    pub principal_inertia_local_frame: crate::math::Rot,
    /// The principal angular inertia of the rigid-body.
    #[cfg(feature = "dim3")]
    pub principal_inertia: Vect,
}

impl MassProperties {
    /// Converts these mass-properties to Rapier’s `MassProperties` structure.
    #[cfg(feature = "dim2")]
    pub fn into_rapier(self, physics_scale: f32) -> rapier::dynamics::MassProperties {
        rapier::dynamics::MassProperties::new(
            (self.local_center_of_mass / physics_scale).into(),
            self.mass,
            #[allow(clippy::useless_conversion)] // Need to convert if dim3 enabled
            (self.principal_inertia / (physics_scale * physics_scale)).into(),
        )
    }

    /// Converts these mass-properties to Rapier’s `MassProperties` structure.
    #[cfg(feature = "dim3")]
    pub fn into_rapier(self, physics_scale: f32) -> rapier::dynamics::MassProperties {
        rapier::dynamics::MassProperties::with_principal_inertia_frame(
            (self.local_center_of_mass / physics_scale).into(),
            self.mass,
            (self.principal_inertia / (physics_scale * physics_scale)).into(),
            self.principal_inertia_local_frame.into(),
        )
    }

    /// Converts Rapier’s `MassProperties` structure to `Self`.
    pub fn from_rapier(mprops: rapier::dynamics::MassProperties, physics_scale: f32) -> Self {
        #[allow(clippy::useless_conversion)] // Need to convert if dim3 enabled
        Self {
            mass: mprops.mass(),
            local_center_of_mass: (mprops.local_com * physics_scale).into(),
            principal_inertia: (mprops.principal_inertia() * (physics_scale * physics_scale))
                .into(),
            #[cfg(feature = "dim3")]
            principal_inertia_local_frame: mprops.principal_inertia_local_frame.into(),
        }
    }
}

bitflags::bitflags! {
    #[derive(Default, Component, Reflect, FromReflect)]
    #[reflect(Component, PartialEq)]
    /// Flags affecting the behavior of the constraints solver for a given contact manifold.
    pub struct LockedAxes: u8 {
        /// Flag indicating that the rigid-body cannot translate along the `X` axis.
        const TRANSLATION_LOCKED_X = 1 << 0;
        /// Flag indicating that the rigid-body cannot translate along the `Y` axis.
        const TRANSLATION_LOCKED_Y = 1 << 1;
        /// Flag indicating that the rigid-body cannot translate along the `Z` axis.
        const TRANSLATION_LOCKED_Z = 1 << 2;
        /// Flag indicating that the rigid-body cannot translate along any direction.
        const TRANSLATION_LOCKED = Self::TRANSLATION_LOCKED_X.bits | Self::TRANSLATION_LOCKED_Y.bits | Self::TRANSLATION_LOCKED_Z.bits;
        /// Flag indicating that the rigid-body cannot rotate along the `X` axis.
        const ROTATION_LOCKED_X = 1 << 3;
        /// Flag indicating that the rigid-body cannot rotate along the `Y` axis.
        const ROTATION_LOCKED_Y = 1 << 4;
        /// Flag indicating that the rigid-body cannot rotate along the `Z` axis.
        const ROTATION_LOCKED_Z = 1 << 5;
        /// Combination of flags indicating that the rigid-body cannot rotate along any axis.
        const ROTATION_LOCKED = Self::ROTATION_LOCKED_X.bits | Self::ROTATION_LOCKED_Y.bits | Self::ROTATION_LOCKED_Z.bits;
    }
}

impl From<LockedAxes> for RapierLockedAxes {
    fn from(locked_axes: LockedAxes) -> RapierLockedAxes {
        RapierLockedAxes::from_bits(locked_axes.bits()).expect("Internal conversion error.")
    }
}

/// Constant external forces applied continuously to a rigid-body.
///
/// This force is applied at each timestep.
#[derive(Copy, Clone, Debug, Default, PartialEq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct ExternalForce {
    /// The linear force applied to the rigid-body.
    pub force: Vect,
    /// The angular torque applied to the rigid-body.
    #[cfg(feature = "dim2")]
    pub torque: f32,
    /// The angular torque applied to the rigid-body.
    #[cfg(feature = "dim3")]
    pub torque: Vect,
}

/// Instantaneous external impulse applied continuously to a rigid-body.
///
/// The impulse is only applied once, and whenever it it modified (based
/// on Bevy’s change detection).
#[derive(Copy, Clone, Debug, Default, PartialEq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct ExternalImpulse {
    /// The linear impulse applied to the rigid-body.
    pub impulse: Vect,
    /// The angular impulse applied to the rigid-body.
    #[cfg(feature = "dim2")]
    pub torque_impulse: f32,
    /// The angular impulse applied to the rigid-body.
    #[cfg(feature = "dim3")]
    pub torque_impulse: Vect,
}

/// Gravity is multiplied by this scaling factor before it's
/// applied to this rigid-body.
#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct GravityScale(pub f32);

impl Default for GravityScale {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Information used for Continuous-Collision-Detection.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct Ccd {
    /// Is CCD enabled for this rigid-body?
    pub enabled: bool,
}

impl Ccd {
    /// Enable CCD for a rigid-body.
    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    /// Disable CCD for a rigid-body.
    ///
    /// Note that a rigid-body without the Ccd component attached
    /// has CCD disabled by default.
    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

/// The dominance groups of a rigid-body.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct Dominance {
    // FIXME: rename this to `group` (no `s`).
    /// The dominance groups of a rigid-body.
    pub groups: i8,
}

impl Dominance {
    /// Initialize the dominance to the given group.
    pub fn group(group: i8) -> Self {
        Self { groups: group }
    }
}

/// The activation status of a body.
///
/// This controls whether a body is sleeping or not.
/// If the threshold is negative, the body never sleeps.
#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct Sleeping {
    /// The threshold linear velocity bellow which the body can fall asleep.
    pub linear_threshold: f32,
    /// The angular linear velocity bellow which the body can fall asleep.
    pub angular_threshold: f32,
    /// Is this body sleeping?
    pub sleeping: bool,
}

impl Sleeping {
    /// Creates a components that disables sleeping for the associated rigid-body.
    pub fn disabled() -> Self {
        Self {
            linear_threshold: -1.0,
            angular_threshold: -1.0,
            sleeping: false,
        }
    }
}

impl Default for Sleeping {
    fn default() -> Self {
        Self {
            linear_threshold: RigidBodyActivation::default_linear_threshold(),
            angular_threshold: RigidBodyActivation::default_angular_threshold(),
            sleeping: false,
        }
    }
}

/// Damping factors to gradually slow down a rigid-body.
#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect, FromReflect)]
#[reflect(Component, PartialEq)]
pub struct Damping {
    // TODO: rename these to "linear" and "angular"?
    /// Damping factor for gradually slowing down the translational motion of the rigid-body.
    pub linear_damping: f32,
    /// Damping factor for gradually slowing down the angular motion of the rigid-body.
    pub angular_damping: f32,
}

impl Default for Damping {
    fn default() -> Self {
        Self {
            linear_damping: 0.0,
            angular_damping: 0.0,
        }
    }
}

/// If the `TimestepMode::Interpolated` mode is set and this component is present,
/// the associated rigid-body will have its position automatically interpolated
/// between the last two rigid-body positions set by the physics engine.
#[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
pub struct TransformInterpolation {
    /// The starting point of the interpolation.
    pub start: Option<Isometry<f32>>,
    /// The end point of the interpolation.
    pub end: Option<Isometry<f32>>,
}

impl TransformInterpolation {
    /// Interpolates between the start and end positions with `t` in the range `[0..1]`.
    pub fn lerp_slerp(&self, t: f32) -> Option<Isometry<f32>> {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            Some(start.lerp_slerp(&end, t))
        } else {
            None
        }
    }
}
