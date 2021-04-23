
use std::sync::mpsc::Sender;

use crate::math::Vector2;
use crate::ui::{
	UiElement,
	UiElementContainer,
	UiElementContainerData,
	UiElementContainerHandle,
	UiEvent,
	UiEventResponse,
	UiEventResponseButtonClicked,
	UiImage,
};

#[derive(Debug)]
pub struct SettingsDialog {
}

impl SettingsDialog {
	pub fn new( ) -> Self {
		Self {
		}
	}
}

impl UiElement for SettingsDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container( &mut self, container: &mut UiElementContainerData ) {
		let background = container.add_child_element( UiImage::new( "screen_frame_2", &Vector2::new( 1024.0, 1024.0 ) ) );
//		let image = container.add_child_element( UiImage::new( &self.imagename, &self.imagesize ) );
//		self.image = Some( image );
	}
	fn handle_ui_event( &mut self, container: &mut UiElementContainerData, _event: &UiEvent, event_sender: &Sender< Box< dyn UiEventResponse > > ) -> bool {
/*		
		println!("Button clicked");
		let ev = Box::new( UiEventResponseButtonClicked{ button_name: container.name.clone() } );
		event_sender.send( ev ).unwrap();

		true
*/
		false
	}
	fn preferred_size( &self ) -> Option< &Vector2 > {
//		Some( &self.imagesize )
		None
	}
}
