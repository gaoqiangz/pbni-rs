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
// ------------------------------------------------------------------------
//
//    Filename :        pbarray.h
//
//    Author   :        PB kernel team
//
//    Purpose  :        Definition of PBArrayAccessor
//
//***************************************************************************

#ifndef _PBARRAY_H_
#define _PBARRAY_H_

#include "pbtraits.h"

//****************************************************************************
//	PBUnboundedArrayCreator template
//
//	PBUnboundedArrayCreator is for creating unbounded array of standard data
//	types.
//****************************************************************************
template <pbvalue_type T>
class PBUnboundedArrayCreator
{
	typedef typename PBNI_NAMESPACE::PBValueTraits<T>::ValueType ValueType;

	IPB_Session*	d_session;
	pbarray			d_array;

public:
	explicit PBUnboundedArrayCreator(IPB_Session* session, pblong initialSize=0);
	~PBUnboundedArrayCreator();

	void	SetAt(pblong pos, ValueType v);
	pbarray	GetArray();

private:	// prevention
	PBUnboundedArrayCreator(const PBUnboundedArrayCreator&);
	PBUnboundedArrayCreator operator=(const PBUnboundedArrayCreator&);
};

//***************************************************************************
//	template <typename T> PBUnboundedArrayCreator
//***************************************************************************
template <pbvalue_type T>
PBUnboundedArrayCreator<T>::PBUnboundedArrayCreator
(
	IPB_Session* session,
	pblong initialSize
)
:	d_session(session),
	d_array(0)
{
	d_array = session->NewUnboundedSimpleArray(PBValueTraits<T>::GetValueType());
	if (initialSize > 0)
	{
	}
}

template <pbvalue_type T>
PBUnboundedArrayCreator<T>::~PBUnboundedArrayCreator()
{
}

template <pbvalue_type T>
void	PBUnboundedArrayCreator<T>::SetAt(pblong pos, ValueType v)
{
	pblong dim[1];
	dim[0] = pos;
	PBValueTraits<T>::SetAt(d_session, d_array, &dim[0], v);
}

template <pbvalue_type T>
pbarray	PBUnboundedArrayCreator<T>::GetArray()
{
	return d_array;
}


//****************************************************************************
//	PBUnboundedArrayCreator<pbstring>
//
//	PBUnboundedArrayCreator is for creating unbounded array of pbstring
//	types.
//****************************************************************************
template <>
class PBUnboundedArrayCreator<pbvalue_string>
{
	IPB_Session*	d_session;
	pbarray			d_array;

public:
	explicit PBUnboundedArrayCreator(IPB_Session* session, pblong initialSize=0);
	~PBUnboundedArrayCreator();

	void	SetAt(pblong pos, pbstring v);
	void	SetAt(pblong pos, LPCTSTR v);
	pbarray	GetArray();

private:	// prevention
	PBUnboundedArrayCreator(const PBUnboundedArrayCreator&);
	PBUnboundedArrayCreator operator=(const PBUnboundedArrayCreator&);
};

typedef PBUnboundedArrayCreator<pbvalue_int>		PBUnboundedIntArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_long>		PBUnboundedLongArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_real>		PBUnboundedRealArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_double>		PBUnboundedDoubleArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_dec>		PBUnboundedDecArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_string>		PBUnboundedStringArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_boolean>	PBUnboundedBooleanArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_uint>		PBUnboundedUintArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_ulong>		PBUnboundedUlongArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_blob>		PBUnboundedBlobArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_date>		PBUnboundedDateArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_time>		PBUnboundedTimeArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_datetime>	PBUnboundedDateTimeArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_char>		PBUnboundedCharArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_longlong>	PBUnboundedLongLongArrayCreator;
typedef PBUnboundedArrayCreator<pbvalue_byte>	       PBUnboundedByteArrayCreator;


//****************************************************************************
//	PBUnboundedObjectArrayCreator
//
//	PBUnboundedObjectArrayCreator is for creating unbounded array of pbobjects
//	types.
//****************************************************************************
class PBUnboundedObjectArrayCreator
{
	IPB_Session*	d_session;
	pbarray			d_array;
	pbclass			d_class;

public:
	explicit PBUnboundedObjectArrayCreator
		(
		IPB_Session* session,
		pbclass cls,
		pblong initialSize=0
		);
	~PBUnboundedObjectArrayCreator();

	void	SetAt(pblong pos, pbobject obj);
	pbarray	GetArray();

private:	// prevention
	PBUnboundedObjectArrayCreator(const PBUnboundedObjectArrayCreator&);
	PBUnboundedObjectArrayCreator operator=(const PBUnboundedObjectArrayCreator&);
};

//****************************************************************************
//	PBBoundedArrayCreator template
//
//	PBBoundedArrayCreator is for creating bounded array of standard data
//	types.
//****************************************************************************
template <pbvalue_type T>
class PBBoundedArrayCreator
{
	typedef typename PBNI_NAMESPACE::PBValueTraits<T>::ValueType ValueType;

	IPB_Session*	d_session;
	pbarray			d_array;

public:
	explicit PBBoundedArrayCreator
		(
		IPB_Session* session,
		pbuint dimensions,
		PBArrayInfo::ArrayBound* bounds
		);

	~PBBoundedArrayCreator();

	void	SetAt(pblong dim[], ValueType v);
	pbarray	GetArray();

private:	// prevention
	PBBoundedArrayCreator(const PBBoundedArrayCreator&);
	PBBoundedArrayCreator operator=(const PBBoundedArrayCreator&);
};

template <pbvalue_type T>
PBBoundedArrayCreator<T>::PBBoundedArrayCreator
(
	IPB_Session* session,
	pbuint dimensions,
	PBArrayInfo::ArrayBound* bounds
)
:	d_session(session),
	d_array(0)
{
	d_array = session->NewBoundedSimpleArray(PBValueTraits<T>::GetValueType(),
		dimensions, bounds);
}

template <pbvalue_type T>
PBBoundedArrayCreator<T>::~PBBoundedArrayCreator()
{
}

template <pbvalue_type T>
void	PBBoundedArrayCreator<T>::SetAt(pblong dim[], ValueType v)
{
	PBValueTraits<T>::SetAt(d_session, d_array, dim, v);
}

template <pbvalue_type T>
pbarray	PBBoundedArrayCreator<T>::GetArray()
{
	return d_array;
}

//****************************************************************************
//	PBBoundedArrayCreator<pbstring>
//
//	PBBoundedArrayCreator is for creating bounded array of pbstring
//	types.
//****************************************************************************
template <>
class PBBoundedArrayCreator<pbvalue_string>
{
	IPB_Session*	d_session;
	pbarray			d_array;

public:
	explicit PBBoundedArrayCreator
		(
		IPB_Session* session,
		pbuint dimensions,
		PBArrayInfo::ArrayBound* bounds
		);

	~PBBoundedArrayCreator();

	void	SetAt(pblong dim[], pbstring v);
	void	SetAt(pblong dim[], LPCTSTR v);
	pbarray	GetArray();

private:	// prevention
	PBBoundedArrayCreator(const PBBoundedArrayCreator&);
	PBBoundedArrayCreator operator=(const PBBoundedArrayCreator&);
};

typedef PBBoundedArrayCreator<pbvalue_int>		PBBoundedIntArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_long>		PBBoundedLongArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_real>		PBBoundedRealArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_double>	PBBoundedDoubleArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_dec>		PBBoundedDecArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_string>	PBBoundedStringArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_boolean>	PBBoundedBooleanArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_uint>		PBBoundedUintArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_ulong>	PBBoundedUlongArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_blob>		PBBoundedBlobArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_date>		PBBoundedDateArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_time>		PBBoundedTimeArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_datetime>	PBBoundedDateTimeArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_char>		PBBoundedCharArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_longlong>	PBBoundedLongLongArrayCreator;
typedef PBBoundedArrayCreator<pbvalue_byte>	       PBBoundedByteArrayCreator;

//****************************************************************************
//	PBBoundedObjectArrayCreator
//
//	PBBoundedObjectArrayCreator is for creating bounded array of pbobjects
//	types.
//****************************************************************************
class PBBoundedObjectArrayCreator
{
	IPB_Session*	d_session;
	pbarray			d_array;
	pbclass			d_class;

public:
	explicit PBBoundedObjectArrayCreator
		(
		IPB_Session* session,
		pbclass cls,
		pbuint dimensions,
		PBArrayInfo::ArrayBound* bounds
		);

	~PBBoundedObjectArrayCreator();

	void	SetAt(pblong dim[], pbobject obj);
	pbarray	GetArray();

private:	// prevention
	PBBoundedObjectArrayCreator(const PBBoundedObjectArrayCreator&);
	PBBoundedObjectArrayCreator operator=(const PBBoundedObjectArrayCreator&);
};

//****************************************************************************
//	template <pbvalue_type T> PBArrayAccessor
//
//	PBArrayAccessor template is for accessing the items of an array
//****************************************************************************
template <pbvalue_type T>
class PBArrayAccessor
{
	IPB_Session*	d_session;
	pbarray			d_array;

public:
	typedef typename PBNI_NAMESPACE::PBValueTraits<T>::ValueType ValueType;

	explicit PBArrayAccessor
		(
		IPB_Session* session,
		pbarray	array
		);

	~PBArrayAccessor();

	void	SetAt(pblong dim[], ValueType v);
	ValueType	GetAt(pblong dim[], pbboolean& isNull);

	pbboolean	IsNull(pblong dim[]);
	void	SetToNull(pblong dim[]);


private:	// prevention
	PBArrayAccessor(const PBArrayAccessor&);
	PBArrayAccessor operator=(const PBArrayAccessor&);
};

template <pbvalue_type T>
PBArrayAccessor<T>::PBArrayAccessor
(
	IPB_Session* session,
	pbarray array
)
:	d_session(session),
	d_array(array)
{
}

template <pbvalue_type T>
PBArrayAccessor<T>::~PBArrayAccessor()
{
}

template <pbvalue_type T>
void	PBArrayAccessor<T>::SetAt(pblong dim[], ValueType v)
{
	PBValueTraits<T>::SetAt(d_session, d_array, dim, v);
}

template <pbvalue_type T>
typename PBArrayAccessor<T>::ValueType	PBArrayAccessor<T>::GetAt(pblong dim[], pbboolean& isNull)
{
	return PBValueTraits<T>::GetAt(d_session, d_array, dim, isNull);
}

template <pbvalue_type T>
pbboolean	PBArrayAccessor<T>::IsNull(pblong dim[])
{
	return d_session->IsArrayItemNull(d_array, dim);
}

template <pbvalue_type T>
void	PBArrayAccessor<T>::SetToNull(pblong dim[])
{
	d_session->SetArrayItemToNull(d_array, dim);
}


//****************************************************************************
//	template <> PBArrayAccessor<pbvalue_string>
//
//	PBArrayAccessor template is for accessing the items of a string array
//****************************************************************************
template <>
class PBArrayAccessor<pbvalue_string>
{
	IPB_Session*	d_session;
	pbarray			d_array;

public:
	explicit PBArrayAccessor
		(
		IPB_Session* session,
		pbarray	array
		);

	~PBArrayAccessor();

	void	SetAt(pblong dim[], LPCTSTR str);
	void	SetAt(pblong dim[], pbstring str);
	pbstring	GetAt(pblong dim[], pbboolean& isNull);

private:	// prevention
	PBArrayAccessor(const PBArrayAccessor&);
	PBArrayAccessor operator=(const PBArrayAccessor&);
};

typedef PBArrayAccessor<pbvalue_int>		PBIntArrayAccessor;
typedef PBArrayAccessor<pbvalue_long>		PBLongArrayAccessor;
typedef PBArrayAccessor<pbvalue_real>		PBRealArrayAccessor;
typedef PBArrayAccessor<pbvalue_double>		PBDoubleArrayAccessor;
typedef PBArrayAccessor<pbvalue_dec>		PBDecArrayAccessor;
typedef PBArrayAccessor<pbvalue_string>		PBStringArrayAccessor;
typedef PBArrayAccessor<pbvalue_boolean>	PBBooleanArrayAccessor;
typedef PBArrayAccessor<pbvalue_uint>		PBUintArrayAccessor;
typedef PBArrayAccessor<pbvalue_ulong>		PBUlongArrayAccessor;
typedef PBArrayAccessor<pbvalue_blob>		PBBlobArrayAccessor;
typedef PBArrayAccessor<pbvalue_date>		PBDateArrayAccessor;
typedef PBArrayAccessor<pbvalue_time>		PBTimeArrayAccessor;
typedef PBArrayAccessor<pbvalue_datetime>	PBDateTimeArrayAccessor;
typedef PBArrayAccessor<pbvalue_char>		PBCharArrayAccessor;
typedef PBArrayAccessor<pbvalue_longlong>	PBLongLongArrayAccessor;
typedef PBArrayAccessor<pbvalue_byte>	       PBByteArrayAccessor;

//****************************************************************************
//	PBObjectArrayAccessor
//
//	PBObjectArrayAccessor is for accessing the elements of an object array
//****************************************************************************
class PBObjectArrayAccessor
{
	IPB_Session*	d_session;
	pbarray			d_array;

public:
	explicit PBObjectArrayAccessor
		(
		IPB_Session* session,
		pbarray		array
		);

	~PBObjectArrayAccessor();

	void	SetAt(pblong dim[], pbobject obj);
	pbobject	GetAt(pblong dim[], pbboolean& isNull);

private:	// prevention
	PBObjectArrayAccessor(const PBObjectArrayAccessor&);
	PBObjectArrayAccessor operator=(const PBObjectArrayAccessor&);
};

#endif
