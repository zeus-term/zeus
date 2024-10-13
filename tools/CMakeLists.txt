cmake_minimum_required(VERSION 3.28.0)

project (zeus_tools)

file(GLOB_RECURSE SOURCES "src/main/*.c")

foreach(SOURCE_FILE ${SOURCES})
  get_filename_component(FILE_NAME ${SOURCE_FILE} NAME_WE)


  add_executable(${FILE_NAME} ${SOURCE_FILE})
endforeach()
