use crate::input::raw;
use crate::input::action::{self, Action, ActionID};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContextID(pub(super) usize);


/// A set of actions that can be bound to system inputs
/// Can be enabled/disabled based on game state and several can be enabled at once
/// Also holds conversions for Axis based actions

#[derive(Debug)]
pub struct InputContext {
	name: String,
	id: ContextID,

	actions: Vec<Action>,

	/// The active bindings from buttons to an action index
	button_mappings: HashMap<raw::Button, usize>,

	/// The current sensitivity for any Mouse action, if there is one
	mouse_sensitivity: Option<f32>,
}

impl InputContext {
	pub(super) fn new_empty(name: String, id: ContextID) -> InputContext {
		InputContext {
			name,
			id,
			actions: Vec::new(),
			button_mappings: HashMap::new(),
			mouse_sensitivity: None,
		}
	}

	pub fn mouse_action(&self) -> Option<(&Action, ActionID)> {
		let context_id = self.id;

		self.actions.iter()
			.enumerate()
			.find(|(_, a)| a.kind().is_mouse_kind())
			.map(|(index, action)| (action, ActionID {context_id, index}))
	}

	pub fn action_for_button(&self, button: raw::Button) -> Option<(&Action, ActionID)> {
		let context_id = self.id;

		self.button_mappings.get(&button)
			.map(|&index| (&self.actions[index], ActionID {context_id, index}))
	}

	pub fn mouse_sensitivity(&self) -> Option<f32> {
		self.mouse_sensitivity
	}

	fn build_default_bindings(&mut self) {
		use action::BindingInfo;

		self.button_mappings = self.actions.iter()
			.enumerate()
			.filter_map(|(index, a)| match a.default_binding_info() {
				BindingInfo::Button(b) => Some((b, index)),
				_ => None,
			})
			.collect();

		self.mouse_sensitivity = self.actions.iter()
			.find_map(|action| match action.default_binding_info() {
				BindingInfo::Mouse{sensitivity} => Some(sensitivity),
				_ => None,
			});
	}
}



pub struct Builder<'is> {
	context: &'is mut InputContext,
}


impl<'is> Builder<'is> {
	pub(super) fn new(context: &'is mut InputContext) -> Builder {
		Builder {
			context,
		}
	}


	pub fn build(self) -> ContextID {
		self.context.build_default_bindings();
		self.context.id
	}

	pub fn new_action(&mut self, action: Action) -> ActionID {
		self.context.actions.push(action);

		ActionID {
			context_id: self.context.id,
			index: self.context.actions.len()-1,
		}
	}

	pub fn new_trigger(&mut self, name: impl Into<String>, default_binding: impl Into<raw::Button>) -> ActionID {
		self.new_action(Action::new_trigger(name, default_binding))
	}

	pub fn new_state(&mut self, name: impl Into<String>, default_binding: impl Into<raw::Button>) -> ActionID {
		self.new_action(Action::new_state(name, default_binding))
	}

	pub fn new_mouse(&mut self, name: impl Into<String>, default_sensitivity: f32) -> ActionID {
		self.new_action(Action::new_mouse(name, default_sensitivity))
	}

	pub fn new_pointer(&mut self, name: impl Into<String>) -> ActionID {
		self.new_action(Action::new_pointer(name))
	}
}