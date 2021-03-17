use sfml::graphics::{RenderWindow, RenderTarget, Color, Texture, Sprite, RenderStates, Transformable};
use sfml::window::{VideoMode, Style, ContextSettings, Event};
use sfml::system::Vector2;

fn main() {
	let mut window = RenderWindow::new(
		VideoMode::desktop_mode(),
		"Cool game",
		Style::default(),
		&ContextSettings::default()
	);

	let texture = Texture::from_file("assets/character.png").unwrap();
	let mut sprite = Sprite::with_texture(&texture);
	sprite.scale(Vector2::new(10.0, 10.0));
	sprite.set_position(Vector2::new(
		(window.size().x / 2) as f32,
		(window.size().y / 2) as f32
	));

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
