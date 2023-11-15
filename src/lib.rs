#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
	Key(&'static str),
	Modifier { key: Modifier, pressed: bool },
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
