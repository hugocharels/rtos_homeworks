use crate::models::{Job, Task, TimeStep};

#[derive(Debug)]
pub enum SchedulingError {
	DeadlineMiss { job: Job, t: TimeStep },
}

#[derive(Debug)]
pub enum PartitionedError {
	CouldNotAssignTask { task: Task },
}