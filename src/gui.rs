use {
	eframe::{
		egui::{
			self, CentralPanel, FontData, FontFamily, Layout, RichText, TextStyle, TopBottomPanel,
		},
		emath::Align,
		epaint::Vec2,
		NativeOptions, Renderer, Theme,
	},
	keyscreen::{Event, Modifier},
	std::sync::mpsc,
};

pub struct KeyScreen {
	modifiers: [bool; 4],
	text: String,
	rx: mpsc::Receiver<Event>,
}

impl KeyScreen {
	const MAX_CHARS: usize = 10;
	const THEME: catppuccin_egui::Theme = catppuccin_egui::MOCHA;
	const WINDOW_SIZE: Vec2 = Vec2::new(430.0, 155.0);

	pub const fn new(rx: mpsc::Receiver<Event>) -> Self {
		Self { modifiers: [false; 4], text: String::new(), rx }
	}

	pub fn run(self) {
		eframe::run_native(
			"KeyScreen",
			NativeOptions {
				always_on_top: true,
				decorated: false,
				resizable: false,
				transparent: true,
				mouse_passthrough: true,
				renderer: Renderer::Wgpu,
				follow_system_theme: false,
				default_theme: Theme::Dark,
				centered: true,
				initial_window_size: Some(Self::WINDOW_SIZE),
				..Default::default()
			},
			Box::new(|ctx| {
				self.setup_fonts(&ctx.egui_ctx);
				Box::new(self)
			}),
		)
		.expect("Failed to run egui app.");
	}

	fn setup_fonts(&self, ctx: &egui::Context) {
		let mut fonts = egui::FontDefinitions::default();

		fonts.font_data.insert(
			String::from("JetBrains Mono"),
			FontData::from_static(include_bytes!(
				"../static/fonts/JetBrainsMonoNerdFont-Regular.ttf"
			)),
		);

		fonts
			.families
			.entry(FontFamily::Proportional)
			.or_default()
			.insert(0, String::from("JetBrains Mono"));

		fonts
			.families
			.entry(FontFamily::Monospace)
			.or_default()
			.push(String::from("JetBrains Mono"));

		ctx.set_fonts(fonts);

		ctx.style_mut(|style| {
			style
				.text_styles
				.iter_mut()
				.for_each(|(style, font)| {
					font.size *= match style {
						TextStyle::Body => 3.5,
						TextStyle::Monospace => 5.0,
						_ => 1.0,
					}
				});
		});
	}
}

impl eframe::App for KeyScreen {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.apply_theme(ctx);

		for message in self.rx.try_iter() {
			use rdev::Key as K;

			macro_rules! pressed {
				($mod:ident) => {
					self.modifiers[Modifier::$mod as usize]
				};
			}

			match message {
				Event::Modifier { key, pressed } => {
					self.modifiers[key as usize] = pressed;
				}
				Event::Key(K::BackQuote) if pressed!(Shift) => {
					self.text.push('°');
				}
				Event::Key(K::BackQuote) if pressed!(Alt) => {
					self.text.push('~');
				}
				Event::Key(K::BackQuote) => {
					self.text.push('^');
				}
				Event::Key(K::BackSlash) if pressed!(Shift) => {
					self.text.push('\\');
				}
				Event::Key(K::BackSlash) if pressed!(Alt) => {
					self.text.push('|');
				}
				Event::Key(K::BackSlash) => {
					self.text.push('/');
				}
				Event::Key(K::Backspace) => {
					self.text.push('⌫');
				}
				Event::Key(K::Comma) if pressed!(Shift) => {
					self.text.push(';');
				}
				Event::Key(K::Comma) => {
					self.text.push(',');
				}
				Event::Key(K::Dot) if pressed!(Shift) => {
					self.text.push(':');
				}
				Event::Key(K::Dot) => {
					self.text.push('.');
				}
				Event::Key(K::DownArrow) => {
					self.text.push('↓');
				}
				Event::Key(K::Equal) if pressed!(Control) => {}
				Event::Key(K::Equal) if pressed!(Shift) => {
					self.text.push('´');
				}
				Event::Key(K::Equal) => {
					self.text.push('`');
				}
				Event::Key(K::Escape) => {
					self.text.push('⎋');
				}
				Event::Key(K::F1) => {
					self.text.push_str("F1");
				}
				Event::Key(K::F2) => {
					self.text.push_str("F2");
				}
				Event::Key(K::F3) => {
					self.text.push_str("F3");
				}
				Event::Key(K::F4) => {
					self.text.push_str("F4");
				}
				Event::Key(K::F5) => {
					self.text.push_str("F5");
				}
				Event::Key(K::F6) => {
					self.text.push_str("F6");
				}
				Event::Key(K::F7) => {
					self.text.push_str("F7");
				}
				Event::Key(K::F8) => {
					self.text.push_str("F8");
				}
				Event::Key(K::F9) => {
					self.text.push_str("F9");
				}
				Event::Key(K::F10) => {
					self.text.push_str("F10");
				}
				Event::Key(K::F11) => {
					self.text.push_str("F11");
				}
				Event::Key(K::F12) => {
					self.text.push_str("F12");
				}
				Event::Key(K::KeyA) if pressed!(Shift) && pressed!(Alt) => {
					self.text.push('Ä');
				}
				Event::Key(K::KeyA) if pressed!(Shift) => {
					self.text.push('A');
				}
				Event::Key(K::KeyA) if pressed!(Alt) => {
					self.text.push('ä');
				}
				Event::Key(K::KeyA) => {
					self.text.push('a');
				}
				Event::Key(K::KeyB) if pressed!(Shift) => {
					self.text.push('B');
				}
				Event::Key(K::KeyB) => {
					self.text.push('b');
				}
				Event::Key(K::KeyC) if pressed!(Shift) => {
					self.text.push('C');
				}
				Event::Key(K::KeyC) => {
					self.text.push('c');
				}
				Event::Key(K::KeyD) if pressed!(Shift) => {
					self.text.push('D');
				}
				Event::Key(K::KeyD) => {
					self.text.push('d');
				}
				Event::Key(K::KeyE) if pressed!(Shift) => {
					self.text.push('E');
				}
				Event::Key(K::KeyE) if pressed!(Alt) => {
					self.text.push('€');
				}
				Event::Key(K::KeyE) => {
					self.text.push('e');
				}
				Event::Key(K::KeyF) if pressed!(Shift) => {
					self.text.push('F');
				}
				Event::Key(K::KeyF) => {
					self.text.push('f');
				}
				Event::Key(K::KeyG) if pressed!(Shift) => {
					self.text.push('G');
				}
				Event::Key(K::KeyG) => {
					self.text.push('g');
				}
				Event::Key(K::KeyH) if pressed!(Shift) => {
					self.text.push('H');
				}
				Event::Key(K::KeyH) => {
					self.text.push('h');
				}
				Event::Key(K::KeyI) if pressed!(Shift) => {
					self.text.push('I');
				}
				Event::Key(K::KeyI) if pressed!(Alt) => {
					self.text.push('∞');
				}
				Event::Key(K::KeyI) => {
					self.text.push('i');
				}
				Event::Key(K::KeyJ) if pressed!(Shift) => {
					self.text.push('J');
				}
				Event::Key(K::KeyJ) => {
					self.text.push('j');
				}
				Event::Key(K::KeyK) if pressed!(Shift) => {
					self.text.push('K');
				}
				Event::Key(K::KeyK) => {
					self.text.push('k');
				}
				Event::Key(K::KeyL) if pressed!(Shift) && pressed!(Alt) => {
					self.text.push('Λ');
				}
				Event::Key(K::KeyL) if pressed!(Shift) => {
					self.text.push('L');
				}
				Event::Key(K::KeyL) if pressed!(Alt) => {
					self.text.push('λ');
				}
				Event::Key(K::KeyL) => {
					self.text.push('l');
				}
				Event::Key(K::KeyM) if pressed!(Shift) => {
					self.text.push('M');
				}
				Event::Key(K::KeyM) => {
					self.text.push('m');
				}
				Event::Key(K::KeyN) if pressed!(Shift) => {
					self.text.push('N');
				}
				Event::Key(K::KeyN) => {
					self.text.push('n');
				}
				Event::Key(K::KeyO) if pressed!(Shift) && pressed!(Alt) => {
					self.text.push('Ö');
				}
				Event::Key(K::KeyO) if pressed!(Shift) => {
					self.text.push('O');
				}
				Event::Key(K::KeyO) if pressed!(Alt) => {
					self.text.push('ö');
				}
				Event::Key(K::KeyO) => {
					self.text.push('o');
				}
				Event::Key(K::KeyP) if pressed!(Shift) && pressed!(Alt) => {
					self.text.push('Π');
				}
				Event::Key(K::KeyP) if pressed!(Shift) => {
					self.text.push('P');
				}
				Event::Key(K::KeyP) if pressed!(Alt) => {
					self.text.push('π');
				}
				Event::Key(K::KeyP) => {
					self.text.push('p');
				}
				Event::Key(K::KeyQ) if pressed!(Shift) => {
					self.text.push('Q');
				}
				Event::Key(K::KeyQ) if pressed!(Alt) => {
					self.text.push('@');
				}
				Event::Key(K::KeyQ) => {
					self.text.push('q');
				}
				Event::Key(K::KeyR) if pressed!(Shift) => {
					self.text.push('R');
				}
				Event::Key(K::KeyR) => {
					self.text.push('r');
				}
				Event::Key(K::KeyS) if pressed!(Shift) => {
					self.text.push('S');
				}
				Event::Key(K::KeyS) if pressed!(Alt) => {
					self.text.push('ß');
				}
				Event::Key(K::KeyS) => {
					self.text.push('s');
				}
				Event::Key(K::KeyT) if pressed!(Shift) => {
					self.text.push('T');
				}
				Event::Key(K::KeyT) => {
					self.text.push('t');
				}
				Event::Key(K::KeyU) if pressed!(Shift) && pressed!(Alt) => {
					self.text.push('Ü');
				}
				Event::Key(K::KeyU) if pressed!(Shift) => {
					self.text.push('U');
				}
				Event::Key(K::KeyU) if pressed!(Alt) => {
					self.text.push('ü');
				}
				Event::Key(K::KeyU) => {
					self.text.push('u');
				}
				Event::Key(K::KeyV) if pressed!(Shift) => {
					self.text.push('V');
				}
				Event::Key(K::KeyV) => {
					self.text.push('v');
				}
				Event::Key(K::KeyW) if pressed!(Shift) => {
					self.text.push('W');
				}
				Event::Key(K::KeyW) => {
					self.text.push('w');
				}
				Event::Key(K::KeyX) if pressed!(Shift) => {
					self.text.push('X');
				}
				Event::Key(K::KeyX) => {
					self.text.push('x');
				}
				Event::Key(K::KeyY) if pressed!(Shift) => {
					self.text.push('Z');
				}
				Event::Key(K::KeyY) => {
					self.text.push('z');
				}
				Event::Key(K::KeyZ) if pressed!(Shift) => {
					self.text.push('Y');
				}
				Event::Key(K::KeyZ) => {
					self.text.push('y');
				}
				Event::Key(K::LeftArrow) => {
					self.text.push('←');
				}
				Event::Key(K::LeftBracket) if pressed!(Shift) => {
					self.text.push('{');
				}
				Event::Key(K::LeftBracket) => {
					self.text.push('[');
				}
				Event::Key(K::Minus) if pressed!(Shift) => {
					self.text.push('#');
				}
				Event::Key(K::Minus) => {
					self.text.push('?');
				}
				Event::Key(K::Num0) if pressed!(Shift) => {
					self.text.push('=');
				}
				Event::Key(K::Num0) => {
					self.text.push('0');
				}
				Event::Key(K::Num1) if pressed!(Shift) => {
					self.text.push('!');
				}
				Event::Key(K::Num1) => {
					self.text.push('1');
				}
				Event::Key(K::Num2) if pressed!(Shift) => {
					self.text.push('"');
				}
				Event::Key(K::Num2) if pressed!(Alt) => {
					self.text.push('\'');
				}
				Event::Key(K::Num2) => {
					self.text.push('2');
				}
				Event::Key(K::Num3) if pressed!(Shift) => {
					self.text.push('§');
				}
				Event::Key(K::Num3) => {
					self.text.push('3');
				}
				Event::Key(K::Num4) if pressed!(Shift) => {
					self.text.push('$');
				}
				Event::Key(K::Num4) => {
					self.text.push('4');
				}
				Event::Key(K::Num5) if pressed!(Shift) => {
					self.text.push('%');
				}
				Event::Key(K::Num5) => {
					self.text.push('5');
				}
				Event::Key(K::Num6) if pressed!(Shift) => {
					self.text.push('&');
				}
				Event::Key(K::Num6) => {
					self.text.push('6');
				}
				Event::Key(K::Num7) if pressed!(Shift) => {
					self.text.push('/');
				}
				Event::Key(K::Num7) => {
					self.text.push('7');
				}
				Event::Key(K::Num8) if pressed!(Shift) => {
					self.text.push('(');
				}
				Event::Key(K::Num8) => {
					self.text.push('8');
				}
				Event::Key(K::Num9) if pressed!(Shift) => {
					self.text.push(')');
				}
				Event::Key(K::Num9) => {
					self.text.push('9');
				}
				Event::Key(K::PrintScreen) => {
					self.text.push('󱞆');
				}
				Event::Key(K::Quote) if pressed!(Shift) => {
					self.text.push('+');
				}
				Event::Key(K::Quote) => {
					self.text.push('>');
				}
				Event::Key(K::Return) => {
					self.text.push('↩');
				}
				Event::Key(K::RightArrow) => {
					self.text.push('→');
				}
				Event::Key(K::RightBracket) if pressed!(Shift) => {
					self.text.push('}');
				}
				Event::Key(K::RightBracket) => {
					self.text.push(']');
				}
				Event::Key(K::SemiColon) if pressed!(Shift) => {
					self.text.push('*');
				}
				Event::Key(K::SemiColon) => {
					self.text.push('<');
				}
				Event::Key(K::Slash) if pressed!(Shift) => {
					self.text.push('_');
				}
				Event::Key(K::Slash) => {
					self.text.push('-');
				}
				Event::Key(K::Space) => {
					self.text.push('󱁐');
				}
				Event::Key(K::Tab) => {
					self.text.push('⇥');
				}
				Event::Key(K::UpArrow) => {
					self.text.push('↑');
				}
				_ => {}
			};
		}

		let chars = self.text.chars().count();

		if chars > Self::MAX_CHARS {
			self.text = self
				.text
				.chars()
				.skip(chars - Self::MAX_CHARS)
				.collect();
		}

		TopBottomPanel::bottom("modifiers").show(ctx, |ui| {
			ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
				self.modifiers
					.iter()
					.enumerate()
					.rev()
					.map(|(modifier, pressed)| {
						RichText::new(Modifier::CHARS[modifier])
							.monospace()
							.color(match pressed {
								true => Self::THEME.text,
								false => Self::THEME.surface1,
							})
					})
					.for_each(|text| {
						ui.add_space(15.0);
						ui.label(text);
					});
			});
		});

		CentralPanel::default().show(ctx, |ui| {
			ui.label(RichText::new(&self.text).extra_letter_spacing(15.0));
		});

		ctx.request_repaint();
	}
}

impl KeyScreen {
	fn apply_theme(&self, ctx: &egui::Context) {
		catppuccin_egui::set_theme(ctx, Self::THEME);

		ctx.style_mut(|style| {
			style.visuals.window_fill = Self::THEME.crust;
			style.visuals.panel_fill = Self::THEME.crust;
		});
	}
}
