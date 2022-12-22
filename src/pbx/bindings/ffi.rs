use super::*;
use std::ffi::c_void;

// VM
extern "C" {
    pub fn pbvm_CreateSession(
        vm: pbvm,
        applicationName: LPCTSTR,
        libraryList: *const LPCTSTR,
        numLibs: pbuint,
        session: *mut pbsession
    ) -> PBXRESULT;
    pub fn pbvm_RunApplication(
        vm: pbvm,
        applicationName: LPCTSTR,
        libraryList: *const LPCTSTR,
        numLibs: pbuint,
        commandLine: LPCTSTR,
        session: *mut pbsession
    ) -> PBXRESULT;
}

// pbsession
extern "C" {
    /*
        pbsession
    */

    pub fn pbsession_Release(session: pbsession);

    pub fn pbsession_SetProp(session: pbsession, name: LPCTSTR, data: *const c_void);
    pub fn pbsession_GetProp(session: pbsession, name: LPCTSTR) -> *const c_void;
    pub fn pbsession_RemoveProp(session: pbsession, name: LPCTSTR);

    pub fn pbsession_ProcessPBMessage(session: pbsession) -> pbboolean;

    pub fn pbsession_GetEnumItemValue(session: pbsession, enumName: LPCTSTR, enumItemName: LPCTSTR)
    -> pblong;
    pub fn pbsession_GetEnumItemName(session: pbsession, enumName: LPCTSTR, enumItemValue: pblong)
    -> LPCTSTR;

    pub fn pbsession_RestartRequested(session: pbsession) -> pbboolean;
    pub fn pbsession_HasPBVisualObject(session: pbsession) -> pbboolean;

    pub fn pbsession_SetDebugTrace(session: pbsession, traceFile: LPCTSTR);

    pub fn pbsession_AddLocalRef(session: pbsession, obj: pbobject);
    pub fn pbsession_RemoveLocalRef(session: pbsession, obj: pbobject);
    pub fn pbsession_AddGlobalRef(session: pbsession, obj: pbobject);
    pub fn pbsession_RemoveGlobalRef(session: pbsession, obj: pbobject);
    pub fn pbsession_PushLocalFrame(session: pbsession);
    pub fn pbsession_PopLocalFrame(session: pbsession);

    pub fn pbsession_HasExceptionThrown(session: pbsession) -> pbboolean;
    pub fn pbsession_GetException(session: pbsession) -> pbobject;
    pub fn pbsession_ClearException(session: pbsession);
    pub fn pbsession_ThrowException(session: pbsession, ex: pbobject);

    /*
        Interface to access Global variables
    */

    pub fn pbsession_GetGlobalVarID(session: pbsession, varName: LPCTSTR) -> FieldId;
    pub fn pbsession_GetGlobalVarType(session: pbsession, fid: FieldId) -> ValueType;

    pub fn pbsession_IsGlobalVarNull(session: pbsession, fid: FieldId) -> pbboolean;
    pub fn pbsession_SetGlobalVarToNull(session: pbsession, fid: FieldId);
    pub fn pbsession_IsGlobalVarArray(session: pbsession, fid: FieldId) -> pbboolean;
    pub fn pbsession_IsGlobalVarObject(session: pbsession, fid: FieldId) -> pbboolean;

    pub fn pbsession_GetIntGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbint;
    pub fn pbsession_GetLongGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pblong;
    pub fn pbsession_GetRealGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbreal;
    pub fn pbsession_GetDoubleGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean)
    -> pbdouble;
    pub fn pbsession_GetDecGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbdec;
    pub fn pbsession_GetStringGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean)
    -> pbstring;
    pub fn pbsession_GetBoolGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbboolean;
    pub fn pbsession_GetUintGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbuint;
    pub fn pbsession_GetUlongGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbulong;
    pub fn pbsession_GetBlobGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbblob;
    pub fn pbsession_GetDateGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbdate;
    pub fn pbsession_GetTimeGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbtime;
    pub fn pbsession_GetDateTimeGlobalVar(
        session: pbsession,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdatetime;
    pub fn pbsession_GetCharGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> PBChar;
    pub fn pbsession_GetByteGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbbyte;
    pub fn pbsession_GetLongLongGlobalVar(
        session: pbsession,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pblonglong;
    pub fn pbsession_GetObjectGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean)
    -> pbobject;
    pub fn pbsession_GetArrayGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbarray;
    pub fn pbsession_GetPBAnyGlobalVar(session: pbsession, fid: FieldId, isNull: *mut pbboolean) -> pbvalue;

    pub fn pbsession_SetIntGlobalVar(session: pbsession, fid: FieldId, value: pbint) -> PBXRESULT;
    pub fn pbsession_SetLongGlobalVar(session: pbsession, fid: FieldId, value: pblong) -> PBXRESULT;
    pub fn pbsession_SetRealGlobalVar(session: pbsession, fid: FieldId, value: pbreal) -> PBXRESULT;
    pub fn pbsession_SetDoubleGlobalVar(session: pbsession, fid: FieldId, value: pbdouble) -> PBXRESULT;
    pub fn pbsession_SetDecGlobalVar(session: pbsession, fid: FieldId, value: pbdec) -> PBXRESULT;
    pub fn pbsession_SetPBStringGlobalVar(session: pbsession, fid: FieldId, value: pbstring) -> PBXRESULT;
    pub fn pbsession_SetStringGlobalVar(session: pbsession, fid: FieldId, value: LPCTSTR) -> PBXRESULT;
    pub fn pbsession_SetBoolGlobalVar(session: pbsession, fid: FieldId, value: pbboolean) -> PBXRESULT;
    pub fn pbsession_SetUintGlobalVar(session: pbsession, fid: FieldId, value: pbuint) -> PBXRESULT;
    pub fn pbsession_SetUlongGlobalVar(session: pbsession, fid: FieldId, value: pbulong) -> PBXRESULT;
    pub fn pbsession_SetBlobGlobalVar(session: pbsession, fid: FieldId, value: pbblob) -> PBXRESULT;
    pub fn pbsession_SetDateGlobalVar(session: pbsession, fid: FieldId, value: pbdate) -> PBXRESULT;
    pub fn pbsession_SetTimeGlobalVar(session: pbsession, fid: FieldId, value: pbtime) -> PBXRESULT;
    pub fn pbsession_SetDateTimeGlobalVar(session: pbsession, fid: FieldId, value: pbdatetime) -> PBXRESULT;
    pub fn pbsession_SetCharGlobalVar(session: pbsession, fid: FieldId, value: PBChar) -> PBXRESULT;
    pub fn pbsession_SetByteGlobalVar(session: pbsession, fid: FieldId, value: pbbyte) -> PBXRESULT;
    pub fn pbsession_SetLongLongGlobalVar(session: pbsession, fid: FieldId, value: pblonglong) -> PBXRESULT;
    pub fn pbsession_SetObjectGlobalVar(session: pbsession, fid: FieldId, value: pbobject) -> PBXRESULT;
    pub fn pbsession_SetArrayGlobalVar(session: pbsession, fid: FieldId, pbarray: pbarray) -> PBXRESULT;

    /*
        Interface to access Argument
    */

    pub fn pbsession_AddIntArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbint,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddLongArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pblong,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddRealArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbreal,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddDoubleArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbdouble,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddDecArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbdec,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddPBStringArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbstring,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddStringArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: LPCTSTR,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddBoolArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbboolean,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddUintArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbuint,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddUlongArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbulong,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddBlobArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbblob,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddDateArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbdate,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddTimeArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbtime,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddDateTimeArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbdatetime,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddCharArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: PBChar,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddLongLongArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pblonglong,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddObjectArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbobject,
        isNull: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_AddArrayArgument(
        session: pbsession,
        ci: pbcallinfo,
        value: pbarray,
        isNull: pbboolean
    ) -> PBXRESULT;

    /*
        Method calling
    */

    pub fn pbsession_GetClass(session: pbsession, obj: pbobject) -> Option<pbclass>;
    pub fn pbsession_GetSystemGroup(session: pbsession) -> pbgroup;
    pub fn pbsession_GetMethodID(
        session: pbsession,
        cls: pbclass,
        methodName: LPCTSTR,
        rt: RoutineType,
        signature: LPCTSTR,
        publicOnly: pbboolean
    ) -> MethodId;
    pub fn pbsession_FindMatchingFunction(
        session: pbsession,
        cls: pbclass,
        methodName: LPCTSTR,
        rt: RoutineType,
        readableSignature: LPCTSTR
    ) -> MethodId;
    pub fn pbsession_GetMethodIDByEventID(session: pbsession, cls: pbclass, eventID: LPCTSTR) -> MethodId;

    pub fn pbsession_InitCallInfo(
        session: pbsession,
        cls: pbclass,
        mid: MethodId,
        ci: pbcallinfo
    ) -> PBXRESULT;
    pub fn pbsession_FreeCallInfo(session: pbsession, ci: pbcallinfo);

    pub fn pbsession_InvokeClassFunction(
        session: pbsession,
        cls: pbclass,
        mid: MethodId,
        ci: pbcallinfo
    ) -> PBXRESULT;
    pub fn pbsession_InvokeObjectFunction(
        session: pbsession,
        obj: pbobject,
        mid: MethodId,
        ci: pbcallinfo
    ) -> PBXRESULT;
    pub fn pbsession_TriggerEvent(
        session: pbsession,
        obj: pbobject,
        mid: MethodId,
        ci: pbcallinfo
    ) -> PBXRESULT;

    /*
        Interface to access Object
    */

    pub fn NewNonVisualObject(om: *const c_void) -> pbuserobject;
    pub fn NewVisualObject(om: *const c_void) -> pbuserobject;
    pub fn GetSafeContext(obj: pbuserobject, type_id: u64) -> *mut c_void;

    pub fn pbsession_NewObject(session: pbsession, cls: pbclass) -> pbobject;

    pub fn pbsession_IsNativeObject(session: pbsession, obj: pbobject) -> pbboolean;
    pub fn pbsession_GetNativeInterface(session: pbsession, obj: pbobject) -> Option<pbuserobject>;

    pub fn pbsession_GetCurrGroup(session: pbsession) -> pbgroup;
    pub fn pbsession_FindGroup(session: pbsession, name: LPCTSTR, r#type: GroupType) -> Option<pbgroup>;
    pub fn pbsession_FindClass(session: pbsession, group: pbgroup, name: LPCTSTR) -> Option<pbclass>;
    pub fn pbsession_FindClassByClassID(
        session: pbsession,
        group: pbgroup,
        classID: pbint
    ) -> Option<pbclass>;
    pub fn pbsession_GetClassName(session: pbsession, cls: pbclass) -> LPCTSTR;
    pub fn pbsession_GetSuperClass(session: pbsession, cls: pbclass) -> Option<pbclass>;
    pub fn pbsession_GetSystemClass(session: pbsession, cls: pbclass) -> pbclass;
    pub fn pbsession_IsAutoInstantiate(session: pbsession, pbcls: pbclass) -> pbboolean;

    pub fn pbsession_GetFieldID(session: pbsession, cls: pbclass, fieldName: LPCTSTR) -> FieldId;
    pub fn pbsession_GetFieldType(session: pbsession, cls: pbclass, fid: FieldId) -> ValueType;
    pub fn pbsession_GetNumOfFields(session: pbsession, cls: pbclass) -> pbulong;
    pub fn pbsession_GetFieldName(session: pbsession, cls: pbclass, fid: FieldId) -> LPCTSTR;

    pub fn pbsession_IsFieldNull(session: pbsession, obj: pbobject, fid: FieldId) -> pbboolean;
    pub fn pbsession_SetFieldToNull(session: pbsession, obj: pbobject, fid: FieldId);
    pub fn pbsession_IsFieldArray(session: pbsession, cls: pbclass, fid: FieldId) -> pbboolean;
    pub fn pbsession_IsFieldObject(session: pbsession, cls: pbclass, fid: FieldId) -> pbboolean;
    pub fn pbsession_UpdateField(session: pbsession, obj: pbobject, fid: FieldId) -> PBXRESULT;

    pub fn pbsession_GetIntField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbint;
    pub fn pbsession_GetLongField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pblong;
    pub fn pbsession_GetRealField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbreal;
    pub fn pbsession_GetDoubleField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdouble;
    pub fn pbsession_GetDecField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdec;
    #[link_name = "pbsession_GetStringField"]
    pub fn pbsession_GetStrField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbstring;
    pub fn pbsession_GetStringField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbstring;
    pub fn pbsession_GetBoolField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbboolean;
    pub fn pbsession_GetUintField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbuint;
    pub fn pbsession_GetUlongField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbulong;
    pub fn pbsession_GetBlobField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbblob;
    pub fn pbsession_GetDateField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdate;
    pub fn pbsession_GetTimeField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbtime;
    #[link_name = "pbsession_GetDateTimeField"]
    pub fn pbsession_GetDatetimeField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdatetime;
    pub fn pbsession_GetCharField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> PBChar;
    pub fn pbsession_GetByteField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbbyte;
    #[link_name = "pbsession_GetLongLongField"]
    pub fn pbsession_GetLonglongField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pblonglong;
    pub fn pbsession_GetObjectField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbobject;
    pub fn pbsession_GetArrayField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbarray;
    #[link_name = "pbsession_GetPBAnyField"]
    pub fn pbsession_GetAnyField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbvalue;

    pub fn pbsession_SetIntField(session: pbsession, obj: pbobject, fid: FieldId, value: pbint) -> PBXRESULT;
    pub fn pbsession_SetLongField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pblong
    ) -> PBXRESULT;
    pub fn pbsession_SetRealField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbreal
    ) -> PBXRESULT;
    pub fn pbsession_SetDoubleField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbdouble
    ) -> PBXRESULT;
    pub fn pbsession_SetDecField(session: pbsession, obj: pbobject, fid: FieldId, value: pbdec) -> PBXRESULT;
    pub fn pbsession_SetPBStringField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbstring
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetStringField"]
    pub fn pbsession_SetStrField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: LPCTSTR
    ) -> PBXRESULT;
    pub fn pbsession_SetStringField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: LPCTSTR
    ) -> PBXRESULT;
    pub fn pbsession_SetBoolField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_SetUintField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbuint
    ) -> PBXRESULT;
    pub fn pbsession_SetUlongField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbulong
    ) -> PBXRESULT;
    pub fn pbsession_SetBlobField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbblob
    ) -> PBXRESULT;
    pub fn pbsession_SetDateField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbdate
    ) -> PBXRESULT;
    pub fn pbsession_SetTimeField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbtime
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetDateTimeField"]
    pub fn pbsession_SetDatetimeField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbdatetime
    ) -> PBXRESULT;
    pub fn pbsession_SetCharField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: PBChar
    ) -> PBXRESULT;
    pub fn pbsession_SetByteField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbbyte
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetLongLongField"]
    pub fn pbsession_SetLonglongField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pblonglong
    ) -> PBXRESULT;
    pub fn pbsession_SetObjectField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbobject
    ) -> PBXRESULT;
    pub fn pbsession_SetArrayField(
        session: pbsession,
        obj: pbobject,
        fid: FieldId,
        value: pbarray
    ) -> PBXRESULT;

    pub fn pbsession_GetSharedVarID(session: pbsession, group: pbgroup, fieldName: LPCTSTR) -> FieldId;
    pub fn pbsession_GetSharedVarType(session: pbsession, group: pbgroup, fid: FieldId) -> ValueType;

    pub fn pbsession_IsSharedVarNull(session: pbsession, group: pbgroup, fid: FieldId) -> pbboolean;
    pub fn pbsession_SetSharedVarToNull(session: pbsession, group: pbgroup, fid: FieldId);
    pub fn pbsession_IsSharedVarArray(session: pbsession, group: pbgroup, fid: FieldId) -> pbboolean;
    pub fn pbsession_IsSharedVarObject(session: pbsession, group: pbgroup, fid: FieldId) -> pbboolean;

    pub fn pbsession_GetIntSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbint;
    pub fn pbsession_GetLongSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pblong;
    pub fn pbsession_GetRealSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbreal;
    pub fn pbsession_GetDoubleSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdouble;
    pub fn pbsession_GetDecSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdec;
    #[link_name = "pbsession_GetStringSharedVar"]
    pub fn pbsession_GetStrSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbstring;
    pub fn pbsession_GetStringSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbstring;
    pub fn pbsession_GetBoolSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbboolean;
    pub fn pbsession_GetUintSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbuint;
    pub fn pbsession_GetUlongSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbulong;
    pub fn pbsession_GetBlobSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbblob;
    pub fn pbsession_GetDateSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdate;
    pub fn pbsession_GetTimeSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbtime;
    #[link_name = "pbsession_GetDateTimeSharedVar"]
    pub fn pbsession_GetDatetimeSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbdatetime;
    pub fn pbsession_GetCharSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> PBChar;
    pub fn pbsession_GetByteSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbbyte;
    #[link_name = "pbsession_GetLongLongSharedVar"]
    pub fn pbsession_GetLonglongSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pblonglong;
    pub fn pbsession_GetObjectSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbobject;
    pub fn pbsession_GetArraySharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbarray;
    #[link_name = "pbsession_GetPBAnySharedVar"]
    pub fn pbsession_GetAnySharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        isNull: *mut pbboolean
    ) -> pbvalue;

    pub fn pbsession_SetIntSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbint
    ) -> PBXRESULT;
    pub fn pbsession_SetLongSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pblong
    ) -> PBXRESULT;
    pub fn pbsession_SetRealSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbreal
    ) -> PBXRESULT;
    pub fn pbsession_SetDoubleSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbdouble
    ) -> PBXRESULT;
    pub fn pbsession_SetDecSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbdec
    ) -> PBXRESULT;
    pub fn pbsession_SetPBStringSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbstring
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetStringSharedVar"]
    pub fn pbsession_SetStrSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: LPCTSTR
    ) -> PBXRESULT;
    pub fn pbsession_SetStringSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: LPCTSTR
    ) -> PBXRESULT;
    pub fn pbsession_SetBoolSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_SetUintSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbuint
    ) -> PBXRESULT;
    pub fn pbsession_SetUlongSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbulong
    ) -> PBXRESULT;
    pub fn pbsession_SetBlobSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbblob
    ) -> PBXRESULT;
    pub fn pbsession_SetDateSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbdate
    ) -> PBXRESULT;
    pub fn pbsession_SetTimeSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbtime
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetDateTimeSharedVar"]
    pub fn pbsession_SetDatetimeSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbdatetime
    ) -> PBXRESULT;
    pub fn pbsession_SetCharSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: PBChar
    ) -> PBXRESULT;
    pub fn pbsession_SetByteSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbbyte
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetLongLongSharedVar"]
    pub fn pbsession_SetLonglongSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pblonglong
    ) -> PBXRESULT;
    pub fn pbsession_SetObjectSharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbobject
    ) -> PBXRESULT;
    pub fn pbsession_SetArraySharedVar(
        session: pbsession,
        group: pbgroup,
        fid: FieldId,
        value: pbarray
    ) -> PBXRESULT;

    /*
        Interface to access Array
    */

    pub fn pbsession_NewBoundedSimpleArray(
        session: pbsession,
        r#type: ValueType,
        dimensions: pbuint,
        bounds: *const ArrayBound
    ) -> Option<pbarray>;
    pub fn pbsession_NewUnboundedSimpleArray(session: pbsession, r#type: ValueType) -> Option<pbarray>;
    pub fn pbsession_NewBoundedObjectArray(
        session: pbsession,
        cls: pbclass,
        dimensions: pbuint,
        bounds: *const ArrayBound
    ) -> Option<pbarray>;
    pub fn pbsession_NewUnboundedObjectArray(session: pbsession, cls: pbclass) -> Option<pbarray>;

    pub fn pbsession_GetArrayLength(session: pbsession, array: pbarray) -> pblong;
    pub fn pbsession_GetArrayInfo(session: pbsession, array: pbarray) -> pbarrayinfo;
    pub fn pbsession_ReleaseArrayInfo(session: pbsession, arrInfo: pbarrayinfo) -> PBXRESULT;

    pub fn pbsession_IsArrayItemNull(session: pbsession, array: pbarray, dim: *const pblong) -> pbboolean;
    pub fn pbsession_SetArrayItemToNull(session: pbsession, array: pbarray, dim: *const pblong);
    pub fn pbsession_GetArrayItemType(session: pbsession, array: pbarray, dim: *const pblong) -> ValueType;

    pub fn pbsession_GetIntArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbint;
    pub fn pbsession_GetLongArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pblong;
    pub fn pbsession_GetRealArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbreal;
    pub fn pbsession_GetDoubleArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbdouble;
    pub fn pbsession_GetDecArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbdec;
    #[link_name = "pbsession_GetStringArrayItem"]
    pub fn pbsession_GetStrArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbstring;
    pub fn pbsession_GetStringArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbstring;
    pub fn pbsession_GetBoolArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbboolean;
    pub fn pbsession_GetUintArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbuint;
    pub fn pbsession_GetUlongArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbulong;
    pub fn pbsession_GetBlobArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbblob;
    pub fn pbsession_GetDateArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbdate;
    pub fn pbsession_GetTimeArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbtime;
    #[link_name = "pbsession_GetDateTimeArrayItem"]
    pub fn pbsession_GetDatetimeArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbdatetime;
    pub fn pbsession_GetCharArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> PBChar;
    #[link_name = "pbsession_GetLongLongArrayItem"]
    pub fn pbsession_GetLonglongArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pblonglong;
    pub fn pbsession_GetObjectArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbobject;
    pub fn pbsession_GetByteArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbbyte;
    #[link_name = "pbsession_GetPBAnyArrayItem"]
    pub fn pbsession_GetAnyArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        isNull: *mut pbboolean
    ) -> pbvalue;

    pub fn pbsession_SetIntArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbint
    ) -> PBXRESULT;
    pub fn pbsession_SetLongArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pblong
    ) -> PBXRESULT;
    pub fn pbsession_SetRealArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbreal
    ) -> PBXRESULT;
    pub fn pbsession_SetDoubleArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbdouble
    ) -> PBXRESULT;
    pub fn pbsession_SetDecArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbdec
    ) -> PBXRESULT;
    pub fn pbsession_SetPBStringArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbstring
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetStringArrayItem"]
    pub fn pbsession_SetStrArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: LPCTSTR
    ) -> PBXRESULT;
    pub fn pbsession_SetStringArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: LPCTSTR
    ) -> PBXRESULT;
    pub fn pbsession_SetBoolArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbboolean
    ) -> PBXRESULT;
    pub fn pbsession_SetUintArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbuint
    ) -> PBXRESULT;
    pub fn pbsession_SetUlongArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbulong
    ) -> PBXRESULT;
    pub fn pbsession_SetBlobArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbblob
    ) -> PBXRESULT;
    pub fn pbsession_SetDateArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbdate
    ) -> PBXRESULT;
    pub fn pbsession_SetTimeArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbtime
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetDateTimeArrayItem"]
    pub fn pbsession_SetDatetimeArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbdatetime
    ) -> PBXRESULT;
    pub fn pbsession_SetCharArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: PBChar
    ) -> PBXRESULT;
    #[link_name = "pbsession_SetLongLongArrayItem"]
    pub fn pbsession_SetLonglongArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pblonglong
    ) -> PBXRESULT;
    pub fn pbsession_SetObjectArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbobject
    ) -> PBXRESULT;
    pub fn pbsession_SetByteArrayItem(
        session: pbsession,
        array: pbarray,
        dim: *const pblong,
        value: pbbyte
    ) -> PBXRESULT;

    /*
        Interface to access String
    */

    pub fn pbsession_NewString(session: pbsession, str: LPCTSTR) -> pbstring;
    pub fn pbsession_SetString(session: pbsession, pbstr: pbstring, str: LPCTSTR) -> PBXRESULT;
    pub fn pbsession_GetStringLength(session: pbsession, pbstr: pbstring) -> pblong;
    pub fn pbsession_GetString(session: pbsession, pbstr: pbstring) -> LPCTSTR;
    pub fn pbsession_ReleaseString(session: pbsession, str: LPCTSTR);

    /*
        Interface to access Binary
    */

    pub fn pbsession_NewBlob(session: pbsession, bin: *const c_void, len: pblong) -> pbblob;
    pub fn pbsession_SetBlob(session: pbsession, pbbin: pbblob, bin: *const c_void, len: pblong)
    -> PBXRESULT;
    pub fn pbsession_GetBlobLength(session: pbsession, pbbin: pbblob) -> pblong;
    pub fn pbsession_GetBlob(session: pbsession, pbbin: pbblob) -> *const c_void;

    /*
        Interface to access Date, Time and DateTime
    */

    pub fn pbsession_NewDate(session: pbsession) -> pbdate;
    pub fn pbsession_NewTime(session: pbsession) -> pbtime;
    pub fn pbsession_NewDateTime(session: pbsession) -> pbdatetime;
    pub fn pbsession_SetDate(
        session: pbsession,
        date: pbdate,
        year: pbint,
        month: pbint,
        day: pbint
    ) -> PBXRESULT;
    pub fn pbsession_SetTime(
        session: pbsession,
        time: pbtime,
        hour: pbint,
        minute: pbint,
        second: pbdouble
    ) -> PBXRESULT;
    pub fn pbsession_SetDateTime(
        session: pbsession,
        dt: pbdatetime,
        year: pbint,
        month: pbint,
        day: pbint,
        hour: pbint,
        minute: pbint,
        second: pbdouble
    ) -> PBXRESULT;
    pub fn pbsession_CopyDateTime(session: pbsession, dest: pbdatetime, src: pbdatetime) -> PBXRESULT;

    pub fn pbsession_SplitDate(
        session: pbsession,
        date: pbdate,
        year: *mut pbint,
        month: *mut pbint,
        day: *mut pbint
    ) -> PBXRESULT;
    pub fn pbsession_SplitTime(
        session: pbsession,
        time: pbtime,
        hour: *mut pbint,
        minute: *mut pbint,
        second: *mut pbdouble
    ) -> PBXRESULT;
    pub fn pbsession_SplitDateTime(
        session: pbsession,
        dt: pbdatetime,
        year: *mut pbint,
        month: *mut pbint,
        day: *mut pbint,
        hour: *mut pbint,
        minute: *mut pbint,
        second: *mut pbdouble
    ) -> PBXRESULT;
    pub fn pbsession_GetDateString(session: pbsession, dt: pbdate) -> LPCTSTR;
    pub fn pbsession_ReleaseDateString(session: pbsession, str: LPCTSTR);
    pub fn pbsession_GetTimeString(session: pbsession, time: pbtime) -> LPCTSTR;
    pub fn pbsession_ReleaseTimeString(session: pbsession, str: LPCTSTR);
    pub fn pbsession_GetDateTimeString(session: pbsession, dt: pbdatetime) -> LPCTSTR;
    pub fn pbsession_ReleaseDateTimeString(session: pbsession, str: LPCTSTR);

    /*
        Interface to access Decimal number
    */

    pub fn pbsession_NewDecimal(session: pbsession) -> pbdec;
    pub fn pbsession_SetDecimal(session: pbsession, dec: pbdec, dec_str: LPCTSTR) -> PBXRESULT;
    pub fn pbsession_GetDecimalString(session: pbsession, dec: pbdec) -> LPCTSTR;
    pub fn pbsession_ReleaseDecimalString(session: pbsession, str: LPCTSTR);

    /*
        Interface to access Value
    */

    pub fn pbsession_AcquireValue(session: pbsession, value: pbvalue) -> pbvalue;
    pub fn pbsession_AcquireArrayItemValue(session: pbsession, arr: pbarray, dim: *const pblong) -> pbvalue;
    pub fn pbsession_SetValue(session: pbsession, dest: pbvalue, src: pbvalue);
    pub fn pbsession_SetArrayItemValue(session: pbsession, arr: pbarray, dim: *const pblong, src: pbvalue);
    pub fn pbsession_ReleaseValue(session: pbsession, value: pbvalue);

}

// Arguments
extern "C" {
    pub fn pbargs_GetCount(pArgs: pbarguments) -> pbint;
    pub fn pbargs_GetAt(pArgs: pbarguments, index: pbint) -> pbvalue;
}

// Value
extern "C" {
    pub fn pbvalue_GetClass(pVal: pbvalue) -> Option<pbclass>;
    pub fn pbvalue_GetType(pVal: pbvalue) -> pbuint;
    pub fn pbvalue_IsArray(pVal: pbvalue) -> pbboolean;
    pub fn pbvalue_IsObject(pVal: pbvalue) -> pbboolean;
    pub fn pbvalue_IsEnum(pVal: pbvalue) -> pbboolean;
    pub fn pbvalue_IsByRef(pVal: pbvalue) -> pbboolean;

    pub fn pbvalue_IsNull(pVal: pbvalue) -> pbboolean;
    pub fn pbvalue_SetToNull(pVal: pbvalue) -> PBXRESULT;

    pub fn pbvalue_GetInt(pVal: pbvalue) -> pbint;
    pub fn pbvalue_GetUint(pVal: pbvalue) -> pbuint;
    pub fn pbvalue_GetBool(pVal: pbvalue) -> pbboolean;
    pub fn pbvalue_GetLong(pVal: pbvalue) -> pblong;
    pub fn pbvalue_GetUlong(pVal: pbvalue) -> pbulong;
    pub fn pbvalue_GetReal(pVal: pbvalue) -> pbreal;
    pub fn pbvalue_GetDouble(pVal: pbvalue) -> pbdouble;
    #[link_name = "pbvalue_GetDecimal"]
    pub fn pbvalue_GetDec(pVal: pbvalue) -> pbdec;
    pub fn pbvalue_GetChar(pVal: pbvalue) -> PBChar;
    #[link_name = "pbvalue_GetString"]
    pub fn pbvalue_GetStr(pVal: pbvalue) -> pbstring;
    pub fn pbvalue_GetString(pVal: pbvalue) -> pbstring;
    pub fn pbvalue_GetObject(pVal: pbvalue) -> pbobject;
    pub fn pbvalue_GetArray(pVal: pbvalue) -> pbarray;
    pub fn pbvalue_GetTime(pVal: pbvalue) -> pbtime;
    pub fn pbvalue_GetDate(pVal: pbvalue) -> pbdate;
    #[link_name = "pbvalue_GetDateTime"]
    pub fn pbvalue_GetDatetime(pVal: pbvalue) -> pbdatetime;
    #[link_name = "pbvalue_GetLongLong"]
    pub fn pbvalue_GetLonglong(pVal: pbvalue) -> pblonglong;
    pub fn pbvalue_GetBlob(pVal: pbvalue) -> pbblob;

    pub fn pbvalue_SetInt(pVal: pbvalue, v: pbint) -> PBXRESULT;
    pub fn pbvalue_SetUint(pVal: pbvalue, v: pbuint) -> PBXRESULT;
    pub fn pbvalue_SetBool(pVal: pbvalue, v: pbboolean) -> PBXRESULT;
    pub fn pbvalue_SetLong(pVal: pbvalue, v: pblong) -> PBXRESULT;
    pub fn pbvalue_SetUlong(pVal: pbvalue, v: pbulong) -> PBXRESULT;
    pub fn pbvalue_SetReal(pVal: pbvalue, v: pbreal) -> PBXRESULT;
    pub fn pbvalue_SetDouble(pVal: pbvalue, v: pbdouble) -> PBXRESULT;
    #[link_name = "pbvalue_SetDecimal"]
    pub fn pbvalue_SetDec(pVal: pbvalue, v: pbdec) -> PBXRESULT;
    pub fn pbvalue_SetChar(pVal: pbvalue, v: PBChar) -> PBXRESULT;
    pub fn pbvalue_SetPBString(pVal: pbvalue, v: pbstring) -> PBXRESULT;
    #[link_name = "pbvalue_SetString"]
    pub fn pbvalue_SetStr(pVal: pbvalue, v: LPCTSTR) -> PBXRESULT;
    pub fn pbvalue_SetString(pVal: pbvalue, v: LPCTSTR) -> PBXRESULT;
    pub fn pbvalue_SetArray(pVal: pbvalue, v: pbarray) -> PBXRESULT;
    pub fn pbvalue_SetTime(pVal: pbvalue, v: pbtime) -> PBXRESULT;
    pub fn pbvalue_SetDate(pVal: pbvalue, v: pbdate) -> PBXRESULT;
    #[link_name = "pbvalue_SetDateTime"]
    pub fn pbvalue_SetDatetime(pVal: pbvalue, v: pbdatetime) -> PBXRESULT;
    #[link_name = "pbvalue_SetLongLong"]
    pub fn pbvalue_SetLonglong(pVal: pbvalue, v: pblonglong) -> PBXRESULT;
    pub fn pbvalue_SetBlob(pVal: pbvalue, v: pbblob) -> PBXRESULT;
    pub fn pbvalue_SetObject(pVal: pbvalue, v: pbobject) -> PBXRESULT;

    pub fn pbvalue_IsReadOnly(pVal: pbvalue) -> pbboolean;

    pub fn pbvalue_GetByte(pVal: pbvalue) -> pbbyte;
    pub fn pbvalue_SetByte(pVal: pbvalue, v: pbbyte) -> PBXRESULT;
}
