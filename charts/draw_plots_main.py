import json
import os
import matplotlib.pyplot as plt
import numpy as np
import math
from collections import namedtuple
from typing import List

criterion_path = "/Users/lukasz/source/jsonpath-compiler-benchmarks/target/criterion"

benchmark_groups = [
    {
        "name": "ast",
        "benchmarks": [
            {"name": "._data_ast_ast.json_ast__deepest"},
            {"name": "._data_ast_ast.json_ast__nested_inner"},
            {"name": "._data_ast_ast.json_inner_array"}
        ]
    },
    {
        "name": "canada",
        "benchmarks": [
            {"name": "._data_nativejson_canada.json_canada__coord_476_1446_1"},
            {"name": "._data_nativejson_canada.json_canada__second_coord_component"}
        ]
    },
    {
        "name": "citm",
        "benchmarks": [
            {"name": "._data_nativejson_citm.json_citm__seatCategoryId"}
        ]
    },
    {
        "name": "bestbuy",
        "benchmarks": [
            {"name": "._data_pison_bestbuy_short_record.json_bestbuy__all_nodes"}
        ]
    },
    {
        "name": "google_map",
        "benchmarks": [
            {"name": "._data_pison_google_map_short_record.json_google_map__routes"}
        ]
    },
    {
        "name": "twitter",
        "benchmarks": [
            {"name": "._data_twitter_twitter.json_user_mentions_indices"}
        ]
    },
]

TargetData = namedtuple("TargetData", ["target_name", "mean_throughput"])


def get_child_dirs(path: str) -> List[str]:
    return next(os.walk(path))[1]


def get_target_data(target_dir_path: str) -> TargetData:
    benchmark_path = os.path.join(target_dir_path, "base", "benchmark.json")
    with open(benchmark_path, "r") as f:
        benchmark = json.load(f)
        target_name = benchmark["function_id"]
        input_size = benchmark["throughput"]["BytesDecimal"]
    estimates_path = os.path.join(target_dir_path, "base", "estimates.json")
    with open(estimates_path, "r") as f:
        estimates = json.load(f)
        mean_time = estimates["mean"]["point_estimate"]
    return TargetData(target_name, round(input_size / mean_time * 1000))


def plot_benchmark_group(group):
    benchmarks = group["benchmarks"]
    max_measurement = 0
    target_measurements = {}
    for i, benchmark in enumerate(benchmarks):
        targets_data = benchmark["targets_data"]
        for target_data in targets_data:
            measurement = target_data.mean_throughput
            max_measurement = max(max_measurement, measurement)
            label = target_data.target_name
            if label not in target_measurements:
                target_measurements[label] = [float("NaN")] * i
            target_measurements[label].append(measurement)
        for measurements in target_measurements.values():
            if len(measurements) != i + 1:
                measurements.append(float("NaN"))
    x = np.arange(len(benchmarks))
    fig, ax = plt.subplots(layout='constrained')
    width = 0.2
    multiplier = 0
    for target, measurement in target_measurements.items():
        offset = width * multiplier
        rects = ax.bar(x + offset, measurement, width, label=target)
        ax.bar_label(rects, padding=3)
        multiplier += 1
    ax.set_ylabel("Throughput (MB/s)")
    ax.set_title(group["name"] + ".json")
    benchmarks_short_names = [benchmark["name"].split(".")[-1].replace("json_", "").replace(f"{group["name"]}__", "")
                              for benchmark in benchmarks]
    ax.set_xticks(x + width, benchmarks_short_names)
    ax.legend(loc="best", ncols=1)
    ax.set_ylim(0, ((max_measurement // 500) + 1) * 500)
    plt.savefig(group["name"] + ".png")


def main():
    for group in benchmark_groups:
        for benchmark in group["benchmarks"]:
            benchmark_dir_path = os.path.join(criterion_path, benchmark["name"])
            target_dirs = filter(lambda dir_name: dir_name != "report", get_child_dirs(benchmark_dir_path))
            benchmark["targets_data"] = []
            for target_dir in target_dirs:
                target_dir_path = os.path.join(benchmark_dir_path, target_dir)
                target_data = get_target_data(target_dir_path)
                benchmark["targets_data"].append(target_data)
        plot_benchmark_group(group)


main()
