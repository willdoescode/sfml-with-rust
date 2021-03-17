use sfml::graphics::{RenderWindow, RenderTarget, Color, Texture, Sprite, RenderStates};
use sfml::window::{VideoMode, Style, ContextSettings, Event};

fn main() {
	let mut window = RenderWindow::new(
		VideoMode::desktop_mode(),
		"Cool game",
		Style::default(),
		&ContextSettings::default()
	);

	let texture = Texture::from_file("assets/character.png").unwrap();
	let sprite = Sprite::with_texture(&texture);

	while window.is_open() {
		match window.poll_event() {
			Some(Event::Closed) => window.close(),
			_ => ()
		}

		window.clear(Color::BLACK);
		window.draw_sprite(&sprite, RenderStates::default());
		window.display();
	}
}
