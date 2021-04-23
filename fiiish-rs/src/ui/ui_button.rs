
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
pub struct UiButton {
	imagesize: Vector2,
	imagename: String,
	image: Option< UiElementContainerHandle >,
}

impl UiButton {
	pub fn new( imagename: &str, imagesize: &Vector2 ) -> Self {
		Self {
			imagesize: *imagesize,
			imagename: imagename.to_owned(),
			image: None,
		}
	}
}

impl UiElement for UiButton {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container( &mut self, container: &mut UiElementContainerData ) {
		let image = container.add_child_element( UiImage::new( &self.imagename, &self.imagesize ) );
		self.image = Some( image );
	}
	fn handle_ui_event( &mut self, container: &mut UiElementContainerData, _event: &UiEvent ) -> Option< Box < dyn UiEventResponse > > {
		println!("Button clicked");
		let ev = UiEventResponseButtonClicked{ button_name: container.name.clone() };
		Some( Box::new( ev ) )
	}
	fn preferred_size( &self ) -> Option< &Vector2 > {
		Some( &self.imagesize )
	}
}
