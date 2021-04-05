

use crate::math::Vector2;
use crate::renderer::{
//	Color,
//	Effect,
	Renderer,
	Texture,
};
use crate::system::System;

//use crate::window_update_context::WindowUpdateContext;
use crate::fiiish::app_update_context::AppUpdateContext;
use super::effect_ids::EffectId;


#[derive(Debug)]
pub struct Demo {
	total_time: f64,
	click_positions: Vec< Vector2 >,
}


impl Demo {
	pub fn new() -> Self {
		Self {
			total_time: 0.0,
			click_positions: Vec::new(),
		}
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
		renderer.register_texture( Texture::create( system, "fish_swim0000" ) );
	}

	pub fn teardown( &mut self ) {

	}

	pub fn update( &mut self, auc: &mut AppUpdateContext ) {
		self.total_time += auc.time_step();

		if let Some( wuc ) = auc.wuc() {
			if wuc.was_mouse_button_pressed( 0 ) {
				println!("Left Mouse Button was pressed!");
				let cp = auc.cursor_pos();

				self.click_positions.push( *cp );
			}
			if wuc.mouse_buttons[ 1 ] {
	//			println!("Middle Mouse Button is pressed! -> {}", self.click_positions.len());
				let cp = auc.cursor_pos();

				for _ in 0..1 /*000*/ {
					self.click_positions.push( *cp );
				}
			}
		}
	}

	pub fn render( &mut self, renderer: &mut Renderer) {
		renderer.use_effect( EffectId::Default as u16 );
		renderer.use_texture( "fish_swim0000" );

		// renderer.use_material( "rainbow" );
		for i in 0..100 {
			if i % 10 == 0 {
				renderer.use_effect( EffectId::Default as u16 );
			} else {
				renderer.use_effect( EffectId::Textured as u16 );
			}
			let s = 64.0;
			let fi = i as f32;
			let t = self.total_time as f32 + fi*1.01;
			let y = 0.2*t.sin() as f32;
			let x = 0.2*t.cos() as f32;
			let d = 1.0+(0.5+0.5*t.sin());
			let x = d*512.0*3.0 * x;
			let y = d*512.0*3.0 * y;

			let pos = Vector2::new( x, y );
			let size = Vector2::new( 2.0*s, 2.0*s );
			renderer.render_textured_quad_with_rotation( &pos, &size, -t*57.29577951289617186797 + 270.0 );

		}

		renderer.use_effect( EffectId::Default as u16 );
		for cp in &self.click_positions {
			renderer.render_quad( &cp, &Vector2::new( 64.0, 64.0 ) );
		}

		renderer.use_effect( EffectId::Textured as u16 );
		renderer.use_texture( "test_texture_1" );
		renderer.render_textured_quad( &Vector2::new( -256.0, 512.0 - 0.5*64.0 ), &Vector2::new( 64.0, 64.0 ) );
		renderer.use_texture( "test_texture_2" );
		renderer.render_textured_quad( &Vector2::new(  512.0 - 0.5*128.0, -512.0 + 0.5*128.0 ), &Vector2::new( 128.0, 128.0 ) );
//		renderer.use_texture( "test_texture_3" );
	}
}
