RUST_FLAGS := --release
TARGET_DIR := ../../target/release
CXX_FLAGS := -g -std=c++20
RUST_FILES := $(wildcard ../**/*.rs)
LIBS := -ldl -lpthread

main: main.cpp rune.h librune_ffi.a
	$(CXX) $(CXX_FLAGS) main.cpp -o $@ librune_ffi.a $(LIBS)

librune_ffi.a: $(RUST_FILES)
	cargo build $(RUST_FLAGS)
	cp $(TARGET_DIR)/$@ .

rune.h: $(RUST_FILES)
	cbindgen -o $@ --config ../cbindgen.toml ..

clean:
	$(RM) rune.h librune_ffi.a main

.PHONY: clean
