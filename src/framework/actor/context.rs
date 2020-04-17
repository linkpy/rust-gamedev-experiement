
#![deny(missing_docs)]

use super::actor::Actor;



/// A component's context. Holds multiple information about a component.
///
pub struct ActorComponentContext<'a> {
    /// Actor owning the component.
    actor: &'a mut Actor,

    /// Index of the component.
    component_index: usize,
}


impl ActorComponentContext<'_> {
    /// Creates a new context.
    ///
    pub fn new<'a>(actor: &'a mut Actor, idx: usize) -> ActorComponentContext<'a> {
        ActorComponentContext {
            actor: actor,
            component_index: idx
        }
    }

}
