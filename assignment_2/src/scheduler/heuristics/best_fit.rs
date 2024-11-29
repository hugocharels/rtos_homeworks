use crate::scheduler::heuristics::strategy::HeuristicStrategy;

pub struct BestFit;

impl BestFit {
	pub fn new() -> Self {
		BestFit
	}
}

impl HeuristicStrategy for BestFit {
	fn somethings(&self) {
		unimplemented!()
	}
}