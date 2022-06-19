use bevy::math::Vec3;

/// Animate a vector (pos) towards a goal with a simplified velocity simulation
///   Used by a CameraModel
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AnimVect {
    /// Where we are now (for instance a camera, or an aim position)
    pos: Vec3,
    /// How fast we are moving per second (from pos, hopefully towards goal)
    vel: Vec3,
    /// Goal location
    goal: Vec3,

    /// A kind of "desired move time" in seconds.
    ///
    /// Try to adjust acceleration to reach goal in this time.
    move_time: f32,

    /// Increases each tick we are accelerating and decreases again after, to smooth sudden
    ///   acceleration
    accel_multiple: f32,
    /// Absolute upper limit, beyond which we will teleport pos towards goal on next update.
    max_distance: f32,
}

/// A dynamic blending routine for vectors from Pos -> Goal with a somewhat physical feel
///
/// Simulates blending using a dynamic acceleration that increases over time to try to deliver
///   a faster or slower movement blend
impl AnimVect {
    pub fn new(pos: Vec3, move_time: f32, max_distance: f32) -> Self {
        AnimVect {
            pos,
            vel: Vec3::new(0.0, 0.0, 0.0),
            goal: pos,

            move_time,
            accel_multiple: 1.0,
            max_distance,
        }
    }

    pub fn set_goal(&mut self, goal: Vec3) {
        self.goal = goal
    }

    pub fn update(&mut self, _delta: f32) {
        // How far away are we?
        let dist_left = self.goal - self.pos;
        let dist_mag = dist_left.length();
        if dist_mag > self.max_distance {
            // clamp distance (apply self.max_distance)
            //   dist_left * (self.max_distance / dist_mag) applies a normalise and a
            //   "set length to self.max_distance" into one step.
            self.pos = self.goal - dist_left * (self.max_distance / dist_mag);
        } else {
            // How much more do we need to accelerate to reach goal in self.move_time seconds?
            // let delta_req = dist_left / self.move_time;
            // self.vel = vlerp(self.vel, delta_req, 0.7);
            // self.pos += self.vel * delta;
            self.pos = vlerp(self.pos, self.goal, 0.7);
        }
    }

    pub fn pos(&self) -> Vec3 {
        self.pos
    }
}

/// Blend between a and b using the blend factor provided (typically 0.0 .. 1.0)
pub fn vlerp(a: Vec3, b: Vec3, blend: f32) -> Vec3 {
    a * (1.0 - blend) + b * blend
}
