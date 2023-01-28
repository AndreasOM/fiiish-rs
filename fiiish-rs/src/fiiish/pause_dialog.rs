use std::cell::RefCell;
use std::rc::Rc;

use oml_game::math::Vector2;

use crate::fiiish::game::Game;
use crate::ui::*;

#[derive(Debug)]
pub struct PauseDialog {
	game:               Rc<RefCell<Game>>,
	pause_togglebutton: Option<UiElementContainerHandle>,
	settings_button:    Option<UiElementContainerHandle>,
}

impl PauseDialog {
	pub fn new(game: &mut Rc<RefCell<Game>>) -> Self {
		Self {
			game:               game.clone(),
			pause_togglebutton: None,
			settings_button:    None,
		}
	}
}

impl UiElement for PauseDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		let mut pause_menu = UiHbox::new();
		pause_menu.set_padding(16.0);

		let mut pause_menu = container.add_child_element(pause_menu);
		let mut pause_menu = pause_menu.borrow_mut();
		pause_menu.set_name("PauseMenu");

		let pause_togglebutton = pause_menu.add_child_element(UiToggleButton::new(
			"button_pause",
			"button_play",
			&Vector2::new(128.0, 128.0),
		));
		{
			let mut p = pause_togglebutton.borrow_mut();
			p.set_name("ButtonPause");
		}
		self.pause_togglebutton = Some(pause_togglebutton.clone());

		let settings_button = pause_menu.add_child_element(UiButton::new(
			"button_settings",
			&Vector2::new(128.0, 128.0),
		));
		{
			let mut settings_button = settings_button.borrow_mut();
			settings_button.set_name("ButtonSettings");
			settings_button.fade_out(0.0);
			settings_button.fade_in(1.0);
		}
		self.settings_button = Some(settings_button.clone());
		container.set_size(pause_menu.size());
	}
	fn handle_ui_event_response(
		&mut self,
		response: Box<dyn UiEventResponse>,
	) -> Option<Box<dyn UiEventResponse>> {
		match response
			.as_any()
			.downcast_ref::<UiEventResponseButtonClicked>()
		{
			Some(e) => {
				println!("SettingsDialog: Button {} clicked", &e.button_name);
				match e.button_name.as_str() {
					"ButtonPause" => {
						self.game.borrow_mut().toggle_pause();
					},
					_ => {
						//						println!( "Unhandled button click from {}", &e.button_name );
					},
				}
			},
			_ => {},
		}
		Some(response)
	}

	fn update(&mut self, _container: &UiElementContainerData, _time_step: f64) {
		let game = self.game.borrow();
		if let Some(p) = &mut self.pause_togglebutton {
			let mut p = p.borrow_mut();
			let p = p.borrow_element_mut();
			let tb: &mut UiToggleButton = match p.as_any_mut().downcast_mut::<UiToggleButton>() {
				Some(p) => p,
				None => panic!("{:?} isn't a UiToggleButton!", &p),
			};
			if game.is_paused() {
				tb.goto_b();
			} else {
				tb.goto_a();
			}
		}
		if let Some(settings_button) = &mut self.settings_button {
			let mut settings_button = settings_button.borrow_mut();
			if game.is_paused() {
				settings_button.fade_in(1.0);
			} else {
				settings_button.fade_out(1.0);
			}
		}
	}
}
