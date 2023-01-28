#[derive(Debug, Copy, Clone)]
pub enum EffectId {
	None            = 0,
	Default         = 1,
	White           = 2,
	Colored         = 3,
	Textured        = 4,
	ColoredTextured = 5,
	Background      = 6,
	FontColored     = 7,
}
