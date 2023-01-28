#[derive(Debug, Copy, Clone)]
pub enum LayerId {
	None            = 0,
	Background      = 1,
	Decoration      = 2,
	Pickups         = 3,
	Fish            = 4,
	Obstacles       = 5,
	DecorationFront = 6,
	Debug           = 7,
	Ui              = 8,
	UiFront         = 9,
	Overlay         = 10,
	DebugRenderer   = 11,
}
