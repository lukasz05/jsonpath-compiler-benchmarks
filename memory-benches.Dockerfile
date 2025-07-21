FROM debian:bookworm-slim@sha256:6ac2c08566499cc2415926653cf2ed7c3aedac445675a013cc09469c9e118fdd

WORKDIR /app

# Install required packages
RUN apt-get update && apt-get install -y git wget gzip python3 build-essential ca-certificates libssl-dev \
    pkg-config valgrind

# Install Rust 1.88.0
RUN wget -q -O - https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.88.0
ENV PATH="/root/.cargo/bin:${PATH}"

# Download simdjson 3.13.0
WORKDIR /app/simdjson
RUN wget -q https://raw.githubusercontent.com/simdjson/simdjson/refs/tags/v3.13.0/singleheader/simdjson.cpp \
            https://raw.githubusercontent.com/simdjson/simdjson/refs/tags/v3.13.0/singleheader/simdjson.h
ENV SIMDJSON_PATH="/app/simdjson"

# Download and build jsonpath-compiler
WORKDIR /app
RUN git clone https://github.com/lukasz05/jsonpath-compiler
WORKDIR /app/jsonpath-compiler
RUN git checkout thesis-submission
RUN cargo build --release
ENV PATH="/app/jsonpath-compiler/target/release:${PATH}"

# Download datasets
WORKDIR /app/jsonpath-compiler-benchmarks
ADD https://zenodo.org/record/8395641/files/twitter_large_record.json.gz data/pison/twitter_large_record.json.gz
RUN gunzip data/pison/twitter_large_record.json.gz

# Copy and run benchmarks
COPY benches/memory ./benches/memory
WORKDIR /app/jsonpath-compiler-benchmarks/benches/memory

CMD ["python3", "run_benches.py"]