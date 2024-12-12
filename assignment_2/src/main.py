import matplotlib.pyplot as plt
import pandas as pd


def plot_results(csv_file):
	# Read the CSV file
	data = pd.read_csv(csv_file)

	# Convert columns to appropriate data types
	data['workers'] = data['workers'].astype(int)
	data['duration'] = data['duration'].astype(int)

	# Group by workers and calculate mean duration
	mean_data = data.groupby('workers', as_index=False)['duration'].mean()

	# Plot data
	plt.figure(figsize=(10, 6))
	plt.plot(mean_data['workers'], mean_data['duration'], marker='o', label='Mean Duration')

	# Add labels and title
	plt.xlabel('Number of Workers', fontsize=12)
	plt.ylabel('Mean Duration (µs)', fontsize=12)
	plt.title('Mean Execution Time vs Number of Workers', fontsize=14)

	# Show grid and plot
	plt.grid(True)
	plt.tight_layout()
	plt.show()


# Call the function with your CSV file path
plot_results('results.csv')
