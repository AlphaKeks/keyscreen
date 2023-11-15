use {
	eframe::{
		egui::{self, CentralPanel, FontData, FontFamily, RichText, TextStyle, TopBottomPanel},
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
	const THEME: catppuccin_egui::Theme = catppuccin_egui::MOCHA;
	const WINDOW_SIZE: Vec2 = Vec2::new(215.0, 175.0);

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
				.filter(|(style, _)| matches!(style, TextStyle::Body | TextStyle::Monospace))
				.for_each(|(_, font)| {
					font.size *= 5.0;
				});
		});
	}
}

impl eframe::App for KeyScreen {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.apply_theme(ctx);

		for message in self.rx.try_iter() {
			match message {
				Event::Key(key) => {
					self.text.push_str(key);
				}
				Event::Modifier { key, pressed } => {
					self.modifiers[key as usize] = pressed;
				}
			};
		}

		let chars = self.text.chars().count();

		if chars > 5 {
			self.text = self.text.chars().skip(chars - 5).collect();
		}

		TopBottomPanel::bottom("modifiers").show(ctx, |ui| {
			ui.horizontal(|ui| {
				self.modifiers
					.iter()
					.enumerate()
					.map(|(modifier, pressed)| {
						RichText::new(Modifier::CHARS[modifier])
							.monospace()
							.color(match pressed {
								true => Self::THEME.text,
								false => Self::THEME.surface1,
							})
					})
					.for_each(|text| {
						ui.label(text);
					});
			});
		});

		CentralPanel::default().show(ctx, |ui| {
			ui.label(RichText::new(&self.text).monospace());
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
