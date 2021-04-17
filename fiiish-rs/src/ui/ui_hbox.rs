
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
pub struct UiHbox {
	base: UiElementBase,
	padding: f32,
	children: Vec< Box< dyn UiElement > > ,
	fade_state: UiElementFadeState,
}

impl UiHbox {
	pub fn new( ) -> Self {
		Self {
			base: UiElementBase::new(),
			padding: 0.0,
			children: Vec::new(),
			fade_state: UiElementFadeState::FadedIn,
		}
	}

	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
		self.recalculate_size();
	}

	pub fn add_child( &mut self, child: Box< dyn UiElement > ) {
		self.children.push( child );
		self.recalculate_size();
	}

	fn recalculate_size( &mut self ) {
		let mut total_size = Vector2::zero();

		for c in self.children.iter() {
			let cs = c.size();
			total_size.x += cs.x + self.padding;
			if total_size.y < cs.y {
				total_size.y = cs.y;
			}
		}
		total_size.x -= self.padding;

		self.borrow_base_mut().size = total_size;
	}

}

impl UiElement for UiHbox {
	fn update( &mut self, time_step: f64 ) {
		self.update_fade_state( time_step );
		for c in self.children.iter_mut() {
			c.update( time_step );
		}
	}
	fn render( &self, ui_renderer: &mut UiRenderer) {
		if self.fade_state != UiElementFadeState::FadedOut {
			ui_renderer.push_translation( &self.borrow_base().pos );
			let l = self.get_fade_level();
			ui_renderer.push_opacity( l );
			for c in self.children.iter() {
				c.render( ui_renderer );
			}
			ui_renderer.pop_opacity();
			ui_renderer.pop_transform();
		}
	}
	fn layout( &mut self, pos: &Vector2 ) {
		let mut total_size = Vector2::zero();
		let mut c_positions_x = Vec::new();

		let mut w1 = 0.0;
		let mut w0 = 0.0;

		for c in self.children.iter() {
			let cs = c.size();
			total_size.x += cs.x + self.padding;
			if total_size.y < cs.y {
				total_size.y = cs.y;
			}
			w0 = w1;
			w1 = 0.5 * cs.x;
			c_positions_x.push( w0 +  w1 );
		}
		total_size.x -= self.padding;

		c_positions_x.push( 0.0 );

		let mut cpos = Vector2::new( -0.5*total_size.x - self.padding, 0.0 );

		for (i, c ) in self.children.iter_mut().enumerate() {
			let x = c_positions_x[ i ];
			cpos.x += x + self.padding;
			c.layout( &cpos );
		}

		self.borrow_base_mut().pos = *pos;
		self.borrow_base_mut().size = total_size;
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
