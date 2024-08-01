####################################################################################################
## Builder
####################################################################################################
FROM rust:bookworm AS builder

# Install necessary tools and dependencies
RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ protobuf-compiler && \
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

# Set the working directory
WORKDIR /usr/src/app

# Copy the entire monorepo
COPY . .

# Specify the path to the REST API project if it's not in the root
WORKDIR /usr/src/app/database

# Build the application and set the target directory
RUN cargo build --release --features="server" --bin database --target-dir /usr/src/app/database/target

RUN ls -la /usr/src/app/database/target/
RUN ls -la /usr/src/app/database/*

# Verify the build output
RUN ls -la /usr/src/app/database/target/release/

####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-slim

# Install necessary libraries
RUN apt-get update -y && \
  apt-get install -y ca-certificates && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# Import user and group info
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy the built REST service binary
COPY --from=builder /usr/src/app/database/target/release/database ./

# Use the non-root user
USER app:app

# Specify the default command to run the REST service
CMD ["/app/database"]
