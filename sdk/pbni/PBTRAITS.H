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
//    Filename :	pbtraits.h
//
//    Author   :	PB kernel team
//
//    Purpose  : 	Definition of PBValueTraits
//
//****************************************************************************

#ifndef _PBTRAITS_H_
#define _PBTRAITS_H_

//	Currently it is only to make PB.NET compiler compile in .NET framework 2.0,
//  in the future, We should take a more appropriate namespace for it
#define PBNI_NAMESPACE 

//****************************************************************************
//	PBValueTraits
//****************************************************************************
template <pbvalue_type v> struct PBValueTraits
{
	enum { kValueType = v };
};

#if !defined(_AIX) && !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_int>
#else
template <> struct PBValueTraits<pbvalue_int>
#endif
{
	typedef pbint ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_int;
	}

	static void SetValue(IPB_Value* value, pbint v)
	{
		value->SetInt(v);
	}

	static pbint GetValue(IPB_Value* value)
	{
		return value->GetInt();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbint v)
	{
		session->SetIntField(obj, fid, v);
	}

	static pbint GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetIntField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbint value)
	{
		session->SetIntArrayItem(array, dim, value);
	}

	static pbint GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetIntArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbint v)
	{
		session->SetIntGlobalVar(fid, v);
	}

	static pbint GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetIntGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbint v)
	{
		session->SetIntSharedVar(group, fid, v);
	}

	static pbint GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetIntSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_uint>
#else
template <> struct PBValueTraits<pbvalue_uint>
#endif
{
	typedef pbuint ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_uint;
	}

	static void SetValue(IPB_Value* value, pbuint v)
	{
		value->SetUint(v);
	}

	static pbuint GetValue(IPB_Value* value)
	{
		return value->GetUint();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbuint v)
	{
		session->SetUintField(obj, fid, v);
	}

	static pbuint GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetUintField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbuint value)
	{
		session->SetUintArrayItem(array, dim, value);
	}

	static pbuint GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetUintArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbuint v)
	{
		session->SetUintGlobalVar(fid, v);
	}

	static pbuint GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetUintGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbuint v)
	{
		session->SetUintSharedVar(group, fid, v);
	}

	static pbuint GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetUintSharedVar(group, fid, isNull);
	}
};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_long>
#else
template <> struct PBValueTraits<pbvalue_long>
#endif
{
	typedef pblong ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_long;
	}

	static void SetValue(IPB_Value* value, pblong v)
	{
		value->SetLong(v);
	}

	static pblong GetValue(IPB_Value* value)
	{
		return value->GetLong();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pblong v)
	{
		session->SetLongField(obj, fid, v);
	}

	static pblong GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetLongField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pblong value)
	{
		session->SetLongArrayItem(array, dim, value);
	}

	static pblong GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetLongArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pblong v)
	{
		session->SetLongGlobalVar(fid, v);
	}

	static pblong GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetLongGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pblong v)
	{
		session->SetLongSharedVar(group, fid, v);
	}

	static pblong GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetLongSharedVar(group, fid, isNull);
	}
};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_ulong>
#else
template <> struct PBValueTraits<pbvalue_ulong>
#endif
{
	typedef pbulong ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_ulong;
	}

	static void SetValue(IPB_Value* value, pbulong v)
	{
		value->SetUlong(v);
	}

	static pbulong GetValue(IPB_Value* value)
	{
		return value->GetUlong();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbulong v)
	{
		session->SetUlongField(obj, fid, v);
	}

	static pbulong GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetUlongField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbulong value)
	{
		session->SetUlongArrayItem(array, dim, value);
	}

	static pbulong GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetUlongArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbulong v)
	{
		session->SetUlongGlobalVar(fid, v);
	}

	static pbulong GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetUlongGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbulong v)
	{
		session->SetUlongSharedVar(group, fid, v);
	}

	static pbulong GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetUlongSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_real>
#else
template <> struct PBValueTraits<pbvalue_real>
#endif
{
	typedef pbreal ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_real;
	}

	static void SetValue(IPB_Value* value, pbreal v)
	{
		value->SetReal(v);
	}

	static pbreal GetValue(IPB_Value* value)
	{
		return value->GetReal();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbreal v)
	{
		session->SetRealField(obj, fid, v);
	}

	static pbreal GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetRealField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbreal value)
	{
		session->SetRealArrayItem(array, dim, value);
	}

	static pbreal GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetRealArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbreal v)
	{
		session->SetRealGlobalVar(fid, v);
	}

	static pbreal GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetRealGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbreal v)
	{
		session->SetRealSharedVar(group, fid, v);
	}

	static pbreal GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetRealSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_double>
#else
template <> struct PBValueTraits<pbvalue_double>
#endif
{
	typedef pbdouble ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_double;
	}

	static void SetValue(IPB_Value* value, pbdouble v)
	{
		value->SetDouble(v);
	}

	static pbdouble GetValue(IPB_Value* value)
	{
		return value->GetDouble();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbdouble v)
	{
		session->SetDoubleField(obj, fid, v);
	}

	static pbdouble GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDoubleField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbdouble value)
	{
		session->SetDoubleArrayItem(array, dim, value);
	}

	static pbdouble GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetDoubleArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbdouble v)
	{
		session->SetDoubleGlobalVar(fid, v);
	}

	static pbdouble GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDoubleGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbdouble v)
	{
		session->SetDoubleSharedVar(group, fid, v);
	}

	static pbdouble GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDoubleSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_dec>
#else
template <> struct PBValueTraits<pbvalue_dec>
#endif
{
	typedef pbdec ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_dec;
	}

	static void SetValue(IPB_Value* value, pbdec v)
	{
		value->SetDecimal(v);
	}

	static pbdec GetValue(IPB_Value* value)
	{
		return value->GetDecimal();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbdec v)
	{
		session->SetDecField(obj, fid, v);
	}

	static pbdec GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDecField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbdec value)
	{
		session->SetDecArrayItem(array, dim, value);
	}

	static pbdec GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetDecArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbdec v)
	{
		session->SetDecGlobalVar(fid, v);
	}

	static pbdec GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDecGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbdec v)
	{
		session->SetDecSharedVar(group, fid, v);
	}

	static pbdec GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDecSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_string>
#else
template <> struct PBValueTraits<pbvalue_string>
#endif
{
	typedef pbstring ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_string;
	}

	static void SetValue(IPB_Value* value, pbstring v)
	{
		value->SetPBString(v);
	}

	static pbstring GetValue(IPB_Value* value)
	{
		return value->GetString();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbstring v)
	{
		session->SetPBStringField(obj, fid, v);
	}

	static pbstring GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetStringField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbstring value)
	{
		session->SetPBStringArrayItem(array, dim, value);
	}

	static pbstring GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetStringArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbstring v)
	{
		session->SetPBStringGlobalVar(fid, v);
	}

	static pbstring GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetStringGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbstring v)
	{
		session->SetPBStringSharedVar(group, fid, v);
	}

	static pbstring GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetStringSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_blob>
#else
template <> struct PBValueTraits<pbvalue_blob>
#endif
{
	typedef pbblob ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_blob;
	}

	static void SetValue(IPB_Value* value, pbblob v)
	{
		value->SetBlob(v);
	}

	static pbblob GetValue(IPB_Value* value)
	{
		return value->GetBlob();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbblob v)
	{
		session->SetBlobField(obj, fid, v);
	}

	static pbblob GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetBlobField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbblob value)
	{
		session->SetBlobArrayItem(array, dim, value);
	}

	static pbblob GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetBlobArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbblob v)
	{
		session->SetBlobGlobalVar(fid, v);
	}

	static pbblob GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetBlobGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbblob v)
	{
		session->SetBlobSharedVar(group, fid, v);
	}

	static pbblob GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetBlobSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_boolean>
#else
template <> struct PBValueTraits<pbvalue_boolean>
#endif
{
	typedef pbboolean ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_boolean;
	}

	static void SetValue(IPB_Value* value, pbboolean v)
	{
		value->SetBool(v);
	}

	static pbboolean GetValue(IPB_Value* value)
	{
		return value->GetBool();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean v)
	{
		session->SetBoolField(obj, fid, v);
	}

	static pbboolean GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetBoolField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean value)
	{
		session->SetBoolArrayItem(array, dim, value);
	}

	static pbboolean GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetBoolArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean v)
	{
		session->SetBoolGlobalVar(fid, v);
	}

	static pbboolean GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetBoolGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean v)
	{
		session->SetBoolSharedVar(group, fid, v);
	}

	static pbboolean GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetBoolSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_char>
#else
template <> struct PBValueTraits<pbvalue_char>
#endif
{
	typedef pbchar ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_char;
	}

	static void SetValue(IPB_Value* value, pbchar v)
	{
		value->SetChar(v);
	}

	static pbchar GetValue(IPB_Value* value)
	{
		return value->GetChar();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbchar v)
	{
		session->SetCharField(obj, fid, v);
	}

	static pbchar GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetCharField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbchar value)
	{
		session->SetCharArrayItem(array, dim, value);
	}

	static pbchar GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetCharArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbchar v)
	{
		session->SetCharGlobalVar(fid, v);
	}

	static pbchar GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetCharGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbchar v)
	{
		session->SetCharSharedVar(group, fid, v);
	}

	static pbchar GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetCharSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_date>
#else
template <> struct PBValueTraits<pbvalue_date>
#endif
{
	typedef pbdate ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_date;
	}

	static void SetValue(IPB_Value* value, pbdate v)
	{
		value->SetDate(v);
	}

	static pbdate GetValue(IPB_Value* value)
	{
		return value->GetDate();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbdate v)
	{
		session->SetDateField(obj, fid, v);
	}

	static pbdate GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDateField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbdate value)
	{
		session->SetDateArrayItem(array, dim, value);
	}

	static pbdate GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetDateArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbdate v)
	{
		session->SetDateGlobalVar(fid, v);
	}

	static pbdate GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDateGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbdate v)
	{
		session->SetDateSharedVar(group, fid, v);
	}

	static pbdate GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDateSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_time>
#else
template <> struct PBValueTraits<pbvalue_time>
#endif
{
	typedef pbtime ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_time;
	}

	static void SetValue(IPB_Value* value, pbtime v)
	{
		value->SetTime(v);
	}

	static pbtime GetValue(IPB_Value* value)
	{
		return value->GetTime();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbtime v)
	{
		session->SetTimeField(obj, fid, v);
	}

	static pbtime GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetTimeField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbtime value)
	{
		session->SetTimeArrayItem(array, dim, value);
	}

	static pbtime GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetTimeArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbtime v)
	{
		session->SetTimeGlobalVar(fid, v);
	}

	static pbtime GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetTimeGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbtime v)
	{
		session->SetTimeSharedVar(group, fid, v);
	}

	static pbtime GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetTimeSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_datetime>
#else
template <> struct PBValueTraits<pbvalue_datetime>
#endif
{
	typedef pbdatetime ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_datetime;
	}

	static void SetValue(IPB_Value* value, pbdatetime v)
	{
		value->SetDateTime(v);
	}

	static pbdatetime GetValue(IPB_Value* value)
	{
		return value->GetDateTime();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbdatetime v)
	{
		session->SetDateTimeField(obj, fid, v);
	}

	static pbdatetime GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDateTimeField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbdatetime value)
	{
		session->SetDateTimeArrayItem(array, dim, value);
	}

	static pbdatetime GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetDateTimeArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbdatetime v)
	{
		session->SetDateTimeGlobalVar(fid, v);
	}

	static pbdatetime GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDateTimeGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbdatetime v)
	{
		session->SetDateTimeSharedVar(group, fid, v);
	}

	static pbdatetime GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetDateTimeSharedVar(group, fid, isNull);
	}

};

#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_longlong>
#else
template <> struct PBValueTraits<pbvalue_longlong>
#endif
{
	typedef pblonglong ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_longlong;
	}

	static void SetValue(IPB_Value* value, pblonglong v)
	{
		value->SetLongLong(v);
	}

	static pblonglong GetValue(IPB_Value* value)
	{
		return value->GetLongLong();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pblonglong v)
	{
		session->SetLongLongField(obj, fid, v);
	}

	static pblonglong GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetLongLongField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pblonglong value)
	{
		session->SetLongLongArrayItem(array, dim, value);
	}

	static pblonglong GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetLongLongArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pblonglong v)
	{
		session->SetLongLongGlobalVar(fid, v);
	}

	static pblonglong GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetLongLongGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pblonglong v)
	{
		session->SetLongLongSharedVar(group, fid, v);
	}

	static pblonglong GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetLongLongSharedVar(group, fid, isNull);
	}

};

//**********************************************************
// Begin of the new type BYTE
//**********************************************************
#if !defined(_AIX)&& !defined(PBOS_LINUX)
template <> struct PBNI_NAMESPACE::PBValueTraits<pbvalue_byte>
#else
template <> struct PBValueTraits<pbvalue_byte>
#endif
{
	typedef pbbyte ValueType;

	static pbvalue_type GetValueType()
	{
		return pbvalue_byte;
	}

	static void SetValue(IPB_Value* value, pbbyte v)
	{
		value->SetByte(v);
	}

	static pbuint GetValue(IPB_Value* value)
	{
		return value->GetByte();
	}

	static void SetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbbyte v)
	{
		session->SetByteField(obj, fid, v);
	}

	static pbuint GetField(IPB_Session* session, pbobject obj, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetByteField(obj, fid, isNull);
	}

	static void SetAt(IPB_Session* session, pbarray array, pblong dim[], pbbyte value)
	{
		session->SetByteArrayItem(array, dim, value);
	}

	static pbuint GetAt(IPB_Session* session, pbarray array, pblong dim[], pbboolean& isNull)
	{
		return session->GetByteArrayItem(array, dim, isNull);
	}

	static void SetGlobalVar(IPB_Session* session, pbfieldID fid, pbbyte v)
	{
		session->SetByteGlobalVar(fid, v);
	}

	static pbuint GetGlobalVar(IPB_Session* session, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetByteGlobalVar(fid, isNull);
	}

	static void SetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbbyte v)
	{
		session->SetByteSharedVar(group, fid, v);
	}

	static pbuint GetSharedVar(IPB_Session* session, pbgroup group, pbfieldID fid, pbboolean& isNull)
	{
		return session->GetByteSharedVar(group, fid, isNull);
	}
};
//**********************************************************
// End of the new type BYTE
//**********************************************************

#endif
