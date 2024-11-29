use crate::{
	models::{Job, TaskSet, TimeStep},
	scheduler::result::SchedulabilityResult,
	scheduler::scheduler::Scheduler,
	scheduler::simulator::SchedulerSimulator,
};
use rayon::prelude::*;

pub struct EDF {
	k: u32,
}

impl EDF {
	pub fn new(k: u32) -> Self {
		Self { k }
	}
}

impl Scheduler for EDF {
	fn is_schedulable(&self, task_set: &mut TaskSet, cores: &u32) -> SchedulabilityResult {
		SchedulabilityResult::Unknown
	}
}

impl SchedulerSimulator for EDF {
	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job> {
		queue.par_iter_mut().min_by_key(|j| j.deadline())
	}

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		let mut l = task_set.tasks().par_iter().map(|task| task.wcet()).sum::<TimeStep>();
		loop {
			let l_next = task_set.tasks().par_iter()
				.map(|task| ((l as f64 / task.period() as f64).ceil() as TimeStep) * task.wcet())
				.sum::<TimeStep>();
			if l_next == l {
				break;
			}
			l = l_next;
		}
		l
	}
}