use crate::gfx;

use std::time;


#[derive(Copy, Clone, Debug)]
enum State {
	Recording,
	Waiting,
}

pub struct Instrumenter {
	section_cache: Vec<Section>,
	recording_section: Option<Section>,
	waiting_sections: Vec<Section>,
	state: State,

	summary: Option<Summary>,
}

#[derive(Clone, Debug)]
pub struct Summary {
	pub total_triangles: usize,
	pub total_gpu_time_ms: f64,
	pub total_cpu_time_ms: f64,
	pub sections: Vec<SectionSummary>,
}

#[derive(Clone, Debug)]
pub struct SectionSummary {
	pub name: String,
	pub triangles: usize,
	pub gpu_time_ms: f64,
	pub cpu_time_ms: f64,
}


// TODO(pat.m): queries for total frame time/geometry - use indexed queries
impl Instrumenter {
	pub fn new(gl_ctx: &mut gfx::Context) -> Instrumenter {
		assert!(gl_ctx.capabilities().max_simultaneous_primitive_queries > 0);
		assert!(gl_ctx.capabilities().max_simultaneous_time_elapsed_queries > 0);

		let mut section_cache = Vec::new();

		for _ in 0..20 {
			section_cache.push(Section::new(gl_ctx));
		}

		Instrumenter {
			section_cache,
			recording_section: None,
			waiting_sections: Vec::new(),
			state: State::Recording,

			summary: None,
		}
	}

	pub fn summary(&self) -> Option<&'_ Summary> {
		self.summary.as_ref()
	}

	pub fn start_section(&mut self, name: &str) {
		match self.state {
			State::Recording => {},
			State::Waiting => return,
		};

		if self.recording_section.is_some() {
			self.end_section();
		}

		let mut section = self.section_cache.pop()
			.expect("Query cache empty!");

		section.start(name.into());

		self.recording_section = Some(section);
	}

	pub fn end_section(&mut self) {
		match self.state {
			State::Recording => {},
			State::Waiting => return,
		};

		let mut section = self.recording_section.take()
			.expect("Mismatched start/end query section!");

		section.end();

		self.waiting_sections.push(section);
	}


	pub fn scoped_section(&mut self, name: &str) -> ScopedSection<'_> {
		self.start_section(name);
		ScopedSection(self)
	}


	pub fn end_frame(&mut self) {
		if self.recording_section.is_some() {
			self.end_section();
		}

		self.state = State::Waiting;

		let queries_ready = self.waiting_sections.iter_mut()
			.map(Section::result)
			.all(|r| r.is_some());

		if queries_ready {
			let mut total_gpu_time_ms = 0.0f64;
			let mut total_cpu_time_ms = 0.0f64;
			let mut total_triangles = 0usize;

			let mut sections = Vec::with_capacity(self.waiting_sections.len());

			for mut section in self.waiting_sections.drain(..) {
				let (time_nanos, triangles, duration) = section.result().unwrap();
				let gpu_time_ms = time_nanos as f64 / 1000_000.0;
				let cpu_time_ms = duration.as_secs_f64() * 1000.0;

				sections.push(SectionSummary {
					name: std::mem::take(&mut section.name),
					triangles,
					gpu_time_ms,
					cpu_time_ms,
				});

				total_gpu_time_ms += gpu_time_ms;
				total_cpu_time_ms += cpu_time_ms;
				total_triangles += triangles;

				self.section_cache.push(section);
			}

			self.summary = Some(Summary {
				total_triangles,
				total_gpu_time_ms,
				total_cpu_time_ms,
				sections,
			});

			self.state = State::Recording;
		}
	}
}




struct Section {
	name: String,
	timer_query_object: gfx::QueryObject,
	geo_query_object: gfx::QueryObject,

	timer_query: Option<gfx::InFlightQuery>,
	geo_query: Option<gfx::InFlightQuery>,

	start_time: time::Instant,
	end_time: time::Instant,
}

impl Section {
	fn new(gl_ctx: &mut gfx::Context) -> Section {
		Section {
			name: String::new(),
			timer_query_object: gl_ctx.new_query(),
			geo_query_object: gl_ctx.new_query(),

			timer_query: None,
			geo_query: None,

			start_time: time::Instant::now(),
			end_time: time::Instant::now(),
		}
	}

	fn start(&mut self, name: String) {
		self.timer_query = Some(self.timer_query_object.start_query(gfx::QueryType::Timer));
		self.geo_query = Some(self.geo_query_object.start_query(gfx::QueryType::Primitive));

		self.name = name;
		self.start_time = time::Instant::now();
	}

	fn end(&mut self) {
		if let Some(query) = self.timer_query.as_mut() {
			query.end_query();
		}

		if let Some(query) = self.geo_query.as_mut() {
			query.end_query();
		}

		self.end_time = time::Instant::now();
	}

	fn result(&mut self) -> Option<(usize, usize, time::Duration)> {
		let timer_value = self.timer_query.as_mut().and_then(gfx::InFlightQuery::query_result)?;
		let geo_value = self.geo_query.as_mut().and_then(gfx::InFlightQuery::query_result)?;

		let duration = self.end_time.saturating_duration_since(self.start_time);
		Some((timer_value as usize, geo_value as usize, duration))
	}
}



pub struct ScopedSection<'inst> (&'inst mut Instrumenter);

impl<'inst> Drop for ScopedSection<'inst> {
	fn drop(&mut self) {
		self.0.end_section();
	}
}

