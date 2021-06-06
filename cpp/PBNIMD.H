//**************************************************************************
//
//                            Copyright 2001
//                              Sybase Inc.
//
//								Copyright ?2002
//						Sybase, Inc. and its subsidiaries.
//	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-
//	Sybase, Inc.("Sybase") claims copyright in this program and documentation
//	as an unpublished work, versions of which were first licensed on the date
//	indicated in the foregoing notice. This claim of copyright does not imply
//	waiver of Sybase's other rights.
//	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-
//
//    Filename :	psppmd.h
//
//    Author   :	PB kernel team
//
//    Purpose  : 	machine-dependent typedefs to be used in pbni.h
//
//****************************************************************************

#ifndef PBPSPPMD_H
#define PBPSPPMD_H

#include "windows.h"
#include <tchar.h>

//****************************************************************************
//	Basic definitions and typedefs
//****************************************************************************
#define PBXEXPORT __declspec(dllexport)
#define PBXCALL __stdcall

typedef __int64			pblonglong;

#endif
