use crate::scheduler::edf::EDF;
use crate::scheduler::global::Global;
use crate::scheduler::heuristics::best_fit::BestFit;
use crate::scheduler::heuristics::first_fit::FirstFit;
use crate::scheduler::heuristics::next_fit::NextFit;
use crate::scheduler::heuristics::strategy::HeuristicStrategy;
use crate::scheduler::heuristics::worst_fit::WorstFit;
use crate::scheduler::orderings::decreasing::Decreasing;
use crate::scheduler::orderings::increasing::Increasing;
use crate::scheduler::orderings::strategy::OrderingStrategy;
use crate::scheduler::partitionned::Partitioned;
use crate::scheduler::scheduler::Scheduler;


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
		println!("{:?}", self);
		if self.version.is_none() { return None; }

		match self.version.unwrap().as_str() {
			"global" => {
				if self.heuristic.is_none() || self.ordering.is_none() {
					return Some(Box::new(Global) as Box<dyn Scheduler>);
				}
				None
			}
			"partitioned" => {
				if !(self.heuristic.is_none() || self.ordering.is_none()) {
					// TODO
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
				let k = self.version.unwrap().parse::<u32>().unwrap();
				if self.heuristic.is_none() || self.ordering.is_none() {
					return Some(Box::new(EDF::new(k)) as Box<dyn Scheduler>);
				}
				None
			}
		}
	}
}