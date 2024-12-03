use crate::models::TaskSet;

pub trait OrderingStrategy {
	fn apply_order(&self, taskset: &mut TaskSet);
}