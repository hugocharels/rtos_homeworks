use crate::models::TaskSet;

pub trait OrderingStrategy: Sync + Send {
	fn apply_order(&self, task_set: &mut TaskSet);
}