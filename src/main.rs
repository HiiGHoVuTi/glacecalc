use std::collections::HashMap;
use std::any::Any;

extern crate piston_window;

mod components;

use piston_window::*;

use components::{SceneObject, Button};

fn main() {
	let mut window: PistonWindow = WindowSettings::new("Hello Calculator", [640, 480]).exit_on_esc(true).build().unwrap();
	let buttons: Vec<Box<Button>> = vec![Box::new(Button{ coords: [0.0, 0.0, 100.0, 100.0], color: [1.0, 0.0, 0.0, 1.0] }), Box::new(Button{ coords: [0.0, 150.0, 100.0, 100.0], color: [1.0, 0.0, 0.0, 1.0] })];
	while let Some(event) = window.next() {
		window.draw_2d(&(event), |context, graphics, device| {
						clear([0.2, 0.2, 0.2, 1.0], graphics);
						for button in buttons.iter() {
							button.update(&(event));
							button.render(&(context), graphics);
						}
					});
	}
}

