//**************************************************************************
//
//                            Copyright 2004
//                              Sybase Inc.
//
//								Copyright ?2004
//						Sybase, Inc. and its subsidiaries.
//	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-
//	Sybase, Inc.("Sybase") claims copyright in this program and documentation
//	as an unpublished work, versions of which were first licensed on the date
//	indicated in the foregoing notice. This claim of copyright does not imply
//	waiver of Sybase's other rights.
//	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-
//
//    Filename :	PBRSA.h
//
//    Author   :	PB Kernel team
//
//    Purpose  : 	structures and interfaces for PowerBuilder ResultSet interface
//
//****************************************************************************

#ifndef __PBRSA_H__
#define __PBRSA_H__

#ifdef PBOS_UNIX
#include "oswindow.h"
#endif
#include "pbni.h"

struct PB_DateData
{
	short year;
	short month;
	short day;
	short filler;  //just for structure alignment
};

struct PB_TimeData
{
	short hour;
	short minute;
	short second;
	short filler;	//just for structure alignment
	unsigned long microsecond;
};

struct PB_DatetimeData
{
	PB_DateData date;
	PB_TimeData time;
};

struct IPB_RSItemData
{
	virtual void SetNull() = 0;
	virtual void SetData(unsigned long len, void* data) = 0;
};


struct IPB_ResultSetAccessor
{
	virtual void AddRef() = 0;
	virtual void Release() = 0;

	virtual unsigned long	GetRowCount() = 0;
	virtual unsigned long	GetColumnCount() = 0;
	virtual void	GetColumnMetaData
							(	unsigned long columnNum,
								LPTSTR columnName,								
								pbvalue_type* type,
								unsigned long* width
							) = 0;

	virtual bool GetItemData(unsigned long row, unsigned long col, IPB_RSItemData* data) = 0;
};


#endif  // __IPB_RESULTSET_ACCESSOR_H__
