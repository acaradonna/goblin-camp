cmake_minimum_required(VERSION 3.20)
project(ape LANGUAGES C CXX VERSION 0.0.1)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

option(APE_BUILD_TESTS "Build APE tests" ON)
option(APE_BUILD_SHARED "Build shared library" ON)
option(APE_ENABLE_WARNINGS "Enable strict compiler warnings" ON)
option(APE_ENABLE_SANITIZERS "Enable ASan/UBSan in Debug" OFF)
option(APE_C_ABI_SHARED "Build C ABI library as a shared library" ON)
option(APE_BUILD_WASM "Build WebAssembly module (requires Emscripten toolchain)" OFF)

add_library(ape_core
    src/ape.cpp
    src/foundation/job.cpp
    src/collision/broadphase.cpp
)

target_include_directories(ape_core PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

target_compile_features(ape_core PUBLIC cxx_std_20)

if(APE_ENABLE_WARNINGS)
    if(MSVC)
        target_compile_options(ape_core PRIVATE /W4)
    else()
        target_compile_options(ape_core PRIVATE -Wall -Wextra -Wpedantic)
    endif()
endif()

# C ABI for bindings
if(APE_C_ABI_SHARED)
    add_library(ape_c SHARED cbindings/ape_c.cpp)
else()
    add_library(ape_c STATIC cbindings/ape_c.cpp)
endif()

target_include_directories(ape_c PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

target_link_libraries(ape_c PUBLIC ape_core)

if(APE_ENABLE_WARNINGS)
    if(MSVC)
        target_compile_options(ape_c PRIVATE /W4)
    else()
        target_compile_options(ape_c PRIVATE -Wall -Wextra -Wpedantic)
    endif()
endif()

if(APE_BUILD_SHARED)
    add_library(ape SHARED src/ape_export.cpp)
    target_link_libraries(ape PRIVATE ape_core)
    target_include_directories(ape PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
    )
    if(APE_ENABLE_WARNINGS)
        if(MSVC)
            target_compile_options(ape PRIVATE /W4)
        else()
            target_compile_options(ape PRIVATE -Wall -Wextra -Wpedantic)
        endif()
    endif()
endif()

# Simple smoke test
if(APE_BUILD_TESTS)
    enable_testing()
    add_executable(ape_smoke test/smoke.cpp)
    target_link_libraries(ape_smoke PRIVATE ape_core)
    add_test(NAME smoke COMMAND ape_smoke)

    if(APE_ENABLE_WARNINGS)
        if(MSVC)
            target_compile_options(ape_smoke PRIVATE /W4)
        else()
            target_compile_options(ape_smoke PRIVATE -Wall -Wextra -Wpedantic)
        endif()
    endif()
    add_executable(ape_determinism test/determinism.cpp)
    target_link_libraries(ape_determinism PRIVATE ape_core)
    add_test(NAME determinism COMMAND ape_determinism)

    if(APE_ENABLE_WARNINGS)
        if(MSVC)
            target_compile_options(ape_determinism PRIVATE /W4)
        else()
            target_compile_options(ape_determinism PRIVATE -Wall -Wextra -Wpedantic)
        endif()
    endif()
    add_executable(ape_c_smoke test/c_api_smoke.c)
    target_link_libraries(ape_c_smoke PRIVATE ape_c)
    add_test(NAME c_smoke COMMAND ape_c_smoke)

    if(APE_ENABLE_WARNINGS)
        if(MSVC)
            target_compile_options(ape_c_smoke PRIVATE /W4)
        else()
            target_compile_options(ape_c_smoke PRIVATE -Wall -Wextra -Wpedantic)
        endif()
    endif()
    add_executable(ape_handles test/handles.cpp)
    target_link_libraries(ape_handles PRIVATE ape_core)
    add_test(NAME handles COMMAND ape_handles)

    if(APE_ENABLE_WARNINGS)
        if(MSVC)
            target_compile_options(ape_handles PRIVATE /W4)
        else()
            target_compile_options(ape_handles PRIVATE -Wall -Wextra -Wpedantic)
        endif()
    endif()
    add_executable(ape_broadphase test/broadphase.cpp)
    target_link_libraries(ape_broadphase PRIVATE ape_core)
    add_test(NAME broadphase COMMAND ape_broadphase)

    if(APE_ENABLE_WARNINGS)
        if(MSVC)
            target_compile_options(ape_broadphase PRIVATE /W4)
        else()
            target_compile_options(ape_broadphase PRIVATE -Wall -Wextra -Wpedantic)
        endif()
    endif()
    add_executable(ape_world_bp test/world_broadphase_integration.cpp)
    target_link_libraries(ape_world_bp PRIVATE ape_core)
    add_test(NAME world_broadphase COMMAND ape_world_bp)

    if(APE_ENABLE_WARNINGS)
        if(MSVC)
            target_compile_options(ape_world_bp PRIVATE /W4)
        else()
            target_compile_options(ape_world_bp PRIVATE -Wall -Wextra -Wpedantic)
        endif()
    endif()
endif()

add_subdirectory(examples)

add_executable(ape_bench_micro bench/micro_step.cpp)
target_link_libraries(ape_bench_micro PRIVATE ape_core)
if(APE_ENABLE_WARNINGS)
    if(MSVC)
        target_compile_options(ape_bench_micro PRIVATE /W4)
    else()
        target_compile_options(ape_bench_micro PRIVATE -Wall -Wextra -Wpedantic)
    endif()
endif()

if(APE_ENABLE_SANITIZERS AND CMAKE_BUILD_TYPE MATCHES Debug AND NOT MSVC)
    foreach(tgt IN ITEMS ape_core ape_c ape ape_smoke ape_determinism ape_c_smoke)
        if(TARGET ${tgt})
            target_compile_options(${tgt} PRIVATE -fsanitize=address,undefined)
            target_link_options(${tgt} PRIVATE -fsanitize=address,undefined)
        endif()
    endforeach()
endif()

# Install rules
include(GNUInstallDirs)
install(TARGETS ape_core ape_c
    EXPORT APETargets
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
if(APE_BUILD_SHARED)
    install(TARGETS ape
        EXPORT APETargets
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
        RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
endif()
install(DIRECTORY include/ DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})

install(EXPORT APETargets
    NAMESPACE APE::
    DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/APE)

include(CMakePackageConfigHelpers)
write_basic_package_version_file(
    ${CMAKE_CURRENT_BINARY_DIR}/APEConfigVersion.cmake
    VERSION ${PROJECT_VERSION}
    COMPATIBILITY SameMajorVersion)

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/cmake/APEConfig.cmake.in
           ${CMAKE_CURRENT_BINARY_DIR}/APEConfig.cmake
           @ONLY)

install(FILES
    ${CMAKE_CURRENT_BINARY_DIR}/APEConfig.cmake
    ${CMAKE_CURRENT_BINARY_DIR}/APEConfigVersion.cmake
    DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/APE)

# WebAssembly build (optional, requires Emscripten toolchain)
if(APE_BUILD_WASM)
    if (CMAKE_CXX_COMPILER_ID STREQUAL "Emscripten")
        add_executable(ape_wasm
            src/ape.cpp
            src/foundation/job.cpp
            src/collision/broadphase.cpp
            cbindings/ape_c.cpp)

        target_include_directories(ape_wasm PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
        # Place outputs directly into web/js
        set_target_properties(ape_wasm PROPERTIES
            OUTPUT_NAME ape_wasm
            RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/web/js)

        # Emscripten-specific link options
        target_link_options(ape_wasm PRIVATE
            -O3
            -s MODULARIZE=1
            -s EXPORT_NAME=Module
            -s ENVIRONMENT=web
            -s EXPORTED_FUNCTIONS='[_ape_world_create,_ape_world_destroy,_ape_world_step,_ape_world_create_rigidbody_p,_ape_world_get_position_out,_ape_world_set_gravity_p,_ape_version_major,_ape_version_minor,_ape_version_patch]'
            -s EXPORTED_RUNTIME_METHODS='[cwrap,ccall,HEAPF32,_malloc,_free]')
    else()
        message(WARNING "APE_BUILD_WASM is ON but compiler is not Emscripten. Configure with emsdk toolchain to build WASM.")
    endif()
endif()
