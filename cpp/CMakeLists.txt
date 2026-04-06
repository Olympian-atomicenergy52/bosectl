cmake_minimum_required(VERSION 3.14)
project(bmap VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(bmap INTERFACE)
target_include_directories(bmap INTERFACE src)
