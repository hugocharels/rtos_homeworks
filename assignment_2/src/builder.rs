use crate::scheduler::{
	edfk::EDFK,
	global_edf::GlobalEDF,
	heuristics::best_fit::BestFit,
	heuristics::first_fit::FirstFit,
	heuristics::next_fit::NextFit,
	heuristics::strategy::HeuristicStrategy,
	heuristics::worst_fit::WorstFit,
	orderings::decreasing::Decreasing,
	orderings::increasing::Increasing,
	orderings::strategy::OrderingStrategy,
	partitioned::Partitioned,
	scheduler::Scheduler,
};

#[derive(Debug)]
pub struct Builder<'a> {
	version: Option<&'a String>,
	heuristic: Option<&'a String>,
	ordering: Option<&'a String>,
}

impl<'a> Builder<'a> {
	pub fn new() -> Self {
		Self {
			version: None,
			heuristic: None,
			ordering: None,
		}
	}

	pub fn set_version(mut self, version: &'a String) -> Self {
		self.version = Some(version);
		self
	}

	pub fn set_heuristic(mut self, heuristic: Option<&'a String>) -> Self {
		self.heuristic = heuristic;
		self
	}

	pub fn set_ordering(mut self, ordering: Option<&'a String>) -> Self {
		self.ordering = ordering;
		self
	}

	pub fn build(self) -> Option<Box<dyn Scheduler>> {
		if self.version.is_none() { return None; }
		match self.version.unwrap().as_str() {
			"global" => {
				if self.heuristic.is_none() || self.ordering.is_none() {
					return Some(Box::new(GlobalEDF) as Box<dyn Scheduler>);
				}
				None
			}
			"partitioned" => {
				if !(self.heuristic.is_none() || self.ordering.is_none()) {
					let mut partitioned = Partitioned::new();
					let heuristic = match self.heuristic.unwrap().as_str() {
						"ff" => Box::new(FirstFit) as Box<dyn HeuristicStrategy>,
						"nf" => Box::new(NextFit),
						"bf" => Box::new(BestFit),
						"wf" => Box::new(WorstFit),
						_ => return None,
					};
					let ordering = match self.ordering.unwrap().as_str() {
						"iu" => Box::new(Increasing) as Box<dyn OrderingStrategy>,
						"du" => Box::new(Decreasing),
						_ => return None,
					};
					partitioned.set_heuristic(heuristic);
					partitioned.set_ordering(ordering);
					return Some(Box::new(partitioned) as Box<dyn Scheduler>);
				}
				None
			}
			_ => {
				let k = self.version.unwrap().parse::<usize>().unwrap();
				if self.heuristic.is_none() || self.ordering.is_none() {
					return Some(Box::new(EDFK::new(k)) as Box<dyn Scheduler>);
				}
				None
			}
		}
	}
}