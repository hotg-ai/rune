# Note: in order to compile Rune on Windows we need to explicitly install clang
# and record where libclang is.
# https://github.com/rust-lang/rust-bindgen/issues/1797

echo "LIBCLANG_PATH=$((gcm clang).source -replace 'clang.exe')" >> $env:GITHUB_ENV
# tflite needs C++14 or higher to generate STL bindings
echo "BINDGEN_EXTRA_CLANG_ARGS=-std=c++17" >> $env:GITHUB_ENV
