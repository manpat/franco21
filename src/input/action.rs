use crate::input::{raw, context};


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionID {
	pub(super) context_id: context::ContextID,
	pub(super) index: usize,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActionKind {
	/// One-off, immediate action
	Trigger,

	/// Continuous binary input
	State,

	/// Per-frame relative mouse input
	/// Only one per context
	Mouse,

	/// Absolute mouse position relative to window
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
pub enum ActionDefaultInfo {
	None,
	Button(raw::Button),
	Mouse { sensitivity: f32 },
}


#[derive(Debug)]
pub struct Action {
	name: String,
	kind: ActionKind,

	default_info: ActionDefaultInfo,
}


impl Action {
	pub fn new_trigger(name: impl Into<String>, default_binding: impl Into<raw::Button>) -> Action {
		Action {
			name: name.into(),
			kind: ActionKind::Trigger,
			default_info: ActionDefaultInfo::Button(default_binding.into()),
		}
	}

	pub fn new_state(name: impl Into<String>, default_binding: impl Into<raw::Button>) -> Action {
		Action {
			name: name.into(),
			kind: ActionKind::State,
			default_info: ActionDefaultInfo::Button(default_binding.into()),
		}
	}

	pub fn new_mouse(name: impl Into<String>, sensitivity: f32) -> Action {
		Action {
			name: name.into(),
			kind: ActionKind::Mouse,
			default_info: ActionDefaultInfo::Mouse { sensitivity },
		}
	}

	pub fn new_pointer(name: impl Into<String>) -> Action {
		Action {
			name: name.into(),
			kind: ActionKind::Pointer,
			default_info: ActionDefaultInfo::None,
		}
	}

	pub fn name(&self) -> &str { &self.name }
	pub fn kind(&self) -> ActionKind { self.kind }
	pub fn default_info(&self) -> ActionDefaultInfo { self.default_info }
}