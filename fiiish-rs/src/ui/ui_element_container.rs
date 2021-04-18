
use crate::DebugRenderer;
use crate::math::Vector2;
use crate::renderer::Color;
use crate::ui::{
	UiElement,
	UiElementFadeData,
	UiElementFadeState,
	UiRenderer,
};

#[derive(Debug)]
pub struct UiElementContainerData {
	pub name: String,
	pub pos: Vector2,
	pub size: Vector2,
	pub fade_state: UiElementFadeState,
	pub children: Vec< UiElementContainer >,
}

impl UiElementContainerData {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			pos: Vector2::zero(),
			size: Vector2::zero(),
			fade_state: UiElementFadeState::FadedIn,
			children: Vec::new(),			
		}		
	}
	pub fn set_size( &mut self, size: &Vector2 ) {
		self.size = *size;
	}
	pub fn set_pos( &mut self, pos: &Vector2 ) {
		self.pos = *pos;
	}
	pub fn borrow_children( &self ) -> &Vec< UiElementContainer > {
		&self.children
	}
	pub fn borrow_children_mut( &mut self ) -> &mut Vec< UiElementContainer > {
		&mut self.children
	}
	pub fn fade_state( &self ) -> &UiElementFadeState {
		&self.fade_state
	}
	pub fn get_fade_level( &self ) -> f32 {
		let fs = self.fade_state;
		match fs {
			UiElementFadeState::FadedOut => 0.0,
			UiElementFadeState::FadedIn => 1.0,
			UiElementFadeState::FadingIn( d ) => d.level,
			UiElementFadeState::FadingOut( d ) => d.level,
		}
	}

}

#[derive(Debug)]
pub struct UiElementContainer {
	element: Box< dyn UiElement >,
	data: UiElementContainerData,
}

impl UiElementContainer {
	pub fn new( element: Box< dyn UiElement > ) -> Self {
		let mut data = UiElementContainerData::new();
		if let Some( size ) = element.preferred_size() {
//			println!("{:?} has a preferred size of {:?}", &element, &size );
			data.set_size( size );
		}
		Self {			
			element,
			data,
		}
	}

	pub fn update( &mut self, time_step: f64 ) {
		self.element.update( time_step );
		self.update_fade_state( time_step );
		for c in self.data.children.iter_mut() {
			c.update( time_step );
		}
	}

	pub fn render( &self, ui_renderer: &mut UiRenderer) {
		self.element.render( &self.data, ui_renderer );
		self.render_children( ui_renderer );
	}

	pub fn render_children( &self, ui_renderer: &mut UiRenderer ) {
		if self.data.fade_state != UiElementFadeState::FadedOut {
			ui_renderer.push_translation( &self.data.pos );
			let l = self.get_fade_level();
			ui_renderer.push_opacity( l );
			for c in self.data.children.iter() {
				c.render( ui_renderer );
			}
			ui_renderer.pop_opacity();
			ui_renderer.pop_transform();
		}		
	}

	pub fn fade_state( &self ) -> &UiElementFadeState {
		&self.data.fade_state
	}
	pub fn set_fade_state( &mut self, fade_state: &UiElementFadeState ) {
		self.data.fade_state = *fade_state;
	}

	pub fn fade_in( &mut self, duration: f32 ) {
		let fs = self.fade_state();
		if duration == 0.0 {
			self.set_fade_state( &UiElementFadeState::FadedIn );
		} else {
			let speed = 1.0/duration;
			match fs {
				UiElementFadeState::FadedIn => (),
				UiElementFadeState::FadedOut => {
					let fs = UiElementFadeState::FadingIn( UiElementFadeData{ level: 0.0, speed } );
					self.data.fade_state = fs;
				},
				UiElementFadeState::FadingIn( d ) => {
					let fs = UiElementFadeState::FadingIn( UiElementFadeData{ level: d.level, speed } );
					self.data.fade_state = fs;
				},
				UiElementFadeState::FadingOut( d ) => {
					let fs = UiElementFadeState::FadingIn( UiElementFadeData{ level: d.level, speed } );
					self.data.fade_state = fs;
				}
			}
		}
	}
	pub fn fade_out( &mut self, duration: f32 ) {
		let fs = self.fade_state();
		if duration == 0.0 {
			self.set_fade_state( &UiElementFadeState::FadedOut );
		} else {
			let speed = 1.0/duration;
			match fs {
				UiElementFadeState::FadedOut => (),
				UiElementFadeState::FadedIn => {
					let fs = UiElementFadeState::FadingOut( UiElementFadeData{ level: 1.0, speed } );
					self.data.fade_state = fs;
				},
				UiElementFadeState::FadingIn( d ) => {
					let fs = UiElementFadeState::FadingOut( UiElementFadeData{ level: d.level, speed } );
					self.data.fade_state = fs;
				},
				UiElementFadeState::FadingOut( d ) => {
					let fs = UiElementFadeState::FadingOut( UiElementFadeData{ level: d.level, speed } );
					self.data.fade_state = fs;
				}
			}
		}
	}
	fn update_fade_state( &mut self, time_step: f64 ) {
		let fs = self.data.fade_state;
		match fs {
			UiElementFadeState::FadedOut => (),
			UiElementFadeState::FadedIn => (),
			UiElementFadeState::FadingIn( d ) => {
				let new_level = d.level + d.speed * time_step as f32;
				if new_level < 1.0 {
					let fs = UiElementFadeState::FadingIn( UiElementFadeData{ level: new_level, speed: d.speed } );
					self.data.fade_state = fs;					
				} else {
					self.data.fade_state = UiElementFadeState::FadedIn;
				}
			}
			UiElementFadeState::FadingOut( d ) => {
				let new_level = d.level - d.speed * time_step as f32;
				if new_level > 0.0 {
					let fs = UiElementFadeState::FadingOut( UiElementFadeData{ level: new_level, speed: d.speed } );
					self.data.fade_state = fs;
				} else {
					self.data.fade_state = UiElementFadeState::FadedOut;
				}
			}
		}
	}

	pub fn get_fade_level( &self ) -> f32 {
		self.data.get_fade_level()
	}


	pub fn render_debug( &self, debug_renderer: &mut DebugRenderer, offset: &Vector2 ) {
		self.element.render_debug( &self.data, debug_renderer, offset );
		for c in self.data.borrow_children().iter() {
			let co = offset.add( c.pos() );
			c.render_debug( debug_renderer, &co );
		}
		debug_renderer.add_line( &Vector2::zero(), &Vector2::zero().add( &offset ), 3.0, &Color::white() );
	}

	pub fn dump_info( &self, indent: &str, offset: &Vector2 ) {
		println!("{} {}: {},{} {},{}", indent, &self.data.name, self.pos().x, self.pos().y, self.size().x, self.size().y );
		let new_indent = format!("{}  ", indent);
		for c in self.data.borrow_children().iter() {
			let co = offset;//.add( c.pos() );
			c.dump_info( &new_indent, &co );
		}
	}




	pub fn borrow_element( &self ) -> &Box< dyn UiElement > {
		&self.element
	}
	pub fn borrow_element_mut( &mut self ) -> &mut Box< dyn UiElement > {
		&mut self.element
	}

	pub fn borrow_children( &self ) -> &Vec< UiElementContainer > {
		&self.data.children
	}

	pub fn borrow_children_mut( &mut self ) -> &mut Vec< UiElementContainer > {
		&mut self.data.children
	}

	pub fn add_child( &mut self, mut child: UiElementContainer ) -> &mut UiElementContainer {
		self.element.add_child( &mut child.data );
		self.data.children.push( child );
		self.element.recalculate_size( &mut self.data );
		let last = self.data.children.len() - 1;
		&mut self.data.children[ last ]
	}

	pub fn add_child_element( &mut self, element: impl UiElement + 'static ) -> &mut UiElementContainer {
		self.add_child( UiElementContainer::new( Box::new( element ) ) )
	}

	pub fn layout( &mut self, pos: &Vector2 ) {
		self.element.layout( &mut self.data, pos );
		self.data.pos = *pos;
	}

	pub fn size( &self ) -> &Vector2 {
		&self.data.size
	}
	pub fn set_size( &mut self, size: &Vector2 ) {
//		self.element.set_size( size );
		self.data.size = *size;
	}

	pub fn name( &self ) -> &str {
		&self.data.name
	}
	pub fn set_name( &mut self, name: &str ) {
		self.data.name = name.to_owned();
	}

	pub fn pos( &self ) -> &Vector2 {
		&self.data.pos
	}
	pub fn set_pos( &mut self, pos: &Vector2 ) {
		self.data.pos = *pos;
	}

}