#[derive(Debug)]
pub enum SchedulabilityResult {
	SchedulableSimulated = 0,       // Exit code 0
	SchedulableShortcut = 1,        // Exit code 1
	NotSchedulableSimulated = 2,    // Exit code 2
	NotSchedulableShortcut = 3,     // Exit code 3
	Unknown = 4,                    // Exit code 4
}
