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

	let event = match event.event_type {
		rdev::EventType::KeyPress(K::ControlLeft) => {
			Event::Modifier { key: Modifier::Control, pressed: true }
		}
		rdev::EventType::KeyRelease(K::ControlLeft) => {
			Event::Modifier { key: Modifier::Control, pressed: false }
		}
		rdev::EventType::KeyPress(K::ShiftLeft) => {
			Event::Modifier { key: Modifier::Shift, pressed: true }
		}
		rdev::EventType::KeyRelease(K::ShiftLeft) => {
			Event::Modifier { key: Modifier::Shift, pressed: false }
		}
		rdev::EventType::KeyPress(K::AltGr) => {
			Event::Modifier { key: Modifier::Alt, pressed: true }
		}
		rdev::EventType::KeyRelease(K::AltGr) => {
			Event::Modifier { key: Modifier::Alt, pressed: false }
		}
		rdev::EventType::KeyPress(K::MetaLeft) => {
			Event::Modifier { key: Modifier::Meta, pressed: true }
		}
		rdev::EventType::KeyRelease(K::MetaLeft) => {
			Event::Modifier { key: Modifier::Meta, pressed: false }
		}
		rdev::EventType::KeyPress(key) => Event::Key(key),
		_ => return Ok(()),
	};

	sender
		.send(event)
		.context("Failed to send message.")
}
