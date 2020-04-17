
#![deny(missing_docs)]


use raylib::prelude::*;
use failure::Error;


/// Represents a scene in the game.
///
pub trait BaseScene {
    /// Initializes the scene.
    ///
    fn initialize(&mut self) -> Option<Error>;
    /// Loads the scene.
    ///
    fn load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Option<Error>;

    /// Updates the scene.
    ///
    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<bool, Error>;
    /// Draws the scene.
    ///
    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);



    /// Runs the scene.
    ///
    fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Option<Error> {
        if let Some(err) = self.initialize() {
            return Some(err);
        }

        if let Some(err) = self.load(rl, thread) {
            return Some(err);
        }

        loop {
            self.draw(rl, thread);
            let res = self.update(rl, thread);

            match res {
                Ok(cont) => {
                    if !cont {
                        break None;
                    }
                },
                Err(err) => {
                    break Some(err);
                }
            }
        }
    }
}
