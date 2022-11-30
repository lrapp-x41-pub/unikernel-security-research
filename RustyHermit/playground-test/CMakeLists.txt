cmake_minimum_required(VERSION 3.10)

project(playground-test)

# add the executable
add_executable(test test.c)

include(../hermit-playground/cmake/HermitCore-Application.cmake)
