
use crate::math::Vector2;
use crate::ui::{
	UiElement,
	UiElementContainer,
	UiElementContainerData,
	UiElementContainerHandle,
	UiEvent,
	UiImage,
};

#[derive(Debug)]
pub struct UiToggleButton {
	imagesize: Vector2,
	imagename_a: String,
	imagename_b: String,
	image_a: Option< UiElementContainerHandle >,
	image_b: Option< UiElementContainerHandle >,
}

impl UiToggleButton {
	pub fn new( imagename_a: &str, imagename_b: &str, imagesize: &Vector2 ) -> Self {
		Self {
			imagesize: *imagesize,
			imagename_a: imagename_a.to_owned(),
			imagename_b: imagename_b.to_owned(),
			image_a: None,
			image_b: None,
		}
	}

	pub fn goto_a( &mut self ) {
		if let Some( image_a ) = &mut self.image_a {
			image_a.borrow_mut().fade_in( 1.0 );
		}
		if let Some( image_b ) = &mut self.image_b {
			image_b.borrow_mut().fade_out( 1.0 );
		}
	}
	pub fn goto_b( &mut self ) {
		if let Some( image_a ) = &mut self.image_a {
			image_a.borrow_mut().fade_out( 1.0 );
		}
		if let Some( image_b ) = &mut self.image_b {
			image_b.borrow_mut().fade_in( 1.0 );
		}
	}
}

impl UiElement for UiToggleButton {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container( &mut self, container: &mut UiElementContainerData ) {
		let mut image_a = container.add_child_element( UiImage::new( &self.imagename_a, &self.imagesize ) );
		{
			let mut img_a = image_a.borrow_mut();
			img_a.set_name( "A" );
		}
		self.image_a = Some( image_a );
		let mut image_b = container.add_child_element( UiImage::new( &self.imagename_b, &self.imagesize ) );
		{
			let mut img_b = image_b.borrow_mut();
			img_b.set_name( "B" );
			img_b.fade_out( 0.0 );
		}
		self.image_b = Some( image_b );
	}
	fn handle_ui_event( &mut self, _container: &mut UiElementContainerData, _event: &UiEvent ) -> bool {	// bool will change to ... Option< Something >
		false
	}
	fn preferred_size( &self ) -> Option< &Vector2 > {
		Some( &self.imagesize )
	}
}
