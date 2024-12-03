use crate::models::TaskSet;
use crate::scheduler::orderings::strategy::OrderingStrategy;

pub struct Increasing;

impl Increasing {}

impl OrderingStrategy for Increasing {
	fn apply_order(&self, taskset: &mut TaskSet) {
		taskset.tasks().sort_by(|a, b| a.utilization().partial_cmp(&b.utilization()).unwrap());
	}
}