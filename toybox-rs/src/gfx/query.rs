use crate::gfx;


#[derive(Copy, Clone, Debug)]
pub enum QueryType {
	Primitive,
	Timer,
}

impl QueryType {
	fn to_gl(self) -> u32 {
		match self {
			QueryType::Primitive => gfx::raw::PRIMITIVES_GENERATED,
			QueryType::Timer => gfx::raw::TIME_ELAPSED,
		}
	}
}



#[derive(Debug)]
pub struct QueryObject(pub(super) u32);

impl QueryObject {
	#[must_use]
	pub fn start_query(&self, query_type: QueryType) -> InFlightQuery {
		unsafe {
			gfx::raw::BeginQuery(query_type.to_gl(), self.0);
		}

		InFlightQuery::InProgress {
			handle: self.clone(),
			query_type
		}
	}

	pub fn ready(&self) -> bool {
		unsafe {
			let mut ready = 0;
			gfx::raw::GetQueryObjectiv(self.0, gfx::raw::QUERY_RESULT_AVAILABLE, &mut ready);
			ready != 0
		}
	}

	fn fetch_value(&self) -> u64 {
		unsafe {
			let mut value = 0;
			gfx::raw::GetQueryObjectui64v(self.0, gfx::raw::QUERY_RESULT, &mut value);
			value
		}
	}

	fn clone(&self) -> QueryObject {
		QueryObject(self.0)
	}
}


#[derive(Debug)]
pub enum InFlightQuery {
	InProgress {
		handle: QueryObject,
		query_type: QueryType,
	},

	Pending {
		handle: QueryObject,
	},

	Ready {
		value: u64,
	},
}

impl InFlightQuery {
	pub fn end_query(&mut self) {
		match self {
			InFlightQuery::InProgress { handle, query_type } => {
				unsafe {
					gfx::raw::EndQuery(query_type.to_gl());
				}

				*self = InFlightQuery::Pending { handle: handle.clone() }
			}

			InFlightQuery::Pending { .. } => {}
			InFlightQuery::Ready { .. } => {}
		}
	}

	pub fn query_result(&mut self) -> Option<u64> {
		match self {
			InFlightQuery::InProgress { .. } => None,
			InFlightQuery::Ready { value } => Some(*value),
			InFlightQuery::Pending { handle } => if handle.ready() {
				let value = handle.fetch_value();
				*self = InFlightQuery::Ready { value };
				Some(value)
			} else {
				None
			}
		}
	}
}