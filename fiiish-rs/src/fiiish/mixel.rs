

use crate::math::Vector2;
use crate::renderer::{
	Color,
	Effect,
	Renderer,
	Texture,
};
use crate::system::System;

use crate::window_update_context::WindowUpdateContext;
use super::effect_ids::EffectId;


#[derive(Debug)]
pub struct Mixel {
	color: Color,
	time: f64,
	pos: Vector2,
	size: u16,
	time_since_last_key: f64,
	set_pixel: bool,
	clear_canvas: bool,
	window_size: Vector2,
	scaling: f32,
}


impl Mixel {
	pub fn new() -> Self {
		Self {
//			color: Color::from_rgba( 1.0, 1.0, 1.0, 1.0 ),
			color: Color::green(),
			time: 0.0,
			pos: Vector2::zero(),
			size: 32,
			time_since_last_key: f64::MAX,
			set_pixel: false,
			clear_canvas: false,
			window_size: Vector2::zero(),
			scaling: 1.0,
		}
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
//		renderer.register_texture( Texture::create( system, "fish_die00" ) );
//		canvas.set_texel( &Vector2::new( 15.0, 15.0 ), 0x0000ffff );
//		canvas.set_texel( &Vector2::new( 1.0, 1.0 ), 0xffff0000 );
//		canvas.update_canvas();
		renderer.register_texture( Texture::create_canvas( system, "m_canvas", self.size.into() ) );
		renderer.register_texture( Texture::create_canvas( system, "m_cursor", self.size.into() ) );
	}

	pub fn teardown( &mut self ) {

	}

	pub fn update( &mut self, wuc: &mut WindowUpdateContext ) {
		self.time += wuc.time_step;
		let t = self.time as f32;
		self.color.r = 0.5+0.5*t.sin();
		self.color.g = 0.5+0.5*t.sin();
		self.color.b = 0.5+0.5*t.sin();

		self.window_size = wuc.window_size;

		let scaling = 1024.0/self.window_size.y;
		self.scaling = 0.5*scaling;

		self.window_size.y *= self.scaling * 2.0;
		self.window_size.x = self.window_size.y;

		//   w  
		// a s d

		self.set_pixel = wuc.is_space_pressed;
		self.clear_canvas = wuc.is_key_pressed[ 'c' as usize ];
		if self.time_since_last_key > 0.1 {
			if wuc.is_key_pressed[ 'a' as usize ] {
				self.pos.x -= 1.0;
				self.time_since_last_key = 0.0;
			} else if wuc.is_key_pressed[ 'd' as usize ] {
				self.pos.x += 1.0;
				self.time_since_last_key = 0.0;
			} else if wuc.is_key_pressed[ 'w' as usize ] {
				self.pos.y -= 1.0;
				self.time_since_last_key = 0.0;
			} else if wuc.is_key_pressed[ 's' as usize ] {
				self.pos.y += 1.0;
				self.time_since_last_key = 0.0;
			}
		} else {
			self.time_since_last_key += wuc.time_step;
		}

		if self.pos.x <= 0.0 {
			self.pos.x = 0.0;
		} else if self.pos.x >= self.size as f32 - 1.0 {
			self.pos.x = self.size as f32 - 1.0;
		}
		if self.pos.y <= 0.0 {
			self.pos.y = 0.0;
		} else if self.pos.y >= self.size as f32 - 1.0 {
			self.pos.y = self.size as f32 -1.0;
		}
//		dbg!(&self.pos);
	}

	pub fn render( &mut self, renderer: &mut Renderer) {
		if let Some( canvas ) = renderer.find_texture_mut( "m_canvas" ) {
			if self.clear_canvas {
				self.clear_canvas = false;
				canvas.clear();
				canvas.update_canvas();
			} else if self.set_pixel {
				self.set_pixel = false;
				canvas.set_texel( &self.pos, self.color.as_abgr8() );
				canvas.update_canvas();
			}
		}
		if let Some( canvas ) = renderer.find_texture_mut( "m_cursor" ) {
			canvas.clear();
			let w = self.size as u16;
			let h = self.size as u16;
			let c = Color::from_rgba( 0.5, 0.5, 0.8, 0.9 );
			let c = c.as_abgr8();
			for x in 0..w {
				if x != self.pos.x as u16 {
					canvas.set_texel( &Vector2::new( x as f32, self.pos.y ), c );
				}
			}
			for y in 0..h {
				if y != self.pos.y as u16 {
					canvas.set_texel( &Vector2::new( self.pos.x, y as f32 ), c );
				}
			}
//			canvas.set_texel( &self.pos, Color::red().as_abgr8() );
			canvas.update_canvas();
		}
		renderer.use_effect( EffectId::Textured as u16 );
		renderer.use_texture( "m_canvas" );
		renderer.render_textured_quad( &Vector2::new( 0.0, 0.0 ), &self.window_size );
		renderer.use_texture( "m_cursor" );
		renderer.render_textured_quad( &Vector2::new( 0.0, 0.0 ), &self.window_size );
	}

}