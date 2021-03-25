use std::collections::HashMap;
use std::any::Any;

extern crate piston_window;

use piston_window::*;

pub trait SceneObject {
	fn render<G: Graphics>(&self, context: &Context, graphics: &mut G);
	fn update(&self, e: &Event);
}

pub struct Button {
	pub coords: [f64 ; 4],
	pub color: [f32 ; 4],
}
impl SceneObject for Button {
	fn render<G: Graphics>(&self, context: &Context, graphics: &mut G) {
		rectangle(self.color, self.coords, context.transform, graphics);
	}
	fn update(&self, e: &Event) {
	}
}

