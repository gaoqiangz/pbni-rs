//**************************************************************************
//
//                            Copyright 2002
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
//    Filename :	pbfield.h
//
//    Author   :	PB kernel team
//
//    Purpose  : 	Declaring PBFieldAccessor template class
//
//****************************************************************************

#ifndef _PBFIELD_H_
#define _PBFIELD_H_

#include "pbtraits.h"

template <pbvalue_type T>
class PBFieldAccessor
{
	IPB_Session*	d_session;
	pbobject		d_object;
	pbfieldID		d_fid;

public:
	PBFieldAccessor(IPB_Session* session, pbobject obj, pbfieldID fid)
	:	d_session(session), d_object(obj), d_fid(fid)
	{
	}

	~PBFieldAccessor()
	{
	}

	void SetField(typename PBNI_NAMESPACE::PBValueTraits<T>::ValueType v, pbboolean isNull)
	{
		PBValueTraits<T>::SetField(d_session, d_object, d_fid, v, isNull);
	}

	typename PBNI_NAMESPACE::PBValueTraits<T>::ValueType GetField(pbboolean& isNull)
	{
		return PBValueTraits<T>::GetField(d_session, d_object, d_fid, isNull);
	}
};

#endif

