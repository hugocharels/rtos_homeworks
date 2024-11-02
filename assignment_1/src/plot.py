import os
import subprocess
import matplotlib.pyplot as plt
from collections import defaultdict, Counter
from concurrent.futures import ProcessPoolExecutor
import multiprocessing

# Define paths for the algorithms and task sets folders
# algorithms = ["dm", "edf", "rr"]
algorithms = ["rr"]
tasksets_root = "../tasksets/10-tasks/"  # Update with actual path
#tasksets_root = "../tasksets/80-percent/"  # Update with actual path


def init_results_dict():
    """Initialize the results dictionary with expected exit codes."""
    return {0: 0, 1: 0, 2: 0, 3: 0, "error": 0}


def run_algorithm(alg, subdir, taskset_path):
    """Run the algorithm on a specific taskset file and return the subdir and exit code."""
    print(f"Processing file: {taskset_path}", flush=True)
    try:
        result = subprocess.run(
            ["cargo", "run", alg, taskset_path], capture_output=True, text=True
        )
        exit_code = result.returncode
        if exit_code not in (0, 1, 2, 3):
            exit_code = "error"
        return (alg, subdir, exit_code)
    except Exception as e:
        print(f"Error running {alg} on {taskset_path}: {e}")
        return (alg, subdir, "error")


def get_alg_name(w):
    return {
        "dm": "Deadline Monotonic",
        "edf": "Earliest Deadline First",
        "rr": "Round Robin",
    }[w]


def main():
    if not os.path.isdir(tasksets_root):
        print(f"Error: The path '{tasksets_root}' does not exist.")
        return

    # Initialize an empty results structure
    results = {alg: defaultdict(init_results_dict) for alg in algorithms}

    sorted_subdirs = sorted(
        [
            subdir
            for subdir in os.listdir(tasksets_root)
            if os.path.isdir(os.path.join(tasksets_root, subdir))
        ],
        key=len,  # Sort by length of subdir names
    )

    # Using ProcessPoolExecutor for parallel processing
    with ProcessPoolExecutor(max_workers=multiprocessing.cpu_count()) as executor:
        futures = []
        for alg in algorithms:
            print(f"Running algorithm: {alg}")
            for subdir in sorted_subdirs:
                subdir_path = os.path.join(tasksets_root, subdir)
                for file in os.listdir(subdir_path):
                    taskset_path = os.path.join(subdir_path, file)
                    futures.append(
                        executor.submit(run_algorithm, alg, subdir, taskset_path)
                    )

        # Collect results in a thread-safe way
        for future in futures:
            alg, subdir, exit_code = future.result()
            results[alg][subdir][exit_code] += 1

    # Plot results if any files were processed
    for alg, data in results.items():
        if data:
            subdirs = list(data.keys())
            exit_code_0 = [data[subdir].get(0, 0) for subdir in subdirs]
            exit_code_1 = [data[subdir].get(1, 0) for subdir in subdirs]
            exit_code_2 = [data[subdir].get(2, 0) for subdir in subdirs]
            exit_code_3 = [data[subdir].get(3, 0) for subdir in subdirs]
            error_count = [data[subdir].get("error", 0) for subdir in subdirs]

            plt.figure(figsize=(12, 6))
            plt.bar(
                subdirs,
                exit_code_0,
                color="lightgreen",
                label="Schedulable - Simulated",
            )
            plt.bar(
                subdirs,
                exit_code_1,
                bottom=exit_code_0,
                color="lightblue",
                label="Schedulable - Shortcut",
            )
            plt.bar(
                subdirs,
                exit_code_2,
                bottom=[x + y for x, y in zip(exit_code_0, exit_code_1)],
                color="salmon",
                label="Not Schedulable - Simulated",
            )
            plt.bar(
                subdirs,
                exit_code_3,
                bottom=[
                    x + y + z for x, y, z in zip(exit_code_0, exit_code_1, exit_code_2)
                ],
                color="orange",
                label="Not Schedulable - Shortcut",
            )
            plt.bar(
                subdirs,
                error_count,
                bottom=[
                    x + y + z + w
                    for x, y, z, w in zip(
                        exit_code_0, exit_code_1, exit_code_2, exit_code_3
                    )
                ],
                color="grey",
                label="Error",
            )

            plt.xlabel("Task Set Folders (Utilization Levels)")
            plt.ylabel("Count of Task Sets")
            plt.title(f"Schedulability Analysis for {get_alg_name(alg)} Algorithm")
            plt.legend()
            plt.xticks(rotation=45)
            plt.tight_layout()
            plt.savefig(f"{alg}_schedulability_analysis.png")
        else:
            print(f"No files processed for algorithm: {alg}")


if __name__ == "__main__":
    main()
