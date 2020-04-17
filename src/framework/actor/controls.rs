

#[deny(missing_docs)]

use super::component::ActorComponent;



/// Object controlling the overal actor.
///
pub struct ActorControls {
    /// List of component to add to the actor.
    components_to_add: Vec<Box<dyn ActorComponent>>,
    /// List of component indexes to remove from the actor.
    components_to_remove: Vec<usize>,

    /// If true, the actor will be removed from the scene owning it.
    queued_for_deletion: bool,
}

impl ActorControls {

    /// Creates a new instance.
    ///
    pub fn new() -> ActorControls {
        ActorControls {
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
            queued_for_deletion: false,
        }
    }



    /// Queues the addition of the given component to the actor.
    ///
    pub fn add_component(&mut self, ac: impl ActorComponent + 'static) {
        self.components_to_add.push(Box::new(ac));
    }
    /// Queues the deletion of the given component from the actor.
    ///
    pub fn remove_component(&mut self, idx: usize) {
        self.components_to_remove.push(idx);
    }


    /// Check if the actor is queued for deletion.
    ///
    pub fn is_queued_for_deletion(&self) -> bool {
        return self.queued_for_deletion;
    }
    /// Queues the actor for deletion.
    ///
    pub fn queue_for_deletion(&mut self) {
        self.queued_for_deletion = true;
    }

}
