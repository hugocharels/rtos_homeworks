use super::task::Task;

struct TaskSet {
	tasks: Vec<Task>,
}


impl TaskSet {
	pub fn get_from_file(file: &str) -> TaskSet {
		// Read the file and parse the task set
		// the file is in CSV format, where each line is a task and the columns are:
		// Oi, Ci, Ti, Di
		// where Oi is the offset, Ci is the computation time, Ti is the period, and Di is the deadline
		let mut id = 0;
		let mut tasks = Vec::new();
		let file = std::fs::read_to_string(file).expect("Failed to read file");
		for line in file.lines() {
			let task: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
			tasks.push(Task::new(id, task[0], task[1], task[2], task[3]));
			id += 1;
		}
		TaskSet { tasks }
	}
}