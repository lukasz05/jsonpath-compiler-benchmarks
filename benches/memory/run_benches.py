import os
import subprocess

SIMDJSON_PATH = os.environ["SIMDJSON_PATH"]

datasets = {
    "twitter_large": "../../data/pison/twitter_large_record.json"
}

benchmarks = [
    {
        "name": "status_with_id_screen_name_large",
        "query": "$[?(@.id == 787994024502190080)].entities.user_mentions[0].screen_name",
        "dataset": "twitter_large"
    },
    {
        "name": "status_with_id_descendants_large",
        "query": "$[?(@.id == 787994024502190080)]..*",
        "dataset": "twitter_large"
    }
]

def get_bench_file_name_base(bench, eager_filters):
    return bench["name"] + ("_eager" if eager_filters else "_lazy") + "_filters"

def generate_query_code(bench, eager_filters):
    out_file_name = get_bench_file_name_base(bench, eager_filters) + ".cpp"
    query = bench["query"]
    args = [
        "jsonpath-compiler",
        "--target", "simdjson-ondemand",
        "--standalone",
        "-o", out_file_name,
        "--mmap"
    ]
    if eager_filters:
        args.append("--eager-filter-evaluation")
    subprocess.run(args + [query])


def compile_query_code(bench, eager_filters):
    out_file_name = get_bench_file_name_base(bench, eager_filters)
    code_file_name = out_file_name + ".cpp"
    subprocess.run([
        "c++", code_file_name, f"{SIMDJSON_PATH}/simdjson.cpp",
        "-O3",
        "-std=c++20",
        f"-I{SIMDJSON_PATH}",
        "-o", out_file_name
    ])


def run_valgrind(bench, eager_filters):
    prog_file_name = get_bench_file_name_base(bench, eager_filters)
    massif_file_name = prog_file_name + ".out"
    dataset_path = datasets[bench["dataset"]]
    subprocess.run([
        "valgrind",
        "--tool=massif",
        "--massif-out-file=" + massif_file_name,
        "./" + prog_file_name, dataset_path
    ])


for bench in benchmarks:
    for eager_filters in (False, True):
        generate_query_code(bench, eager_filters)
        compile_query_code(bench, eager_filters)
        run_valgrind(bench, eager_filters)

