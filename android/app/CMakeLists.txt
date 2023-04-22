# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html

# Sets the minimum version of CMake required to build the native library.

cmake_minimum_required(VERSION 3.22.1)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED)

message(STATUS "Begin ${CMAKE_BUILD_TYPE} Build")

if (ANDROID_ABI STREQUAL "x86")
    set(VCPKG_ABI_PREFIX "x86")
elseif(ANDROID_ABI STREQUAL "x86_64")
    set(VCPKG_ABI_PREFIX "x64")
elseif(ANDROID_ABI STREQUAL "arm64-v8a")
    set(VCPKG_ABI_PREFIX "arm64")
elseif(ANDROID_ABI STREQUAL "armeabi-v7a")
    set(VCPKG_ABI_PREFIX "arm-neon")
endif()

set(VCPKG_PLATFORM_DIR "$ENV{VCPKG_ROOT}/installed/${VCPKG_ABI_PREFIX}-android")
set(VCPKG_INCLUDE_DIR "${VCPKG_PLATFORM_DIR}/include")
set(SQLiteCpp_DIR "${VCPKG_PLATFORM_DIR}/share/SQLiteCpp")
set(unofficial-sqlite3_DIR "${VCPKG_PLATFORM_DIR}/share/unofficial-sqlite3")
set(Protobuf_DIR "${VCPKG_PLATFORM_DIR}/share/protobuf")
set(Protobuf_PROTOC_EXECUTABLE "${VCPKG_PLATFORM_DIR}/tools/protobuf/protoc.exe")
# Maybe needed if building on non-windows?
# set(Protobuf_PROTOC_EXECUTABLE "{VCPKG_ABI_DIR}/tools/protobuf/protoc")
set(Protobuf_LIBRARIES "${VCPKG_PLATFORM_DIR}/lib")
set(Protobuf_INCLUDE_DIR ${VCPKG_INCLUDE_DIR})

include_directories("../..")
add_subdirectory("../../engine" "khiin")

# Declares and names the project.
project ("khiindroid" CXX)

find_package(Protobuf CONFIG REQUIRED)
find_package(SQLiteCpp REQUIRED)

include_directories(${PROJECT_SOURCE_DIR} ${VCPKG_INCLUDE_DIR})

# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.

add_library( # Sets the name of the library.
        khiindroid

        # Sets the library as a shared library.
        SHARED

        # Provides a relative path to your source file(s).
        src/main/cpp/EngineController.cpp)

# Searches for a specified prebuilt library and stores the path as a
# variable. Because CMake includes system libraries in the search path by
# default, you only need to specify the name of the public NDK library
# you want to add. CMake verifies that the library exists before
# completing its build.

find_library( # Sets the name of the path variable.
        log-lib

        # Specifies the name of the NDK library that
        # you want CMake to locate.
        log)

# Specifies libraries CMake should link to your target library. You
# can link multiple libraries, such as libraries you define in this
# build script, prebuilt third-party libraries, or system libraries.

target_link_libraries(khiindroid

        # Links target to third party libraries
        khiin SQLiteCpp protobuf::libprotobuf-lite

        # Links the target library to the log library
        # included in the NDK.
        ${log-lib})
