use std::collections::HashMap;
use std::any::Any;

extern crate piston_window;

extern crate find_folder;

mod components;

use piston_window::*;

use components::{SceneObject, GUIButton, Calculator};

fn main() {
	let mut window: PistonWindow = WindowSettings::new("Hello Calculator", [480, 600]).exit_on_esc(true).build().unwrap();
	let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	let mut glyphs: Glyphs = window.load_font(assets.join("FiraSans-Black.ttf")).unwrap();
	let mut main: Calculator = Calculator{ buttons: [Box::new(GUIButton{ coords: [360.0, 0.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "del".to_string() }), Box::new(GUIButton{ coords: [0.0, 120.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "7".to_string() }), Box::new(GUIButton{ coords: [120.0, 120.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "8".to_string() }), Box::new(GUIButton{ coords: [240.0, 120.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "9".to_string() }), Box::new(GUIButton{ coords: [360.0, 120.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "/".to_string() }), Box::new(GUIButton{ coords: [0.0, 240.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "4".to_string() }), Box::new(GUIButton{ coords: [120.0, 240.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "5".to_string() }), Box::new(GUIButton{ coords: [240.0, 240.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "6".to_string() }), Box::new(GUIButton{ coords: [360.0, 240.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "*".to_string() }), Box::new(GUIButton{ coords: [0.0, 360.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "1".to_string() }), Box::new(GUIButton{ coords: [120.0, 360.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "2".to_string() }), Box::new(GUIButton{ coords: [240.0, 360.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "3".to_string() }), Box::new(GUIButton{ coords: [360.0, 360.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "-".to_string() }), Box::new(GUIButton{ coords: [0.0, 480.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: ".".to_string() }), Box::new(GUIButton{ coords: [120.0, 480.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "0".to_string() }), Box::new(GUIButton{ coords: [240.0, 480.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "=".to_string() }), Box::new(GUIButton{ coords: [360.0, 480.0, 100.0, 100.0], color: [0.65, 0.55, 0.4, 1.0], clicked: false, id: "+".to_string() })], buffer: "".to_string() };
	let mut mouse_pos: Option<[f64 ; 2]> = None;
	let mut click: bool = false;
	while let Some(event) = window.next() {
		if let Some(pos) = event.mouse_cursor_args() {
			mouse_pos = Some(pos);
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
				object.insert("mouse_pos", Box::new(mouse_pos));
				object
			};
			click = false;
			main.update(&(event), &(extra));
		}
		if let Some(_args) = event.render_args() {
			window.draw_2d(&(event), |context, graphics, device| {
								clear([0.45, 0.4, 0.3, 1.0], graphics);
								main.render(&(context), graphics, &({
					let mut object: HashMap<&str, Box<dyn Any + 'static>> = HashMap::new();
					object
				}));
								text::Text::new_color([0.95, 0.9, 0.8, 1.0], 32).draw(&(main.buffer), &mut (glyphs), &(context.draw_state), context.transform.trans(20.0, 52.0), graphics).unwrap();
								for button in main.buttons.iter() {
									text::Text::new_color([0.95, 0.9, 0.8, 1.0], 32).draw(&(button.id), &mut (glyphs), &(context.draw_state), context.transform.trans(button.coords[0] + button.coords[2] / 2.0 - 8.0 * button.id.len() as f64, button.coords[1] + button.coords[3] / 2.0), graphics).unwrap();
								}
								glyphs.factory.encoder.flush(device);
							});
		}
	}
}

