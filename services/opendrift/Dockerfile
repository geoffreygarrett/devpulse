# Use a specific version of OpenDrift
FROM metocean/opendrift:v1.8.0.0

# Install additional dependencies
RUN apt-get update && apt-get install -y \
    python3-pip \
    gdal-bin \
    libgdal-dev

# Set the working directory
WORKDIR /app

# Copy the application files
COPY . /app

# Install Python dependencies
RUN pip3 install --upgrade pip
RUN pip3 install -r requirements.txt

# Generate protobuf files
RUN bash scripts/generate_protos.sh

# Expose ports for gRPC and REST servers
EXPOSE 50051 8080

# Define the entry point to run both servers simultaneously
CMD ["python3", "src/main.py"]
