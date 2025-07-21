# Benchmarks for `jsonpath-compiler`

Benchmark suite for [`jsonpath-compiler`](https://github.com/lukasz05/jsonpath-compiler),
based on [`rsonpath-benchmarks`](https://github.com/V0ldek/rsonpath-benchmarks).

## Running benchmarks

### Criterion

The `criterion` benchmarks can be run in a Docker container as follows:
```commandline
docker build . -t jsonpath-compiler-benches
docker run -it jsonpath-compile-benches
```

The results can be retrieved by copying the `criterion` directory from the container:
```commandline
docker cp <container name>:/app/jsonpath-compiler-benchmarks/target/criterion/ criterion
```

### Memory consumption

Similarly, memory consumption benchmarks can be run with:
```commandline
docker build . -f memory-benches.Dockerfile -t jsonpath-compiler-memory-benches
docker run -it jsonpath-compiler-memory-benches
```

The results can be copied from the container:
```commandline
docker cp <container name>:/app/jsonpath-compiler-benchmarks/benches/memory/ results
```

