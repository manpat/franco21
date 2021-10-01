
#[macro_export]
macro_rules! declare_input_context {
	(
		struct $context_ident:ident $context_name:tt {
			$(
				$binding_type:ident $binding_ident:ident { $( $binding_name_and_default:tt )+ }
			)*
		}
	) => {
		#[derive(Copy, Clone, Debug)]
		pub struct $context_ident {
			__context_id: $crate::input::ContextID,

			$(
				pub $binding_ident: $crate::input::ActionID,
			)*
		}

		impl $context_ident {
			pub fn new(system: &mut $crate::input::InputSystem) -> Self {
				let mut __ctx = system.new_context($context_name);

				$(
					let $binding_ident = $crate::__input__new_action!(__ctx, $binding_type, $($binding_name_and_default)+);
				)*

				Self {
					__context_id: __ctx.build(),
					$( $binding_ident, )*
				}
			}

			pub fn context_id(&self) -> $crate::input::ContextID { self.__context_id }
		}
	};
}


#[macro_export]
#[doc(hidden)]
macro_rules! __input__new_action {
	($ctx:ident, trigger, $name:tt [$default:expr]) => { $ctx.new_trigger($name, $default) };
	($ctx:ident, state, $name:tt [$default:expr]) => { $ctx.new_state($name, $default) };
	($ctx:ident, mouse, $name:tt [$default:expr]) => { $ctx.new_mouse($name, $default) };
	($ctx:ident, pointer, $name:tt) => { $ctx.new_pointer($name) };
}

