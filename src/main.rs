#![allow(dead_code)]
use sfml::graphics::{RenderWindow, RenderTarget, Color, Texture, Sprite, RenderStates, Transformable};
use sfml::window::{VideoMode, Style, ContextSettings, Event, Key};
use sfml::system::{Vector2, Vector2f};

static SPEED: f32 = 10.0;

fn main() {
	let mut window = RenderWindow::new(
		VideoMode::desktop_mode(),
		"Cool game",
		Style::default(),
		&ContextSettings::default(),
	);

	window.set_key_repeat_enabled(true);

	let texture = Texture::from_file("assets/character.png").unwrap();
	let mut sprite = Sprite::with_texture(&texture);
	sprite.scale(Vector2::new(10.0, 10.0));
	sprite.set_position(Vector2::new(
		(window.size().x / 2) as f32 - sprite.global_bounds().width / 2.0,
		(window.size().y / 2) as f32 - sprite.global_bounds().height / 2.0,
	));

	let mut w = false;
	let mut a = false;
	let mut s = false;
	let mut d = false;
	while window.is_open() {
		while let Some(event) = window.poll_event() {
			match event {
				Event::Closed => window.close(),
				Event::KeyPressed {code, ..} => {
					handle_key_press(
						&mut w,
						&mut a,
						&mut s,
						&mut d,
						code,
					);

					if w { sprite.set_position(Vector2f::new(sprite.position().x, sprite.position().y - SPEED)) };
					if s { sprite.set_position(Vector2f::new(sprite.position().x, sprite.position().y + SPEED)) };
					if a { sprite.set_position(Vector2f::new(sprite.position().x - SPEED, sprite.position().y)) };
					if d { sprite.set_position(Vector2f::new(sprite.position().x + SPEED, sprite.position().y)) };
				},
				Event::KeyReleased {code, ..} => {
					handle_key_release(
						&mut w,
						&mut a,
						&mut s,
						&mut d,
						code,
					);
				},
				_ => ()
			}
		}

		window.clear(Color::BLACK);
		window.draw_sprite(&sprite, RenderStates::default());
		window.display();
	}
}

fn handle_key_press(w: &mut bool, a: &mut bool, s: &mut bool, d: &mut bool, code: Key) {
	if code == Key::W { *w = true };
	if code == Key::A { *a = true };
	if code == Key::S { *s = true };
	if code == Key::D { *d = true };
}

fn handle_key_release(w: &mut bool, a: &mut bool, s: &mut bool, d: &mut bool, code: Key) {
	if code == Key::W { *w = false };
	if code == Key::A { *a = false };
	if code == Key::S { *s = false };
	if code == Key::D { *d = false };
}
