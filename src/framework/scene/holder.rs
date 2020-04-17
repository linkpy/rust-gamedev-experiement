
#![deny(missing_docs)]


use raylib::prelude::*;
use failure::Error;

use crate::framework::actor::*;

use super::base_scene::*;
use super::scene::*;
use super::controls::*;



/// Struct holding an actor-based scene.
///
pub struct SceneHolder<T> {
    scene: Box<dyn Scene<T>>,
    data: SceneData<T>,
}

pub struct SceneData<T> {
    controls: SceneControls<T>,
    actors: Vec<ActorHolder<T>>,
}



impl<T> BaseScene for SceneHolder<T> {
    fn initialize(&mut self) -> Option<Error> {
        return self.scene.initialize(&mut self.data);
    }

    fn load(&mut self, rl: &mut RaylibHandle, _: &RaylibThread) -> Option<Error> {
        return self.scene.load(&mut self.data, rl);
    }

    fn draw(&mut self, _: &mut RaylibHandle, _: &RaylibThread) { unimplemented!() }
    fn update(&mut self, _: &mut RaylibHandle, _: &RaylibThread) -> Result<bool, Error> { unimplemented!() }
}
