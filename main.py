import os
import time
import tracemalloc
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from prettytable import PrettyTable
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import DBquery, CRUD_Create, CRUD_Read, CRUD_Update, CRUD_Delete


def cleanup_old_files():
    """Remove old performance files"""
    files_to_remove = [
        "performance_comparison.csv",
        "performance_comparison.png",
        "python_benchmarks.csv",
    ]

    for file in files_to_remove:
        if os.path.exists(file):
            print(f"Removing old file: {file}")
            os.remove(file)


def track_performance(operation_name, func, *args, **kwargs):
    """Tracks and returns memory and time for a function execution."""
    tracemalloc.start()
    start_time = time.time()

    result = func(*args, **kwargs)

    end_time = time.time()
    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    elapsed_time = end_time - start_time
    memory_used = float(peak)  # Remove division by 1024, keep it in KB
    return elapsed_time, memory_used, result


def save_performance_data(performance_summary):
    """Save performance data to CSV for later comparison with Rust"""
    data = []
    for operation, elapsed_time, memory_used, _ in performance_summary:
        data.append(
            {
                "operation": operation,
                "language": "Python",
                "execution_time": round(elapsed_time, 3),
                "memory_used": round(memory_used, 2),  # Keep in KB
            }
        )

    df = pd.DataFrame(data)
    df.to_csv("python_benchmarks.csv", index=False)
    return df


def format_speed_difference(py_time, rust_time):
    """Format speed difference as human readable string"""
    ratio = py_time / rust_time if rust_time > 0 else float("inf")
    if ratio > 1:
        return f"Rust {ratio:.1f}x faster"
    elif ratio < 1:
        return f"Python {(1/ratio):.1f}x faster"
    else:
        return "Same speed"


def format_memory_savings(py_mem, rust_mem):
    """Format memory savings as human readable string"""
    if py_mem > rust_mem:
        savings = ((py_mem - rust_mem) / py_mem) * 100
        return f"Rust uses {savings:.1f}% less"
    elif rust_mem > py_mem:
        excess = ((rust_mem - py_mem) / py_mem) * 100
        return f"Rust uses {excess:.1f}% more"
    else:
        return "Same memory usage"


def compare_with_rust():
    """Compare Python performance with Rust and generate visualizations"""
    try:
        # Read benchmark results
        python_df = pd.read_csv("python_benchmarks.csv")
        rust_df = pd.read_csv("rust_benchmarks.csv")

        # Combine results
        df = pd.concat([python_df, rust_df])

        # Create comparison table
        comparison = df.pivot_table(
            index="operation",
            columns="language",
            values=["execution_time", "memory_used"],
            aggfunc="mean",
        ).round(3)

        # Create pretty table for comparison
        table = PrettyTable()
        table.field_names = [
            "Operation",
            "Python Time (s)",
            "Rust Time (s)",
            "Speed Comparison",
            "Python Memory (KB)",
            "Rust Memory (KB)",
            "Memory Comparison",
        ]

        # Track totals for averages
        total_py_time = 0
        total_rust_time = 0
        total_py_mem = 0
        total_rust_mem = 0

        for operation in comparison.index:
            py_time = comparison.loc[operation, ("execution_time", "Python")]
            rust_time = comparison.loc[operation, ("execution_time", "Rust")]
            py_mem = comparison.loc[operation, ("memory_used", "Python")]
            rust_mem = comparison.loc[operation, ("memory_used", "Rust")]

            total_py_time += py_time
            total_rust_time += rust_time
            total_py_mem += py_mem
            total_rust_mem += rust_mem

            table.add_row(
                [
                    operation,
                    f"{py_time:.3f}",
                    f"{rust_time:.3f}",
                    format_speed_difference(py_time, rust_time),
                    f"{py_mem:.2f}",
                    f"{rust_mem:.2f}",
                    format_memory_savings(py_mem, rust_mem),
                ]
            )

        # Add average row
        table.add_row(
            [
                "Overall",
                f"{total_py_time:.3f}",
                f"{total_rust_time:.3f}",
                format_speed_difference(total_py_time, total_rust_time),
                f"{total_py_mem:.2f}",
                f"{total_rust_mem:.2f}",
                format_memory_savings(total_py_mem, total_rust_mem),
            ]
        )

        # Create visualization
        plt.figure(figsize=(15, 6))

        # Time comparison
        plt.subplot(1, 2, 1)
        sns.barplot(data=df, x="operation", y="execution_time", hue="language")
        plt.xticks(rotation=45)
        plt.title("Execution Time Comparison (seconds)")
        plt.ylabel("Seconds")

        # Memory comparison
        plt.subplot(1, 2, 2)
        sns.barplot(data=df, x="operation", y="memory_used", hue="language")
        plt.xticks(rotation=45)
        plt.title("Memory Usage Comparison (MB)")
        plt.ylabel("Megabytes")

        plt.tight_layout()
        plt.savefig("performance_comparison.png")

        # Save comparison table
        comparison.to_csv("performance_comparison.csv")

        # Print comparison summary
        print("\nDetailed Performance Comparison:")
        print(table)

        # Print overall summary
        speed_diff = format_speed_difference(total_py_time, total_rust_time)
        mem_diff = format_memory_savings(total_py_mem, total_rust_mem)
        print("\nOverall Performance Summary:")
        print(f"Speed: {speed_diff}")
        print(f"Memory: {mem_diff}")

        return comparison

    except FileNotFoundError:
        print(
            "\nNote: Rust benchmark data not found. "
            "Run Rust benchmarks first for comparison."
        )
        return None


def main():
    # Clean up old performance files
    cleanup_old_files()

    # Define list to store operation summaries
    performance_summary = []
    total_time = 0
    total_memory = 0

    # URL for extract operation
    url = "https://github.com/fivethirtyeight/data/raw/refs/heads/master/goose/goose_rawdata.csv"
    file_path = "data/goose_rawdata.csv"

    # Perform each operation and track its performance
    operations = [
        ("Extract", extract, [url, file_path]),
        ("Transform/Load", load, ["data/goose_rawdata.csv"]),
        ("Query", DBquery, []),
        ("CRUD-Create", CRUD_Create, []),
        ("CRUD-Read", CRUD_Read, []),
        ("CRUD-Update", CRUD_Update, []),
        ("CRUD-Delete", CRUD_Delete, []),
    ]

    for operation_name, operation_func, args in operations:
        elapsed_time, memory_used, result = track_performance(
            operation_name, operation_func, *args
        )
        performance_summary.append((operation_name, elapsed_time, memory_used, result))
        total_time += elapsed_time
        total_memory += memory_used

    # Save performance data for comparison
    save_performance_data(performance_summary)

    # Compare with Rust if available
    comparison = compare_with_rust()

    if comparison is not None:
        print(
            "\nDetailed comparison data has been saved to 'performance_comparison.csv'"
        )
        print("Visualization has been saved to 'performance_comparison.png'")


if __name__ == "__main__":
    main()
