PROJECT_ROOT := $(shell git rev-parse --show-toplevel)
CRATE_DIR := ${PROJECT_ROOT}/bindings/native
MANIFEST := ${CRATE_DIR}/Cargo.toml
TARGET_DIR := ${PROJECT_ROOT}/target
RUNE_HEADER_FILE := ${TARGET_DIR}/rune.h
RUST_SOURCE := $(shell find ${CRATE_DIR} -iname *.rs)
RUST_LIB_DIR := ${TARGET_DIR}/debug
RUNE_LIB := "${RUST_LIB_DIR}/librune_native.so"
CXXFLAGS := -std=c++20 -g -Wl,-rpath,${RUNE_LIB}
LIBS := -ldl -lpthread -lstdc++

example: example.cpp rune.h ${RUNE_LIB}
	${CXX} ${CXXFLAGS} -o example -I ${TARGET_DIR} example.cpp ${RUNE_LIB} ${LIBS}

rune.h: ${RUNE_LIB}
	cargo test --manifest-path ${MANIFEST} --features c-headers -- generate_headers
	cp ${RUNE_HEADER_FILE} rune.h
	clang-format -i rune.h

${RUNE_LIB}: ${RUST_SOURCE}
	cargo build --manifest-path ${MANIFEST}

clean:
	$(RM) example
	$(RM) rune.h

.PHONY: clean
