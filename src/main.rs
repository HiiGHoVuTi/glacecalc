use std::collections::HashMap;
use std::any::Any;

extern crate piston_window;

mod components;

use piston_window::*;

use components::{SceneObject, GUIButton, Calculator};

fn main() {
	let mut window: PistonWindow = WindowSettings::new("Hello Calculator", [640, 480]).exit_on_esc(true).build().unwrap();
	let mut main: Calculator = Calculator{ buttons: [Box::new(GUIButton{ coords: [0.0, 0.0, 100.0, 100.0], color: [1.0, 0.0, 0.0, 1.0], clicked: false, id: 0 }), Box::new(GUIButton{ coords: [0.0, 200.0, 100.0, 100.0], color: [1.0, 0.0, 0.0, 1.0], clicked: false, id: 1 })], buffer: "".to_string() };
	let mut mousePos: Option<[f64 ; 2]> = None;
	let mut click: bool = false;
	while let Some(event) = window.next() {
		if let Some(pos) = event.mouse_cursor_args() {
			mousePos = Some(pos);
		}
		if let Some(button) = event.release_args() {
			if button == Button::Mouse(MouseButton::Left) {
				click = true;
			}
		}
		if let Some(args) = event.update_args() {
			let extra = {
				let mut object: HashMap<&str, Box<dyn Any + 'static>> = HashMap::new();
				object.insert("dt", Box::new(args.dt));
				object.insert("clicked", Box::new(click));
				object.insert("mousePos", Box::new(mousePos));
				object
			};
			click = false;
			main.update(&(event), &(extra));
		}
		if let Some(_args) = event.render_args() {
			window.draw_2d(&(event), |context, graphics, device| {
								clear([0.2, 0.2, 0.2, 1.0], graphics);
								main.render(&(context), graphics);
							});
		}
	}
}

