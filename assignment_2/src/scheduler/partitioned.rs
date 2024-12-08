use crate::models::{Job, Task, TaskSet, TimeStep};
use crate::scheduler::{
	heuristics::strategy::HeuristicStrategy,
	orderings::strategy::OrderingStrategy,
	result::SchedulabilityResult,
	scheduler::Scheduler,
	simulator::SchedulerSimulator,
};

pub struct Partitioned {
	heuristic: Option<Box<dyn HeuristicStrategy>>,
	ordering: Option<Box<dyn OrderingStrategy>>,
	core_assignment: Option<Vec<Processor>>,
	last_next_jobs: Vec<Option<Job>>,
}

#[derive(Clone)]
pub struct Processor {
	utilization: TimeStep,
	tasks: Vec<Task>,
}

impl Processor {
	pub fn new() -> Processor {
		Self {
			utilization: 0,
			tasks: Vec::new(),
		}
	}

	pub fn does_fit(&self, task: &Task) -> bool {
		self.utilization + task.wcet() <= task.deadline()
	}

	pub fn add_task(&mut self, task: Task) {
		if self.tasks.is_empty() {
			self.utilization = task.offset();
		}
		self.utilization += task.wcet();
		self.tasks.push(task);
	}

	pub fn utilization(&self) -> TimeStep {
		self.utilization
	}
}

impl Partitioned {
	pub fn new() -> Partitioned {
		Self {
			heuristic: None,
			ordering: None,
			core_assignment: None,
			last_next_jobs: Vec::new(),
		}
	}

	pub fn set_heuristic(&mut self, heuristic: Box<dyn HeuristicStrategy>) {
		self.heuristic = Some(heuristic);
	}

	pub fn set_ordering(&mut self, ordering: Box<dyn OrderingStrategy>) {
		self.ordering = Some(ordering);
	}
}

impl Scheduler for Partitioned {
	fn is_schedulable(&mut self, taskset: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if self.heuristic.is_none() || self.ordering.is_none() {
			return SchedulabilityResult::Unknown;
		} else if taskset.system_utilization() > cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if cores >= taskset.len() {
			return SchedulabilityResult::SchedulableShortcut;
		// TODO: Check if FF DU instead of false
		} else if false && taskset.system_utilization() <= (cores + 1) as f64 / 2f64 {
			return SchedulabilityResult::SchedulableShortcut;
		}

		// Apply the ordering strategy
		self.ordering.as_ref().unwrap().apply_order(taskset);

		// Apply the heuristic strategy
		match self.heuristic.as_ref().unwrap().assign_cores(taskset, cores) {
			Ok(processors) => {
				self.core_assignment = Some(processors);
			},
			Err(e) => {
				println!("{:?}", e);
				return SchedulabilityResult::UnschedulableSimulated
			},
		}

		// Simulate the scheduling
		match self.simulate(taskset, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SchedulerSimulator for Partitioned {
	fn next_jobs<'a>(&'a mut self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		if self.last_next_jobs.is_empty() {
			self.last_next_jobs = vec![None; cores];
		}
		// For each processor find the next job
		// The next job is the same as the previous one if it is still running
		// Or the next one, or None if there is no more job assigned to the processor

		// TODO: as explained just above
		for (i, processor) in self.core_assignment.as_mut().unwrap().iter_mut().enumerate() {
			let mut new_job = None;
			if let Some(last_job) = self.last_next_jobs[i].as_mut() {
				if !last_job.is_complete() {
					new_job = queue.iter_mut().find(|job| job.task().id() == last_job.task().id());
				} else {
					todo!("Implement the case where the last job is complete");
				}
			} else {
				todo!("Implement the case where the last job is None");
			}

			self.last_next_jobs[i] = new_job.cloned();
		}

		// Return the next jobs
		self.last_next_jobs.iter_mut().map(|job| job.as_mut().unwrap()).collect()
	}
}