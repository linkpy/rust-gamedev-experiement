
#![deny(missing_docs)]

use failure::Error;
use raylib::prelude::*;

use super::controls::ActorControls;
use super::component::ActorComponent;
use super::context::ActorComponentContext;



/// Struct holding an actor and its component.
pub struct ActorHolder<T> {
    pub data: T,

    controls: ActorControls<T>,
    components: Vec<Box<dyn ActorComponent<T>>>
}


impl<T> ActorHolder<T> {
    /// Creates a new holder.
    ///
    pub fn new(data: T) -> ActorHolder<T> {
        ActorHolder::with_capacity(data, 0)
    }
    /// Creates a new holder with `cap` preallocated components.
    ///
    pub fn with_capacity(data: T, cap: usize) -> ActorHolder<T> {
        ActorHolder {
            data: data,
            components: Vec::with_capacity(cap),
            controls: ActorControls::new(),
        }
    }



    /// Adds a new component to the actor.
    ///
    pub fn add_component(&mut self, ac: impl ActorComponent<T> + 'static) {
        self.components.push(Box::new(ac));
    }
    /// Removes the component at the given index.
    ///
    pub fn remove_component(&mut self, idx: usize) {
        self.components.remove(idx);
    }



    /// Initializes the actor's components.
    ///
    pub fn initialize(&mut self) -> Option<Error> {
        for (idx, c) in self.components.iter_mut().enumerate() {
            let ctx = ActorComponentContext::new(&mut self.data, &mut self.controls, idx);
            if let Some(err) = c.initialize(ctx) {
                return Some(err);
            }
        }

        None
    }
    /// Loads the actor's components.
    ///
    pub fn load(&mut self, rl: &mut RaylibHandle) -> Option<Error> {
        for (idx, c) in self.components.iter_mut().enumerate() {
            let ctx = ActorComponentContext::new(&mut self.data, &mut self.controls, idx);
            if let Some(err) = c.load(ctx, rl) {
                return Some(err);
            }
        }

        None
    }
    /// Updates the actor's components.
    ///
    pub fn update(&mut self, dt: f32, rl: &mut RaylibHandle) -> Option<Error> {
        for (idx, c) in self.components.iter_mut().enumerate() {
            let ctx = ActorComponentContext::new(&mut self.data, &mut self.controls, idx);
            if let Some(err) = c.update(ctx, dt, rl) {
                return Some(err);
            }
        }

        None
    }
    /// Draws the actor's components.
    ///
    pub fn draw(&mut self, rl: &mut RaylibHandle) {
        for (idx, c) in self.components.iter_mut().enumerate() {
            let ctx = ActorComponentContext::new(&mut self.data, &mut self.controls, idx);
            c.draw(ctx, rl);
        }
    }



    /// Makes the actor's controls to take effect.
    ///
    pub fn execute_controls(&mut self) {
        for (cnt, idx) in self.controls.get_queued_component_removals().enumerate() {
            self.components.remove(idx - cnt);
        }

        for ac in self.controls.get_queued_components() {
            self.components.push(ac);
        }

        // TODO: support queued_for_deletion.
    }

}
