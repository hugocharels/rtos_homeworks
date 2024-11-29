use crate::scheduler::heuristics::strategy::HeuristicStrategy;

pub struct FirstFit;

impl FirstFit {
	pub fn new() -> Self {
		FirstFit
	}
}

impl HeuristicStrategy for FirstFit {
	fn somethings(&self) {
		unimplemented!()
	}
}