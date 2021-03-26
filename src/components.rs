use std::collections::HashMap;
use std::any::Any;

extern crate piston_window;

use piston_window::*;

pub trait SceneObject {
	fn render<G: Graphics>(&self, context: &Context, graphics: &mut G);
	fn update(&mut self, e: &Event, extra: &HashMap<&str, Box<dyn Any + 'static>>);
}

pub struct GUIButton {
	pub coords: [f64 ; 4],
	pub color: [f32 ; 4],
	pub clicked: bool,
	pub id: i8,
}
impl SceneObject for GUIButton {
	fn render<G: Graphics>(&self, context: &Context, graphics: &mut G) {
		rectangle(self.color, self.coords, context.transform, graphics);
	}
	fn update(&mut self, e: &Event, extra: &HashMap<&str, Box<dyn Any + 'static>>) {
		let clicked = extra.get("clicked").unwrap().downcast_ref::<bool>().unwrap();
		let mousePos = extra.get("mousePos").unwrap().downcast_ref::<Option<[f64 ; 2]>>().unwrap();
		if mousePos.is_none() {
			return();
		}
		let mouse_pos = mousePos.unwrap();
		if mouse_pos[0] > self.coords[0] && mouse_pos[0] < self.coords[0] + self.coords[2] && mouse_pos[1] > self.coords[1] && mouse_pos[1] < self.coords[1] + self.coords[3] {
			self.clicked = *(clicked);
		}
	}
}

pub struct Calculator {
	pub buttons: [Box<GUIButton> ; 2],
	pub buffer: String,
}
impl Calculator {
	fn evaluate(&self) -> String {
		"".to_string()
	}
}
impl SceneObject for Calculator {
	fn render<G: Graphics>(&self, context: &Context, graphics: &mut G) {
		for button in self.buttons.iter() {
			button.render(context, graphics);
		}
	}
	fn update(&mut self, e: &Event, extra: &HashMap<&str, Box<dyn Any + 'static>>) {
		for button in self.buttons.iter_mut() {
			button.update(e, extra);
			if button.clicked {
				println!("{:#?}", button.id);
			}
		}
	}
}

