cmake_minimum_required(VERSION 4.0)
project(SimpleCaloriesCounter)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17 -Wall")
set(CMAKE_CXX_STANDARD 17)

add_executable(main src/main.cpp)

# sqlite3 block start
add_library(sqlite3 STATIC 
    libs/sqlite/sqlite3.c
)

target_include_directories(sqlite3 PUBLIC 
    ${CMAKE_CURRENT_SOURCE_DIR}/libs/sqlite
)

target_compile_options(sqlite3 PRIVATE -O2 -DSQLITE_DEFAULT_MEMSTATUS=0)

target_link_libraries(main PRIVATE sqlite3)
# sqlite3 block end