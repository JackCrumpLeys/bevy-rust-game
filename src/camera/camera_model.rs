use bevy::math::{Quat, Vec3};

use super::AnimVect;

/// Models a camera which points generally at look_at, but sits at a relative position (relative)
///   which wiggles slightly (holder).
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct CameraModel {
    look_at: AnimVect,
    relative: AnimVect,
    holder: AnimVect,
}

impl CameraModel {
    pub fn new(look: Vec3, relative: Vec3) -> Self {
        CameraModel {
            look_at: AnimVect::new(look, 0.4, 5.0),
            relative: AnimVect::new(relative, 0.2, 5.0),
            holder: AnimVect::new(look + relative, 0.4, 3.0),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.look_at.update(delta);
        self.relative.update(delta);

        self.holder.set_goal(self.look_at.pos() + self.relative.pos());
        self.holder.update(delta);
    }

    pub fn lookat_pos(&self) -> Vec3 {
        self.look_at.pos()
    }

    pub fn abs_camera_pos(&self) -> Vec3 {
        self.holder.pos()
        //self.look_at.pos() + self.relative.pos()
    }

    pub fn rel_camera_pos(&self) -> Vec3 {
        self.holder.pos() - self.look_at.pos()
    }

    pub fn camera_aim(&self) -> Vec3 {
        self.look_at.pos()
    }

    pub fn set_look(&mut self, look: Vec3) {
        self.look_at.set_goal(look);
    }

    pub fn set_relative(&mut self, rel: Vec3) {
        self.relative.set_goal(rel);
    }
}
