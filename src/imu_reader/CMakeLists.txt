cmake_minimum_required(VERSION 3.8)
project(imu_reader)

# 打开编译器警告（可选）
if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 17)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(sensor_msgs REQUIRED)

add_executable(imu_reader src/imu_reader.cpp)
ament_target_dependencies(imu_reader rclcpp sensor_msgs)

install(TARGETS imu_reader
  DESTINATION lib/${PROJECT_NAME})

ament_package()
