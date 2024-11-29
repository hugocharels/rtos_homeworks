use crate::scheduler::heuristics::strategy::HeuristicStrategy;

pub struct NextFit;

impl NextFit {
	pub fn new() -> Self {
		NextFit
	}
}

impl HeuristicStrategy for NextFit {
	fn somethings(&self) {
		unimplemented!()
	}
}
