#!/bin/bash

proto_dir="src/proto"
out_dir="src/proto"

if [ ! -d "$out_dir" ]; then
  mkdir -p "$out_dir"
fi

python3 -m grpc_tools.protoc \
    -I${proto_dir} \
    --proto_path=. \
    --python_out=. \
    --grpc_python_out=. \
    --python_out=${out_dir} \
    --grpc_python_out=.
    proto/opendrift.proto
