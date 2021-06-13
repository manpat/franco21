use crate::input::raw;
use crate::input::action::{self, ActionID};
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

	actions: Vec<action::Action>,

	/// The active bindings from buttons to an action index
	button_mappings: HashMap<raw::Button, usize>,
}

impl InputContext {
	pub(super) fn new_empty(name: String, id: ContextID) -> InputContext {
		InputContext {
			name,
			id,
			actions: Vec::new(),
			button_mappings: HashMap::new(),
		}
	}

	pub fn mouse_action(&self) -> Option<(&action::Action, ActionID)> {
		let context_id = self.id;

		self.actions.iter()
			.enumerate()
			.find(|(_, a)| a.kind().is_mouse_kind())
			.map(|(index, action)| (action, ActionID {context_id, index}))
	}

	pub fn action_for_button(&self, button: raw::Button) -> Option<(&action::Action, ActionID)> {
		let context_id = self.id;

		self.button_mappings.get(&button)
			.map(|&index| (&self.actions[index], ActionID {context_id, index}))
	}

	fn build_default_bindings(&mut self) {
		use action::ActionDefaultInfo;

		self.button_mappings = self.actions.iter()
			.enumerate()
			.filter_map(|(index, a)| match a.default_info() {
				ActionDefaultInfo::Button(b) => Some((b, index)),
				_ => None,
			})
			.collect()
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

	pub fn new_action(&mut self, action: action::Action) -> action::ActionID {
		self.context.actions.push(action);

		action::ActionID {
			context_id: self.context.id,
			index: self.context.actions.len()-1,
		}
	}

}