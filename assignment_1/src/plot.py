import os
import subprocess
import matplotlib.pyplot as plt
from sys import argv
from collections import defaultdict

# Define paths for the algorithms and task sets folders
algorithms = ["dm", "edf", "rr"]
tasksets_root = argv[-1]  # Update with actual path


# Initialize results dictionary to count each exit code per folder
results = {alg: defaultdict(lambda: {0: 0, 1: 0, 2: 0, 3: 0}) for alg in algorithms}

# Run each algorithm on each task set file and collect exit codes
for alg in algorithms:
	print(f"Running algorithm: {alg}")
	for root, _, files in os.walk(tasksets_root):
		print(f"Entering directory: {root}")
		for file in files:
			taskset_path = os.path.join(root, file)
			print(f"Processing file: {taskset_path}")
			try:
				# Run cargo command with algorithm and taskset file
				result = subprocess.run(["cargo", "run", alg, taskset_path], capture_output=True, text=True)
				exit_code = result.returncode

				# Count exit code occurrences
				results[alg][root][exit_code] += 1
			except subprocess.CalledProcessError as e:
				print(f"Error running {alg} on {taskset_path}: {e}")

# Plot results
for alg, data in results.items():
	folders = list(data.keys())
	exit_code_0 = [data[folder][0] for folder in folders]
	exit_code_1 = [data[folder][1] for folder in folders]
	exit_code_2 = [data[folder][2] for folder in folders]
	exit_code_3 = [data[folder][3] for folder in folders]

	# Create a stacked bar chart for each algorithm
	plt.figure(figsize=(12, 6))
	plt.bar(folders, exit_code_0, color="lightgreen", label="Exit Code 0 (Schedulable - Simulated)")
	plt.bar(folders, exit_code_1, bottom=exit_code_0, color="lightblue", label="Exit Code 1 (Schedulable - Shortcut)")
	plt.bar(folders, exit_code_2, bottom=[x + y for x, y in zip(exit_code_0, exit_code_1)], color="salmon", label="Exit Code 2 (Not Schedulable - Simulated)")
	plt.bar(folders, exit_code_3, bottom=[x + y + z for x, y, z in zip(exit_code_0, exit_code_1, exit_code_2)], color="orange", label="Exit Code 3 (Not Schedulable - Shortcut)")

	plt.xlabel("Task Set Folders")
	plt.ylabel("Count of Task Sets")
	plt.title(f"Schedulability Analysis for {alg.upper()} Algorithm by Exit Code")
	plt.legend()
	plt.xticks(rotation=45)
	plt.tight_layout()
	plt.savefig(f"{alg}_schedulability_analysis.png")
