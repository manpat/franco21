pub mod raw;
pub mod system;
pub mod action;
pub mod context;
pub mod context_macro;

pub use system::InputSystem;
pub use raw::{MouseButton, Scancode, Keycode, Button};
pub use action::*;
pub use context::{ContextID, InputContext};

// https://www.gamedev.net/tutorials/_/technical/game-programming/designing-a-robust-input-handling-system-for-games-r2975/
