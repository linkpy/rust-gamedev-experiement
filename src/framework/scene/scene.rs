
#![deny(missing_docs)]


use raylib::prelude::*;
use failure::Error;

use super::holder::SceneData;


/// Trait used by actor-based scenes.
///
pub trait Scene<T> {
    /// Initializes the scene.
    ///
    fn initialize(&mut self, sh: &mut SceneData<T>) -> Option<Error>;
    /// Loads the scene.
    ///
    fn load(&mut self, sh: &mut SceneData<T>, rl: &mut RaylibHandle) -> Option<Error>;

    /// Updates the scene.
    ///
    fn update(&mut self, sh: &mut SceneData<T>, rl: &mut RaylibHandle) -> Result<bool, Error>;
    /// Draws the scene.
    ///
    fn draw(&mut self, sh: &mut SceneData<T>, l: &mut RaylibHandle);
}
