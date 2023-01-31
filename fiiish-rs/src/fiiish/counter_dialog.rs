use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use oml_game::math::Vector2;
use oml_game::renderer::Color;

use crate::fiiish::game::Game;
use crate::ui::*;

#[derive(Debug)]
pub struct CounterDialog {
	game:           Rc<RefCell<Game>>,
	distance_label: Option<UiElementContainerHandle>,
	coins_label:    Option<UiElementContainerHandle>,
}

impl CounterDialog {
	pub fn new(game: &mut Rc<RefCell<Game>>) -> Self {
		Self {
			game:           game.clone(),
			distance_label: None,
			coins_label:    None,
		}
	}
}

impl UiElement for CounterDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		let mut vbox = UiVbox::new();
		vbox.set_padding(16.0);
		let mut vbox = container.add_child_element(vbox);

		let mut vbox = vbox.borrow_mut();

		{
			let mut hbox = UiHbox::new();
			hbox.set_padding(16.0);
			let mut hbox = UiElementContainer::new_from_element(hbox);
			let mut l = UiLabel::new(&Vector2::new(128.0, 64.0), "666");
			l.set_alignment(&Vector2::new(1.0, 0.0));
			//			l.set_color( &Color::from_rgba( 0.8, 0.35, 0.05, 1.0 ) ) ;
			l.set_color(&Color::from_rgba(0.9, 0.9, 0.9, 1.0));
			let l = hbox.add_child_element(l);
			self.coins_label = Some(l.clone());
			hbox.add_child_element(UiImage::new("mini_icon_coin", &Vector2::new(64.0, 64.0)));
			vbox.add_child(hbox);
		}

		{
			let mut hbox = UiHbox::new();
			hbox.set_padding(16.0);
			let mut hbox = UiElementContainer::new_from_element(hbox);
			let mut l = UiLabel::new(&Vector2::new(128.0, 64.0), "1024m");
			l.set_alignment(&Vector2::new(1.0, 0.0));
			//			l.set_color( &Color::from_rgba( 1.0, 0.35, 0.05, 1.0 ) ) ;
			l.set_color(&Color::from_rgba(0.9, 0.9, 0.9, 1.0));
			let l = hbox.add_child_element(l);
			self.distance_label = Some(l.clone());

			hbox.add_child_element(UiImage::new("mini_icon_flag", &Vector2::new(64.0, 64.0)));
			vbox.add_child(hbox);
		}

		container.set_size(vbox.size());
	}
	fn update(&mut self, _container: &UiElementContainerData, _time_step: f64) {
		let game = self.game.borrow_mut();

		let coins = game.coins();
		let distance = game.distance();

		if let Some(coins_label) = &mut self.coins_label {
			let mut coins_label = coins_label.borrow_mut();
			let coins_label = coins_label.borrow_element_mut();
			match coins_label.as_any_mut().downcast_mut::<UiLabel>() {
				Some(cl) => {
					let t = format!("{}", coins);
					cl.set_text(&t);
				},
				None => panic!("{:?} isn't a UiLabel!", &coins_label),
			};
		}
		if let Some(distance_label) = &mut self.distance_label {
			let mut distance_label = distance_label.borrow_mut();
			let distance_label = distance_label.borrow_element_mut();
			match distance_label.as_any_mut().downcast_mut::<UiLabel>() {
				Some(dl) => {
					let t = format!("{}m", distance);
					dl.set_text(&t);
				},
				None => panic!("{:?} isn't a UiLabel!", &distance_label),
			};
		}
	}

	fn handle_ui_event(
		&mut self,
		_container: &mut UiElementContainerData,
		_event: &UiEvent,
		_event_sender: &Sender<Box<dyn UiEventResponse>>,
	) -> Option<Box<dyn UiEventResponse>> {
		// stop traversal into children
		None
	}
}
