
#![deny(missing_docs)]

use failure::Error;
use raylib::prelude::*;

use super::context::ActorComponentContext;



/// Component of an actor.
///
pub trait ActorComponent<T> {
    /// Initializes the component.
    ///
    fn initialize(&mut self, ctx: ActorComponentContext<T>) -> Option<Error>;
    /// Loads the component.
    ///
    fn load(&mut self, ctx: ActorComponentContext<T>, rl: &mut RaylibHandle) -> Option<Error>;


    /// Updates the component.
    ///
    fn update(&mut self, ctx: ActorComponentContext<T>, dt: f32, rl: &mut RaylibHandle) -> Option<Error>;
    /// Draws the component.
    ///
    fn draw(&mut self, ctx: ActorComponentContext<T>, rl: &mut RaylibHandle);
}
