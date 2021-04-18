
use crate::math::Vector2;
use crate::ui::{
	UiElement,
	UiElementContainer,
	UiElementContainerData,
	UiEvent,
	UiImage,
};

#[derive(Debug)]
pub struct UiToggleButton {
	imagesize: Vector2,
	imagename_a: String,
	imagename_b: String,
}

impl UiToggleButton {
	pub fn new( imagename_a: &str, imagename_b: &str, imagesize: &Vector2 ) -> Self {
		Self {
			imagesize: *imagesize,
			imagename_a: imagename_a.to_owned(),
			imagename_b: imagename_b.to_owned(),
		}
	}

	pub fn goto_a( &mut self ) {
		/*
		image_a.fade_in( 1.0 );
		image_b.fade_out( 1.0 );
		*/
	}
	pub fn goto_b( &mut self ) {
		/*
		image_a.fade_out( 1.0 );
		image_b.fade_in( 1.0 );
		*/
	}
}

impl UiElement for UiToggleButton {
	fn setup_within_container( &mut self, container: &mut UiElementContainerData ) {
		let image_a = container.add_child_element( UiImage::new( &self.imagename_a, &self.imagesize.scaled( 1.2 ) ) );
		image_a.set_name( "A" );
		let image_b = container.add_child_element( UiImage::new( &self.imagename_b, &self.imagesize ) );
		image_b.set_name( "B" );
		image_b.fade_out( 0.0 );
	}
	fn handle_ui_event( &mut self, _container: &mut UiElementContainerData, _event: &UiEvent ) -> bool {	// bool will change to ... Option< Something >
		false
	}
	fn preferred_size( &self ) -> Option< &Vector2 > {
		Some( &self.imagesize )
	}
}
