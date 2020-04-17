
#![deny(missing_docs)]


use crate::framework::actor::*;


/// Object controlling the scene.
///
pub struct SceneControls<T> {
    actors_to_add: Vec<ActorHolder<T>>,
    actors_to_remove: Vec<usize>,
}
