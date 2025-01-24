# list commands
default:
    @just --list

# list commands
list:
	@just --list

protoc-version:
	protoc --version

build-proto-rs:
	protoc --prost_out=./backend/drawduel_engine/src/game/full --proto_path=./agnostic game.proto
	protoc --prost_out=./backend/drawduel_engine/src/game/mini --proto_path=./agnostic mini_game.proto

build-proto-ts:
	protoc --plugin=./frontend/node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./frontend/src/game/full/engine --proto_path=./agnostic game.proto
	protoc --plugin=./frontend/node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./frontend/src/game/mini/engine --proto_path=./agnostic mini_game.proto

build-proto: build-proto-rs build-proto-ts

build-wasm:
    cd ./backend/drawduel_wasm && wasm-pack build --release --target nodejs
    cp ./backend/drawduel_wasm/pkg/drawduel_wasm* ./frontend/tests/wasm

normalize-words:
    cd ./backend/drawduel_scripts && cargo run --bin normalize_words

generate-words:
    cd ./backend/drawduel_scripts && cargo run --bin generate_words
