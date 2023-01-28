use std::cell::RefCell;
use std::rc::Rc;

use oml_game::math::Vector2;
use oml_game::renderer::Color;

use crate::fiiish::game::Game;
use crate::ui::*;

const RESULTINDEX_COINS: usize = 0;
const RESULTINDEX_DISTANCE: usize = 1;
const RESULTINDEX_BESTDISTANCE: usize = 2;
const RESULTINDEX_TOTALDISTANCE: usize = 3;

#[derive(Debug)]
pub struct ResultDialog {
	game:           Rc<RefCell<Game>>,
	play_button:    Option<UiElementContainerHandle>,
	total_labels:   [Option<UiElementContainerHandle>; 4],
	current_labels: [Option<UiElementContainerHandle>; 4],
}

impl ResultDialog {
	pub fn new(game: &mut Rc<RefCell<Game>>) -> Self {
		Self {
			game:           game.clone(),
			play_button:    None,
			total_labels:   [None, None, None, None],
			current_labels: [None, None, None, None],
		}
	}
	fn set_total_label_text(&mut self, index: usize, text: &str) {
		if let Some(l) = &mut self.total_labels[index] {
			let mut l = l.borrow_mut();
			let l = l.borrow_element_mut();
			match l.as_any_mut().downcast_mut::<UiLabel>() {
				Some(l) => {
					l.set_text(text);
				},
				None => panic!("{:?} isn't a UiLabel!", &l),
			};
		}
	}
	fn set_current_label_text(&mut self, index: usize, text: &str) {
		if let Some(l) = &mut self.current_labels[index] {
			let mut l = l.borrow_mut();
			let l = l.borrow_element_mut();
			match l.as_any_mut().downcast_mut::<UiLabel>() {
				Some(l) => {
					l.set_text(text);
				},
				None => panic!("{:?} isn't a UiLabel!", &l),
			};
		}
	}
}

impl UiElement for ResultDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		let mut background = container
			.add_child_element(UiImage::new("screen_frame", &Vector2::new(1024.0, 1024.0)));

		{
			let mut parent = background.borrow_mut();
			let vbox = UiVbox::new();
			// hbox.set_padding( 0.0 );
			let vbox = parent.add_child_element(vbox);
			{
				let mut parent = vbox.borrow_mut();

				parent.add_child_element(UiSpacer::new(
					&Vector2::new(16.0, 240.0),
					&Color::from_rgba(0.2, 0.6, 0.8, 0.5),
				)); // top space

				let hbox = UiHbox::new();
				// hbox.set_padding( 0.0 );
				let hbox = parent.add_child_element(hbox);
				// center box with actual results
				{
					let mut parent = hbox.borrow_mut();
					parent.add_child_element(UiSpacer::new(
						&Vector2::new(350.0, 16.0),
						&Color::from_rgba(0.8, 0.2, 0.8, 0.5),
					)); // left space
					{
						let table_space = parent.add_child_element(UiSpacer::new(
							&Vector2::new(520.0, 412.0),
							&Color::from_rgba(0.2, 0.2, 0.8, 0.5),
						)); // placeholder

						let mut parent = table_space.borrow_mut();
						let mut vbox = UiVbox::new();
						vbox.set_padding(32.0);
						let vbox = parent.add_child_element(vbox);
						{
							let mut parent = vbox.borrow_mut();

							for i in 0..4 {
								let mut hbox = UiHbox::new();
								hbox.set_padding(20.0);
								let hbox = parent.add_child_element(hbox);

								{
									let mut parent = hbox.borrow_mut();

									let mut l = UiLabel::new(&Vector2::new(250.0, 79.0), "");
									l.set_alignment(&Vector2::new(1.0, 0.0));
									self.total_labels[i] =
										Some(parent.add_child_element(l).clone());

									let mut l = UiLabel::new(&Vector2::new(250.0, 79.0), "");
									l.set_alignment(&Vector2::new(1.0, 0.0));
									self.current_labels[i] =
										Some(parent.add_child_element(l).clone());
								}
							}
						}
					}
					parent.add_child_element(UiSpacer::new(
						&Vector2::new(154.0, 16.0),
						&Color::from_rgba(0.8, 0.2, 0.2, 0.5),
					)); // right space
				}
				parent.add_child_element(UiSpacer::new(
					&Vector2::new(16.0, 54.0),
					&Color::from_rgba(0.5, 0.5, 0.5, 0.5),
				)); // middle space
				{
					// button row

					let hbox = UiHbox::new();
					// hbox.set_padding( 0.0 );
					let hbox = parent.add_child_element(hbox);
					{
						let mut parent = hbox.borrow_mut();
						parent.add_child_element(UiSpacer::new(
							&Vector2::new(128.0, 16.0),
							&Color::from_rgba(0.8, 0.2, 0.8, 0.5),
						)); // left space
						{
							let button_space = parent.add_child_element(UiSpacer::new(
								&Vector2::new(384.0, 128.0),
								&Color::from_rgba(0.5, 0.5, 0.5, 0.5),
							)); // placeholder for buttons

							{
								let mut parent = button_space.borrow_mut();
								let hbox = UiHbox::new();
								// hbox.set_padding( 0.0 );
								let hbox = parent.add_child_element(hbox);
								{
									let mut parent = hbox.borrow_mut();
									let play_button = parent.add_child_element(UiButton::new(
										"button_play",
										&Vector2::new(128.0, 128.0),
									));
									play_button.borrow_mut().set_name("PlayButton");
									self.play_button = Some(play_button.clone());
									parent.add_child_element(UiSpacer::new(
										&Vector2::new(256.0, 16.0),
										&Color::from_rgba(0.8, 0.6, 0.2, 0.5),
									)); // right space
								}
							}
						}
						parent.add_child_element(UiSpacer::new(
							&Vector2::new(512.0, 16.0),
							&Color::from_rgba(0.8, 0.2, 0.2, 0.5),
						)); // right space
					}
				}
				parent.add_child_element(UiSpacer::new(
					&Vector2::new(16.0, 190.0),
					&Color::from_rgba(0.5, 0.5, 0.5, 0.5),
				)); // bottom space
			}
		}
		container.set_size(background.borrow().size());
	}

	fn update(&mut self, _container: &UiElementContainerData, _time_step: f64) {
		let distance; // = 0;
		let coins; // = 0;
		let player_coins; // = 0;
		let last_distance; // = 0;
		let best_distance; // = 0;
		let total_distance; // = 0;
		{
			let game = self.game.borrow_mut();
			distance = game.distance();
			coins = game.coins();

			let player = game.player();
			player_coins = player.coins();
			last_distance = player.last_distance();
			best_distance = player.best_distance();
			total_distance = player.total_distance();

			if let Some(play_button) = &mut self.play_button {
				if game.can_respawn() {
					play_button.borrow_mut().fade_in(1.0);
				} else {
					play_button.borrow_mut().fade_out(1.0);
				}
			}
		}

		self.set_total_label_text(RESULTINDEX_COINS, &format!("{}", player_coins));
		self.set_total_label_text(RESULTINDEX_DISTANCE, &format!("{}m", last_distance));
		self.set_total_label_text(RESULTINDEX_BESTDISTANCE, &format!("{}m", best_distance));
		self.set_total_label_text(RESULTINDEX_TOTALDISTANCE, &format!("{}m", total_distance));
		if coins == 0 {
			self.set_current_label_text(RESULTINDEX_COINS, &format!(""));
		} else {
			self.set_current_label_text(RESULTINDEX_COINS, &format!("{}", coins));
		}
		if distance == 0 {
			self.set_current_label_text(RESULTINDEX_DISTANCE, &format!(""));
		} else {
			self.set_current_label_text(RESULTINDEX_DISTANCE, &format!("{}m", distance));
		}
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
				println!("ResultDialog: Button {} clicked", &e.button_name);
				match e.button_name.as_str() {
					"PlayButton" => {
						self.game.borrow_mut().play();
						return None;
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
}
