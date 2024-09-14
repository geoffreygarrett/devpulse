####################################################################################################
## Stage 1: Chef - Prepare
####################################################################################################
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /usr/src/app

####################################################################################################
## Stage 2: Planner - Create Recipe
####################################################################################################
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Stage 3: Builder - Cache Dependencies and Build Application
####################################################################################################
FROM chef AS builder

# Copy the recipe file to the builder stage
COPY --from=planner /usr/src/app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the source code and build the application
COPY . .
RUN cargo build --release --bin devpulse --target-dir /usr/src/app/api/target

# Verify the build output
RUN ls -la /usr/src/app/api/target/release/

####################################################################################################
## Stage 4: Final - Run Application
####################################################################################################
FROM debian:bookworm-slim

# Install necessary libraries
RUN apt-get update -y && \
  apt-get install -y ca-certificates curl && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# Create a non-root user
ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Import user and group info
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy the built REST service binary
COPY --from=builder /usr/src/app/api/target/release/devpulse ./

# Use the non-root user
USER app:app

# Specify the default command to run the REST service
CMD ["/app/devpulse"]
