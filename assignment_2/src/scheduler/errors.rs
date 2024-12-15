use crate::models::{Job, Task, TimeStep};

#[derive(Debug)]
#[allow(dead_code)]
pub enum SchedulingError {
	DeadlineMiss { job: Job, t: TimeStep },
	PartitionedError(PartitionedError),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum PartitionedError {
	CouldNotAssignTask { task: Task },
}