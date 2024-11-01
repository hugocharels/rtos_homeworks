import os
import subprocess
import matplotlib.pyplot as plt
from collections import defaultdict

# Define paths for the algorithms and task sets folders
algorithms = ["dm"]#, "edf", "rr"]
tasksets_root = "../tasksets/10-tasks/"  # Update with actual path

# Verify if the directory exists
if not os.path.isdir(tasksets_root):
	print(f"Error: The path '{tasksets_root}' does not exist.")
else:
	# Initialize results dictionary to count each exit code per subdirectory
	results = {alg: defaultdict(lambda: {0: 0, 1: 0, 2: 0, 3: 0}) for alg in algorithms}

	# Run each algorithm on each task set file in each subdirectory and collect exit codes
	for alg in algorithms:
		print(f"Running algorithm: {alg}")
		for subdir in os.listdir(tasksets_root):
			subdir_path = os.path.join(tasksets_root, subdir)
			if os.path.isdir(subdir_path):
				print(f"Processing folder: {subdir}")
				for file in os.listdir(subdir_path):
					taskset_path = os.path.join(subdir_path, file)
					print(f"Processing file: {taskset_path}")
					try:
						# Run cargo command with algorithm and taskset file
						result = subprocess.run(["cargo", "run", alg, taskset_path], capture_output=True, text=True)
						exit_code = result.returncode

						# Count exit code occurrences for each subdirectory
						results[alg][subdir][exit_code] += 1
					except subprocess.CalledProcessError as e:
						print(f"Error running {alg} on {taskset_path}: {e}")

	# Plot results if any files were processed
	for alg, data in results.items():
		if data:
			subdirs = list(data.keys())
			exit_code_0 = [data[subdir][0] for subdir in subdirs]
			exit_code_1 = [data[subdir][1] for subdir in subdirs]
			exit_code_2 = [data[subdir][2] for subdir in subdirs]
			exit_code_3 = [data[subdir][3] for subdir in subdirs]

			# Create a stacked bar chart for each algorithm
			plt.figure(figsize=(12, 6))
			plt.bar(subdirs, exit_code_0, color="lightgreen", label="Exit Code 0 (Schedulable - Simulated)")
			plt.bar(subdirs, exit_code_1, bottom=exit_code_0, color="lightblue", label="Exit Code 1 (Schedulable - Shortcut)")
			plt.bar(subdirs, exit_code_2, bottom=[x + y for x, y in zip(exit_code_0, exit_code_1)], color="salmon", label="Exit Code 2 (Not Schedulable - Simulated)")
			plt.bar(subdirs, exit_code_3, bottom=[x + y + z for x, y, z in zip(exit_code_0, exit_code_1, exit_code_2)], color="orange", label="Exit Code 3 (Not Schedulable - Shortcut)")

			plt.xlabel("Task Set Folders (Utilization Levels)")
			plt.ylabel("Count of Task Sets")
			plt.title(f"Schedulability Analysis for {alg.upper()} Algorithm by Exit Code")
			plt.legend()
			plt.xticks(rotation=45)
			plt.tight_layout()
			plt.savefig(f"{alg}_schedulability_analysis.png")
		else:
			print(f"No files processed for algorithm: {alg}")