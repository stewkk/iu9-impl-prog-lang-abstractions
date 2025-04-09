debug: build
	rust-gdb --eval-command "source gdb/plugin.py" --args target/debug/stack-assembly-interpreter main.asm

run:
	cargo run -- main.asm

build:
	cargo build

.PHONY: debug run build
