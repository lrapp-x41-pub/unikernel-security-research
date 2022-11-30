include(../hermit-playground/cmake/HermitCore-Application.cmake)

cmake_minimum_required(VERSION 3.10)

project(exploitable-application)

add_executable(application application.c)
