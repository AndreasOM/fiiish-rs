
use crate::renderer::gl;

pub struct Debug {


}

impl Debug {

	pub fn check_gl_error( file: &str, line: u32 ) {
		unsafe {
			match gl::GetError() {
				gl::NO_ERROR => {},
				e => {
					println!("GL Error in {}:{} => {}", file, line,
						match e {
							gl::INVALID_VALUE => "INVALID_VALUE".to_string(),
							gl::INVALID_OPERATION => "INVALID_OPERATION".to_string(),
							e => format!("{}",e ),
						}
					);
				},
			}
		}
	}

}
