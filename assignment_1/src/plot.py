import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

# Set the seaborn style for more professional-looking plots
sns.set_theme(style="whitegrid")

custom_palette = {
	'rr': 'lightblue',
	'dm': 'lightcoral',  # light red
	'edf': 'plum'  # light purple
}

# Load the CSV file into a pandas DataFrame
csv_file = '../scheduling_results.csv'  # Replace with your actual CSV file path
df = pd.read_csv(csv_file)

# Updated column names based on your CSV structure
tasksets_col = 'Taskset'
num_tasks_col = 'Number of Tasks'
algorithm_col = 'Algorithm'
utilization_col = 'Utilization'
feasible_col = 'Schedulable'
feasible_res_col = 'Schedulable Result'

# Ensure that 'Schedulable' is a binary column (1 for schedulable, 0 for not schedulable)
df[feasible_res_col] = df[feasible_col].apply(lambda x: 1 if x in (0, 1) else 0 if x in (2, 3) else x)

# Convert 'Utilization' from percentage string (e.g., "80%") to float (e.g., 0.8)
df[utilization_col] = df[utilization_col].str.rstrip('%').astype(float) / 100

# Ensure the 'Algorithm' column is treated with a categorical type for ordering
algorithm_order = ['rr', 'dm', 'edf']
df[algorithm_col] = pd.Categorical(df[algorithm_col], categories=algorithm_order, ordered=True)

# Plot 1: Ratio of task sets that are feasible according to the number of tasks (80% Utilization)
df_80 = df[df[tasksets_col].str.contains('80-percent/')]
feasible_ratio_per_task_count = df_80.groupby(num_tasks_col, observed=False)[feasible_res_col].mean().reset_index()

plt.figure(figsize=(12, 6))
sns.barplot(x=num_tasks_col, y=feasible_res_col, data=feasible_ratio_per_task_count, hue=num_tasks_col, legend=False,
            palette="deep")
plt.title('Ratio of Feasible Task Sets by Number of Tasks (80% Utilization)', fontsize=14)
plt.xlabel('Number of Tasks', fontsize=12)
plt.ylabel('Feasible Ratio', fontsize=12)
plt.xticks(rotation=45)
plt.tight_layout()
plt.savefig('../feasible_ratio_per_task_count_80_percent.png')

# Plot 2: Success rate of each algorithm according to the number of tasks (80% Utilization)
success_rate_per_algorithm_80 = df_80.groupby([num_tasks_col, algorithm_col], observed=False)[
	feasible_res_col].mean().reset_index()

plt.figure(figsize=(14, 7))
sns.barplot(x=num_tasks_col, y=feasible_res_col, hue=algorithm_col, data=success_rate_per_algorithm_80,
            palette=custom_palette)
plt.title('Success Rate of Each Algorithm by Number of Tasks (80% Utilization)', fontsize=14)
plt.xlabel('Number of Tasks', fontsize=12)
plt.ylabel('Success Rate', fontsize=12)
plt.xticks(rotation=45)
plt.tight_layout()
plt.savefig('../success_rate_per_algorithm_80_percent.png')

### Plot 3: Ratio of tasks that are feasible according to the utilization (10 Tasks)
df_10 = df[df[tasksets_col].str.contains('10-tasks/')]
feasible_ratio_per_utilization = df_10.groupby(utilization_col, observed=False)[feasible_res_col].mean().reset_index()

plt.figure(figsize=(12, 6))
sns.lineplot(x=utilization_col, y=feasible_res_col, data=feasible_ratio_per_utilization, marker='o')
plt.title('Ratio of Feasible Task Sets by Utilization (10 Tasks)', fontsize=14)
plt.xlabel('Utilization', fontsize=12)
plt.ylabel('Feasible Ratio', fontsize=12)
plt.tight_layout()
plt.savefig('../feasible_ratio_per_utilization_10_tasks.png')

### Plot 4: Success rate of each algorithm according to the utilization (10 Tasks)
success_rate_per_algorithm_10 = df_10.groupby([utilization_col, algorithm_col], observed=False)[
	feasible_res_col].mean().reset_index()

plt.figure(figsize=(14, 7))
sns.lineplot(x=utilization_col, y=feasible_res_col, hue=algorithm_col, data=success_rate_per_algorithm_10, marker='o',
             palette=custom_palette)
plt.title('Success Rate of Each Algorithm by Utilization (10 Tasks)', fontsize=14)
plt.xlabel('Utilization', fontsize=12)
plt.ylabel('Success Rate', fontsize=12)
plt.tight_layout()
plt.savefig('../success_rate_per_algorithm_10_tasks.png')

for alg in ("rr", "dm", "edf"):
	plt.figure(figsize=(14, 8))

	# Get unique taskset names for alignment
	subdirs = df[df[algorithm_col] == alg][tasksets_col].unique()

	# Calculate counts and reindex for alignment with subdirs
	exit_code_0 = df[(df[feasible_col] == 0) & (df[algorithm_col] == alg)].groupby(tasksets_col).size().reindex(subdirs,
	                                                                                                            fill_value=0)
	exit_code_1 = df[(df[feasible_col] == 1) & (df[algorithm_col] == alg)].groupby(tasksets_col).size().reindex(subdirs,
	                                                                                                            fill_value=0)
	exit_code_2 = df[(df[feasible_col] == 2) & (df[algorithm_col] == alg)].groupby(tasksets_col).size().reindex(subdirs,
	                                                                                                            fill_value=0)
	exit_code_3 = df[(df[feasible_col] == 3) & (df[algorithm_col] == alg)].groupby(tasksets_col).size().reindex(subdirs,
	                                                                                                            fill_value=0)

	# Plot the bars with stacking
	plt.bar(subdirs, exit_code_0, color="lightgreen", label="Schedulable - Simulated")
	plt.bar(subdirs, exit_code_1, bottom=exit_code_0, color="lightblue", label="Schedulable - Shortcut")
	plt.bar(subdirs, exit_code_2, bottom=exit_code_0 + exit_code_1, color="orange", label="Not Schedulable - Simulated")
	plt.bar(subdirs, exit_code_3, bottom=exit_code_0 + exit_code_1 + exit_code_2, color="red",
	        label="Not Schedulable - Shortcut")

	# Add a title and labels
	plt.title(f'Schedulability Analysis by Taskset for {alg}', fontsize=16)
	plt.xlabel('Taskset', fontsize=14)
	plt.ylabel('Number of Occurrences', fontsize=14)
	plt.xticks(rotation=45, ha='right')
	plt.legend(loc='upper left', fontsize=12)
	plt.tight_layout()

	# Save or show the plot
	plt.savefig(f'../schedulability_analysis_{alg}.png')

# Show all plots (optional)
plt.show()
