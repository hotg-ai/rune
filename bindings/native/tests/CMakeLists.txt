cmake_minimum_required(VERSION 3.14)
project(rune_native_tests)
set(CMAKE_CXX_STANDARD 14)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/609281088cfefc76f9d0ce82e1ff6c30cc3591e5.zip
)
FetchContent_MakeAvailable(googletest)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG origin/master
)
FetchContent_MakeAvailable(Corrosion)

set(CRATE_MANIFEST_DIR ${CMAKE_SOURCE_DIR}/..)

# Make sure we regenerate the rune.h header automatically.
add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/rune.h
    COMMAND cbindgen ${CRATE_MANIFEST_DIR} --output ${CMAKE_CURRENT_BINARY_DIR}/rune.h
    COMMENT "Generating rune.h"
)
add_custom_target(rune_header ALL DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/rune.h)

corrosion_import_crate(MANIFEST_PATH ${CRATE_MANIFEST_DIR}/Cargo.toml)


enable_testing()

add_executable(
  integration_tests
  rune_test.cpp
  ${CMAKE_CURRENT_BINARY_DIR}/rune.h
)
add_dependencies(integration_tests rune_header rune-native)
include_directories(${rune_native_tests_BINARY_DIR})

target_link_libraries(integration_tests gtest_main rune-native)

include(GoogleTest)
gtest_discover_tests(integration_tests)
