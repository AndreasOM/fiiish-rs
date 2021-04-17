
use crate::math::Vector2;
use crate::renderer::{
//	Color,
	Renderer,
//	Texture,
};

use crate::ui::{
	UiElement,
	UiElementBase,
	UiElementFadeState,
	UiRenderer,
};

#[derive(Debug)]
struct Child {
	pub element: Box< dyn UiElement >,
	pub gravity: Vector2,
}

#[derive(Debug)]
pub struct UiGravityBox {
	base: UiElementBase,
	padding: f32,
	children: Vec< Child >,
	fade_state: UiElementFadeState,
}

impl UiGravityBox {
	pub fn new( ) -> Self {
		Self {
			base: UiElementBase::new(),
			padding: 0.0,
			children: Vec::new(),
			fade_state: UiElementFadeState::FadedIn,
		}
	}

	pub fn set_size( &mut self, size: &Vector2 ) {
		self.borrow_base_mut().size = *size;
	}
	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
	}

	pub fn add_child( &mut self, element: Box< dyn UiElement >, gravity: &Vector2 ) {
		self.children.push( Child { element, gravity: *gravity } );
	}
}

impl UiElement for UiGravityBox {
	fn update( &mut self, time_step: f64 ) {
		self.update_fade_state( time_step );
		for c in self.children.iter_mut() {
			c.element.update( time_step );
		}
	}
	fn render( &self, ui_renderer: &mut UiRenderer) {
//		dbg!(&self.fade_state);
		if self.fade_state != UiElementFadeState::FadedOut {
			ui_renderer.push_translation( &self.borrow_base().pos );
			let l = self.get_fade_level();
			ui_renderer.push_opacity( l );
			for c in self.children.iter() {
				c.element.render( ui_renderer );
			}
			ui_renderer.pop_opacity();
			ui_renderer.pop_transform();
		}
	}
	fn layout( &mut self, pos: &Vector2 ) {
		let ws = self.borrow_base().size.sub( &Vector2::new( 2.0*self.padding, 2.0*self.padding ) );
		for c in self.children.iter_mut() {
			let cs = c.element.size();
			let cpos = ws.sub( &cs ).scaled( 0.5 ).scaled_vector2( &c.gravity );
			c.element.layout( &cpos );
		}

		self.borrow_base_mut().pos = *pos;
	}

	fn borrow_base( &self ) -> &UiElementBase {
		&self.base
	}

	fn borrow_base_mut( &mut self ) -> &mut UiElementBase {
		&mut self.base
	}

	fn fade_state( &self ) -> &UiElementFadeState {
		&self.fade_state
	}
	fn set_fade_state( &mut self, fade_state: &UiElementFadeState ) {
		self.fade_state = *fade_state;
	}


}
