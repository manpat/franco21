pub use sdl2::mouse::MouseButton;
pub use sdl2::keyboard::Scancode;
pub use sdl2::keyboard::Keycode;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Button {
	Mouse(MouseButton),
	Key(Scancode),
}


impl From<MouseButton> for Button {
	fn from(mb: MouseButton) -> Button {
		Button::Mouse(mb)
	}
}

impl From<Scancode> for Button {
	fn from(sc: Scancode) -> Button {
		Button::Key(sc)
	}
}

impl From<Keycode> for Button {
	fn from(virtual_key: Keycode) -> Button {
		Button::Key(Scancode::from_keycode(virtual_key).expect("Failed to map virtual keycode to scancode"))
	}
}


impl<T> From<&T> for Button
	where Button: From<T>, T: Copy
{
	fn from(b: &T) -> Button {
		Button::from(*b)
	}
}