#include "PBEXT.H"

extern "C"
{

#pragma region IPB_VM

    void pbvm_SetLoggingService(
        IPB_VM *vm, IPBX_Logging *log_svc)
    {
        vm->SetLoggingService(log_svc);
    }
    PBXRESULT pbvm_CreateSession(IPB_VM *vm,
                                 LPCTSTR applicationName,
                                 LPCTSTR *libraryList,
                                 pbuint numLibs,
                                 IPB_Session **session)
    {
        return vm->CreateSession(applicationName, libraryList, numLibs, session);
    }
    PBXRESULT pbvm_RunApplication(
        IPB_VM *vm,
        LPCTSTR applicationName,
        LPCTSTR *libraryList,
        pbuint numLibs,
        LPCTSTR commandLine,
        IPB_Session **session)
    {
        return vm->RunApplication(applicationName, libraryList, numLibs, commandLine, session);
    }

#pragma endregion

#pragma region IPB_Value

    pbclass pbvalue_GetClass(IPB_Value *pVal)
    {
        return pVal->GetClass();
    }
    pbuint pbvalue_GetType(IPB_Value *pVal) { return pVal->GetType(); }
    pbboolean pbvalue_IsArray(IPB_Value *pVal) { return pVal->IsArray(); }
    pbboolean pbvalue_IsObject(IPB_Value *pVal) { return pVal->IsObject(); }
    pbboolean pbvalue_IsEnum(IPB_Value *pVal) { return pVal->IsEnum(); }
    pbboolean pbvalue_IsByRef(IPB_Value *pVal) { return pVal->IsByRef(); }

    pbboolean pbvalue_IsNull(IPB_Value *pVal) { return pVal->IsNull(); }
    PBXRESULT pbvalue_SetToNull(IPB_Value *pVal) { return pVal->SetToNull(); }

    pbint pbvalue_GetInt(IPB_Value *pVal) { return pVal->GetInt(); }
    pbuint pbvalue_GetUint(IPB_Value *pVal) { return pVal->GetUint(); }
    pbboolean pbvalue_GetBool(IPB_Value *pVal) { return pVal->GetBool(); }
    pblong pbvalue_GetLong(IPB_Value *pVal) { return pVal->GetLong(); }
    pbulong pbvalue_GetUlong(IPB_Value *pVal) { return pVal->GetUlong(); }
    pbreal pbvalue_GetReal(IPB_Value *pVal) { return pVal->GetReal(); }
    pbdouble pbvalue_GetDouble(IPB_Value *pVal) { return pVal->GetDouble(); }
    pbdec pbvalue_GetDecimal(IPB_Value *pVal) { return pVal->GetDecimal(); }
    pbchar pbvalue_GetChar(IPB_Value *pVal) { return pVal->GetChar(); }
    pbstring pbvalue_GetString(IPB_Value *pVal) { return pVal->GetString(); }
    pbobject pbvalue_GetObject(IPB_Value *pVal) { return pVal->GetObject(); }
    pbarray pbvalue_GetArray(IPB_Value *pVal) { return pVal->GetArray(); }
    pbtime pbvalue_GetTime(IPB_Value *pVal) { return pVal->GetTime(); }
    pbdate pbvalue_GetDate(IPB_Value *pVal) { return pVal->GetDate(); }
    pbdatetime pbvalue_GetDateTime(IPB_Value *pVal) { return pVal->GetDateTime(); }
    pblonglong pbvalue_GetLongLong(IPB_Value *pVal) { return pVal->GetLongLong(); }
    pbblob pbvalue_GetBlob(IPB_Value *pVal) { return pVal->GetBlob(); }

    PBXRESULT pbvalue_SetInt(IPB_Value *pVal, pbint v) { return pVal->SetInt(v); }
    PBXRESULT pbvalue_SetUint(IPB_Value *pVal, pbuint v) { return pVal->SetUint(v); }
    PBXRESULT pbvalue_SetBool(IPB_Value *pVal, pbboolean v) { return pVal->SetBool(v); }
    PBXRESULT pbvalue_SetLong(IPB_Value *pVal, pblong v) { return pVal->SetLong(v); }
    PBXRESULT pbvalue_SetUlong(IPB_Value *pVal, pbulong v) { return pVal->SetUlong(v); }
    PBXRESULT pbvalue_SetReal(IPB_Value *pVal, pbreal v) { return pVal->SetReal(v); }
    PBXRESULT pbvalue_SetDouble(IPB_Value *pVal, pbdouble v) { return pVal->SetDouble(v); }
    PBXRESULT pbvalue_SetDecimal(IPB_Value *pVal, pbdec v) { return pVal->SetDecimal(v); }
    PBXRESULT pbvalue_SetChar(IPB_Value *pVal, pbchar v) { return pVal->SetChar(v); }
    PBXRESULT pbvalue_SetPBString(IPB_Value *pVal, pbstring v) { return pVal->SetPBString(v); }
    PBXRESULT pbvalue_SetString(IPB_Value *pVal, LPCTSTR v) { return pVal->SetString(v); }
    PBXRESULT pbvalue_SetArray(IPB_Value *pVal, pbarray v) { return pVal->SetArray(v); }
    PBXRESULT pbvalue_SetTime(IPB_Value *pVal, pbtime v) { return pVal->SetTime(v); }
    PBXRESULT pbvalue_SetDate(IPB_Value *pVal, pbdate v) { return pVal->SetDate(v); }
    PBXRESULT pbvalue_SetDateTime(IPB_Value *pVal, pbdatetime v) { return pVal->SetDateTime(v); }
    PBXRESULT pbvalue_SetLongLong(IPB_Value *pVal, pblonglong v) { return pVal->SetLongLong(v); }
    PBXRESULT pbvalue_SetBlob(IPB_Value *pVal, pbblob v) { return pVal->SetBlob(v); }
    PBXRESULT pbvalue_SetObject(IPB_Value *pVal, pbobject v) { return pVal->SetObject(v); }

    pbboolean pbvalue_IsReadOnly(IPB_Value *pVal) { return pVal->IsReadOnly(); }

    pbbyte pbvalue_GetByte(IPB_Value *pVal) { return pVal->GetByte(); }
    PBXRESULT pbvalue_SetByte(IPB_Value *pVal, pbbyte v) { return pVal->SetByte(v); }

#pragma endregion

#pragma region IPB_Session

    void pbsession_Release(IPB_Session *session)
    {
        session->Release();
    }

    pbclass pbsession_GetClass(IPB_Session *session, pbobject obj) { return session->GetClass(obj); }
    pbgroup pbsession_GetSystemGroup(IPB_Session *session) { return session->GetSystemGroup(); }

    pbmethodID pbsession_GetMethodID(IPB_Session *session, pbclass cls, LPCTSTR methodName,
                                     PBRoutineType rt, LPCTSTR signature, pbboolean publicOnly) { return session->GetMethodID(cls, methodName, rt, signature, publicOnly); }
    pbmethodID pbsession_FindMatchingFunction(IPB_Session *session, pbclass cls, LPCTSTR methodName,
                                              PBRoutineType rt, LPCTSTR readableSignature) { return session->FindMatchingFunction(cls, methodName, rt, readableSignature); }
    pbmethodID pbsession_GetMethodIDByEventID(IPB_Session *session, pbclass cls, LPCTSTR eventID) { return session->GetMethodIDByEventID(cls, eventID); }

    PBXRESULT pbsession_InitCallInfo(IPB_Session *session, pbclass cls, pbmethodID mid, PBCallInfo *ci) { return session->InitCallInfo(cls, mid, ci); }
    void pbsession_FreeCallInfo(IPB_Session *session, PBCallInfo *ci) { session->FreeCallInfo(ci); }

    void pbsession_AddLocalRef(IPB_Session *session, pbobject obj) { session->AddLocalRef(obj); }
    void pbsession_RemoveLocalRef(IPB_Session *session, pbobject obj) { session->RemoveLocalRef(obj); }
    void pbsession_AddGlobalRef(IPB_Session *session, pbobject obj) { session->AddGlobalRef(obj); }
    void pbsession_RemoveGlobalRef(IPB_Session *session, pbobject obj) { session->RemoveGlobalRef(obj); }
    void pbsession_PushLocalFrame(IPB_Session *session) { session->PushLocalFrame(); }
    void pbsession_PopLocalFrame(IPB_Session *session) { session->PopLocalFrame(); }

    // For passing variable arguments.
    PBXRESULT pbsession_AddIntArgument(IPB_Session *session, PBCallInfo *ci, pbint value, pbboolean isNull) { return session->AddIntArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddLongArgument(IPB_Session *session, PBCallInfo *ci, pblong value, pbboolean isNull) { return session->AddLongArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddRealArgument(IPB_Session *session, PBCallInfo *ci, pbreal value, pbboolean isNull) { return session->AddRealArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddDoubleArgument(IPB_Session *session, PBCallInfo *ci, pbdouble value, pbboolean isNull) { return session->AddDoubleArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddDecArgument(IPB_Session *session, PBCallInfo *ci, pbdec value, pbboolean isNull) { return session->AddDecArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddPBStringArgument(IPB_Session *session, PBCallInfo *ci, pbstring value, pbboolean isNull) { return session->AddPBStringArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddStringArgument(IPB_Session *session, PBCallInfo *ci, LPCTSTR value, pbboolean isNull) { return session->AddStringArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddBoolArgument(IPB_Session *session, PBCallInfo *ci, pbboolean value, pbboolean isNull) { return session->AddBoolArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddUintArgument(IPB_Session *session, PBCallInfo *ci, pbuint value, pbboolean isNull) { return session->AddUintArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddUlongArgument(IPB_Session *session, PBCallInfo *ci, pbulong value, pbboolean isNull) { return session->AddUlongArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddBlobArgument(IPB_Session *session, PBCallInfo *ci, pbblob value, pbboolean isNull) { return session->AddBlobArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddDateArgument(IPB_Session *session, PBCallInfo *ci, pbdate value, pbboolean isNull) { return session->AddDateArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddTimeArgument(IPB_Session *session, PBCallInfo *ci, pbtime value, pbboolean isNull) { return session->AddTimeArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddDateTimeArgument(IPB_Session *session, PBCallInfo *ci, pbdatetime value, pbboolean isNull) { return session->AddDateTimeArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddCharArgument(IPB_Session *session, PBCallInfo *ci, pbchar value, pbboolean isNull) { return session->AddCharArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddLongLongArgument(IPB_Session *session, PBCallInfo *ci, pblonglong value, pbboolean isNull) { return session->AddLongLongArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddObjectArgument(IPB_Session *session, PBCallInfo *ci, pbobject value, pbboolean isNull) { return session->AddObjectArgument(ci, value, isNull); }
    PBXRESULT pbsession_AddArrayArgument(IPB_Session *session, PBCallInfo *ci, pbarray value, pbboolean isNull) { return session->AddArrayArgument(ci, value, isNull); }

    PBXRESULT pbsession_InvokeClassFunction(IPB_Session *session, pbclass cls, pbmethodID mid, PBCallInfo *ci) { return session->InvokeClassFunction(cls, mid, ci); }
    PBXRESULT pbsession_InvokeObjectFunction(IPB_Session *session, pbobject obj, pbmethodID mid, PBCallInfo *ci) { return session->InvokeObjectFunction(obj, mid, ci); }
    PBXRESULT pbsession_TriggerEvent(IPB_Session *session, pbobject obj, pbmethodID mid, PBCallInfo *ci) { return session->TriggerEvent(obj, mid, ci); }

    pbboolean pbsession_HasExceptionThrown(IPB_Session *session) { return session->HasExceptionThrown(); }
    pbobject pbsession_GetException(IPB_Session *session) { return session->GetException(); }
    void pbsession_ClearException(IPB_Session *session) { return session->ClearException(); }
    void pbsession_ThrowException(IPB_Session *session, pbobject ex) { session->ThrowException(ex); }

    pbgroup pbsession_GetCurrGroup(IPB_Session *session) { return session->GetCurrGroup(); }
    pbgroup pbsession_FindGroup(IPB_Session *session, LPCTSTR name, pbgroup_type type) { return session->FindGroup(name, type); }
    pbclass pbsession_FindClass(IPB_Session *session, pbgroup group, LPCTSTR name) { return session->FindClass(group, name); }
    pbclass pbsession_FindClassByClassID(IPB_Session *session, pbgroup group, pbint classID) { return session->FindClassByClassID(group, classID); }
    LPCTSTR pbsession_GetClassName(IPB_Session *session, pbclass cls) { return session->GetClassName(cls); }
    pbclass pbsession_GetSuperClass(IPB_Session *session, pbclass cls) { return session->GetSuperClass(cls); }
    pbclass pbsession_GetSystemClass(IPB_Session *session, pbclass cls) { return session->GetSystemClass(cls); }
    pbboolean pbsession_IsAutoInstantiate(IPB_Session *session, pbclass pbcls) { return session->IsAutoInstantiate(pbcls); }

    pbobject pbsession_NewObject(IPB_Session *session, pbclass cls) { return session->NewObject(cls); }

    pbfieldID pbsession_GetFieldID(IPB_Session *session, pbclass cls, LPCTSTR fieldName) { return session->GetFieldID(cls, fieldName); }
    pbuint pbsession_GetFieldType(IPB_Session *session, pbclass cls, pbfieldID fid) { return session->GetFieldType(cls, fid); }
    pbulong pbsession_GetNumOfFields(IPB_Session *session, pbclass cls) { return session->GetNumOfFields(cls); }
    LPCTSTR pbsession_GetFieldName(IPB_Session *session, pbclass cls, pbfieldID fid) { return session->GetFieldName(cls, fid); }

    pbboolean pbsession_IsFieldNull(IPB_Session *session, pbobject obj, pbfieldID fid) { return session->IsFieldNull(obj, fid); }
    void pbsession_SetFieldToNull(IPB_Session *session, pbobject obj, pbfieldID fid) { session->SetFieldToNull(obj, fid); }
    pbboolean pbsession_IsFieldArray(IPB_Session *session, pbclass cls, pbfieldID fid) { return session->IsFieldArray(cls, fid); }
    pbboolean pbsession_IsFieldObject(IPB_Session *session, pbclass cls, pbfieldID fid) { return session->IsFieldObject(cls, fid); }
    PBXRESULT pbsession_UpdateField(IPB_Session *session, pbobject obj, pbfieldID fid) { return session->UpdateField(obj, fid); }

    pbint pbsession_GetIntField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetIntField(obj, fid, *isNull); }
    pblong pbsession_GetLongField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetLongField(obj, fid, *isNull); }
    pbreal pbsession_GetRealField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetRealField(obj, fid, *isNull); }
    pbdouble pbsession_GetDoubleField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetDoubleField(obj, fid, *isNull); }
    pbdec pbsession_GetDecField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetDecField(obj, fid, *isNull); }
    pbstring pbsession_GetStringField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetStringField(obj, fid, *isNull); }
    pbboolean pbsession_GetBoolField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetBoolField(obj, fid, *isNull); }
    pbuint pbsession_GetUintField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetUintField(obj, fid, *isNull); }
    pbulong pbsession_GetUlongField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetUlongField(obj, fid, *isNull); }
    pbblob pbsession_GetBlobField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetBlobField(obj, fid, *isNull); }
    pbdate pbsession_GetDateField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetDateField(obj, fid, *isNull); }
    pbtime pbsession_GetTimeField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetTimeField(obj, fid, *isNull); }
    pbdatetime pbsession_GetDateTimeField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetDateTimeField(obj, fid, *isNull); }
    pbchar pbsession_GetCharField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetCharField(obj, fid, *isNull); }
    pblonglong pbsession_GetLongLongField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetLongLongField(obj, fid, *isNull); }
    pbobject pbsession_GetObjectField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetObjectField(obj, fid, *isNull); }
    pbarray pbsession_GetArrayField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetArrayField(obj, fid, *isNull); }

    PBXRESULT pbsession_SetIntField(IPB_Session *session, pbobject obj, pbfieldID fid, pbint value) { return session->SetIntField(obj, fid, value); }
    PBXRESULT pbsession_SetLongField(IPB_Session *session, pbobject obj, pbfieldID fid, pblong value) { return session->SetLongField(obj, fid, value); }
    PBXRESULT pbsession_SetRealField(IPB_Session *session, pbobject obj, pbfieldID fid, pbreal value) { return session->SetRealField(obj, fid, value); }
    PBXRESULT pbsession_SetDoubleField(IPB_Session *session, pbobject obj, pbfieldID fid, pbdouble value) { return session->SetDoubleField(obj, fid, value); }
    PBXRESULT pbsession_SetDecField(IPB_Session *session, pbobject obj, pbfieldID fid, pbdec value) { return session->SetDecField(obj, fid, value); }
    PBXRESULT pbsession_SetPBStringField(IPB_Session *session, pbobject obj, pbfieldID fid, pbstring value) { return session->SetPBStringField(obj, fid, value); }
    PBXRESULT pbsession_SetStringField(IPB_Session *session, pbobject obj, pbfieldID fid, LPCTSTR value) { return session->SetStringField(obj, fid, value); }
    PBXRESULT pbsession_SetBoolField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean value) { return session->SetBoolField(obj, fid, value); }
    PBXRESULT pbsession_SetUintField(IPB_Session *session, pbobject obj, pbfieldID fid, pbuint value) { return session->SetUintField(obj, fid, value); }
    PBXRESULT pbsession_SetUlongField(IPB_Session *session, pbobject obj, pbfieldID fid, pbulong value) { return session->SetUlongField(obj, fid, value); }
    PBXRESULT pbsession_SetBlobField(IPB_Session *session, pbobject obj, pbfieldID fid, pbblob value) { return session->SetBlobField(obj, fid, value); }
    PBXRESULT pbsession_SetDateField(IPB_Session *session, pbobject obj, pbfieldID fid, pbdate value) { return session->SetDateField(obj, fid, value); }
    PBXRESULT pbsession_SetTimeField(IPB_Session *session, pbobject obj, pbfieldID fid, pbtime value) { return session->SetTimeField(obj, fid, value); }
    PBXRESULT pbsession_SetDateTimeField(IPB_Session *session, pbobject obj, pbfieldID fid, pbdatetime value) { return session->SetDateTimeField(obj, fid, value); }
    PBXRESULT pbsession_SetCharField(IPB_Session *session, pbobject obj, pbfieldID fid, pbchar value) { return session->SetCharField(obj, fid, value); }
    PBXRESULT pbsession_SetLongLongField(IPB_Session *session, pbobject obj, pbfieldID fid, pblonglong value) { return session->SetLongLongField(obj, fid, value); }
    PBXRESULT pbsession_SetObjectField(IPB_Session *session, pbobject obj, pbfieldID fid, pbobject value) { return session->SetObjectField(obj, fid, value); }
    PBXRESULT pbsession_SetArrayField(IPB_Session *session, pbobject obj, pbfieldID fid, pbarray value) { return session->SetArrayField(obj, fid, value); }

    pbfieldID pbsession_GetSharedVarID(IPB_Session *session, pbgroup group, LPCTSTR fieldName) { return session->GetSharedVarID(group, fieldName); }
    pbuint pbsession_GetSharedVarType(IPB_Session *session, pbgroup group, pbfieldID fid) { return session->GetSharedVarType(group, fid); }

    pbboolean pbsession_IsSharedVarNull(IPB_Session *session, pbgroup group, pbfieldID fid) { return session->IsSharedVarNull(group, fid); }
    void pbsession_SetSharedVarToNull(IPB_Session *session, pbgroup group, pbfieldID fid) { session->SetSharedVarToNull(group, fid); }
    pbboolean pbsession_IsSharedVarArray(IPB_Session *session, pbgroup group, pbfieldID fid) { return session->IsSharedVarArray(group, fid); }
    pbboolean pbsession_IsSharedVarObject(IPB_Session *session, pbgroup group, pbfieldID fid) { return session->IsSharedVarObject(group, fid); }

    pbint pbsession_GetIntSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetIntSharedVar(group, fid, *isNull); }
    pblong pbsession_GetLongSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetLongSharedVar(group, fid, *isNull); }
    pbreal pbsession_GetRealSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetRealSharedVar(group, fid, *isNull); }
    pbdouble pbsession_GetDoubleSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetDoubleSharedVar(group, fid, *isNull); }
    pbdec pbsession_GetDecSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetDecSharedVar(group, fid, *isNull); }
    pbstring pbsession_GetStringSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetStringSharedVar(group, fid, *isNull); }
    pbboolean pbsession_GetBoolSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetBoolSharedVar(group, fid, *isNull); }
    pbuint pbsession_GetUintSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetUintSharedVar(group, fid, *isNull); }
    pbulong pbsession_GetUlongSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetUlongSharedVar(group, fid, *isNull); }
    pbblob pbsession_GetBlobSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetBlobSharedVar(group, fid, *isNull); }
    pbdate pbsession_GetDateSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetDateSharedVar(group, fid, *isNull); }
    pbtime pbsession_GetTimeSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetTimeSharedVar(group, fid, *isNull); }
    pbdatetime pbsession_GetDateTimeSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetDateTimeSharedVar(group, fid, *isNull); }
    pbchar pbsession_GetCharSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetCharSharedVar(group, fid, *isNull); }
    pblonglong pbsession_GetLongLongSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetLongLongSharedVar(group, fid, *isNull); }
    pbobject pbsession_GetObjectSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetObjectSharedVar(group, fid, *isNull); }
    pbarray pbsession_GetArraySharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetArraySharedVar(group, fid, *isNull); }

    PBXRESULT pbsession_SetIntSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbint value) { return session->SetIntSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetLongSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pblong value) { return session->SetLongSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetRealSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbreal value) { return session->SetRealSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetDoubleSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbdouble value) { return session->SetDoubleSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetDecSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbdec value) { return session->SetDecSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetPBStringSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbstring value) { return session->SetPBStringSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetStringSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, LPCTSTR value) { return session->SetStringSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetBoolSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean value) { return session->SetBoolSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetUintSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbuint value) { return session->SetUintSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetUlongSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbulong value) { return session->SetUlongSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetBlobSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbblob value) { return session->SetBlobSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetDateSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbdate value) { return session->SetDateSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetTimeSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbtime value) { return session->SetTimeSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetDateTimeSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbdatetime value) { return session->SetDateTimeSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetCharSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbchar value) { return session->SetCharSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetLongLongSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pblonglong value) { return session->SetLongLongSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetObjectSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbobject value) { return session->SetObjectSharedVar(group, fid, value); }
    PBXRESULT pbsession_SetArraySharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbarray value) { return session->SetArraySharedVar(group, fid, value); }

    pbfieldID pbsession_GetGlobalVarID(IPB_Session *session, LPCTSTR varName) { return session->GetGlobalVarID(varName); }
    pbuint pbsession_GetGlobalVarType(IPB_Session *session, pbfieldID fid) { return session->GetGlobalVarType(fid); }

    pbboolean pbsession_IsGlobalVarNull(IPB_Session *session, pbfieldID fid) { return session->IsGlobalVarNull(fid); }
    void pbsession_SetGlobalVarToNull(IPB_Session *session, pbfieldID fid) { session->SetGlobalVarToNull(fid); }
    pbboolean pbsession_IsGlobalVarArray(IPB_Session *session, pbfieldID fid) { return session->IsGlobalVarArray(fid); }
    pbboolean pbsession_IsGlobalVarObject(IPB_Session *session, pbfieldID fid) { return session->IsGlobalVarObject(fid); }

    pbint pbsession_GetIntGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetIntGlobalVar(fid, *isNull); }
    pblong pbsession_GetLongGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetLongGlobalVar(fid, *isNull); }
    pbreal pbsession_GetRealGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetRealGlobalVar(fid, *isNull); }
    pbdouble pbsession_GetDoubleGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetDoubleGlobalVar(fid, *isNull); }
    pbdec pbsession_GetDecGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetDecGlobalVar(fid, *isNull); }
    pbstring pbsession_GetStringGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetStringGlobalVar(fid, *isNull); }
    pbboolean pbsession_GetBoolGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetBoolGlobalVar(fid, *isNull); }
    pbuint pbsession_GetUintGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetUintGlobalVar(fid, *isNull); }
    pbulong pbsession_GetUlongGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetUlongGlobalVar(fid, *isNull); }
    pbblob pbsession_GetBlobGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetBlobGlobalVar(fid, *isNull); }
    pbdate pbsession_GetDateGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetDateGlobalVar(fid, *isNull); }
    pbtime pbsession_GetTimeGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetTimeGlobalVar(fid, *isNull); }
    pbdatetime pbsession_GetDateTimeGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetDateTimeGlobalVar(fid, *isNull); }
    pbchar pbsession_GetCharGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetCharGlobalVar(fid, *isNull); }
    pblonglong pbsession_GetLongLongGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetLongLongGlobalVar(fid, *isNull); }
    pbobject pbsession_GetObjectGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetObjectGlobalVar(fid, *isNull); }
    pbarray pbsession_GetArrayGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetArrayGlobalVar(fid, *isNull); }

    PBXRESULT pbsession_SetIntGlobalVar(IPB_Session *session, pbfieldID fid, pbint value) { return session->SetIntGlobalVar(fid, value); }
    PBXRESULT pbsession_SetLongGlobalVar(IPB_Session *session, pbfieldID fid, pblong value) { return session->SetLongGlobalVar(fid, value); }
    PBXRESULT pbsession_SetRealGlobalVar(IPB_Session *session, pbfieldID fid, pbreal value) { return session->SetRealGlobalVar(fid, value); }
    PBXRESULT pbsession_SetDoubleGlobalVar(IPB_Session *session, pbfieldID fid, pbdouble value) { return session->SetDoubleGlobalVar(fid, value); }
    PBXRESULT pbsession_SetDecGlobalVar(IPB_Session *session, pbfieldID fid, pbdec value) { return session->SetDecGlobalVar(fid, value); }
    PBXRESULT pbsession_SetPBStringGlobalVar(IPB_Session *session, pbfieldID fid, pbstring value) { return session->SetPBStringGlobalVar(fid, value); }
    PBXRESULT pbsession_SetStringGlobalVar(IPB_Session *session, pbfieldID fid, LPCTSTR value) { return session->SetStringGlobalVar(fid, value); }
    PBXRESULT pbsession_SetBoolGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean value) { return session->SetBoolGlobalVar(fid, value); }
    PBXRESULT pbsession_SetUintGlobalVar(IPB_Session *session, pbfieldID fid, pbuint value) { return session->SetUintGlobalVar(fid, value); }
    PBXRESULT pbsession_SetUlongGlobalVar(IPB_Session *session, pbfieldID fid, pbulong value) { return session->SetUlongGlobalVar(fid, value); }
    PBXRESULT pbsession_SetBlobGlobalVar(IPB_Session *session, pbfieldID fid, pbblob value) { return session->SetBlobGlobalVar(fid, value); }
    PBXRESULT pbsession_SetDateGlobalVar(IPB_Session *session, pbfieldID fid, pbdate value) { return session->SetDateGlobalVar(fid, value); }
    PBXRESULT pbsession_SetTimeGlobalVar(IPB_Session *session, pbfieldID fid, pbtime value) { return session->SetTimeGlobalVar(fid, value); }
    PBXRESULT pbsession_SetDateTimeGlobalVar(IPB_Session *session, pbfieldID fid, pbdatetime value) { return session->SetDateTimeGlobalVar(fid, value); }
    PBXRESULT pbsession_SetCharGlobalVar(IPB_Session *session, pbfieldID fid, pbchar value) { return session->SetCharGlobalVar(fid, value); }
    PBXRESULT pbsession_SetLongLongGlobalVar(IPB_Session *session, pbfieldID fid, pblonglong value) { return session->SetLongLongGlobalVar(fid, value); }
    PBXRESULT pbsession_SetObjectGlobalVar(IPB_Session *session, pbfieldID fid, pbobject value) { return session->SetObjectGlobalVar(fid, value); }
    PBXRESULT pbsession_SetArrayGlobalVar(IPB_Session *session, pbfieldID fid, pbarray value) { return session->SetArrayGlobalVar(fid, value); }

    pbboolean pbsession_IsNativeObject(IPB_Session *session, pbobject obj) { return session->IsNativeObject(obj); }
    IPBX_UserObject *pbsession_GetNativeInterface(IPB_Session *session, pbobject obj) { return session->GetNativeInterface(obj); }

    // Interface to access Array
    pbarray pbsession_NewBoundedSimpleArray(IPB_Session *session, pbuint type, pbuint dimensions, PBArrayInfo::ArrayBound *bounds) // bounded array are fixed size, can be multidimension
    {
        return session->NewBoundedSimpleArray(type, dimensions, bounds);
    }
    pbarray pbsession_NewUnboundedSimpleArray(IPB_Session *session, pbuint type) // unbounded array are variable size, can only 1 dimension
    {
        return session->NewUnboundedSimpleArray(type);
    }
    pbarray pbsession_NewBoundedObjectArray(IPB_Session *session, pbclass cls, pbuint dimensions, PBArrayInfo::ArrayBound *bounds) // bounded array are fixed size, can be multidimension
    {
        return session->NewBoundedObjectArray(cls, dimensions, bounds);
    }
    pbarray pbsession_NewUnboundedObjectArray(IPB_Session *session, pbclass cls) // unbounded array are variable size, can only 1 dimension
    {
        return session->NewUnboundedObjectArray(cls);
    }
    pblong pbsession_GetArrayLength(IPB_Session *session, pbarray array) { return session->GetArrayLength(array); }
    PBArrayInfo *pbsession_GetArrayInfo(IPB_Session *session, pbarray array) { return session->GetArrayInfo(array); }
    PBXRESULT pbsession_ReleaseArrayInfo(IPB_Session *session, PBArrayInfo *arrInfo) // pbsession_Release info, if and only if the memory is allocated from pb
    {
        return session->ReleaseArrayInfo(arrInfo);
    }

    pbboolean pbsession_IsArrayItemNull(IPB_Session *session, pbarray array, pblong dim[]) { return session->IsArrayItemNull(array, dim); }
    void pbsession_SetArrayItemToNull(IPB_Session *session, pbarray array, pblong dim[]) { session->SetArrayItemToNull(array, dim); }
    pbuint pbsession_GetArrayItemType(IPB_Session *session, pbarray array, pblong dim[]) { return session->GetArrayItemType(array, dim); }

    pbint pbsession_GetIntArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetIntArrayItem(array, dim, *isNull); }
    pblong pbsession_GetLongArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetLongArrayItem(array, dim, *isNull); }
    pbreal pbsession_GetRealArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetRealArrayItem(array, dim, *isNull); }
    pbdouble pbsession_GetDoubleArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetDoubleArrayItem(array, dim, *isNull); }
    pbdec pbsession_GetDecArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetDecArrayItem(array, dim, *isNull); }
    pbstring pbsession_GetStringArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetStringArrayItem(array, dim, *isNull); }
    pbboolean pbsession_GetBoolArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetBoolArrayItem(array, dim, *isNull); }
    pbuint pbsession_GetUintArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetUintArrayItem(array, dim, *isNull); }
    pbulong pbsession_GetUlongArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetUlongArrayItem(array, dim, *isNull); }
    pbblob pbsession_GetBlobArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetBlobArrayItem(array, dim, *isNull); }
    pbdate pbsession_GetDateArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetDateArrayItem(array, dim, *isNull); }
    pbtime pbsession_GetTimeArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetTimeArrayItem(array, dim, *isNull); }
    pbdatetime pbsession_GetDateTimeArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetDateTimeArrayItem(array, dim, *isNull); }
    pbchar pbsession_GetCharArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetCharArrayItem(array, dim, *isNull); }
    pblonglong pbsession_GetLongLongArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetLongLongArrayItem(array, dim, *isNull); }
    pbobject pbsession_GetObjectArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetObjectArrayItem(array, dim, *isNull); }

    PBXRESULT pbsession_SetIntArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbint value) { return session->SetIntArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetLongArrayItem(IPB_Session *session, pbarray array, pblong dim[], pblong value) { return session->SetLongArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetRealArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbreal value) { return session->SetRealArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetDoubleArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbdouble value) { return session->SetDoubleArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetDecArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbdec value) { return session->SetDecArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetPBStringArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbstring value) { return session->SetPBStringArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetStringArrayItem(IPB_Session *session, pbarray array, pblong dim[], LPCTSTR value) { return session->SetStringArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetBoolArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean value) { return session->SetBoolArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetUintArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbuint value) { return session->SetUintArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetUlongArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbulong value) { return session->SetUlongArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetBlobArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbblob value) { return session->SetBlobArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetDateArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbdate value) { return session->SetDateArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetTimeArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbtime value) { return session->SetTimeArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetDateTimeArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbdatetime value) { return session->SetDateTimeArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetCharArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbchar value) { return session->SetCharArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetLongLongArrayItem(IPB_Session *session, pbarray array, pblong dim[], pblonglong value) { return session->SetLongLongArrayItem(array, dim, value); }
    PBXRESULT pbsession_SetObjectArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbobject value) { return session->SetObjectArrayItem(array, dim, value); }

    // Interface to access String.
    pbstring pbsession_NewString(IPB_Session *session, LPCTSTR str) { return session->NewString(str); }
    PBXRESULT pbsession_SetString(IPB_Session *session, pbstring pbstr, LPCTSTR str) { return session->SetString(pbstr, str); }
    pblong pbsession_GetStringLength(IPB_Session *session, pbstring pbstr) { return session->GetStringLength(pbstr); }
    LPCTSTR pbsession_GetString(IPB_Session *session, pbstring pbstr) // only return the pointer to LPTSTR, don't dup string
    {
        return session->GetString(pbstr);
    }

    // Interface to access Binary.
    pbblob pbsession_NewBlob(IPB_Session *session, const void *bin, pblong len) // dup the binary
    {
        return session->NewBlob(bin, len);
    }

    PBXRESULT pbsession_SetBlob(IPB_Session *session, pbblob pbbin, const void *bin, pblong len) // dup the binary
    {
        return session->SetBlob(pbbin, bin, len);
    }
    pblong pbsession_GetBlobLength(IPB_Session *session, pbblob pbbin)
    {
        return session->GetBlobLength(pbbin);
    }
    void *pbsession_GetBlob(IPB_Session *session, pbblob pbbin) // only return the pointer to void, don't dup buffer
    {
        return session->GetBlob(pbbin);
    }

    // Interface to access Date, Time and DateTime.
    pbdate pbsession_NewDate(IPB_Session *session)
    {
        return session->NewDate();
    }
    pbtime pbsession_NewTime(IPB_Session *session) { return session->NewTime(); }
    pbdatetime pbsession_NewDateTime(IPB_Session *session) { return session->NewDateTime(); }
    PBXRESULT pbsession_SetDate(IPB_Session *session, pbdate date, pbint year, pbint month, pbint day) { return session->SetDate(date, year, month, day); }
    PBXRESULT pbsession_SetTime(IPB_Session *session, pbtime time, pbint hour, pbint minute, pbdouble second) { return session->SetTime(time, hour, minute, second); }
    PBXRESULT pbsession_SetDateTime(IPB_Session *session, pbdatetime dt, pbint year, pbint month, pbint day,
                                    pbint hour, pbint minute, pbdouble second)
    {
        return session->SetDateTime(dt, year, month, day, hour, minute, second);
    }
    PBXRESULT pbsession_CopyDateTime(IPB_Session *session, pbdatetime dest, pbdatetime src) // Copy memory
    {
        return session->CopyDateTime(dest, src);
    }

    PBXRESULT pbsession_SplitDate(IPB_Session *session, pbdate date, pbint *year, pbint *month, pbint *day)
    {
        return session->SplitDate(date, year, month, day);
    }
    PBXRESULT pbsession_SplitTime(IPB_Session *session, pbtime time, pbint *hour, pbint *minute, pbdouble *second) { return session->SplitTime(time, hour, minute, second); }
    PBXRESULT pbsession_SplitDateTime(IPB_Session *session, pbdatetime dt, pbint *year, pbint *month, pbint *day,
                                      pbint *hour, pbint *minute, pbdouble *second)
    {
        return session->SplitDateTime(dt, year, month, day, hour, minute, second);
    }
    LPCTSTR pbsession_GetDateString(IPB_Session *session, pbdate dt) { return session->GetDateString(dt); }
    void pbsession_ReleaseDateString(IPB_Session *session, LPCTSTR str) { session->ReleaseDateString(str); }
    LPCTSTR pbsession_GetTimeString(IPB_Session *session, pbtime time) { return session->GetTimeString(time); }
    void pbsession_ReleaseTimeString(IPB_Session *session, LPCTSTR str) { session->ReleaseTimeString(str); }
    LPCTSTR pbsession_GetDateTimeString(IPB_Session *session, pbdatetime dt) { return session->GetDateTimeString(dt); }
    void pbsession_ReleaseDateTimeString(IPB_Session *session, LPCTSTR str) { session->ReleaseDateTimeString(str); }

    // interface to access decimal number
    pbdec pbsession_NewDecimal(IPB_Session *session) { return session->NewDecimal(); }
    PBXRESULT pbsession_SetDecimal(IPB_Session *session, pbdec dec, LPCTSTR dec_str) { return session->SetDecimal(dec, dec_str); }
    LPCTSTR pbsession_GetDecimalString(IPB_Session *session, pbdec dec) { return session->GetDecimalString(dec); }
    void pbsession_ReleaseDecimalString(IPB_Session *session, LPCTSTR str) { session->ReleaseDecimalString(str); }

    pbproxyObject pbsession_NewProxyObject(IPB_Session *session, pbclass cls) { return session->NewProxyObject(cls); }
    PBXRESULT pbsession_SetMarshaler(IPB_Session *session, pbproxyObject obj, IPBX_Marshaler *marshaler) { return session->SetMarshaler(obj, marshaler); }
    IPBX_Marshaler *pbsession_GetMarshaler(IPB_Session *session, pbproxyObject obj) { return session->GetMarshaler(obj); }

    IPB_Value *pbsession_AcquireValue(IPB_Session *session, IPB_Value *value) { return session->AcquireValue(value); }
    IPB_Value *pbsession_AcquireArrayItemValue(IPB_Session *session, pbarray arr, pblong dim[]) { return session->AcquireArrayItemValue(arr, dim); }
    void pbsession_SetValue(IPB_Session *session, IPB_Value *dest, IPB_Value *src) { session->SetValue(dest, src); }
    void pbsession_SetArrayItemValue(IPB_Session *session, pbarray arr, pblong dim[], IPB_Value *src) { session->SetArrayItemValue(arr, dim, src); }
    void pbsession_ReleaseValue(IPB_Session *session, IPB_Value *value) { session->ReleaseValue(value); }

    void pbsession_SetProp(IPB_Session *session, LPCTSTR name, void *data) { session->SetProp(name, data); }
    void *pbsession_GetProp(IPB_Session *session, LPCTSTR name) { return session->GetProp(name); }
    void pbsession_RemoveProp(IPB_Session *session, LPCTSTR name) { session->RemoveProp(name); }

    pbboolean pbsession_ProcessPBMessage(IPB_Session *session) { return session->ProcessPBMessage(); }

    pblong pbsession_GetEnumItemValue(IPB_Session *session, LPCTSTR enumName, LPCTSTR enumItemName) { return session->GetEnumItemValue(enumName, enumItemName); }
    LPCTSTR pbsession_GetEnumItemName(IPB_Session *session, LPCTSTR enumName, pblong enumItemValue) { return session->GetEnumItemName(enumName, enumItemValue); }

    IPB_Value *pbsession_GetPBAnyField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetPBAnyField(obj, fid, *isNull); }
    IPB_Value *pbsession_GetPBAnySharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetPBAnySharedVar(group, fid, *isNull); }
    IPB_Value *pbsession_GetPBAnyGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetPBAnyGlobalVar(fid, *isNull); }
    IPB_Value *pbsession_GetPBAnyArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetPBAnyArrayItem(array, dim, *isNull); }

    pbobject pbsession_CreateResultSet(IPB_Session *session, IPB_ResultSetAccessor *rs) { return session->CreateResultSet(rs); }
    IPB_ResultSetAccessor *pbsession_GetResultSetAccessor(IPB_Session *session, pbobject rs) { return session->GetResultSetAccessor(rs); }
    void pbsession_ReleaseResultSetAccessor(IPB_Session *session, IPB_ResultSetAccessor *rs) { return session->ReleaseResultSetAccessor(rs); }

    pbboolean pbsession_RestartRequested(IPB_Session *session) { return session->RestartRequested(); }
    pbboolean pbsession_HasPBVisualObject(IPB_Session *session) { return session->HasPBVisualObject(); }

    void pbsession_SetDebugTrace(IPB_Session *session, LPCTSTR traceFile) { return session->SetDebugTrace(traceFile); }

    //**********************************************************
    //	Begin of the new type BYTE
    //**********************************************************
    PBXRESULT pbsession_AddByteArgument(IPB_Session *session, PBCallInfo *ci, pbbyte value, pbboolean isNull) { return session->AddByteArgument(ci, value, isNull); }
    pbbyte pbsession_GetByteField(IPB_Session *session, pbobject obj, pbfieldID fid, pbboolean *isNull) { return session->GetByteField(obj, fid, *isNull); }
    PBXRESULT pbsession_SetByteField(IPB_Session *session, pbobject obj, pbfieldID fid, pbbyte value) { return session->SetByteField(obj, fid, value); }
    pbbyte pbsession_GetByteSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbboolean *isNull) { return session->GetByteSharedVar(group, fid, *isNull); }
    PBXRESULT pbsession_SetByteSharedVar(IPB_Session *session, pbgroup group, pbfieldID fid, pbbyte value) { return session->SetByteSharedVar(group, fid, value); }
    pbbyte pbsession_GetByteGlobalVar(IPB_Session *session, pbfieldID fid, pbboolean *isNull) { return session->GetByteGlobalVar(fid, *isNull); }
    PBXRESULT pbsession_SetByteGlobalVar(IPB_Session *session, pbfieldID fid, pbbyte value) { return session->SetByteGlobalVar(fid, value); }
    pbbyte pbsession_GetByteArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbboolean *isNull) { return session->GetByteArrayItem(array, dim, *isNull); }
    PBXRESULT pbsession_SetByteArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbbyte value) { return session->SetByteArrayItem(array, dim, value); }

    //**********************************************************
    //	End of the new type BYTE
    //**********************************************************
    void pbsession_ReleaseString(IPB_Session *session, LPCTSTR str) { session->ReleaseString(str); }
    PBXRESULT pbsession_SetLargeUnboundedByteArrayItem(IPB_Session *session, pbarray array, pblong dim[], pbbyte value, PBArrayInfo *arrinfo) { return session->SetLargeUnboundedByteArrayItem(array, dim, value, arrinfo); }

#pragma endregion

#pragma region IPB_Arguments

    pbint pbargs_GetCount(IPB_Arguments *pArgs)
    {
        return pArgs->GetCount();
    }
    IPB_Value *pbargs_GetAt(IPB_Arguments *pArgs, pbint index) { return pArgs->GetAt(index); }

#pragma endregion
}

#pragma region IPBX_UserObject

template <class T>
class UserObject : public T
{
    static const INT32 MAGIC_CODE = 0x494E5352;

public:
    typedef PBXRESULT DestroyHandler(
        void *ctx);
    typedef PBXRESULT InvokeHandler(
        void *ctx,
        IPB_Session *session,
        pbobject obj,
        pbmethodID mid,
        PBCallInfo *ci);

public:
    UserObject(void *ctx, INT64 type_id, DestroyHandler *pDestroyHandler, InvokeHandler *pInvokeHandler)
        : _ctx(ctx),
          _magic(MAGIC_CODE),
          _type_id(type_id),
          _pDestroyHandler(pDestroyHandler),
          _pInvokeHandler(pInvokeHandler)
    {
    }
    virtual ~UserObject() override{};

    void *GetSafeContext(INT64 type_id) const
    {
        return (this->_magic == MAGIC_CODE && this->_type_id == type_id) ? this->_ctx : NULL;
    }

private:
    virtual void Destroy() override
    {
        this->_pDestroyHandler(this->_ctx);
        delete this;
    }

    virtual PBXRESULT Invoke(
        IPB_Session *session,
        pbobject obj,
        pbmethodID mid,
        PBCallInfo *ci) override
    {
        return this->_pInvokeHandler(this->_ctx, session, obj, mid, ci);
    }

private:
    DestroyHandler *_pDestroyHandler;
    InvokeHandler *_pInvokeHandler;

protected:
    void *_ctx;
    INT32 _magic;
    INT64 _type_id;
};

class NonVisualObject : public UserObject<IPBX_NonVisualObject>
{
public:
    NonVisualObject(void *ctx, INT64 type_id, DestroyHandler *pDestroyHandler, InvokeHandler *pInvokeHandler)
        : UserObject(ctx, type_id, pDestroyHandler, pInvokeHandler)
    {
    }
};

class VisualObject : public UserObject<IPBX_VisualObject>
{
public:
    typedef HWND CreateControlHandler(
        void *ctx,
        DWORD dwExStyle,
        LPCTSTR lpWindowName,
        DWORD dwStyle,
        int x,
        int y,
        int nWidth,
        int nHeight,
        HWND hWndParent,
        HINSTANCE hInstance);
    typedef int GetEventIDHandler(
        void *ctx,
        HWND hWnd,
        UINT iMsg,
        WPARAM wParam,
        LPARAM lParam);

public:
    VisualObject(void *ctx,
                 INT64 type_id,
                 LPCTSTR lpcsClsName,
                 UserObject::DestroyHandler *pDestroyHandler,
                 UserObject::InvokeHandler *pInvokeHandler,
                 CreateControlHandler pCreateControlHandler,
                 GetEventIDHandler pGetEventIDHandler)
        : UserObject(ctx, type_id, pDestroyHandler, pInvokeHandler),
          _pCreateControlHandler(pCreateControlHandler),
          _pGetEventIDHandler(pGetEventIDHandler),
          _clsName(lpcsClsName)
    {
    }

private:
    virtual LPCTSTR GetWindowClassName() override
    {
        return this->_clsName;
    }

    virtual HWND CreateControl(
        DWORD dwExStyle,      // extended window style
        LPCTSTR lpWindowName, // window name
        DWORD dwStyle,        // window style
        int x,                // horizontal position of window
        int y,                // vertical position of window
        int nWidth,           // window width
        int nHeight,          // window height
        HWND hWndParent,      // handle to parent or owner window
        HINSTANCE hInstance   // handle to application instance
        ) override
    {
        return this->_pCreateControlHandler(this->_ctx, dwExStyle, lpWindowName, dwStyle, x, y, nWidth, nHeight, hWndParent, hInstance);
    }

    virtual int GetEventID(
        HWND hWnd,     /* Handle to owner window */
        UINT iMsg,     /* Owner message */
        WPARAM wParam, /* Owner word parm */
        LPARAM lParam  /* Owner long parm */
        ) override
    {
        return this->_pGetEventIDHandler(this->_ctx, hWnd, iMsg, wParam, lParam);
    }

private:
    LPCTSTR _clsName;
    CreateControlHandler *_pCreateControlHandler;
    GetEventIDHandler *_pGetEventIDHandler;
};

extern "C"
{
    struct NVOM
    {
        void *ctx;
        INT64 type_id;
        NonVisualObject::DestroyHandler *destroy;
        NonVisualObject::InvokeHandler *invoke;
    };

    IPBX_NonVisualObject *NewNonVisualObject(const NVOM *om)
    {
        return new NonVisualObject(om->ctx, om->type_id, om->destroy, om->invoke);
    }

    struct VOM
    {
        void *ctx;
        INT64 type_id;
        LPCTSTR cls_name;
        VisualObject::DestroyHandler *destroy;
        VisualObject::InvokeHandler *invoke;
        VisualObject::CreateControlHandler *create_control;
        VisualObject::GetEventIDHandler *get_event_id;
    };

    IPBX_VisualObject *NewVisualObject(const VOM *om)
    {
        return new VisualObject(om->ctx, om->type_id, om->cls_name, om->destroy, om->invoke, om->create_control, om->get_event_id);
    }

    void *GetSafeContext(const UserObject<IPBX_UserObject> *pObj, INT64 type_id)
    {
        if (!pObj)
            return NULL;
        return pObj->GetSafeContext(type_id);
    }
}

#pragma endregion

#pragma region exports

/*
    FIXME
    导出符号重定向
    解决解决llvm-msvc 32bit dll编译符号被截断的问题，代替`.def`文件定义符号的做法，以支持与`SystemLibrary`导出符号并存
*/

#ifdef GLOBALFUNCTION

extern "C" PBXRESULT __cdecl RS_InvokeGlobalFunction(IPB_Session *session, LPCTSTR functionName, PBCallInfo *ci);

PBXEXPORT PBXRESULT PBXCALL PBX_InvokeGlobalFunction(IPB_Session *session, LPCTSTR functionName, PBCallInfo *ci)
{
    return RS_InvokeGlobalFunction(session, functionName, ci);
}

#endif

#ifdef NONVISUALOBEJCT

extern "C" PBXRESULT __cdecl RS_CreateNonVisualObject(IPB_Session *session, pbobject pbobj, LPCTSTR className, IPBX_NonVisualObject **obj);

PBXEXPORT PBXRESULT PBXCALL PBX_CreateNonVisualObject(IPB_Session *session, pbobject pbobj, LPCTSTR className, IPBX_NonVisualObject **obj)
{
    return RS_CreateNonVisualObject(session, pbobj, className, obj);
}

#endif

#ifdef VISUALOBEJCT

extern "C" PBXRESULT __cdecl RS_CreateVisualObject(IPB_Session *session, pbobject pbobj, LPCTSTR className, IPBX_VisualObject **obj);

PBXEXPORT PBXRESULT PBXCALL PBX_CreateVisualObject(IPB_Session *session, pbobject pbobj, LPCTSTR className, IPBX_VisualObject **obj)
{
    return RS_CreateVisualObject(session, pbobj, className, obj);
}

#endif

#pragma endregion