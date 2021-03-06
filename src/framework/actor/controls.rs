

#![deny(missing_docs)]

use std::vec::Drain;

use super::component::ActorComponent;



/// Object controlling the overal actor.
///
pub struct ActorControls<T> {
    /// List of component to add to the actor.
    components_to_add: Vec<Box<dyn ActorComponent<T>>>,
    /// List of component indexes to remove from the actor.
    components_to_remove: Vec<usize>,

    /// If true, the actor will be removed from the scene owning it.
    queued_for_deletion: bool,
}

impl<T> ActorControls<T> {

    /// Creates a new instance.
    ///
    pub fn new() -> ActorControls<T> {
        ActorControls {
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
            queued_for_deletion: false,
        }
    }



    /// Queues the addition of the given component to the actor.
    ///
    pub fn queue_component(&mut self, ac: impl ActorComponent<T> + 'static) {
        self.components_to_add.push(Box::new(ac));
    }
    /// Queues the deletion of the given component from the actor.
    ///
    pub fn queue_component_removal(&mut self, idx: usize) {
        self.components_to_remove.push(idx);
    }


    /// Gets the components queued for addition to the actor.
    ///
    pub fn get_queued_components(&mut self) -> Drain<Box<dyn ActorComponent<T>>> {
        self.components_to_add.drain(..)
    }
    /// Gets the components queued for removal from the actor.
    ///
    pub fn get_queued_component_removals(&mut self) -> Drain<usize> {
        self.components_to_remove.sort_unstable();
        self.components_to_remove.dedup();
        self.components_to_remove.drain(..)
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
