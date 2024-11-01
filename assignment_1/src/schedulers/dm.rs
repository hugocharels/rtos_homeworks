use super::strategy::SchedulerStrategy;
use crate::{
	models::{Job, TaskSet, TimeStep},
	schedulers::errors::SchedulingError,
	schedulers::result::SchedulabilityResult,
};

pub struct DM;


impl DM {

	fn theorem29(&self, task_set: &TaskSet) -> bool {
		// wi <= Di for all i
	    for (i, task) in task_set.tasks().iter().enumerate() {
	        let mut w = task.wcet() as f64;
	        loop {
	            let mut w_next = task.wcet() as f64;
	            for j in 0..i {
	                w_next += (w as f64 / task_set.tasks()[j].period() as f64).ceil() * (task_set.tasks()[j].wcet() as f64);
	            }
	            if w == w_next {
	                break;
	            }
	            w = w_next;
	        }
	        if w > task.deadline() as f64 {
	            return false;
	        }
	    }
	    true
	}
}



impl SchedulerStrategy for DM {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}
		// } else if task_set.system_utilization() <= task_set.size() as f64 * (2f64.powf(1.0 / task_set.size() as f64) - 1.0) {
		// 	return SchedulabilityResult::SchedulableShortcut;
		// }

		if self.theorem29(task_set) {
			SchedulabilityResult::SchedulableShortcut
		} else {
			SchedulabilityResult::UnschedulableShortcut
		}
	}

}