import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

# Set the seaborn style for professional-looking plots
sns.set(style="whitegrid")

# Load the CSV file into a pandas DataFrame
csv_file = '../scheduling_results.csv'  # Replace with your actual CSV file path
df = pd.read_csv(csv_file)

# Print the column names for verification
print("Columns in CSV file:", df.columns)

# Updated column names based on your CSV structure
tasksets_col = 'Taskset'
num_tasks_col = 'Number of Tasks'
algorithm_col = 'Algorithm'
utilization_col = 'Utilization'
feasible_col = 'Schedulable'

# Ensure that 'Schedulable' is a binary column (1 for schedulable, 0 for not schedulable)
df[feasible_col] = df[feasible_col].apply(lambda x: int(x))

# Convert 'Utilization' from percentage string (e.g., "80%") to float (e.g., 0.8)
df[utilization_col] = df[utilization_col].str.rstrip('%').astype(float) / 100

# Change the content of the column Taskset: from "tasksets/80-percent/..." to "80-percent/..."
df[tasksets_col] = df[tasksets_col].str.replace('tasksets-2/', '')

# Function to get a formatted algorithm name
def get_alg_name(alg):
	return {
		"rr": "Round-Robin",
		"dm": "Deadline Monotonic",
		"edf": "Earliest Deadline First",
	}.get(alg, alg)

for alg in ("rr", "dm", "edf"):
	plt.figure(figsize=(14, 8))

	# Get unique taskset names for alignment
	subdirs = df[df[algorithm_col] == alg][tasksets_col].unique()

	# Calculate counts and reindex for alignment with subdirs
	exit_code_0 = df[(df[feasible_col] == 0) & (df[algorithm_col] == alg)].groupby(tasksets_col).size().reindex(subdirs, fill_value=0)
	exit_code_1 = df[(df[feasible_col] == 1) & (df[algorithm_col] == alg)].groupby(tasksets_col).size().reindex(subdirs, fill_value=0)
	exit_code_2 = df[(df[feasible_col] == 2) & (df[algorithm_col] == alg)].groupby(tasksets_col).size().reindex(subdirs, fill_value=0)
	exit_code_3 = df[(df[feasible_col] == 3) & (df[algorithm_col] == alg)].groupby(tasksets_col).size().reindex(subdirs, fill_value=0)

	# Plot the bars with stacking
	plt.bar(subdirs, exit_code_0, color="lightgreen", label="Schedulable - Simulated")
	plt.bar(subdirs, exit_code_1, bottom=exit_code_0, color="lightblue", label="Schedulable - Shortcut")
	plt.bar(subdirs, exit_code_2, bottom=exit_code_0 + exit_code_1, color="orange", label="Not Schedulable - Simulated")
	plt.bar(subdirs, exit_code_3, bottom=exit_code_0 + exit_code_1 + exit_code_2, color="red", label="Not Schedulable - Shortcut")

	# Add a title and labels
	plt.title(f'Schedulability Analysis by Taskset for {get_alg_name(alg)}', fontsize=16)
	plt.xlabel('Taskset', fontsize=14)
	plt.ylabel('Number of Occurrences', fontsize=14)
	plt.xticks(rotation=45, ha='right')
	plt.legend(loc='upper left', fontsize=12)
	plt.tight_layout()

	# Save or show the plot
	plt.savefig(f'../schedulability_analysis_{alg}.png')
