use crate::models::TaskSet;
use crate::scheduler::orderings::strategy::OrderingStrategy;

pub struct Decreasing;

impl Decreasing {}


impl OrderingStrategy for Decreasing {
	fn apply_order(&self, taskset: &mut TaskSet) {
		taskset.tasks().sort_by(|a, b| b.utilization().partial_cmp(&a.utilization()).unwrap());
	}
}