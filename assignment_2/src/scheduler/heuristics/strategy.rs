use crate::models::{Job, TaskSet};

pub trait HeuristicStrategy {
	fn assign_cores(&self, taskset: &mut TaskSet, cores: usize);
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job>;
}