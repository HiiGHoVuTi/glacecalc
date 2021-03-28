use std::collections::HashMap;
use std::any::Any;

extern crate piston_window;

extern crate evalexpr;

use piston_window::*;

use evalexpr::eval;

pub trait SceneObject {
	fn render<G: Graphics>(&self, context: &Context, graphics: &mut G, extra: &HashMap<&str, Box<dyn Any + 'static>>);
	fn update(&mut self, e: &Event, extra: &HashMap<&str, Box<dyn Any + 'static>>);
}

pub struct GUIButton {
	pub coords: [f64 ; 4],
	pub color: [f32 ; 4],
	pub clicked: bool,
	pub id: String,
}
impl SceneObject for GUIButton {
	fn render<G: Graphics>(&self, context: &Context, graphics: &mut G, extra: &HashMap<&str, Box<dyn Any + 'static>>) {
		rectangle(self.color, self.coords, context.transform, graphics);
	}
	fn update(&mut self, e: &Event, obj2: &HashMap<&str, Box<dyn Any + 'static>>) {
		let clicked = obj2.get("clicked").unwrap().downcast_ref::<bool>().unwrap();
		let mouse_pos = obj2.get("mouse_pos").unwrap().downcast_ref::<Option<[f64 ; 2]>>().unwrap();
		if mouse_pos.is_none() {
			return();
		}
		let actual_pos = mouse_pos.unwrap();
		if actual_pos[0] > self.coords[0] && actual_pos[0] < self.coords[0] + self.coords[2] && actual_pos[1] > self.coords[1] && actual_pos[1] < self.coords[1] + self.coords[3] {
			self.clicked = *(clicked);
		}
	}
}

pub struct Calculator {
	pub buttons: [Box<GUIButton> ; 17],
	pub buffer: String,
}
impl Calculator {
	fn evaluate(&self) -> String {
		let res = eval(&self.buffer);
		if let Ok(val) = res {
			return val.to_string();
		}
		"Error".to_string()
	}
}
impl SceneObject for Calculator {
	fn render<G: Graphics>(&self, context: &Context, graphics: &mut G, extra: &HashMap<&str, Box<dyn Any + 'static>>) {
		for button in self.buttons.iter() {
			button.render(context, graphics, &({
	let mut object: HashMap<&str, Box<dyn Any + 'static>> = HashMap::new();
	object
}));
		}
	}
	fn update(&mut self, e: &Event, extra: &HashMap<&str, Box<dyn Any + 'static>>) {
		let mut will_update: bool = false;
		for button in self.buttons.iter_mut() {
			button.update(e, extra);
			if button.clicked {
				if button.id == "=".to_string() {
					will_update = true;
				}
				else if button.id == "del".to_string() {
					self.buffer.pop();
				}
				else{
self.buffer.push_str(&(button.id.to_string()));

				}
				println!("{:#?}", self.buffer);
			}
		}
		if will_update {
			self.buffer = self.evaluate();
		}
	}
}