 cmake_minimum_required(VERSION 4.1)
project(cpp_robotics_algorithms)

find_package(Eigen3 REQUIRED)
find_package(PCL REQUIRED)
find_package(benchmark REQUIRED) 

add_subdirectory(pointcloud)