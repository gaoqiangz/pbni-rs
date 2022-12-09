#define UNICODE
#define _UNICODE
#define WIN32
#define _WINDOWS
#define _CRT_SECURE_NO_WARNINGS
#define WIN32_LEAN_AND_MEAN

#define NOATOM
#define NOGDICAPMASKS
#define NOMETAFILE
#define NOMINMAX
#define NOOPENFILE
#define NORASTEROPS
#define NOSCROLL
#define NOSOUND
#define NOSYSMETRICS
#define NOTEXTMETRIC
#define NOWH
#define NOCOMM
#define NOKANJI
#define NOCRYPT
#define NOMCX

/**********************************************************************
system library by 781770213@qq.com
大自在 2021/6
**********************************************************************/

#ifndef POINTER_64
#define POINTER_64 __ptr64 // 必须
#endif

#include <windows.h>
#include <comdef.h>

#include "EN32T.h"