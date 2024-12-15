import matplotlib.pyplot as plt
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

	# Convert columns to appropriate data types
	data['result'] = data['result'].astype(int)

	# Ensure "global" version appears first
	data['version'] = pd.Categorical(data['version'], categories=['global'] + sorted(set(data['version']) - {'global'}),
	                                 ordered=True)

	# Replace numeric versions with "EDF(number)"
	data['version'] = data['version'].apply(lambda v: f"EDF({v})" if v.isdigit() else v)

	# Group by version and result, summing the tasksets
	grouped_data = data.groupby(['version', 'result']).size().unstack(fill_value=0)

	# Reorder columns to match the result order
	grouped_data = grouped_data[[1, 2, 3, 4]]

	# Define the colors and labels
	result_colors = ["blue", "red", "purple", "grey"]
	result_labels = [
		'Schedulable (sufficient condition met)',
		'Not schedulable (simulation required)',
		'Not schedulable (necessary condition fails)',
		'Cannot determine'
	]

	# Plot data with custom colors and adjusted bar spacing
	ax = grouped_data.plot(
		kind='bar',
		stacked=True,
		figsize=(10, 6),
		color=result_colors,
		width=1
	)

	# Rotate x-axis labels if space permits
	if len(grouped_data.index) < 10:  # Arbitrary threshold for label density
		ax.set_xticklabels(grouped_data.index, rotation=0, ha='center')
	else:
		ax.set_xticklabels(grouped_data.index, rotation=45, ha='right')

	# Add labels and title
	plt.xlabel('Version', fontsize=12)
	plt.ylabel('Count of Tasksets', fontsize=12)
	plt.title('Taskset Results by Version', fontsize=14)

	# Add legend with descriptive labels
	plt.legend(title='Result', labels=result_labels, fontsize=10)
	plt.tight_layout()
	plt.savefig('taskset_results_by_version.png')


# Call the function with your CSV file path
plot_time_paritioned_bfdu_worker('results_partitioned_bfdu_time_workers.csv')
plot_schedulable_partitioned("results_partitioned.csv")
plot_result_edf("results_result_edf.csv")
