use crate::scheduler::edf::EDF;
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
					// TODO
				}
				None
			}
			"partitioned" => {
				if !(self.heuristic.is_none() || self.ordering.is_none()) {
					// TODO
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