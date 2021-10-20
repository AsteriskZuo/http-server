

#include "routing_server.h"
#include "library-bridge.hpp"

#include <iostream>
#include <stdlib.h>
#include <cstring>
// using namespace std;
int init(const char *path)
{
  std::cout << "[c++ bridge] init " << path << std::endl;
  return Init(path);
}

int findPath(const unsigned char *param, unsigned int paramLength, unsigned char **result, unsigned int *size, char **id, unsigned int *id_size, unsigned int format)
{
  std::cout << "[c++ bridge] findPath in: " << paramLength << std::endl;
  // string scondition = condition;
  char *sid = NULL;
  int ret = StartSearchForServer(param, paramLength, *result, *size, sid, *id_size, format);

  *id = (char *)malloc(sizeof(char) * ((*id_size) + 1));
  memset(*id, 0, (*id_size) + 1);
  memcpy(*id, sid, *id_size);

  free(sid);
  std::cout << "[c++ bridge] findPath out: " << ret << ":" << *size << ":" << *id << ":" << *id_size << std::endl;

  return ret;
}

// int findGuide(const char *condition, unsigned char **result, unsigned int *size)
// {
//   std::cout << "[c++ bridge] findGuide in: " << condition << std::endl;
//   string scondition = condition;
//   char *sid = NULL;
//   int ret = GenerateGuideEventForServer(condition, *result, *size);

//   std::cout << "[c++ bridge] findGuide out: " << ret << ":" << *result << ":" << *size << std::endl;

//   return ret;
// }
