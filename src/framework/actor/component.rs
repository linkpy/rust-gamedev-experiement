
#[deny(missing_docs)]

use failure::Error;
use raylib::prelude::*;

use super::context::ActorComponentContext;



/// Component of an actor.
///
pub trait ActorComponent {
    /// Initializes the component.
    ///
    fn initialize(&mut self, ctx: ActorComponentContext) -> Option<Error>;
    /// Loads the component.
    ///
    fn load(&mut self, ctx: ActorComponentContext, rl: &mut RaylibHandle) -> Option<Error>;


    /// Updates the component.
    ///
    fn update(&mut self, ctx: ActorComponentContext, dt: f32, rl: &mut RaylibHandle) -> Option<Error>;
    /// Draws the component.
    ///
    fn draw(&mut self, ctx: ActorComponentContext, rl: &mut RaylibHandle);
}
