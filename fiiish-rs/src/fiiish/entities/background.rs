use oml_game::math::{Matrix32, Vector2};
use oml_game::renderer::Renderer;

use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::entities::Entity;
use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::entities::EntityData;
use crate::fiiish::entities::EntityType;
use crate::fiiish::game::GameState;
use crate::fiiish::layer_ids::LayerId;
use crate::fiiish::EntityUpdateContext;

#[derive(Debug, Eq, PartialEq)]
enum State {
	FadedOut,
	FadedIn,
	Black,
	FadedOutEnd,
}

pub struct Background {
	name:        String,
	pos:         Vector2,
	phase:       f32,
	state:       State,
	time:        f32,
	entity_data: EntityData,
}

impl Background {
	pub fn new() -> Self {
		Self {
			name:        String::new(),
			pos:         Vector2::zero(),
			phase:       0.0,
			state:       State::FadedOut,
			time:        0.0,
			entity_data: EntityData::default(),
		}
	}
	/*
			{ 0.0f, 0.0f, 1.0f/0.75f, false },						// fadedOut
			{ 16.0f/128.0f, 96.0f/128.0f, 1.0f/5.0f, true },		// fadedIn
			{ 112.0f/128.0f, 112.0f/128.0f, 1.0f/3.0f, false },		// black
			{ 127.0f/128.0f, 127.0f/128.0f, 1.0f/0.75f, false },		// fadedOutEnd
	*/

	pub fn goto_next_state(&mut self) {
		self.state = match self.state {
			State::FadedOut => State::FadedIn,
			State::FadedIn => State::Black,
			State::Black => State::FadedOutEnd,
			State::FadedOutEnd => {
				self.phase = 0.0;
				State::FadedOut
			},
		};
	}

	fn phase_settings_for_current_state(&self) -> (f32, f32) {
		match self.state {
			State::FadedOut => (0.0, 0.0),
			State::FadedIn => (16.0 / 128.0, 96.0 / 128.0),
			State::Black => (112.0 / 128.0, 112.0 / 128.0),
			State::FadedOutEnd => (127.0 / 128.0, 127.0 / 128.0),
		}
	}
	fn calc_phase_for_current_state(&self) -> f32 {
		let (min, max) = self.phase_settings_for_current_state();
		/*
		float delta = d.maxOffset - d.minOffset;
		float o = ( 0.5f+0.5f*Functions::getSin( m_time * 0.5f ) )*delta + d.minOffset;
		*/

		let delta = max - min;

		(0.5 + 0.5 * (0.5 * self.time).sin()) * delta + min
	}
}

impl Entity for Background {
	fn data(&self) -> &EntityData {
		&self.entity_data
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup(&mut self, _ec: &EntityConfiguration) {
		// fake long running time to simulate precision loss
		self.pos.x = -2.0 * 60.0 * 60.0 * 60.0 * 240.0 * 0.5;
	}

	fn teardown(&mut self) {}

	fn update(&mut self, euc: &mut EntityUpdateContext) {
		self.time += euc.time_step() as f32; // :TODO: wrap for precision

		//		self.pos.x -= euc.time_step() as f32 * 240.0 * 0.5; // :HACK: speed is roughly guestimated to feel kind of nearly right
		self.pos.x += euc.world_movement().x * 0.5;
		// repeat value to avoid precision loss
		while self.pos.x < -1024.0 {
			self.pos.x += 1024.0;
		}

		let target_phase = self.calc_phase_for_current_state();
		let s = 0.99;
		//		dbg!(&target_phase, &self.phase);

		self.phase = s * self.phase + (1.0 - s) * target_phase; // :TODO: use time_step();

		// :HACK: needs game logic first
		/*
		if euc.change_background_state() {
			self.goto_next_state();
		}
		*/
		let expected_state = match euc.game_state() {
			GameState::WaitForStart | GameState::Playing => State::FadedIn,
			GameState::Dead => State::Black,
			_ => State::FadedOut,
		};

		if expected_state != self.state {
			self.goto_next_state();
		}
	}

	fn render(&mut self, renderer: &mut Renderer) {
		renderer.use_layer(LayerId::Background as u8);
		renderer.use_effect(EffectId::Background as u16);
		renderer.use_texture("background");
		renderer.use_texture_in_channel("background_grad", 1);
		let a = renderer.aspect_ratio();
		let mut mtx = Matrix32::scaling_xy(1.0 * a, 1.0);
		mtx.pos.x = -self.pos.x / 1024.0;
		renderer.set_tex_matrix(&mtx);
		renderer.set_uniform_float("phase", self.phase);
		renderer.render_textured_fullscreen_quad();

		renderer.set_tex_matrix(&Matrix32::identity());
		renderer.disable_texture_for_channel(1);
	}

	fn name(&self) -> &str {
		&self.name
	}

	fn entity_type(&self) -> EntityType {
		EntityType::Decoration
	}
}
