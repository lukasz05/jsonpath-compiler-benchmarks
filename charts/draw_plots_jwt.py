import json
import os
import matplotlib.pyplot as plt
import numpy as np
import math
from collections import namedtuple
from typing import List

criterion_path = ""

benchmarks = [
    "10_claims",
    "20_claims",
    "30_claims",
    "40_claims",
    "40_claims",
    "50_claims",
    "100_claims",
    "150_claims",
    "200_claims"
]

TargetData = namedtuple("TargetData", ["target_name", "measurements"])


def get_child_dirs(path: str) -> List[str]:
    return next(os.walk(path))[1]

def plot_benchmark(name, targets_data):
    x = np.arange(1, len(targets_data[0].measurements) + 1, 1)
    fig, ax = plt.subplots()
    for target_data in targets_data:
        ax.plot(x, target_data.measurements, label=target_data.target_name, marker="o")
    ax.set(xlabel="selected claims", ylabel="time (us)", title=name.replace("_", " "))
    ax.legend(loc="best", ncols=1)
    plt.xticks(x)
    plt.savefig(name + ".png")

def plot_single_claim_benchmark(targets_data):
    x = np.arange(10, len(list(targets_data.values())[0]) * 10 + 1, 10)
    fig, ax = plt.subplots()
    for target_name, measurements in targets_data.items():
        ax.plot(x, measurements, label=target_name, marker="o")
    ax.set(xlabel="claims in payload", ylabel="time (us)", title="select single claim")
    ax.legend(loc="best", ncols=1)
    plt.savefig("single_claim_growing_payload.png")

def main():
    for benchmark_name in benchmarks:
        benchmark_dir_path = os.path.join(criterion_path, benchmark_name)
        target_dirs = filter(lambda dir_name: dir_name != "report", get_child_dirs(benchmark_dir_path))
        target_name_to_dirs_map = {}
        for target_dir in target_dirs:
            target_name = "_".join(target_dir.split("_")[:-1])
            if target_name not in target_name_to_dirs_map:
                target_name_to_dirs_map[target_name] = []
            target_name_to_dirs_map[target_name].append(target_dir)
        for target_dirs in target_name_to_dirs_map.values():
            target_dirs.sort(key=lambda x: int(x.split("_")[-1]))
        targets_data = []
        for target_name in target_name_to_dirs_map.keys():
            target_data = TargetData(target_name, [])
            for target_dir in target_name_to_dirs_map[target_name]:
                estimates_path = os.path.join(benchmark_dir_path, target_dir, "base", "estimates.json")
                with open(estimates_path, "r") as f:
                    estimates = json.load(f)
                    mean_time = estimates["mean"]["point_estimate"]
                    mean_time = round(mean_time / 1000, 2)
                    target_data.measurements.append(mean_time)
            targets_data.append(target_data)
        plot_benchmark(benchmark_name, targets_data)

    benchmark = "single_claim_growing_payload"
    benchmark_dir_path = os.path.join(criterion_path, benchmark)
    target_dirs = list(filter(lambda dir_name: dir_name != "report", get_child_dirs(benchmark_dir_path)))
    target_dirs.sort(key=lambda x: int(x.split("_")[-1]))
    targets_data = {}
    for target_dir in target_dirs:
        print(target_dir)
        target_name = "_".join(target_dir.split("_")[:-1])
        target_dirs.sort(key=lambda x: int(x.split("_")[-1]))
        estimates_path = os.path.join(benchmark_dir_path, target_dir, "base", "estimates.json")
        with open(estimates_path, "r") as f:
            estimates = json.load(f)
            mean_time = estimates["mean"]["point_estimate"]
            mean_time = round(mean_time / 1000, 2)
            if target_name not in targets_data:
                targets_data[target_name] = []
            targets_data[target_name].append(mean_time)
    print(targets_data)
    plot_single_claim_benchmark(targets_data)

main()
