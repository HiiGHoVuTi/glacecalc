use std::collections::HashMap;
use std::any::Any;

extern crate piston_window;

mod components;

use piston_window::*;

use components::{SceneObject, GUIButton};

fn main() {
	let opengl = OpenGL::V3_2;
	let mut window: PistonWindow = WindowSettings::new("Hello Calculator", [640, 480]).graphics_api(opengl).exit_on_esc(true).build().unwrap();
	let mut buttons: Vec<Box<GUIButton>> = vec![Box::new(GUIButton{ coords: [0.0, 0.0, 100.0, 100.0], color: [1.0, 0.0, 0.0, 1.0], clicked: false }), Box::new(GUIButton{ coords: [0.0, 200.0, 100.0, 100.0], color: [1.0, 0.0, 0.0, 1.0], clicked: false })];
	let mut mousePos: [f64 ; 2] = [-1.0, -1.0];
	let mut mousePressed: bool = false;
	let mut click: bool = false;
	while let Some(event) = window.next() {
		window.draw_2d(&(event), |context, graphics, device| {
						click = false;
						if let Some(pos) = event.mouse_cursor_args() {
							mousePos = pos;
						}
						if let Some(button) = event.press_args() {
							println!("pressed");
							if button == Button::Mouse(MouseButton::Left) {
								mousePressed = true;
							}
						}
						if let Some(button) = event.release_args() {
							if button == Button::Mouse(MouseButton::Left) {
								println!("released");
								if mousePressed {
									click = true;
								}
								mousePressed = false;
							}
						}
						if let Some(args) = event.update_args() {
							let extra = {
								let mut object: HashMap<&str, Box<dyn Any + 'static>> = HashMap::new();
								object.insert("dt", Box::new(0)); //args.dt
								object.insert("clicked", Box::new(click));
								object.insert("mousePos", Box::new(mousePos));
								object
							};
							for button in buttons.get_mut(0) {
								button.update(&(event), &extra);
							}
						}
						clear([0.2, 0.2, 0.2, 1.0], graphics);
						if let Some(_args) = event.render_args() {
							for button in buttons.iter() {
								button.render(&(context), graphics);
							}
						}
					});
	}
}
