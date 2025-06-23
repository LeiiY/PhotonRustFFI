cmake_minimum_required(VERSION 3.14)
project(demo)

include(FetchContent)
set(FETCHCONTENT_QUIET false)

set(PHOTON_ENABLE_FSTACK_DPDK ON CACHE INTERNAL "Enable DPDKs")

FetchContent_Declare(
    photon
    SOURCE_DIR /home/ubuntu/leiiy/PhotonLibOS
)
FetchContent_MakeAvailable(photon)

find_package(gflags REQUIRED)
find_package(PkgConfig REQUIRED)
pkg_check_modules(DPDK REQUIRED libdpdk)

find_path(FSTACK_INCLUDE_DIRS NAMES ff_api.h PATHS /usr/local/include NO_DEFAULT_PATH)
find_library(FSTACK_LIBRARIES NAMES fstack PATHS /usr/local/lib NO_DEFAULT_PATH)

set(FSTACK_INCLUDE_DIRS ${FSTACK_INCLUDE_DIRS} ${DPDK_INCLUDE_DIRS})
set(FSTACK_LIBRARIES ${FSTACK_LIBRARIES} ${DPDK_LIBRARIES})

# Fix include path
add_library(net-perf STATIC net-perf.cpp)
# add_executable(net-perf  net-perf.cpp)
target_include_directories(net-perf PRIVATE ${photon_SOURCE_DIR}/include ${FSTACK_INCLUDE_DIRS})
target_link_libraries(net-perf photon_static gflags ${FSTACK_LIBRARIES})


