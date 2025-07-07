FROM debian:bookworm-slim@sha256:6ac2c08566499cc2415926653cf2ed7c3aedac445675a013cc09469c9e118fdd

WORKDIR /app

# Install OpenJDK 17 JRE and other required packages
RUN apt-get update && apt-get install -y git wget build-essential ca-certificates libssl-dev pkg-config \
    openjdk-17-jre-headless=17.0.15+6-1~deb12u1
ENV JAVA_HOME="/usr"

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
RUN git clone https://github.com/lukasz05/jsonpath-compiler # TODO: checkout a specific version
WORKDIR /app/jsonpath-compiler
RUN cargo build --release
ENV PATH="/app/jsonpath-compiler/target/release:${PATH}"

# Copy and build benchmarks
WORKDIR /app/jsonpath-compiler-benchmarks
COPY Cargo.toml Cargo.lock build.rs ./
COPY .cargo ./cargo
COPY src ./src
COPY benches ./benches
RUN cargo test --no-run --release --benches

CMD ["cargo", "bench"]
