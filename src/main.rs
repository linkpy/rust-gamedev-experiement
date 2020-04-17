
mod framework;

use raylib::prelude::*;
use failure::Error;

use framework::prelude::*;

struct TestScene;


impl BareScene for TestScene {
	fn initialize(&mut self) -> Option<Error> {
		None
	}

	fn load(&mut self, _rl: &mut RaylibHandle, _thread: &RaylibThread) -> Option<Error> {
		None
	}

	fn update(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread) -> Result<bool, Error> {
		Ok(!rl.window_should_close())
	}

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
		let mut d = rl.begin_drawing(&thread);

		d.clear_background(Color::RAYWHITE);
		d.draw_circle_v(Vector2::new(100.0, 100.0), 50.0, Color::MAROON);
	}
}


fn main() {
	let (mut rl, thread) = raylib::init()
		.size(1260, 728)
		.title("yolo")
		.build();

	rl.set_target_fps(60);

	let mut scene = TestScene{};

	if let Some(err) = scene.run(&mut rl, &thread) {
		panic!(err);
	}
}
