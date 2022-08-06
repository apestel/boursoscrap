all: build

clean:
	rm -rf target/*

build:
	cargo build

push:
	docker build -t pestouille/boursoscrap:0.0.1 .
	docker push pestouille/boursoscrap:0.0.1