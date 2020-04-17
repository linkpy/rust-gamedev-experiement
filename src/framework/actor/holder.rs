
#[deny(missing_docs)]

use failure::Error;
use raylib::prelude::*;

use super::actor::Actor;
use super::component::ActorComponent;
use super::context::ActorComponentContext;



/// Struct holding an actor and its component.
pub struct ActorHolder {
    actor: Actor,
    components: Vec<Box<dyn ActorComponent>>
}


impl ActorHolder {
    /// Creates a new holder.
    ///
    pub fn new() -> ActorHolder {
        ActorHolder::with_capacity(0)
    }
    /// Creates a new holder with `cap` preallocated components.
    ///
    pub fn with_capacity(cap: usize) -> ActorHolder {
        ActorHolder {
            actor: Actor::new(),
            components: Vec::with_capacity(cap),
        }
    }



    /// Adds a new component to the actor.
    ///
    pub fn add_component(&mut self, ac: impl ActorComponent + 'static) {
        self.components.push(Box::new(ac));
    }



    /// Initializes the actor's components.
    ///
    pub fn initialize(&mut self) -> Option<Error> {
        for (idx, c) in self.components.iter_mut().enumerate() {
            let ctx = ActorComponentContext::new(&mut self.actor, idx);
            if let Some(err) = c.initialize(ctx) {
                return Some(err);
            }
        }

        None
    }
    /// Loads the actor's components.
    ///
    pub fn load(&mut self, rl: &mut RaylibHandle) -> Option<Error> {
        for (idx, c) in &mut self.components.iter_mut().enumerate() {
            let ctx = ActorComponentContext::new(&mut self.actor, idx);
            if let Some(err) = c.load(ctx, rl) {
                return Some(err);
            }
        }

        None
    }
    /// Updates the actor's components.
    ///
    pub fn update(&mut self, dt: f32, rl: &mut RaylibHandle) -> Option<Error> {
        for (idx, c) in &mut self.components.iter_mut().enumerate() {
            let ctx = ActorComponentContext::new(&mut self.actor, idx);
            if let Some(err) = c.update(ctx, dt, rl) {
                return Some(err);
            }
        }

        None
    }
    /// Draws the actor's components.
    ///
    pub fn draw(&mut self, rl: &mut RaylibHandle) {
        for (idx, c) in &mut self.components.iter_mut().enumerate() {
            let ctx = ActorComponentContext::new(&mut self.actor, idx);
            c.draw(ctx, rl);
        }
    }

}
