PROTO_DIR := proto
OUT_DIR := src/proto

PROTO_FILES := $(wildcard $(PROTO_DIR)/*.proto)

.PHONY: proto clean

# Compile all .proto files using cargo build (calls build.rs)
proto:
	cargo build

# Clean generated proto files
clean-proto:
	rm -rf $(OUT_DIR)/*.rs

# Full reset
clean:
	cargo clean
	make clean-proto