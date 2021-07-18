use common::math::*;
use crate::input::raw;
use crate::input::action::{ActionID, ActionKind};
use crate::input::context::{self, ContextID, InputContext};
use std::collections::HashMap;

// TODO(pat.m): is this actually useful if everything will go through 'sensitivty' anyway?
const RELATIVE_MOUSE_PIXELS_TO_AXIS_FACTOR: f32 = 1.0 / 100.0;


pub struct InputSystem {
	/// All declared input contexts
	contexts: Vec<InputContext>,

	/// Active input contexts, ordered by priority in reverse order.
	/// Contexts at the end will recieve actions first
	active_contexts: Vec<ContextID>,

	/// Set when a context has been pushed or popped
	/// Will cause mouse capture state to be reevaluated when set
	active_contexts_changed: bool,

	/// Used for remapping mouse input
	mouse_interactive_region: Vec2,


	/// The current mouse position in screenspace
	/// Normalised to window height, and will be None if a capturing input context is active
	/// and also if focus is lost
	mouse_absolute: Option<Vec2>,

	/// The mouse delta recorded this frame - if there is one
	/// Used for mouse capturing input contexts
	mouse_delta: Option<Vec2>,

	/// Buttons currently being held
	active_buttons: Vec<raw::Button>,

	/// Buttons that have become pressed this frame
	new_buttons: Vec<raw::Button>,


	frame_state: FrameState,
	prev_frame_state: FrameState,


	sdl2_mouse: sdl2::mouse::MouseUtil,
}

impl InputSystem {
	pub(crate) fn new(sdl2_mouse: sdl2::mouse::MouseUtil, window: &sdl2::video::Window) -> InputSystem {
		let (w, h) = window.drawable_size();

		InputSystem {
			contexts: Vec::new(),
			active_contexts: Vec::new(),
			active_contexts_changed: false,

			mouse_interactive_region: Vec2::new(w as f32, h as f32),

			mouse_absolute: None,
			mouse_delta: None,

			active_buttons: Vec::new(),
			new_buttons: Vec::new(),

			frame_state: FrameState::default(),
			prev_frame_state: FrameState::default(),

			sdl2_mouse,
		}
	}

	pub(crate) fn clear(&mut self) {
		self.mouse_delta.take();
		self.new_buttons.clear();

		if self.active_contexts_changed {
			self.active_contexts_changed = false;

			// Find last active context using mouse input; we want to enable relative mouse mode if it's relative
			let should_capture_mouse = self.active_contexts.iter().rev()
				.flat_map(|&ContextID(id)| self.contexts.get(id))
				.find_map(InputContext::mouse_action)
				.map_or(false, |(action, _)| action.kind() == ActionKind::Mouse);

			self.sdl2_mouse.set_relative_mouse_mode(should_capture_mouse);
		}
	}

	pub(crate) fn handle_event(&mut self, event: &sdl2::event::Event) {
		use sdl2::event::{Event, WindowEvent};

		match event {
			&Event::Window{ win_event: WindowEvent::Resized(w, h), .. } => {
				// TODO(pat.m): this event doesn't get emitted on startup
				self.mouse_interactive_region = Vec2::new(w as f32, h as f32);
			}

			Event::Window{ win_event: WindowEvent::Leave, .. } => {
				self.mouse_absolute = None;
			}

			Event::Window{ win_event: WindowEvent::FocusLost, .. } => {
				self.active_buttons.clear();
			}

			// Event::MouseWheel { y, .. } => {
			// }

			&Event::MouseMotion { xrel, yrel, x, y, .. } => {
				let Vec2{x: w, y: h} = self.mouse_interactive_region;
				// TODO(pat.m): is it actually useful to remap coordinates like this?
				let mouse_x =  (x as f32 / w * 2.0 - 1.0) * (w/h);
				let mouse_y = -(y as f32 / h * 2.0 - 1.0);
				self.mouse_absolute = Some(Vec2::new(mouse_x, mouse_y));

				let mouse_dx =  xrel as f32 * RELATIVE_MOUSE_PIXELS_TO_AXIS_FACTOR;
				let mouse_dy = -yrel as f32 * RELATIVE_MOUSE_PIXELS_TO_AXIS_FACTOR;

				let mouse_delta = Vec2::new(mouse_dx, mouse_dy);
				let current_delta = self.mouse_delta.get_or_insert_with(Vec2::zero);
				*current_delta += mouse_delta;
			}

			Event::MouseButtonDown { mouse_btn, .. } => self.track_button_change(mouse_btn.into(), true),
			Event::MouseButtonUp { mouse_btn, .. } => self.track_button_change(mouse_btn.into(), false),

			Event::KeyDown { scancode: Some(scancode), .. } => self.track_button_change(scancode.into(), true),
			Event::KeyUp { scancode: Some(scancode), .. } => self.track_button_change(scancode.into(), false),

			_ => {}
		}
	}

	pub(crate) fn process_events(&mut self) {
		std::mem::swap(&mut self.frame_state, &mut self.prev_frame_state);

		self.frame_state.button.clear();
		self.frame_state.mouse.take();

		// Calculate mouse action
		let mouse_action = self.active_contexts.iter().rev()
			.flat_map(|&ContextID(id)| self.contexts.get(id))
			.find_map(|ctx| ctx.mouse_action().zip(Some(ctx)));

		if let Some(((action, action_id), context)) = mouse_action {
			if action.kind().is_relative() {
				let sensitivity = context.mouse_sensitivity().unwrap_or(1.0);
				self.frame_state.mouse = self.mouse_delta.map(|state| (action_id, state * sensitivity));
			} else {
				self.frame_state.mouse = self.mouse_absolute.map(|state| (action_id, state));
			}
		}

		// Collect new button actions
		for &button in self.new_buttons.iter() {
			let most_appropriate_action = self.active_contexts.iter().rev()
				.flat_map(|&ContextID(id)| self.contexts.get(id))
				.find_map(|ctx| ctx.action_for_button(button));

			if let Some((_, action_id)) = most_appropriate_action {
				self.frame_state.button.insert(action_id, ActionState::Entered);
			}
		}

		// Collect stateful button actions - triggers _only_ run on button down events
		for &button in self.active_buttons.iter() {
			let most_appropriate_action = self.active_contexts.iter().rev()
				.flat_map(|&ContextID(id)| self.contexts.get(id))
				.flat_map(|ctx| ctx.action_for_button(button))
				.find(|(action, _)| action.kind() == ActionKind::State);

			if let Some((_, action_id)) = most_appropriate_action {
				// If this button was previously entered or active, remain active
				if self.prev_frame_state.button.get(&action_id)
					.filter(|&&state| state != ActionState::Left)
					.is_some()
				{
					self.frame_state.button.insert(action_id, ActionState::Active);
				} else {
					self.frame_state.button.insert(action_id, ActionState::Entered);
				}
			}
		}


		// Combine current active actions with previous frame state
		for (&action_id, _) in self.prev_frame_state.button.iter()
			.filter(|(_, state)| **state != ActionState::Left)
		{
			// If a previously active action doesn't appear in the new framestate
			// register it as a deactivation
			self.frame_state.button.entry(action_id)
				.or_insert(ActionState::Left);
		}
	}

	pub fn new_context(&mut self, name: impl Into<String>) -> context::Builder<'_> {
		let context_id = context::ContextID(self.contexts.len());
		let context = InputContext::new_empty(name.into(), context_id);

		self.contexts.push(context);

		context::Builder::new(self.contexts.last_mut().unwrap())
	}

	pub fn enter_context(&mut self, context_id: ContextID) {
		assert!(!self.active_contexts.contains(&context_id));
		self.active_contexts.push(context_id);
		self.active_contexts_changed = true;
	}

	pub fn leave_context(&mut self, context_id: ContextID) {
		if let Some(context_pos) = self.active_contexts.iter().position(|&id| id == context_id) {
			self.active_contexts.remove(context_pos);
		}

		self.active_contexts_changed = true;
	}

	pub fn is_context_active(&self, context_id: ContextID) -> bool {
		self.active_contexts.contains(&context_id)
	}

	pub fn frame_state(&self) -> &FrameState {
		&self.frame_state
	}

	pub fn contexts(&self) -> impl Iterator<Item = &'_ InputContext> {
		self.contexts.iter()
	}

	pub fn active_contexts(&self) -> impl Iterator<Item = &'_ InputContext> {
		self.active_contexts.iter()
			.filter_map(move |id| self.contexts.get(id.0))
	}

	fn track_button_change(&mut self, button: raw::Button, down: bool) {
		let button_is_active = self.active_buttons.contains(&button);

		if down && !button_is_active {
			self.active_buttons.push(button);
			self.new_buttons.push(button);
		}

		if !down && button_is_active {
			self.active_buttons.retain(|&b| b != button);
		}
	}
}




#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ActionState {
	Entered,
	Active,
	Left,
}


/// The complete state of input for a frame - after system inputs have been parsed into actions
#[derive(Clone, Debug, Default)]
pub struct FrameState {
	/// All the button actions that are active or that changed this frame
	button: HashMap<ActionID, ActionState>,

	/// Mouse state if it is currently available, and the action its bound to
	mouse: Option<(ActionID, Vec2)>,
}


impl FrameState {
	/// For Triggers: returns whether action was triggered this frame
	/// For States: returns whether action is currently active (button is being held)
	pub fn active(&self, action: ActionID) -> bool {
		self.button.get(&action)
			.map_or(false, |state| matches!(state, ActionState::Entered | ActionState::Active))
	}

	/// Whether a state or trigger was actived this frame
	pub fn entered(&self, action: ActionID) -> bool {
		self.button.get(&action)
			.map_or(false, |state| matches!(state, ActionState::Entered))
	}

	/// Whether the state was deactivated this frame
	pub fn left(&self, action: ActionID) -> bool {
		self.button.get(&action)
			.map_or(false, |state| matches!(state, ActionState::Left))
	}

	pub fn mouse(&self, action: ActionID) -> Option<Vec2> {
		self.mouse
			.filter(|&(mouse_action, _)| mouse_action == action)
			.map(|(_, state)| state)
	}
}