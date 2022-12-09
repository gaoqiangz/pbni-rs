//**************************************************************************
//
//                            Copyright 2002
//                              Sybase Inc.
//
//                              Copyright ?2002
//                      Sybase, Inc. and its subsidiaries.
//  -   -   -   -   -   -   -   -   -   -   -   -   -   -   -   -   -   -   -
//  Sybase, Inc.("Sybase") claims copyright in this program and documentation
//  as an unpublished work, versions of which were first licensed on the date
//  indicated in the foregoing notice. This claim of copyright does not imply
//  waiver of Sybase's other rights.
//  -   -   -   -   -   -   -   -   -   -   -   -   -   -   -   -   -   -   -
//
//    Filename :    pbni.h
//
//    Author   :    PB kernel team
//
//    Purpose  :    structures and interfaces for PowerBuilder Native Interface
//
//****************************************************************************

#ifndef _PBNI_H_
#define _PBNI_H_

#include "pbnimd.h"

//****************************************************************************
//  Basic definitions and typedefs
//****************************************************************************
#ifdef PBOS_UNIX
typedef const wchar_t *LPCTSTR;
typedef const char *LPCSTR;
#ifdef RemoveProp
#undef RemoveProp
#endif
#ifdef GetProp
#undef GetProp
#endif
#ifdef SetProp
#undef SetProp
#endif
#ifdef GetClassName
#undef GetClassName
#endif
#endif

typedef short pbint;
typedef unsigned short pbuint;
typedef long pblong;
typedef unsigned long pbulong;
typedef unsigned char pbbyte;
typedef short pbboolean;
typedef pbuint pbchar;
typedef float pbreal;
typedef double pbdouble;

struct _pbobject
{
};
typedef struct _pbobject *pbobject;

struct _pbproxyObject : public _pbobject
{
};
typedef _pbproxyObject *pbproxyObject;

struct _pbclass
{
};
typedef struct _pbclass *pbclass;

struct _pbgroup
{
};
typedef struct _pbgroup *pbgroup;

struct _pbstring
{
};
typedef struct _pbstring *pbstring;

struct _pbarray
{
};
typedef struct _pbarray *pbarray;

struct _pbdec
{
};
typedef struct _pbdec *pbdec;

struct _pbdate
{
};
typedef struct _pbdate *pbdate;

struct _pbtime
{
};
typedef struct _pbtime *pbtime;

struct _pbdatetime
{
};
typedef struct _pbdatetime *pbdatetime;

struct _pbblob
{
};
typedef struct _pbblob *pbblob;

typedef pbuint pbmethodID;

typedef pbuint pbfieldID;

#define TRUE 1
#define FALSE 0

typedef long PBXRESULT;

//****************************************************************************
//  error codes
//****************************************************************************
#define PBX_OK 0
#define PBX_SUCCESS 0
#define PBX_FAIL -1

#define PBX_E_NO_REGISTER_FUNCTION -1
#define PBX_E_REGISTRATION_FAILED -2
#define PBX_E_BUILD_GROUP_FAILED -3
#define PBX_E_INVALID_ARGUMENT -4
#define PBX_E_INVOKE_METHOD_INACCESSABLE -5
#define PBX_E_INVOKE_WRONG_NUM_ARGS -6
#define PBX_E_INVOKE_REFARG_ERROR -7
#define PBX_E_INVOKE_METHOD_AMBIGUOUS -8
#define PBX_E_INVOKE_FAILURE -9
#define PBX_E_MISMATCHED_DATA_TYPE -10
#define PBX_E_OUTOF_MEMORY -11
#define PBX_E_GET_PBVM_FAILED -12
#define PBX_E_NO_SUCH_CLASS -13
#define PBX_E_CAN_NOT_LOCATE_APPLICATION -14
#define PBX_E_INVALID_METHOD_ID -15
#define PBX_E_READONLY_ARGS -16
#define PBX_E_ARRAY_INDEX_OUTOF_BOUNDS -100

//****************************************************************************
//  constants
//****************************************************************************
static const pbuint kUndefinedMethodID = 0xffff;
static const pbuint kUndefinedFieldID = 0xffff;

enum pbgroup_type
{
    pbgroup_application,
    pbgroup_datawindow,
    pbgroup_function,
    pbgroup_menu,
    pbgroup_proxy,
    pbgroup_structure,
    pbgroup_userobject,
    pbgroup_window,
    pbgroup_unknown
};

//****************************************************************************
//  Forward declarations
//****************************************************************************
struct IPB_VM;
struct IPB_Session;
struct IPB_SessionA;
struct IPBX_Marshaler;
struct IPBX_UserObject;
struct PBCallInfo;
class PBXTypeMismatchException;

//****************************************************************************
//  PBValue (used for passing arguments between PB and PowerScript++)
//****************************************************************************
enum pbvalue_type
{
    pbvalue_notype = 0,
    pbvalue_int,
    pbvalue_long,
    pbvalue_real,
    pbvalue_double,
    pbvalue_dec,
    pbvalue_string,
    pbvalue_boolean,
    pbvalue_any,
    pbvalue_uint,
    pbvalue_ulong,
    pbvalue_blob,
    pbvalue_date,
    pbvalue_time,
    pbvalue_datetime,
    pbvalue_dummy1,
    pbvalue_dummy2,
    pbvalue_dummy3,
    pbvalue_char,
    pbvalue_dummy4,
    pbvalue_longlong,
    pbvalue_byte
};

//  PBValue interface class
struct IPB_Value
{
    virtual pbclass GetClass() const = 0;
    virtual pbuint GetType() const = 0;
    virtual pbboolean IsArray() const = 0;
    virtual pbboolean IsObject() const = 0;
    virtual pbboolean IsEnum() const = 0;
    virtual pbboolean IsByRef() const = 0;

    virtual pbboolean IsNull() const = 0;
    virtual PBXRESULT SetToNull() = 0;

    virtual pbint GetInt() const = 0;
    virtual pbuint GetUint() const = 0;
    virtual pbboolean GetBool() const = 0;
    virtual pblong GetLong() const = 0;
    virtual pbulong GetUlong() const = 0;
    virtual pbreal GetReal() const = 0;
    virtual pbdouble GetDouble() const = 0;
    virtual pbdec GetDecimal() const = 0;
    virtual pbchar GetChar() const = 0;
    virtual pbstring GetString() const = 0;
    virtual pbobject GetObject() const = 0;
    virtual pbarray GetArray() const = 0;
    virtual pbtime GetTime() const = 0;
    virtual pbdate GetDate() const = 0;
    virtual pbdatetime GetDateTime() const = 0;
    virtual pblonglong GetLongLong() const = 0;
    virtual pbblob GetBlob() const = 0;

    virtual PBXRESULT SetInt(pbint) = 0;
    virtual PBXRESULT SetUint(pbuint) = 0;
    virtual PBXRESULT SetBool(pbboolean) = 0;
    virtual PBXRESULT SetLong(pblong) = 0;
    virtual PBXRESULT SetUlong(pbulong) = 0;
    virtual PBXRESULT SetReal(pbreal) = 0;
    virtual PBXRESULT SetDouble(pbdouble) = 0;
    virtual PBXRESULT SetDecimal(pbdec) = 0;
    virtual PBXRESULT SetChar(pbchar) = 0;
    virtual PBXRESULT SetPBString(pbstring) = 0;
    virtual PBXRESULT SetString(LPCTSTR) = 0;
    virtual PBXRESULT SetArray(pbarray) = 0;
    virtual PBXRESULT SetTime(pbtime) = 0;
    virtual PBXRESULT SetDate(pbdate) = 0;
    virtual PBXRESULT SetDateTime(pbdatetime) = 0;
    virtual PBXRESULT SetLongLong(pblonglong) = 0;
    virtual PBXRESULT SetBlob(pbblob) = 0;
    virtual PBXRESULT SetObject(pbobject) = 0;

    virtual pbboolean IsReadOnly() const = 0;

    virtual pbbyte GetByte() const = 0;
    virtual PBXRESULT SetByte(pbbyte) = 0;

protected:
    ~IPB_Value(){};
};

struct IPB_Arguments
{
    virtual pbint GetCount() = 0;
    virtual IPB_Value *GetAt(pbint) = 0;

protected:
    ~IPB_Arguments(){};
};

//***************************************************************************
//  Array info descriptor
//***************************************************************************
struct PBArrayInfo
{
    struct ArrayBound
    {
        pblong upperBound, lowerBound;
    };

    enum
    {
        BoundedArray,
        UnboundedArray
    } arrayType; // OUT variable, automatically set by GetArrayInfo(), don't set manually

    pbgroup itemGroup;
    pbuint valueType;
    pbuint numDimensions;
    ArrayBound bounds[1];
};

//****************************************************************************
//  PBCallInfo is used for PBNI to call PowerScript functions and vice versa.
//****************************************************************************
struct PBCallInfo
{
    IPB_Arguments *pArgs;
    IPB_Value *returnValue;
    pbclass returnClass;
};

//****************************************************************************
//  PB Routine Types
//****************************************************************************
enum PBRoutineType
{
    PBRT_FUNCTION,
    PBRT_EVENT,
    PBRT_ANY
};

//****************************************************************************
//  IPBX_Logging should be implemented by executables (either DLL or EXE) that
//  uses PBVM, and intents to log the execution status of PBVM.

//  IPB_VM::SetLoggingService() should be called after PBVM is loaded.
//****************************************************************************
struct IPBX_Logging
{
    virtual void Log(LPCTSTR message, pbboolean displayTimeStamp) = 0;

protected:
    ~IPBX_Logging(){};
};

//****************************************************************************
//  Interfaces provided by PB
//****************************************************************************

//****************************************************************************
//  IPB_VM
//****************************************************************************
struct IPB_VM
{
protected:
    ~IPB_VM(){};

public:
    virtual PBXRESULT CreateSessionA(
        LPCSTR applicationName,
        LPCSTR *libraryList,
        pbuint numLibs,
        IPB_SessionA **session) = 0;

    virtual PBXRESULT RunApplicationA(
        LPCSTR applicationName,
        LPCSTR *libraryList,
        pbuint numLibs,
        LPCSTR commandLine,
        IPB_SessionA **session) = 0;

    virtual void SetLoggingService(IPBX_Logging *) = 0;

    virtual PBXRESULT CreateSession(
        LPCTSTR applicationName,
        LPCTSTR *libraryList,
        pbuint numLibs,
        IPB_Session **session) = 0;

    virtual PBXRESULT RunApplication(
        LPCTSTR applicationName,
        LPCTSTR *libraryList,
        pbuint numLibs,
        LPCTSTR commandLine,
        IPB_Session **session) = 0;
};
//****************************************************************************
// Preprocessor for relocating CreateSession to CreateSessionA if Ansi
//****************************************************************************
#if !defined(UNICODE) && !defined(_UNICODE)
#define CreateSession CreateSessionA
#define RunApplication RunApplicationA
#define IPB_Session IPB_SessionA
#endif

struct IPB_ResultSetAccessor;

//****************************************************************************
//  IPB_Session
//****************************************************************************
struct IPB_Session
{
    virtual void Release() = 0;

    virtual pbclass GetClass(pbobject obj) = 0;
    virtual pbgroup GetSystemGroup() = 0;

    virtual pbmethodID GetMethodID(pbclass cls, LPCTSTR methodName,
                                   PBRoutineType rt, LPCTSTR signature, pbboolean publicOnly = true) = 0;
    virtual pbmethodID FindMatchingFunction(pbclass cls, LPCTSTR methodName,
                                            PBRoutineType rt, LPCTSTR readableSignature) = 0;
    virtual pbmethodID GetMethodIDByEventID(pbclass cls, LPCTSTR eventID) = 0;

    virtual PBXRESULT InitCallInfo(pbclass cls, pbmethodID mid, PBCallInfo *ci) = 0;
    virtual void FreeCallInfo(PBCallInfo *ci) = 0;

    virtual void AddLocalRef(pbobject obj) = 0;
    virtual void RemoveLocalRef(pbobject obj) = 0;
    virtual void AddGlobalRef(pbobject obj) = 0;
    virtual void RemoveGlobalRef(pbobject obj) = 0;
    virtual void PushLocalFrame() = 0;
    virtual void PopLocalFrame() = 0;

    // For passing variable arguments.
    virtual PBXRESULT AddIntArgument(PBCallInfo *ci, pbint value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddLongArgument(PBCallInfo *ci, pblong value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddRealArgument(PBCallInfo *ci, pbreal value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddDoubleArgument(PBCallInfo *ci, pbdouble value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddDecArgument(PBCallInfo *ci, pbdec value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddPBStringArgument(PBCallInfo *ci, pbstring value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddStringArgument(PBCallInfo *ci, LPCTSTR value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddBoolArgument(PBCallInfo *ci, pbboolean value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddUintArgument(PBCallInfo *ci, pbuint value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddUlongArgument(PBCallInfo *ci, pbulong value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddBlobArgument(PBCallInfo *ci, pbblob value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddDateArgument(PBCallInfo *ci, pbdate value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddTimeArgument(PBCallInfo *ci, pbtime value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddDateTimeArgument(PBCallInfo *ci, pbdatetime value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddCharArgument(PBCallInfo *ci, pbchar value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddLongLongArgument(PBCallInfo *ci, pblonglong value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddObjectArgument(PBCallInfo *ci, pbobject value, pbboolean isNull = FALSE) = 0;
    virtual PBXRESULT AddArrayArgument(PBCallInfo *ci, pbarray value, pbboolean isNull = FALSE) = 0;

    virtual PBXRESULT InvokeClassFunction(pbclass cls, pbmethodID mid, PBCallInfo *ci) = 0;
    virtual PBXRESULT InvokeObjectFunction(pbobject obj, pbmethodID mid, PBCallInfo *ci) = 0;
    virtual PBXRESULT TriggerEvent(pbobject obj, pbmethodID mid, PBCallInfo *ci) = 0;

    virtual pbboolean HasExceptionThrown() = 0;
    virtual pbobject GetException() = 0;
    virtual void ClearException() = 0;
    virtual void ThrowException(pbobject ex) = 0;

    virtual pbgroup GetCurrGroup() = 0;
    virtual pbgroup FindGroup(LPCTSTR name, pbgroup_type) = 0;
    virtual pbclass FindClass(pbgroup group, LPCTSTR name) = 0;
    virtual pbclass FindClassByClassID(pbgroup group, pbint classID) = 0;
    virtual LPCTSTR GetClassName(pbclass cls) = 0;
    virtual pbclass GetSuperClass(pbclass cls) = 0;
    virtual pbclass GetSystemClass(pbclass cls) = 0;
    virtual pbboolean IsAutoInstantiate(pbclass pbcls) = 0;

    virtual pbobject NewObject(pbclass cls) = 0;

    virtual pbfieldID GetFieldID(pbclass cls, LPCTSTR fieldName) = 0;
    virtual pbuint GetFieldType(pbclass, pbfieldID) = 0;
    virtual pbulong GetNumOfFields(pbclass) = 0;
    virtual LPCTSTR GetFieldName(pbclass, pbfieldID) = 0;

    virtual pbboolean IsFieldNull(pbobject, pbfieldID) = 0;
    virtual void SetFieldToNull(pbobject, pbfieldID) = 0;
    virtual pbboolean IsFieldArray(pbclass, pbfieldID) = 0;
    virtual pbboolean IsFieldObject(pbclass, pbfieldID) = 0;
    virtual PBXRESULT UpdateField(pbobject obj, pbfieldID fid) = 0;

    virtual pbint GetIntField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pblong GetLongField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbreal GetRealField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdouble GetDoubleField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdec GetDecField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbstring GetStringField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbboolean GetBoolField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbuint GetUintField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbulong GetUlongField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbblob GetBlobField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdate GetDateField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbtime GetTimeField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdatetime GetDateTimeField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbchar GetCharField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pblonglong GetLongLongField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbobject GetObjectField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbarray GetArrayField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;

    virtual PBXRESULT SetIntField(pbobject obj, pbfieldID fid, pbint value) = 0;
    virtual PBXRESULT SetLongField(pbobject obj, pbfieldID fid, pblong value) = 0;
    virtual PBXRESULT SetRealField(pbobject obj, pbfieldID fid, pbreal value) = 0;
    virtual PBXRESULT SetDoubleField(pbobject obj, pbfieldID fid, pbdouble value) = 0;
    virtual PBXRESULT SetDecField(pbobject obj, pbfieldID fid, pbdec value) = 0;
    virtual PBXRESULT SetPBStringField(pbobject obj, pbfieldID fid, pbstring value) = 0;
    virtual PBXRESULT SetStringField(pbobject obj, pbfieldID fid, LPCTSTR value) = 0;
    virtual PBXRESULT SetBoolField(pbobject obj, pbfieldID fid, pbboolean value) = 0;
    virtual PBXRESULT SetUintField(pbobject obj, pbfieldID fid, pbuint value) = 0;
    virtual PBXRESULT SetUlongField(pbobject obj, pbfieldID fid, pbulong value) = 0;
    virtual PBXRESULT SetBlobField(pbobject obj, pbfieldID fid, pbblob value) = 0;
    virtual PBXRESULT SetDateField(pbobject obj, pbfieldID fid, pbdate value) = 0;
    virtual PBXRESULT SetTimeField(pbobject obj, pbfieldID fid, pbtime value) = 0;
    virtual PBXRESULT SetDateTimeField(pbobject obj, pbfieldID fid, pbdatetime value) = 0;
    virtual PBXRESULT SetCharField(pbobject obj, pbfieldID fid, pbchar value) = 0;
    virtual PBXRESULT SetLongLongField(pbobject obj, pbfieldID fid, pblonglong value) = 0;
    virtual PBXRESULT SetObjectField(pbobject obj, pbfieldID fid, pbobject value) = 0;
    virtual PBXRESULT SetArrayField(pbobject obj, pbfieldID fid, pbarray value) = 0;

    virtual pbfieldID GetSharedVarID(pbgroup group, LPCTSTR fieldName) = 0;
    virtual pbuint GetSharedVarType(pbgroup, pbfieldID) = 0;

    virtual pbboolean IsSharedVarNull(pbgroup, pbfieldID) = 0;
    virtual void SetSharedVarToNull(pbgroup, pbfieldID) = 0;
    virtual pbboolean IsSharedVarArray(pbgroup, pbfieldID) = 0;
    virtual pbboolean IsSharedVarObject(pbgroup, pbfieldID) = 0;

    virtual pbint GetIntSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pblong GetLongSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbreal GetRealSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdouble GetDoubleSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdec GetDecSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbstring GetStringSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbboolean GetBoolSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbuint GetUintSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbulong GetUlongSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbblob GetBlobSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdate GetDateSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbtime GetTimeSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdatetime GetDateTimeSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbchar GetCharSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pblonglong GetLongLongSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbobject GetObjectSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbarray GetArraySharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;

    virtual PBXRESULT SetIntSharedVar(pbgroup group, pbfieldID fid, pbint value) = 0;
    virtual PBXRESULT SetLongSharedVar(pbgroup group, pbfieldID fid, pblong value) = 0;
    virtual PBXRESULT SetRealSharedVar(pbgroup group, pbfieldID fid, pbreal value) = 0;
    virtual PBXRESULT SetDoubleSharedVar(pbgroup group, pbfieldID fid, pbdouble value) = 0;
    virtual PBXRESULT SetDecSharedVar(pbgroup group, pbfieldID fid, pbdec value) = 0;
    virtual PBXRESULT SetPBStringSharedVar(pbgroup group, pbfieldID fid, pbstring value) = 0;
    virtual PBXRESULT SetStringSharedVar(pbgroup group, pbfieldID fid, LPCTSTR value) = 0;
    virtual PBXRESULT SetBoolSharedVar(pbgroup group, pbfieldID fid, pbboolean value) = 0;
    virtual PBXRESULT SetUintSharedVar(pbgroup group, pbfieldID fid, pbuint value) = 0;
    virtual PBXRESULT SetUlongSharedVar(pbgroup group, pbfieldID fid, pbulong value) = 0;
    virtual PBXRESULT SetBlobSharedVar(pbgroup group, pbfieldID fid, pbblob value) = 0;
    virtual PBXRESULT SetDateSharedVar(pbgroup group, pbfieldID fid, pbdate value) = 0;
    virtual PBXRESULT SetTimeSharedVar(pbgroup group, pbfieldID fid, pbtime value) = 0;
    virtual PBXRESULT SetDateTimeSharedVar(pbgroup group, pbfieldID fid, pbdatetime value) = 0;
    virtual PBXRESULT SetCharSharedVar(pbgroup group, pbfieldID fid, pbchar value) = 0;
    virtual PBXRESULT SetLongLongSharedVar(pbgroup group, pbfieldID fid, pblonglong value) = 0;
    virtual PBXRESULT SetObjectSharedVar(pbgroup group, pbfieldID fid, pbobject value) = 0;
    virtual PBXRESULT SetArraySharedVar(pbgroup group, pbfieldID fid, pbarray value) = 0;

    virtual pbfieldID GetGlobalVarID(LPCTSTR varName) = 0;
    virtual pbuint GetGlobalVarType(pbfieldID) = 0;

    virtual pbboolean IsGlobalVarNull(pbfieldID) = 0;
    virtual void SetGlobalVarToNull(pbfieldID) = 0;
    virtual pbboolean IsGlobalVarArray(pbfieldID) = 0;
    virtual pbboolean IsGlobalVarObject(pbfieldID) = 0;

    virtual pbint GetIntGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pblong GetLongGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbreal GetRealGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdouble GetDoubleGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdec GetDecGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbstring GetStringGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbboolean GetBoolGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbuint GetUintGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbulong GetUlongGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbblob GetBlobGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdate GetDateGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbtime GetTimeGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbdatetime GetDateTimeGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbchar GetCharGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pblonglong GetLongLongGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbobject GetObjectGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual pbarray GetArrayGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;

    virtual PBXRESULT SetIntGlobalVar(pbfieldID fid, pbint value) = 0;
    virtual PBXRESULT SetLongGlobalVar(pbfieldID fid, pblong value) = 0;
    virtual PBXRESULT SetRealGlobalVar(pbfieldID fid, pbreal value) = 0;
    virtual PBXRESULT SetDoubleGlobalVar(pbfieldID fid, pbdouble value) = 0;
    virtual PBXRESULT SetDecGlobalVar(pbfieldID fid, pbdec value) = 0;
    virtual PBXRESULT SetPBStringGlobalVar(pbfieldID fid, pbstring value) = 0;
    virtual PBXRESULT SetStringGlobalVar(pbfieldID fid, LPCTSTR value) = 0;
    virtual PBXRESULT SetBoolGlobalVar(pbfieldID fid, pbboolean value) = 0;
    virtual PBXRESULT SetUintGlobalVar(pbfieldID fid, pbuint value) = 0;
    virtual PBXRESULT SetUlongGlobalVar(pbfieldID fid, pbulong value) = 0;
    virtual PBXRESULT SetBlobGlobalVar(pbfieldID fid, pbblob value) = 0;
    virtual PBXRESULT SetDateGlobalVar(pbfieldID fid, pbdate value) = 0;
    virtual PBXRESULT SetTimeGlobalVar(pbfieldID fid, pbtime value) = 0;
    virtual PBXRESULT SetDateTimeGlobalVar(pbfieldID fid, pbdatetime value) = 0;
    virtual PBXRESULT SetCharGlobalVar(pbfieldID fid, pbchar value) = 0;
    virtual PBXRESULT SetLongLongGlobalVar(pbfieldID fid, pblonglong value) = 0;
    virtual PBXRESULT SetObjectGlobalVar(pbfieldID fid, pbobject value) = 0;
    virtual PBXRESULT SetArrayGlobalVar(pbfieldID fid, pbarray value) = 0;

    virtual pbboolean IsNativeObject(pbobject obj) = 0;
    virtual IPBX_UserObject *GetNativeInterface(pbobject obj) = 0;

    //Interface to access Array
    virtual pbarray NewBoundedSimpleArray(pbuint type, pbuint dimensions, PBArrayInfo::ArrayBound *bounds) = 0; //bounded array are fixed size, can be multidimension
    virtual pbarray NewUnboundedSimpleArray(pbuint type) = 0;                                                   //unbounded array are variable size, can only 1 dimension
    virtual pbarray NewBoundedObjectArray(pbclass, pbuint dimensions, PBArrayInfo::ArrayBound *bounds) = 0;     //bounded array are fixed size, can be multidimension
    virtual pbarray NewUnboundedObjectArray(pbclass) = 0;                                                       //unbounded array are variable size, can only 1 dimension
    virtual pblong GetArrayLength(pbarray) = 0;
    virtual PBArrayInfo *GetArrayInfo(pbarray) = 0;
    virtual PBXRESULT ReleaseArrayInfo(PBArrayInfo *) = 0; // Release info, if and only if the memory is allocated from pb

    virtual pbboolean IsArrayItemNull(pbarray array, pblong dim[]) = 0;
    virtual void SetArrayItemToNull(pbarray array, pblong dim[]) = 0;
    virtual pbuint GetArrayItemType(pbarray array, pblong dim[]) = 0;

    virtual pbint GetIntArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pblong GetLongArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbreal GetRealArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbdouble GetDoubleArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbdec GetDecArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbstring GetStringArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbboolean GetBoolArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbuint GetUintArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbulong GetUlongArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbblob GetBlobArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbdate GetDateArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbtime GetTimeArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbdatetime GetDateTimeArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbchar GetCharArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pblonglong GetLongLongArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual pbobject GetObjectArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;

    virtual PBXRESULT SetIntArrayItem(pbarray array, pblong dim[], pbint) = 0;
    virtual PBXRESULT SetLongArrayItem(pbarray array, pblong dim[], pblong) = 0;
    virtual PBXRESULT SetRealArrayItem(pbarray array, pblong dim[], pbreal) = 0;
    virtual PBXRESULT SetDoubleArrayItem(pbarray array, pblong dim[], pbdouble) = 0;
    virtual PBXRESULT SetDecArrayItem(pbarray array, pblong dim[], pbdec) = 0;
    virtual PBXRESULT SetPBStringArrayItem(pbarray array, pblong dim[], pbstring) = 0;
    virtual PBXRESULT SetStringArrayItem(pbarray array, pblong dim[], LPCTSTR) = 0;
    virtual PBXRESULT SetBoolArrayItem(pbarray array, pblong dim[], pbboolean) = 0;
    virtual PBXRESULT SetUintArrayItem(pbarray array, pblong dim[], pbuint) = 0;
    virtual PBXRESULT SetUlongArrayItem(pbarray array, pblong dim[], pbulong) = 0;
    virtual PBXRESULT SetBlobArrayItem(pbarray array, pblong dim[], pbblob) = 0;
    virtual PBXRESULT SetDateArrayItem(pbarray array, pblong dim[], pbdate) = 0;
    virtual PBXRESULT SetTimeArrayItem(pbarray array, pblong dim[], pbtime) = 0;
    virtual PBXRESULT SetDateTimeArrayItem(pbarray array, pblong dim[], pbdatetime) = 0;
    virtual PBXRESULT SetCharArrayItem(pbarray array, pblong dim[], pbchar) = 0;
    virtual PBXRESULT SetLongLongArrayItem(pbarray array, pblong dim[], pblonglong) = 0;
    virtual PBXRESULT SetObjectArrayItem(pbarray array, pblong dim[], pbobject) = 0;

    //Interface to access String.
    virtual pbstring NewString(LPCTSTR) = 0;
    virtual PBXRESULT SetString(pbstring, LPCTSTR) = 0;
    virtual pblong GetStringLength(pbstring pbstr) = 0;
    virtual LPCTSTR GetString(pbstring pbstr) = 0; //only return the pointer to LPTSTR, don't dup string

    //Interface to access Binary.
    virtual pbblob NewBlob(const void *bin, pblong len) = 0;            //dup the binary
    virtual PBXRESULT SetBlob(pbblob, const void *bin, pblong len) = 0; //dup the binary
    virtual pblong GetBlobLength(pbblob pbbin) = 0;
    virtual void *GetBlob(pbblob pbbin) = 0; //only return the pointer to void, don't dup buffer

    //Interface to access Date, Time and DateTime.
    virtual pbdate NewDate() = 0;
    virtual pbtime NewTime() = 0;
    virtual pbdatetime NewDateTime() = 0;
    virtual PBXRESULT SetDate(pbdate date, pbint year, pbint month, pbint day) = 0;
    virtual PBXRESULT SetTime(pbtime time, pbint hour, pbint minute, pbdouble second) = 0;
    virtual PBXRESULT SetDateTime(pbdatetime dt, pbint year, pbint month, pbint day,
                                  pbint hour, pbint minute, pbdouble second) = 0;
    virtual PBXRESULT CopyDateTime(pbdatetime dest, pbdatetime src) = 0; //Copy memory

    virtual PBXRESULT SplitDate(pbdate date, pbint *year, pbint *month, pbint *day) = 0;
    virtual PBXRESULT SplitTime(pbtime time, pbint *hour, pbint *minute, pbdouble *second) = 0;
    virtual PBXRESULT SplitDateTime(pbdatetime dt, pbint *year, pbint *month, pbint *day,
                                    pbint *hour, pbint *minute, pbdouble *second) = 0;
    virtual LPCTSTR GetDateString(pbdate) = 0;
    virtual void ReleaseDateString(LPCTSTR) = 0;
    virtual LPCTSTR GetTimeString(pbtime) = 0;
    virtual void ReleaseTimeString(LPCTSTR) = 0;
    virtual LPCTSTR GetDateTimeString(pbdatetime) = 0;
    virtual void ReleaseDateTimeString(LPCTSTR) = 0;

    //interface to access decimal number
    virtual pbdec NewDecimal() = 0;
    virtual PBXRESULT SetDecimal(pbdec dec, LPCTSTR dec_str) = 0;
    virtual LPCTSTR GetDecimalString(pbdec) = 0;
    virtual void ReleaseDecimalString(LPCTSTR) = 0;

    virtual pbproxyObject NewProxyObject(pbclass cls) = 0;
    virtual PBXRESULT SetMarshaler(pbproxyObject obj, IPBX_Marshaler *marshaler) = 0;
    virtual IPBX_Marshaler *GetMarshaler(pbproxyObject obj) = 0;

    virtual IPB_Value *AcquireValue(IPB_Value *value) = 0;
    virtual IPB_Value *AcquireArrayItemValue(pbarray arr, pblong dim[]) = 0;
    virtual void SetValue(IPB_Value *dest, IPB_Value *src) = 0;
    virtual void SetArrayItemValue(pbarray arr, pblong dim[], IPB_Value *src) = 0;
    virtual void ReleaseValue(IPB_Value *value) = 0;

    virtual void SetProp(LPCTSTR name, void *data) = 0;
    virtual void *GetProp(LPCTSTR name) = 0;
    virtual void RemoveProp(LPCTSTR name) = 0;

    virtual pbboolean ProcessPBMessage() = 0;

    virtual pblong GetEnumItemValue(LPCTSTR enumName, LPCTSTR enumItemName) = 0;
    virtual LPCTSTR GetEnumItemName(LPCTSTR enumName, pblong enumItemValue) = 0;

    virtual IPB_Value *GetPBAnyField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual IPB_Value *GetPBAnySharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual IPB_Value *GetPBAnyGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual IPB_Value *GetPBAnyArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;

    virtual pbobject CreateResultSet(IPB_ResultSetAccessor *rs) = 0;
    virtual IPB_ResultSetAccessor *GetResultSetAccessor(pbobject rs) = 0;
    virtual void ReleaseResultSetAccessor(IPB_ResultSetAccessor *rs) = 0;

    virtual pbboolean RestartRequested() = 0;
    virtual pbboolean HasPBVisualObject() = 0;

    virtual void SetDebugTrace(LPCTSTR traceFile) = 0;

    //**********************************************************
    //	Begin of the new type BYTE
    //**********************************************************
    virtual PBXRESULT AddByteArgument(PBCallInfo *ci, pbbyte value, pbboolean isNull = FALSE) = 0;
    virtual pbbyte GetByteField(pbobject obj, pbfieldID fid, pbboolean &isNull) = 0;
    virtual PBXRESULT SetByteField(pbobject obj, pbfieldID fid, pbbyte value) = 0;
    virtual pbbyte GetByteSharedVar(pbgroup group, pbfieldID fid, pbboolean &isNull) = 0;
    virtual PBXRESULT SetByteSharedVar(pbgroup group, pbfieldID fid, pbbyte value) = 0;
    virtual pbbyte GetByteGlobalVar(pbfieldID fid, pbboolean &isNull) = 0;
    virtual PBXRESULT SetByteGlobalVar(pbfieldID fid, pbbyte value) = 0;
    virtual pbbyte GetByteArrayItem(pbarray array, pblong dim[], pbboolean &isNull) = 0;
    virtual PBXRESULT SetByteArrayItem(pbarray array, pblong dim[], pbbyte) = 0;

    //**********************************************************
    //	End of the new type BYTE
    //**********************************************************
    virtual void ReleaseString(LPCTSTR str) = 0;
    virtual PBXRESULT SetLargeUnboundedByteArrayItem(pbarray array, pblong dim[], pbbyte value, PBArrayInfo *arrinfo) = 0;
    virtual void reserved2() = 0;
    virtual void reserved3() = 0;
    virtual void reserved4() = 0;
    virtual void reserved5() = 0;
    virtual void reserved6() = 0;
    virtual void reserved7() = 0;
    virtual void reserved8() = 0;
    virtual void reserved9() = 0;
    virtual void reserved10() = 0;

protected:
    ~IPB_Session(){};
};

//****************************************************************************
//  Functions exposed by PBVM
//****************************************************************************
PBXEXPORT PBXRESULT PB_GetVM(IPB_VM **vm);

//****************************************************************************
//  Utility classes
//****************************************************************************

//****************************************************************************
//  Definition of PBCallInfoHolder
//****************************************************************************
class PBCallInfoHolder
{
    IPB_Session *d_session;
    PBCallInfo d_callInfo;

public:
    PBCallInfoHolder(IPB_Session *session)
        : d_session(session)
    {
    }

    ~PBCallInfoHolder()
    {
        if (d_session != NULL)
            d_session->FreeCallInfo(&d_callInfo);
    }

    PBXRESULT InitCallInfo(pbclass cls, pbmethodID mid)
    {
        return d_session->InitCallInfo(cls, mid, &d_callInfo);
    }

    PBCallInfo *operator->()
    {
        return &d_callInfo;
    }

    PBCallInfo *operator&()
    {
        return &d_callInfo;
    }

    PBCallInfo &operator*()
    {
        return d_callInfo;
    }

protected:
    PBCallInfoHolder() {}

private:
    PBCallInfoHolder(const PBCallInfoHolder &);
    PBCallInfoHolder &operator=(const PBCallInfoHolder &);
};

//****************************************************************************
//  Definition of PBArrayInfoHolder
//****************************************************************************
class PBArrayInfoHolder
{
    IPB_Session *d_session;
    PBArrayInfo *d_arrayInfo;

public:
    PBArrayInfoHolder(IPB_Session *session)
        : d_session(session),
          d_arrayInfo(NULL)
    {
    }

    ~PBArrayInfoHolder()
    {
        if ((d_session != NULL) && d_arrayInfo)
            d_session->ReleaseArrayInfo(d_arrayInfo);
    }

    PBXRESULT GetArrayInfo(pbarray arr)
    {

        if (d_session != NULL)
        {
            if (d_arrayInfo)
            {
                d_session->ReleaseArrayInfo(d_arrayInfo);
                d_arrayInfo = NULL;
            }

            d_arrayInfo = d_session->GetArrayInfo(arr);
        }

        return PBX_OK;
    }

    PBArrayInfo *operator->()
    {
        return d_arrayInfo;
    }

    PBArrayInfo &operator*()
    {
        return *d_arrayInfo;
    }

protected:
    PBArrayInfoHolder() {}

private:
    PBArrayInfoHolder(const PBArrayInfoHolder &);
    PBArrayInfoHolder &operator=(const PBArrayInfoHolder &);
};

//****************************************************************************
//  PBSessionOwner
//
//  This is a smartpointer-like class. The destructor releases the session
//  automatically.
//****************************************************************************
class PBSessionOwner
{
protected:
    IPB_Session *d_session;

public:
    PBSessionOwner(IPB_Session *session);
    ~PBSessionOwner();

    IPB_Session *operator->() const;
    IPB_Session *operator*() const;

private:
    PBSessionOwner(const PBSessionOwner &);
    PBSessionOwner &operator=(const PBSessionOwner &);
};

inline PBSessionOwner::PBSessionOwner(IPB_Session *session) : d_session(session)
{
}

inline PBSessionOwner::~PBSessionOwner()
{
    if (d_session != NULL)
        d_session->Release();
}

inline IPB_Session *PBSessionOwner::operator->() const
{
    return d_session;
}

inline IPB_Session *PBSessionOwner::operator*() const
{
    return d_session;
}

#if (defined(_MSC_VER) && _MSC_VER > 1300)
#define TYPENAME typename
#else
#define TYPENAME
#endif

//***************************************************************************
// Array Accessor
//***************************************************************************
#include "pbarray.h"

//***************************************************************************
// Field Accessor
//***************************************************************************
#include "pbfield.h"

//****************************************************************************
//  PBObjectCreator
//****************************************************************************
class PBObjectCreator
{
    IPB_Session *d_session;
    pbclass d_class;

public:
    PBObjectCreator(
        IPB_Session *session,
        LPCTSTR nvoName);
    ~PBObjectCreator();

    pbobject CreateObject();

    bool IsValid() { return d_class != NULL; }

private: // prevention
    PBObjectCreator(const PBObjectCreator &);
    PBObjectCreator operator=(const PBObjectCreator &);
};

//****************************************************************************
//  PBEventTrigger
//****************************************************************************
class PBEventTrigger
{
    IPB_Session *d_session;
    pbobject d_object;
    pbmethodID d_mid;
    PBCallInfo d_ci;

public:
    PBEventTrigger(
        IPB_Session *session,
        pbobject obj,
        LPCTSTR methodName,
        LPCTSTR signature = (LPCTSTR)_TEXT(""));

    PBEventTrigger(
        IPB_Session *session,
        pbobject obj,
        pbmethodID mid);

    ~PBEventTrigger();

    pblong GetNumArgs() const;
    IPB_Value *GetArg(pblong i);

    IPB_Value *GetReturnValue();

    PBXRESULT Trigger();

    PBCallInfo &GetCallInfo() { return d_ci; }

    bool IsValid() { return d_mid != kUndefinedMethodID; }

private: // prevention
    PBEventTrigger(const PBEventTrigger &);
    PBEventTrigger operator=(const PBEventTrigger &);
};

//****************************************************************************
//  PBObjectFunctionInvoker
//****************************************************************************
class PBObjectFunctionInvoker
{
    IPB_Session *d_session;
    pbobject d_object;
    pbmethodID d_mid;
    PBCallInfo d_ci;

public:
    PBObjectFunctionInvoker(
        IPB_Session *session,
        pbobject obj,
        LPCTSTR methodName,
        LPCTSTR signature = (LPCTSTR)_TEXT(""));

    PBObjectFunctionInvoker(
        IPB_Session *session,
        pbobject obj,
        pbmethodID mid);

    ~PBObjectFunctionInvoker();

    pblong GetNumArgs() const;
    IPB_Value *GetArg(pblong i);
    IPB_Value *GetReturnValue();

    PBXRESULT Invoke();

    PBCallInfo &GetCallInfo() { return d_ci; }

    bool IsValid() { return d_mid != kUndefinedMethodID; }

private: // prevention
    PBObjectFunctionInvoker(const PBObjectFunctionInvoker &);
    PBObjectFunctionInvoker operator=(const PBObjectFunctionInvoker &);
};

//****************************************************************************
//  PBGlobalFunctionInvoker
//****************************************************************************
class PBGlobalFunctionInvoker
{
    IPB_Session *d_session;
    pbclass d_class;
    pbmethodID d_mid;
    PBCallInfo d_ci;

public:
    PBGlobalFunctionInvoker(
        IPB_Session *session,
        LPCTSTR methodName,
        LPCTSTR signature = (LPCTSTR)_TEXT(""));

    ~PBGlobalFunctionInvoker();

    pblong GetNumArgs() const;
    IPB_Value *GetArg(pblong i);

    IPB_Value *GetReturnValue();

    PBXRESULT Invoke();

    PBCallInfo &GetCallInfo() { return d_ci; }

    bool IsValid() { return d_mid != kUndefinedMethodID; }

private: // prevention
    PBGlobalFunctionInvoker(const PBGlobalFunctionInvoker &);
    PBGlobalFunctionInvoker operator=(const PBGlobalFunctionInvoker &);
};

#endif // PBNI_H
