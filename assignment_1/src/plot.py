import pandas as pd
import matplotlib.pyplot as plt

# Read the CSV file
df = pd.read_csv('../scheduling_results.csv')

# Function to enhance the plot
def style_plot(ax, title, xlabel, ylabel):
	ax.set_title(title, fontsize=14, weight='bold')
	ax.set_xlabel(xlabel, fontsize=12)
	ax.set_ylabel(ylabel, fontsize=12)
	ax.grid(True, which='both', linestyle='--', linewidth=0.5)
	ax.tick_params(axis='both', which='major', labelsize=10)
	ax.tick_params(axis='both', which='minor', labelsize=8)
	ax.legend(fontsize=10)
	ax.set_ylim(0, 1)  # Ensure y-axis is between 0 and 1

# Filter tasksets for "tasksets/80-percent"
df_filtered = df[df['Taskset'].str.startswith('tasksets/80-percent')]

# Define schedulability conditions
df_filtered['Schedulable'] = df_filtered['Schedulable'].apply(lambda x: 1 if x in [0, 1] else 0)

# Group by 'Taskset' and 'Number of Tasks'
df_feasibility = df_filtered.groupby(['Taskset', 'Number of Tasks'])['Schedulable'].any().reset_index()

# Ratio of feasible task sets by number of tasks
df_ratio = df_feasibility.groupby('Number of Tasks')['Schedulable'].mean()

# Plot feasibility ratio
fig, ax = plt.subplots(figsize=(10, 6))
ax.plot(df_ratio.index, df_ratio.values, marker='o', color='tab:blue', linestyle='-', markersize=6, label='Feasibility Ratio')
style_plot(ax, 'Ratio of Feasible Task Sets by Number of Tasks (80% Utilization)', 'Number of Tasks', 'Ratio of Feasible Task Sets')
plt.tight_layout()
plt.savefig("../feasibility_ratio_80_percent.png")

# Plot success rate for each algorithm by number of tasks
algorithms = df_filtered['Algorithm'].unique()

fig, ax = plt.subplots(figsize=(12, 7))
for algo in algorithms:
	df_algo = df_filtered[df_filtered['Algorithm'] == algo]
	df_algo_ratio = df_algo.groupby('Number of Tasks')['Schedulable'].mean()
	ax.plot(df_algo_ratio.index, df_algo_ratio.values, marker='o', label=algo)

style_plot(ax, 'Success Rate of Each Algorithm by Number of Tasks (80% Utilization)', 'Number of Tasks', 'Success Rate')
plt.legend(title="Algorithms", loc='upper left', fontsize=10)
plt.tight_layout()
plt.savefig("../success_rate_80_percent.png")

# Filter tasksets for "tasksets/10-tasks"
df_filtered = df[df['Taskset'].str.startswith('tasksets/10-tasks')]

# Define schedulability conditions
df_filtered['Schedulable'] = df_filtered['Schedulable'].apply(lambda x: 1 if x in [0, 1] else 0)

# Group by 'Taskset' and 'Utilization'
df_feasibility = df_filtered.groupby(['Taskset', 'Utilization'])['Schedulable'].any().reset_index()

# Clean and convert 'Utilization' column to numeric
df_feasibility['Utilization'] = pd.to_numeric(df_feasibility['Utilization'].str.strip(), errors='coerce')

# Ratio of feasible task sets by utilization
df_ratio = df_feasibility.groupby('Utilization')['Schedulable'].mean()
df_ratio = df_ratio.sort_index(ascending=True)

# Plot feasibility ratio by utilization
fig, ax = plt.subplots(figsize=(10, 6))
ax.plot(df_ratio.index, df_ratio.values, marker='o', color='tab:green', linestyle='-', markersize=6, label='Feasibility Ratio')
style_plot(ax, 'Ratio of Feasible Task Sets by Utilization (10-Tasks)', 'Utilization (%)', 'Ratio of Feasible Task Sets')
plt.tight_layout()
plt.savefig("../feasibility_ratio_10_tasks.png")

# Plot success rate for each algorithm by utilization
df_filtered['Utilization'] = df_filtered['Utilization'].apply(lambda x: int(x.strip('%')))

fig, ax = plt.subplots(figsize=(12, 7))
for algo in algorithms:
	df_algo = df_filtered[df_filtered['Algorithm'] == algo]
	df_algo_ratio = df_algo.groupby('Utilization')['Schedulable'].mean()
	df_algo_ratio = df_algo_ratio.sort_index(ascending=True)
	ax.plot(df_algo_ratio.index, df_algo_ratio.values, marker='o', label=algo)

style_plot(ax, 'Success Rate of Each Algorithm by Utilization (10-Tasks)', 'Utilization (%)', 'Success Rate')
plt.legend(title="Algorithms", loc='upper left', fontsize=10)
plt.tight_layout()
plt.savefig("../success_rate_10_tasks.png")

# Show all the plots
plt.show()
