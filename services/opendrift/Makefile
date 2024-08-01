.PHONY: proto install build run

proto:
	bash scripts/generate_protos.sh

install:
	pip install -r requirements.txt

build: proto install
	docker build -t opendrift-simulation .

run:
	docker run -p 8080:8080 -p 50051:50051 opendrift-simulation
