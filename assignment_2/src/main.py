import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


def plot_time_paritioned_bfdu_worker(csv_file):
	# Read the CSV file
	data = pd.read_csv(csv_file)

	# Convert columns to appropriate data types
	data['workers'] = data['workers'].astype(int)
	data['duration'] = data['duration'].astype(int)
	data['result'] = data['result'].astype(int)  # Ensure 'result' column is int

	# Filter data to include only rows where result is 0 or 2
	filtered_data = data[(data['result'] == 0) | (data['result'] == 2)]

	# Group by workers and calculate mean duration
	mean_data = filtered_data.groupby('workers', as_index=False)['duration'].mean()

	# Plot data
	plt.figure(figsize=(10, 6))
	plt.plot(mean_data['workers'], mean_data['duration'], marker='o', label='Mean Duration')

	# Add labels and title
	plt.xlabel('Number of Workers', fontsize=12)
	plt.ylabel('Mean Duration (µs)', fontsize=12)
	plt.title('Mean Execution Time vs Number of Workers (Filtered Results)', fontsize=14)

	# Show grid and plot
	plt.grid(True)
	plt.tight_layout()
	plt.legend()
	plt.savefig('mean_duration_vs_workers.png')


def plot_schedulable_partitioned(csv_file):
	# Read the CSV file
	data = pd.read_csv(csv_file)

	# Convert 'result' column to integer
	data['result'] = data['result'].astype(int)

	# Filter data to include only rows where result is 0, 1, 2, or 3
	filtered_data = data[(data['result'] == 0) | (data['result'] == 1) | (data['result'] == 2) | (data['result'] == 3)]

	# Group by heuristic and ordering, summing schedulable (0,1) and not schedulable (2,3) counts
	grouped_data = filtered_data.groupby(['heuristic', 'ordering'])['result'].value_counts().unstack(fill_value=0)
	grouped_data['Schedulable'] = grouped_data.get(0, 0) + grouped_data.get(1, 0)
	grouped_data['Not Schedulable'] = grouped_data.get(2, 0) + grouped_data.get(3, 0)

	# Calculate success rate
	grouped_data['Total'] = grouped_data['Schedulable'] + grouped_data['Not Schedulable']
	grouped_data['Success Rate'] = grouped_data['Schedulable'] / grouped_data['Total'] * 100

	# Reset the index for easier plotting
	grouped_data.reset_index(inplace=True)

	# Sort by success rate for better visualization
	grouped_data.sort_values(by='Success Rate', ascending=False, inplace=True)

	# Create a bar plot for the success rate
	fig, ax = plt.subplots(figsize=(10, 6))
	bars = ax.bar(
		grouped_data.index,
		grouped_data['Success Rate'],
		color='green',
		alpha=0.7
	)

	# Add labels for heuristic and ordering combinations in uppercase
	labels = [f"{row['heuristic']} {row['ordering']}".upper() for _, row in grouped_data.iterrows()]
	ax.set_xticks(grouped_data.index)
	ax.set_xticklabels(labels, rotation=0, ha='center', fontsize=10)

	# Add labels, title, and grid
	ax.set_xlabel('Heuristic and Ordering', fontsize=12)
	ax.set_ylabel('Success Rate (%)', fontsize=12)
	ax.set_title('Success Rate by Heuristic and Ordering', fontsize=14)
	ax.grid(axis='y', linestyle='--', alpha=0.7)

	# Highlight the best combination
	best_index = grouped_data['Success Rate'].idxmax()
	best_value = grouped_data.loc[best_index, 'Success Rate']
	ax.annotate(
		f"Best: {best_value:.1f}%",
		xy=(best_index, best_value),
		xytext=(best_index, best_value + 5),
		textcoords='offset points',
		ha='center',
		color='black',
		fontsize=10,
		fontweight='bold',
		arrowprops=dict(facecolor='black', arrowstyle='->', lw=0.5)
	)

	# Adjust layout and show plot
	fig.tight_layout()
	plt.savefig('success_rate_by_heuristic_and_ordering.png')


def plot_result_edf(csv_file):
	# Read the CSV file
	data = pd.read_csv(csv_file)

	# Convert 'result' column to integer
	data['result'] = data['result'].astype(int)

	# Categorize results
	data['Category'] = data['result'].apply(lambda x: 'Schedulable' if x in (0, 1)
	else 'Unschedulable' if x in (2, 3)
	else 'Undetermined')

	# Modify 'version' column to label EDF(number) correctly
	data['Version_Label'] = data['version'].apply(lambda x: 'EDF(' + str(x) + ')' if str(x).isdigit() else str(x))

	# Group data by Version_Label and Category
	grouped_data = data.groupby(['Version_Label', 'Category']).size().unstack(fill_value=0)

	# Calculate total per version and percentages
	grouped_data['Total'] = grouped_data.sum(axis=1)
	for col in ['Schedulable', 'Unschedulable', 'Undetermined']:
		grouped_data[col + '_Percent'] = grouped_data[col] / grouped_data['Total'] * 100

	# Sort versions for better visualization
	grouped_data.sort_values(by='Schedulable_Percent', ascending=False, inplace=True)

	# Plot settings
	x_labels = grouped_data.index
	x = np.arange(len(x_labels))
	width = 0.6

	# Create 3 bar plots for Schedulable, Unschedulable, and Undetermined percentages
	fig, axes = plt.subplots(3, 1, figsize=(10, 8), sharex=True)

	# Schedulable Plot
	axes[0].bar(x, grouped_data['Schedulable_Percent'], color='green', alpha=0.7, label='Schedulable')
	axes[0].set_title('Schedulable Percentage per Version', fontsize=14)
	axes[0].set_ylabel('Percentage (%)', fontsize=12)
	axes[0].grid(axis='y', linestyle='--', alpha=0.7)
	axes[0].legend(loc='upper left')

	# Unschedulable Plot
	unsched_bars = axes[1].bar(x, grouped_data['Unschedulable_Percent'], color='red', alpha=0.7, label='Unschedulable')
	axes[1].set_ylim(0, 70)  # Set y-axis range
	axes[1].set_title('Unschedulable Percentage per Version', fontsize=14)
	axes[1].set_ylabel('Percentage (%)', fontsize=12)
	axes[1].grid(axis='y', linestyle='--', alpha=0.7)
	axes[1].legend(loc='upper left')

	# Annotate best and worst percentages for Unschedulable
	max_unsched = grouped_data['Unschedulable_Percent'].idxmax()
	min_unsched = grouped_data['Unschedulable_Percent'].idxmin()
	max_unsched_value = grouped_data['Unschedulable_Percent'].max()
	min_unsched_value = grouped_data['Unschedulable_Percent'].min()
	for i, bar in enumerate(unsched_bars):
		height = bar.get_height()
		if x_labels[i] == max_unsched:
			axes[1].text(bar.get_x() + bar.get_width() / 2, height + 1, f'Best: {height:.1f}%',
			             ha='center', va='bottom', fontsize=10, color='black', fontweight='bold', clip_on=True)
		if x_labels[i] == min_unsched:
			axes[1].text(bar.get_x() + bar.get_width() / 2, height + 1, f'Worst: {height:.1f}%',
			             ha='center', va='bottom', fontsize=10, color='black', fontweight='bold', clip_on=True)

	# Undetermined Plot
	undet_bars = axes[2].bar(x, grouped_data['Undetermined_Percent'], color='gray', alpha=0.7, label='Undetermined')
	axes[2].set_ylim(0, 45)  # Set y-axis range
	axes[2].set_title('Undetermined Percentage per Version', fontsize=14)
	axes[2].set_ylabel('Percentage (%)', fontsize=12)
	axes[2].grid(axis='y', linestyle='--', alpha=0.7)
	axes[2].legend(loc='upper left')

	# Annotate best and worst percentages for Undetermined
	max_undet = grouped_data['Undetermined_Percent'].idxmax()
	min_undet = grouped_data['Undetermined_Percent'].idxmin()
	max_undet_value = grouped_data['Undetermined_Percent'].max()
	min_undet_value = grouped_data['Undetermined_Percent'].min()
	for i, bar in enumerate(undet_bars):
		height = bar.get_height()
		if x_labels[i] == max_undet:
			axes[2].text(bar.get_x() + bar.get_width() / 2, height + 1, f'Best: {height:.1f}%',
			             ha='center', va='bottom', fontsize=10, color='black', fontweight='bold', clip_on=True)
		if x_labels[i] == min_undet:
			axes[2].text(bar.get_x() + bar.get_width() / 2, height + 1, f'Worst: {height:.1f}%',
			             ha='center', va='bottom', fontsize=10, color='black', fontweight='bold', clip_on=True)

	# Set x-axis labels
	plt.xticks(x, x_labels, rotation=0, fontsize=10)
	plt.xlabel('Version', fontsize=12)

	# Adjust layout
	plt.tight_layout(rect=[0, 0, 1, 1])
	plt.savefig('success_rate_by_edf_version.png')


# Call the function with your CSV file path
plot_time_paritioned_bfdu_worker('results_partitioned_bfdu_time_workers.csv')
plot_schedulable_partitioned("results_partitioned.csv")
plot_result_edf("results_result_edf.csv")
