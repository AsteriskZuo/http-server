#pragma once
#ifdef __cplusplus
using namespace std;
extern "C"
{
#endif
  int Init(const char *&strPath);
  int StartSearchForServer(const unsigned char *param, unsigned int paramLength, unsigned char *&routeBinData, unsigned int &lengthInBytes, char *&routeID, unsigned int &routeIDLen);
#ifdef __cplusplus
} // extern "C"
#endif