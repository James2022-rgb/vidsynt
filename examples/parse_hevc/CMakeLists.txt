cmake_minimum_required(VERSION 3.15)
project(vidsynt_hevc_example CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Use static runtime library to match Bento4 and Rust library
if(MSVC)
    set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
endif()

# Fetch Bento4 library for MP4 parsing
include(FetchContent)
FetchContent_Declare(
    Bento4
    GIT_REPOSITORY https://github.com/axiomatic-systems/Bento4.git
    GIT_TAG        master
)
FetchContent_MakeAvailable(Bento4)

# Determine build type for library path
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# Convert CMAKE_BUILD_TYPE to lowercase for path matching
string(TOLOWER "${CMAKE_BUILD_TYPE}" BUILD_TYPE_LOWER)

# Find vidsynt library
# Assumes vidsynt was built with: cargo build --release --features ffi
# or: cargo build --features ffi (for debug builds)
set(VIDSYNT_ROOT "${CMAKE_SOURCE_DIR}/../..")

if(BUILD_TYPE_LOWER STREQUAL "debug")
    set(CARGO_BUILD_DIR "${VIDSYNT_ROOT}/target/debug")
else()
    set(CARGO_BUILD_DIR "${VIDSYNT_ROOT}/target/release")
endif()

# Find the vidsynt static library
if(WIN32)
    find_library(VIDSYNT_LIB
        NAMES vidsynt.lib
        HINTS "${CARGO_BUILD_DIR}"
        REQUIRED
    )
else()
    find_library(VIDSYNT_LIB
        NAMES libvidsynt.a
        HINTS "${CARGO_BUILD_DIR}"
        REQUIRED
    )
endif()

if(NOT VIDSYNT_LIB)
    message(FATAL_ERROR "vidsynt library not found. Build with: cargo build --release --features ffi")
endif()

# Include directory for vidsynt.h
set(VIDSYNT_INCLUDE_DIR "${VIDSYNT_ROOT}/vidsynt-sys/include")

if(NOT EXISTS "${VIDSYNT_INCLUDE_DIR}/vidsynt.h")
    message(FATAL_ERROR "vidsynt.h not found. Build vidsynt-sys with: cd vidsynt-sys && cargo build")
endif()

# Create executable
add_executable(parse_hevc main.cpp mp4_demux.cpp)

# Set include directories
target_include_directories(parse_hevc PRIVATE ${VIDSYNT_INCLUDE_DIR})

# Link libraries
target_link_libraries(parse_hevc PRIVATE ${VIDSYNT_LIB} ap4)

# On Windows, link additional system libraries that Rust depends on
if(WIN32)
    target_link_libraries(parse_hevc PRIVATE
        ws2_32      # Windows sockets
        userenv     # User environment
        bcrypt      # Cryptography
        ntdll       # NT kernel
    )
endif()

# On Unix, link additional libraries (pthread, dl, etc.)
if(UNIX AND NOT APPLE)
    target_link_libraries(parse_hevc PRIVATE
        pthread
        dl
        m
    )
elseif(APPLE)
    target_link_libraries(parse_hevc PRIVATE
        "-framework Security"
        "-framework CoreFoundation"
    )
endif()

# Print configuration info
message(STATUS "vidsynt library: ${VIDSYNT_LIB}")
message(STATUS "vidsynt include: ${VIDSYNT_INCLUDE_DIR}")
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
