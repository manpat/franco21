use crate::input::{raw, context};


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionID {
	pub(super) context_id: context::ContextID,
	pub(super) index: usize,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActionKind {
	/// One-off, immediate action
	/// On button down, will emit an Enter state, immediately followed by a Leave state.
	/// Triggers will never enter the Active state, but FrameState::active can still be used.
	/// Will not trigger if its owning context is activated while its bound keys are held - only triggers on button down while context is active
	Trigger,

	/// Continuous binary input
	/// Will emit Enter and Leave states on button down/up, and will remain in the Active state while the button is held.
	/// If the actions bound button is held when its owning context is activated, it will emit Enter and Active states.
	/// Similarly, if the owning context is disabled while the action is active, it will emit a Leave state
	State,

	/// Per-frame relative mouse input
	/// The cursor will be put into 'relative' mode while the owning context is the topmost context with a mouse action.
	/// Input will only be available while the mouse is moving within the window
	/// Cannot exist in a context with any other Mouse or Pointer actions
	Mouse,

	/// Absolute mouse position relative to window
	/// Input will only be available while the mouse is within the window, and will be normalised to the logical height of the window
	/// Cannot exist in a context with any other Mouse or Pointer actions
	Pointer,
}

impl ActionKind {
	pub fn is_mouse_kind(&self) -> bool {
		matches!(*self, ActionKind::Mouse | ActionKind::Pointer)
	}

	pub fn is_button_kind(&self) -> bool {
		matches!(*self, ActionKind::Trigger | ActionKind::State)
	}

	pub fn is_relative(&self) -> bool {
		matches!(*self, ActionKind::Mouse)
	}
}


#[derive(Debug, Copy, Clone)]
pub enum BindingInfo {
	Button(raw::Button),
	Mouse { sensitivity: f32 },
	Pointer,
}


#[derive(Debug)]
pub struct Action {
	name: String,
	kind: ActionKind,

	default_binding_info: BindingInfo,
}


impl Action {
	pub fn new_trigger(name: impl Into<String>, default_binding: impl Into<raw::Button>) -> Action {
		Action {
			name: name.into(),
			kind: ActionKind::Trigger,
			default_binding_info: BindingInfo::Button(default_binding.into()),
		}
	}

	pub fn new_state(name: impl Into<String>, default_binding: impl Into<raw::Button>) -> Action {
		Action {
			name: name.into(),
			kind: ActionKind::State,
			default_binding_info: BindingInfo::Button(default_binding.into()),
		}
	}

	pub fn new_mouse(name: impl Into<String>, default_sensitivity: f32) -> Action {
		Action {
			name: name.into(),
			kind: ActionKind::Mouse,
			default_binding_info: BindingInfo::Mouse { sensitivity: default_sensitivity },
		}
	}

	pub fn new_pointer(name: impl Into<String>) -> Action {
		Action {
			name: name.into(),
			kind: ActionKind::Pointer,
			default_binding_info: BindingInfo::Pointer,
		}
	}

	pub fn name(&self) -> &str { &self.name }
	pub fn kind(&self) -> ActionKind { self.kind }
	pub fn default_binding_info(&self) -> BindingInfo { self.default_binding_info }
}