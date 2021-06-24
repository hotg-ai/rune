RUST_FLAGS := --release
TARGET_DIR := ../../target/release
CXX_FLAGS := -g -std=c++17 -Wall
RUST_FILES := $(wildcard ../**/*.rs)
LIBS := -ldl -lpthread
LIBRARY := librune.a

main: main.cpp rune_sdk.hpp rune.h $(LIBRARY)
	$(CXX) $(CXX_FLAGS) main.cpp -o $@ $(LIBRARY) $(LIBS)

$(LIBRARY): $(RUST_FILES)
	cargo build $(RUST_FLAGS)
	cp $(TARGET_DIR)/$@ .

rune.h: $(RUST_FILES)
	cbindgen -o $@ --config ../cbindgen.toml ..

clean:
	$(RM) rune.h $(LIBRARY) main

.PHONY: clean
