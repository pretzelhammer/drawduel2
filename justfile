# list commands
default:
    @just --list

# list commands
list:
	@just --list

protoc-version:
	protoc --version

generate-rs:
	protoc --prost_out=./backend/src/game/full/engine --proto_path=./agnostic game.proto
	protoc --prost_out=./backend/src/game/mini/engine --proto_path=./agnostic mini_game.proto

generate-ts:
	protoc --plugin=./backend/node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./frontend/src/game/full/engine --proto_path=./agnostic game.proto
	protoc --plugin=./backend/node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./frontend/src/game/mini/engine --proto_path=./agnostic mini_game.proto

generate: generate-rs generate-ts
