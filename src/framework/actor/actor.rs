
#![deny(missing_docs)]

use raylib::prelude::*;

use super::controls::ActorControls;
use super::component::ActorComponent;




/// Actor's properties.
pub struct Actor {
    /// Controls.
    pub controls: ActorControls,

    /// Position
    pub position: Vector2,
    /// Rotation
    pub rotation: f32,
    /// Scale
    pub scaling: Vector2,
}

impl Actor {

    /// Creates a new instance.
    ///
    pub fn new() -> Actor {
        Actor {
            controls: ActorControls::new(),
            position: Vector2::zero(),
            rotation: 0.0,
            scaling: Vector2::one(),
        }
    }



    /// Queues the addition of the given component to the actor.
    ///
    pub fn queue_component_addition(&mut self, ac: impl ActorComponent + 'static) {
        self.controls.add_component(ac);
    }
    /// Queues the removal of the given component from the actor.
    ///
    pub fn queue_component_removal(&mut self, idx: usize) {
        self.controls.remove_component(idx);
    }



    /// Checks if the actor is queued for deletion.
    ///
    pub fn is_queue_for_deletion(&self) -> bool {
        self.controls.is_queued_for_deletion()
    }
    /// Queues the actor for deletetion.
    ///
    pub fn queue_for_deletion(&mut self) {
        self.controls.queue_for_deletion();
    }



    /// Moves the actor by a given amount.
    ///
    pub fn offset(&mut self, offset: Vector2) {
        self.position += offset;
    }
    /// Rotates the actor by a given amount.
    ///
    pub fn rotate(&mut self, offset: f32) {
        self.rotation += offset;
    }
    /// Scales the actor by a given factor.
    ///
    pub fn scale(&mut self, factor: Vector2) {
        self.scaling *= factor;
    }
    /// Uniformally scales the actor by a given factor.
    pub fn scale_uniform(&mut self, factor: f32) {
        self.scale(Vector2::new(factor, factor));
    }

}
