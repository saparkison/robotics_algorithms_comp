add_executable(transform transform.cpp)
target_link_libraries(transform Eigen3::Eigen ${PCL_LIBRARIES} benchmark::benchmark)

add_executable(kd_tree kd_tree.cpp)
target_link_libraries(kd_tree Eigen3::Eigen ${PCL_LIBRARIES} benchmark::benchmark)