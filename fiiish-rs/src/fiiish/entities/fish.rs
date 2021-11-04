
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;
use crate::fiiish::entities::Entity;
use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::entities::EntityData;
use crate::fiiish::EntityUpdateContext;
use crate::fiiish::entities::EntityType;
use crate::math::Matrix22;
use crate::math::Vector2;
use crate::renderer::{
	AnimatedTexture,
	Color,
	Renderer,
};

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum PlayerState {
	WaitForStart,
	Swimming,
	Dying,
	Dead,
}

#[derive(Debug,PartialEq,Eq)]
enum PlayerDirection {
	Up,
	Down,
	Float,
}

#[derive(Debug)]
pub struct Fish {
	name: String,
	spawn_pos: Vector2,
	pos: Vector2,
	angle: f32,
	size: Vector2,
	state: PlayerState,
	direction: PlayerDirection,
	speed: f32,
	movement: Vector2,
	time_since_dying: f32,
	animated_texture: AnimatedTexture,
	animated_texture_dying: AnimatedTexture,
	entity_data: EntityData,
}

impl Fish {
	pub fn new() -> Self {
		Self {
			name: "fish".to_string(),
			spawn_pos: Vector2::new( -512.0, 0.0 ),
			pos: Vector2::zero(),
			angle: 0.0,
			size: Vector2::new( 128.0, 128.0 ),
			state: PlayerState::Dead,
			direction: PlayerDirection::Float,
			speed: 240.0,
			movement: Vector2::zero(),
			time_since_dying: f32::MAX,
			animated_texture: AnimatedTexture::new(),
			animated_texture_dying: AnimatedTexture::new(),
			entity_data: EntityData::default(),
		}
	}

	pub fn name( &self ) -> &str {
		&self.name
	}

	pub fn is_alive( &self ) -> bool {
		match self.state {
			PlayerState::Dead | PlayerState::Dying => false,
			PlayerState::WaitForStart | PlayerState::Swimming => true,
		}
	}

	pub fn can_respawn( &self ) -> bool {
		self.state == PlayerState::Dead
	}

	pub fn movement( &self ) -> &Vector2 {
		&self.movement
	}

	pub fn state( &self ) -> PlayerState {
		self.state
	}

	fn goto_state( &mut self, state: PlayerState ) {
		match state {
			PlayerState::WaitForStart => {
				self.pos = self.spawn_pos;
				self.angle = 0.0;
				self.direction = PlayerDirection::Float;
			},
			PlayerState::Dying => {
				self.time_since_dying = 0.0;
			},
			_ => {},
		}
		self.state = state;	// :TODO: handle transitions if needed
	}

	pub fn respawn( &mut self ) {
		match self.state {
			PlayerState::Dead => {
				self.goto_state( PlayerState::WaitForStart );
			},
			_ => {},
		}		
	}

	pub fn turn_down( &mut self ) {
		match self.state {
			PlayerState::WaitForStart => {
				self.goto_state( PlayerState::Swimming );
			},
			PlayerState::Swimming => {
				self.direction = PlayerDirection::Down;
			}
			_ => {},
		}
	}

	pub fn turn_up( &mut self ) {
		self.direction = PlayerDirection::Up;
	}

	pub fn kill( &mut self ) {
		if self.is_alive() {
			self.goto_state( PlayerState::Dying );
		}
	}

	fn get_angle_range_for_y( y: f32 ) -> ( f32, f32 ) {
        let limit = 35.0;
        let range = 1.0/280.0;
	
		let a = ( y.abs() * range ).sin();
        // float a = Functions::getSin( Functions::getAbs( y )*range );
        let m = limit*( 1.0-a*a*a*a );
        // float m = limit*( 1.0-a*a*a*a );
        
        if y<0.0
        {
        	( -limit, m )
//            *pMinAngle = -limit;
//            *pMaxAngle = m;
        }
        else
        {
        	( -m, limit )
//            *pMinAngle = -m;
//            *pMaxAngle = limit;
        }
	}

	fn update_waiting_for_start( &mut self, euc: &mut EntityUpdateContext ) {
		self.animated_texture.update( euc.time_step() );
		self.movement.x = 0.0;
	}
	fn update_swimming( &mut self, euc: &mut EntityUpdateContext ) {
		self.animated_texture.update( euc.time_step() );

		self.movement.x = self.speed * euc.time_step() as f32;

		let ts = euc.time_step() as f32;
		match self.direction {
			PlayerDirection::Down => {
				self.angle += 120.0 * ts; 
			},
			PlayerDirection::Up => {
				self.angle -= 120.0 * ts; 
			},
			PlayerDirection::Float => {},
		}

		// :TODO: port over angle limiting logic from original game

		let ( min_a, max_a ) = Fish::get_angle_range_for_y( self.pos.y );

//		self.angle = MAX( minAngle, MIN( maxAngle, m_angle ) );
		self.angle = self.angle.clamp( min_a, max_a );
//		println!("{} {} {} {}", &self.pos.y, &self.angle, &min_a, &max_a );

		let a = self.angle;
		let dy = ( ( a/57.2957795 ).sin() )*-350.0*ts;

		// y +=  Functions::getSin( m_angle/57.2957795 )*-350.0*timeStep;

		self.pos.y += dy;

		// should never trigger, but better be safe
		if self.pos.y > 512.0 || self.pos.y < -512.0 {
			self.goto_state( PlayerState::Dying );
		}

	}
	fn update_dying( &mut self, euc: &mut EntityUpdateContext ) {
		self.animated_texture_dying.update( euc.time_step() );
		self.movement.x = 0.0;

		let ts = euc.time_step() as f32;
		self.time_since_dying += ts;
		self.pos.y += 1.5*128.0 * self.time_since_dying * ts;

		// this works in this case, but is a bad idea in general,
		// use angle helpers instead
		self.angle -= 60.0 * ts;
		self.angle = self.angle.max( -90.0 );
//		println!("{} {}", &self.pos.y, &self.angle );
		/*
			m_timeSinceKill += timeStep;
            
            float y = m_position.y;
            
            y += 1.5f*128.0f * m_timeSinceKill * timeStep;
            
            m_position.y = y;
            
            m_angle = Functions::approachAngle( -90.0f, m_angle,  60.0f*timeStep );
            
            m_rotation = m_angle;
		*/

		if self.pos.y > 512.0+128.0 {
			self.goto_state( PlayerState::Dead );
		}
	}



	pub fn set_pos( &mut self, pos: &Vector2 ) {
		self.pos = *pos;
	}
	
	pub fn pos( &self ) -> &Vector2 {
		&self.pos
	}

	pub fn rotation( &self ) -> f32 {
		self.angle
	}
	
	pub fn radius( &self ) -> f32 {
		self.size.length() * 0.5
	}

}

impl Entity for Fish {
	fn data(&self) -> &EntityData {
		&self.entity_data
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup( &mut self, _ec: &EntityConfiguration ) {
		// self.name = name.to_owned();
		self.animated_texture.setup( "fish_swim", 4, 0, 27, 25.0 );
		self.animated_texture_dying.setup( "fish_die", 2, 0, 2, 25.0 );
//		self.animated_texture.setup_from_config( &ec.animated_texture_configuration );
	}

	fn teardown( &mut self ) {

	}

	fn update( &mut self, euc: &mut EntityUpdateContext ) {
		// :TODO: time step
		match self.state {
			PlayerState::WaitForStart => self.update_waiting_for_start( euc ),
			PlayerState::Swimming => self.update_swimming( euc ),
			PlayerState::Dying => self.update_dying( euc ),
			_ => {},
		}

		if let Some( debug_renderer ) = &*euc.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			let color = Color::from_rgba( 0.8, 0.6, 0.3, 0.8 );
			debug_renderer.add_line( &self.pos, &Vector2::zero(), 1.0, &color );
			debug_renderer.add_frame( &self.pos, &self.size, 5.0, &color );
			let target = &Vector2::new( 250.0, 0.0 );
			// rotate
			let mtx = Matrix22::z_rotation( self.angle * 0.01745329252); // DEG to RAD
			let target = mtx.mul_vector2( &target );

			let target = self.pos.add( &target );
			debug_renderer.add_line( &self.pos, &target, 3.0, &color );

//			let radius = self.size.length() * 0.5;
//			debug_renderer.add_circle( &self.pos, radius, 5.0, &color );
		}

	}

	fn render( &mut self, renderer: &mut Renderer ) {
		if self.state == PlayerState::Dead {
			// dead means offscreen, nothing to be rendered
			return;
		}

		renderer.use_layer( LayerId::Fish as u8 );
		renderer.use_effect( EffectId::Textured as u16 );
		match self.state {
			PlayerState::Dying | PlayerState::Dead => self.animated_texture_dying.r#use( renderer ),
			_ => self.animated_texture.r#use( renderer ),
		}
		renderer.render_textured_quad_with_rotation( &self.pos, &self.size, self.angle );
	}

	fn name( &self ) -> &str {
		&self.name
	}

	fn entity_type( &self ) -> EntityType {
		EntityType::Player
	}

}
