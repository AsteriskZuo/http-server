#pragma once
#ifndef LIBRARY_BRIDGE_HPP
#define LIBRARY_BRIDGE_HPP
// #ifdef __cplusplus
// extern "C"
// {
// #endif
int init(const char* path);
int findPath(const unsigned char *condition, unsigned int conditionSize, unsigned char **result, unsigned int *size, char **id, unsigned int *id_size, unsigned int format);
// int findGuide(const char *condition, unsigned char **result, unsigned int *size);
// #ifdef __cplusplus
// } // extern "C"
// #endif

#endif //LIBRARY_BRIDGE_HPP