use oml_game::math::{Matrix32, Vector2};
use oml_game::renderer::{Color, Renderer};

pub struct UiRenderer<'a> {
	renderer:                    &'a mut Renderer,
	transform_stack:             Vec<Matrix32>,
	transform_mtx:               Matrix32,
	opacity_stack:               Vec<f32>,
	opacity:                     f32,
	color_stack:                 StateStack<Color>,
	textured_render_effect_id:   u16,
	untextured_render_effect_id: u16,
	font_render_effect_id:       u16,
	layer_id:                    u8,
	front_layer_id:              u8,
}

impl<'a> UiRenderer<'a> {
	pub fn new(
		renderer: &'a mut Renderer,
		textured_render_effect_id: u16,
		untextured_render_effect_id: u16,
		font_render_effect_id: u16,
		layer_id: u8,
		front_layer_id: u8,
	) -> Self {
		Self {
			renderer,
			transform_stack: Vec::new(),
			transform_mtx: Matrix32::identity(),
			opacity_stack: Vec::new(),
			opacity: 1.0,
			color_stack: StateStack::new(Color::white()),
			textured_render_effect_id,
			untextured_render_effect_id,
			font_render_effect_id,
			layer_id,
			front_layer_id,
		}
	}

	pub fn push_translation(&mut self, t: &Vector2) {
		self.transform_stack.push(self.transform_mtx);
		self.transform_mtx.pos = self.transform_mtx.pos.add(&t);
	}

	pub fn pop_transform(&mut self) {
		// :TODO: protect against mismatched push/pop
		self.transform_mtx = self.transform_stack.pop().unwrap();
	}

	pub fn push_opacity(&mut self, opacity: f32) {
		self.opacity_stack.push(self.opacity);
		self.opacity *= opacity;
	}

	pub fn pop_opacity(&mut self) {
		self.opacity = self.opacity_stack.pop().unwrap();
	}

	pub fn push_color(&mut self, color: &Color) {
		self.color_stack.push(*color);
	}

	pub fn pop_color(&mut self) {
		self.color_stack.pop();
	}

	pub fn use_texture(&mut self, name: &str) {
		self.renderer.use_texture(name);
	}

	pub fn render_textured_quad(&mut self, pos: &Vector2, size: &Vector2) {
		let transformed_pos = self.transform_mtx.mul_vector2(&pos);
		let mut color = *self.color_stack.top();
		color.a *= self.opacity;
		self.renderer.set_color(&color);
		self.renderer.use_layer(self.layer_id);
		self.renderer.use_effect(self.textured_render_effect_id);
		self.renderer.render_textured_quad(&transformed_pos, size);
	}

	pub fn render_quad(&mut self, pos: &Vector2, size: &Vector2) {
		let transformed_pos = self.transform_mtx.mul_vector2(&pos);
		let mut color = *self.color_stack.top();
		color.a *= self.opacity;
		self.renderer.set_color(&color);
		self.renderer.use_layer(self.front_layer_id);
		self.renderer.use_effect(self.untextured_render_effect_id);
		self.renderer.render_quad(&transformed_pos, size);
	}
	pub fn use_font(&mut self, font_id: u8) {
		self.renderer.use_font(font_id);
	}
	pub fn print(&mut self, pos: &Vector2, size: &Vector2, alignment: &Vector2, text: &str) {
		let transformed_pos = self.transform_mtx.mul_vector2(&pos);
		let mut color = *self.color_stack.top();
		color.a *= self.opacity;
		self.renderer.set_color(&color);
		self.renderer.use_layer(self.front_layer_id);
		self.renderer.use_effect(self.font_render_effect_id);
		self.renderer
			.print(&transformed_pos, &size, &alignment, text);
	}
}

struct StateStack<T> {
	elements: Vec<T>,
}

impl<T> StateStack<T> {
	pub fn new(base: T) -> Self {
		let mut elements = Vec::new();
		elements.push(base);
		Self { elements }
	}

	pub fn top(&self) -> &T {
		let t = self.elements.len() - 1;
		&self.elements[t]
	}
	pub fn push(&mut self, e: T) {
		self.elements.push(e);
	}
	pub fn pop(&mut self) -> T {
		self.elements.pop().unwrap() // we do want a panic here
	}
}
