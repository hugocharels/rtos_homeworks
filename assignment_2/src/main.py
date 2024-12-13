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
	plt.show()


import matplotlib.pyplot as plt
import pandas as pd


def plot_results(csv_file):
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
	plt.show()


def plot_result_versions(csv_file):
	# Read the CSV file
	data = pd.read_csv(csv_file)

	# Convert columns to appropriate data types
	data['result'] = data['result'].astype(int)

	# Ensure "global" version appears first
	data['version'] = pd.Categorical(data['version'], categories=['global'] + sorted(set(data['version']) - {'global'}), ordered=True)

	# Replace numeric versions with "EDF(number)"
	data['version'] = data['version'].apply(lambda v: f"EDF({v})" if v.isdigit() else v)

	# Group by version and result, summing the tasksets
	grouped_data = data.groupby(['version', 'result']).size().unstack(fill_value=0)

	# Plot data with professional color palette
	ax = grouped_data.plot(kind='bar', stacked=True, figsize=(10, 6), color=['#4caf50', '#2196f3', '#f44336', '#ff9800'])

	# Rotate x-axis labels if space permits
	if len(grouped_data.index) < 10:  # Arbitrary threshold for label density
		ax.set_xticklabels(grouped_data.index, rotation=0, ha='center')
	else:
		ax.set_xticklabels(grouped_data.index, rotation=45, ha='right')

	# Add labels and title
	plt.xlabel('Version', fontsize=12)
	plt.ylabel('Count of Tasksets', fontsize=12)
	plt.title('Taskset Results by Version', fontsize=14)

	# Add legend and layout adjustments
	plt.legend(title='Result', fontsize=10)
	plt.tight_layout()
	plt.show()


# Call the function with your CSV file path
plot_time_paritioned_bfdu_worker('results_partitioned_bfdu_time_workers.csv')
plot_result_versions("results_result_edf.csv")
