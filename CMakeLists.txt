cmake_minimum_required(VERSION 3.10)

project(woff2-static)

if(CMAKE_SYSTEM_NAME STREQUAL "Emscripten")
  # Disable exceptions
  string(APPEND CMAKE_CXX_FLAGS " -fno-exceptions") 
endif()


set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)
set(BUILD_SHARED_LIBS OFF)
set(NOISY_LOGGING OFF)
add_subdirectory(woff2/brotli EXCLUDE_FROM_ALL)
set(BROTLIDEC_INCLUDE_DIRS ${CMAKE_CURRENT_SOURCE_DIR}/woff2/brotli/c/include)
set(BROTLIENC_INCLUDE_DIRS ${CMAKE_CURRENT_SOURCE_DIR}/woff2/brotli/c/include)
set(BROTLIDEC_LIBRARIES brotlidec-static)
set(BROTLIENC_LIBRARIES brotlienc-static)
add_subdirectory(woff2 EXCLUDE_FROM_ALL)
# set_property(TARGET woff2common PROPERTY EXCLUDE_FROM_ALL FALSE)
# set_property(TARGET woff2dec PROPERTY EXCLUDE_FROM_ALL FALSE)
# set_property(TARGET woff2enc PROPERTY EXCLUDE_FROM_ALL FALSE)
# set_property(TARGET woff2dec PROPERTY EXCLUDE_FROM_ALL FALSE)
add_library(woff2wrapper STATIC wrapper.cpp)
target_link_libraries(woff2wrapper woff2enc woff2dec woff2common brotlidec-static brotlienc-static brotlicommon-static)
target_include_directories(woff2wrapper PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/woff2/include/)

install(
  TARGETS woff2wrapper woff2enc woff2dec woff2common brotlidec-static brotlienc-static brotlicommon-static
  ARCHIVE DESTINATION "${CMAKE_INSTALL_LIBDIR}"
  LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}"
  RUNTIME DESTINATION "${CMAKE_INSTALL_BINDIR}"
)