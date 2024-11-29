use crate::scheduler::heuristics::strategy::HeuristicStrategy;

pub struct WorstFit;

impl WorstFit {
	pub fn new() -> Self {
		WorstFit
	}
}

impl HeuristicStrategy for WorstFit {
	fn somethings(&self) {
		unimplemented!()
	}
}