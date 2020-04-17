
#![deny(missing_docs)]

use super::controls::ActorControls;


/// A component's context. Holds multiple information about a component.
///
pub struct ActorComponentContext<'a, T> {
    /// Actor owning the component.
    pub actor: &'a mut T,
    /// Controls of the actor.
    pub controls: &'a mut ActorControls<T>,

    /// Index of the component.
    index: usize,
}


impl<T> ActorComponentContext<'_, T> {
    /// Creates a new context.
    ///
    pub fn new<'a>(actor: &'a mut T, ctrl: &'a mut ActorControls<T>, idx: usize) -> ActorComponentContext<'a, T> {
        ActorComponentContext {
            actor: actor,
            controls: ctrl,
            index: idx
        }
    }


    /// Gets the component's index.
    ///
    pub fn component_index(&self) -> usize {
        return self.index;
    }
}
