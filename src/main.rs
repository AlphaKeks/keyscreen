#[cfg(not(target_os = "linux"))]
compile_error!("This program only works on Linux.");

use {
	color_eyre::{eyre::Context, Result},
	gui::KeyScreen,
	keyscreen::{Event, Modifier},
	std::sync::mpsc,
	tracing::{debug, error, info, metadata::LevelFilter},
};

mod gui;

fn main() -> Result<()> {
	color_eyre::install()?;

	#[cfg(feature = "debug")]
	let level = LevelFilter::DEBUG;

	#[cfg(not(feature = "debug"))]
	let level = LevelFilter::INFO;

	tracing_subscriber::fmt()
		.pretty()
		.with_writer(std::io::stderr)
		.with_max_level(level)
		.init();

	info!(%level, "initialized logging");

	let (sender, receiver) = mpsc::channel::<Event>();

	std::thread::spawn(move || {
		info!("spawned thread for key events");

		let err = rdev::listen(move |event| {
			debug!(?event, "received event");

			if let Err(err) = process_event(event, sender.clone()) {
				error!(?err, "failed to handle key event");
			}

			debug!("done handling event");
		});

		if let Err(err) = err {
			panic!("Failed to spawn rdev thread: {err:?}");
		}
	});

	KeyScreen::new(receiver).run();

	Ok(())
}
fn process_event(event: rdev::Event, sender: mpsc::Sender<Event>) -> Result<()> {
	use rdev::Key as K;

	if let rdev::EventType::KeyRelease(key) = event.event_type {
		let event = match key {
			K::ControlLeft => Event::Modifier { key: Modifier::Control, pressed: false },
			K::ShiftLeft => Event::Modifier { key: Modifier::Shift, pressed: false },
			K::AltGr => Event::Modifier { key: Modifier::Alt, pressed: false },
			K::MetaLeft => Event::Modifier { key: Modifier::Meta, pressed: false },
			_ => return Ok(()),
		};

		return sender
			.send(event)
			.context("Failed to send message.");
	}

	let rdev::EventType::KeyPress(key) = event.event_type else {
		return Ok(());
	};

	let event = match key {
		K::BackQuote => Event::Key("^"),
		K::BackSlash => Event::Key("/"),
		K::Backspace => Event::Key("⌫"),
		K::Comma => Event::Key(","),
		K::Dot => Event::Key("."),
		K::DownArrow => Event::Key("↓"),
		K::Equal => Event::Key("`"),
		K::Escape => Event::Key("⎋"),
		K::F1 => Event::Key("F1"),
		K::F2 => Event::Key("F2"),
		K::F3 => Event::Key("F3"),
		K::F4 => Event::Key("F4"),
		K::F5 => Event::Key("F5"),
		K::F6 => Event::Key("F6"),
		K::F7 => Event::Key("F7"),
		K::F8 => Event::Key("F8"),
		K::F9 => Event::Key("F9"),
		K::F10 => Event::Key("F10"),
		K::F11 => Event::Key("F11"),
		K::F12 => Event::Key("F12"),
		K::IntlBackslash => Event::Key("\\"),
		K::KeyA => Event::Key("a"),
		K::KeyB => Event::Key("b"),
		K::KeyC => Event::Key("c"),
		K::KeyD => Event::Key("d"),
		K::KeyE => Event::Key("e"),
		K::KeyF => Event::Key("f"),
		K::KeyG => Event::Key("g"),
		K::KeyH => Event::Key("h"),
		K::KeyI => Event::Key("i"),
		K::KeyJ => Event::Key("j"),
		K::KeyK => Event::Key("k"),
		K::KeyL => Event::Key("l"),
		K::KeyM => Event::Key("m"),
		K::KeyN => Event::Key("n"),
		K::KeyO => Event::Key("o"),
		K::KeyP => Event::Key("p"),
		K::KeyQ => Event::Key("q"),
		K::KeyR => Event::Key("r"),
		K::KeyS => Event::Key("s"),
		K::KeyT => Event::Key("t"),
		K::KeyU => Event::Key("u"),
		K::KeyV => Event::Key("v"),
		K::KeyW => Event::Key("w"),
		K::KeyX => Event::Key("x"),
		K::KeyY => Event::Key("y"),
		K::KeyZ => Event::Key("z"),
		K::LeftArrow => Event::Key("←"),
		K::LeftBracket => Event::Key("["),
		K::Minus => Event::Key("?"),
		K::Num0 => Event::Key("0"),
		K::Num1 => Event::Key("1"),
		K::Num2 => Event::Key("2"),
		K::Num3 => Event::Key("3"),
		K::Num4 => Event::Key("4"),
		K::Num5 => Event::Key("5"),
		K::Num6 => Event::Key("6"),
		K::Num7 => Event::Key("7"),
		K::Num8 => Event::Key("8"),
		K::Num9 => Event::Key("9"),
		K::PrintScreen => Event::Key("󱞆"),
		K::Quote => Event::Key(">"),
		K::Return => Event::Key("↩"),
		K::RightArrow => Event::Key("→"),
		K::RightBracket => Event::Key("]"),
		K::SemiColon => Event::Key("<"),
		K::Slash => Event::Key("-"),
		K::Space => Event::Key("󱁐"),
		K::Tab => Event::Key("⇥"),
		K::UpArrow => Event::Key("↑"),
		K::ShiftLeft => Event::Modifier { key: Modifier::Shift, pressed: true },
		K::ControlLeft => Event::Modifier { key: Modifier::Control, pressed: true },
		K::AltGr => Event::Modifier { key: Modifier::Alt, pressed: true },
		K::MetaLeft => Event::Modifier { key: Modifier::Meta, pressed: true },
		key => {
			debug!(?key, "unknown key");
			return Ok(());
		}
	};

	sender
		.send(event)
		.context("Failed to send message.")
}
