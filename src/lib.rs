#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
	Modifier { key: Modifier, pressed: bool },
	Key(rdev::Key),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
	Control,
	Shift,
	Alt,
	Meta,
}

impl Modifier {
	pub const CHARS: [char; 4] = ['⌃', '⇧', '⌥', '🐧'];
}
