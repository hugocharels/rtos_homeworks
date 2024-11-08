import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

# Set the seaborn style for more professional-looking plots
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
df[feasible_col] = df[feasible_col].apply(lambda x: 1 if x in (0, 1) else 0 if x in (2, 3) else x)

# Convert 'Utilization' from percentage string (e.g., "80%") to float (e.g., 0.8)
df[utilization_col] = df[utilization_col].str.rstrip('%').astype(float) / 100

### Plot 1: Ratio of task sets that are feasible according to the number of tasks (80% Utilization)
# Filter data for 80% utilization
df_80 = df[df[tasksets_col].str.contains('80-percent/')]

# Calculate the ratio of feasible task sets by the number of tasks
feasible_ratio_per_task_count = df_80.groupby(num_tasks_col)[feasible_col].mean().reset_index()

# Plotting - Bar plot for feasibility ratio
plt.figure(figsize=(12, 6))
sns.barplot(x=num_tasks_col, y=feasible_col, data=feasible_ratio_per_task_count, palette="Blues_d")
plt.title('Ratio of Feasible Task Sets by Number of Tasks (80% Utilization)', fontsize=14)
plt.xlabel('Number of Tasks', fontsize=12)
plt.ylabel('Feasible Ratio', fontsize=12)
plt.xticks(rotation=45)
plt.tight_layout()
plt.savefig('../feasible_ratio_per_task_count.png')

### Plot 2: Success rate of each algorithm according to the number of tasks (80% Utilization)
# Group by algorithm and number of tasks to calculate mean success rate
success_rate_per_algorithm = df_80.groupby([num_tasks_col, algorithm_col])[feasible_col].mean().reset_index()

# Plotting - Bar plot for success rate by algorithm
plt.figure(figsize=(14, 7))
sns.barplot(x=num_tasks_col, y=feasible_col, hue=algorithm_col, data=success_rate_per_algorithm, palette="Set2")
plt.title('Success Rate of Each Algorithm by Number of Tasks (80% Utilization)', fontsize=14)
plt.xlabel('Number of Tasks', fontsize=12)
plt.ylabel('Success Rate', fontsize=12)
plt.legend(title='Algorithm')
plt.xticks(rotation=45)
plt.tight_layout()
plt.savefig('../success_rate_per_algorithm_80_util.png')

### Plot 3: Ratio of task sets that are feasible according to utilization (10 Tasks)
# Filter data for 10 tasks
df_10_tasks = df[df[tasksets_col].str.contains('10-tasks/')]

# Calculate the ratio of feasible task sets by utilization
feasible_ratio_per_utilization = df_10_tasks.groupby(utilization_col)[feasible_col].mean().reset_index()

# Plotting - Bar plot for feasibility ratio by utilization
plt.figure(figsize=(12, 6))
sns.barplot(x=utilization_col, y=feasible_col, data=feasible_ratio_per_utilization, palette="Greens_d")
plt.title('Ratio of Feasible Task Sets by Utilization (10 Tasks)', fontsize=14)
plt.xlabel('Utilization', fontsize=12)
plt.ylabel('Feasible Ratio', fontsize=12)
plt.xticks(rotation=45)
plt.tight_layout()
plt.savefig('../feasible_ratio_per_utilization.png')

### Plot 4: Success rate of each algorithm according to utilization (10 Tasks)
# Group by algorithm and utilization to calculate mean success rate
success_rate_per_algorithm_util = df_10_tasks.groupby([utilization_col, algorithm_col])[
	feasible_col].mean().reset_index()

# Plotting - Bar plot for success rate by algorithm and utilization
plt.figure(figsize=(14, 7))
sns.barplot(x=utilization_col, y=feasible_col, hue=algorithm_col, data=success_rate_per_algorithm_util, palette="Set1")
plt.title('Success Rate of Each Algorithm by Utilization (10 Tasks)', fontsize=14)
plt.xlabel('Utilization', fontsize=12)
plt.ylabel('Success Rate', fontsize=12)
plt.legend(title='Algorithm')
plt.xticks(rotation=45)
plt.tight_layout()
plt.savefig('../success_rate_per_algorithm_utilization.png')
