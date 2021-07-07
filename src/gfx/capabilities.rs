use crate::gfx::raw;

#[derive(Debug)]
pub struct Capabilities {
	pub max_simultaneous_time_elapsed_queries: i32,
	pub max_simultaneous_primitive_queries: i32,
}


impl Capabilities {
	pub(super) fn new() -> Capabilities {
		let mut max_simultaneous_time_elapsed_queries = 0;
		let mut max_simultaneous_primitive_queries = 0;

		unsafe {
			raw::GetQueryiv(raw::TIME_ELAPSED, raw::QUERY_COUNTER_BITS, &mut max_simultaneous_time_elapsed_queries);
			raw::GetQueryiv(raw::PRIMITIVES_GENERATED, raw::QUERY_COUNTER_BITS, &mut max_simultaneous_primitive_queries);
		}

		Capabilities {
			max_simultaneous_time_elapsed_queries,
			max_simultaneous_primitive_queries,
		}
	}
}