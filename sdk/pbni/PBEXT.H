//**************************************************************************
//
//                            Copyright 2002
//                              Sybase Inc.
//
//    ALL RIGHTS RESERVED.  NO PORTION OF THIS MATERIAL MAY BE COPIED
//    IN ANY WAY WITHOUT PRIOR WRITTEN CONSENT FROM COMPUTER SOLUTIONS
//    INCORPORATED.
//
// ------------------------------------------------------------------------
//
//    Filename :    pbext.h
//
//    Author   :    PB kernel team
//
//    Purpose  :    structures and interfaces used to implement PB Extensions.
//
//****************************************************************************

#ifndef PBPSPP_H
#define PBPSPP_H

#include "pbni.h"
#ifndef PBOS_UNIX
#include "pbevtid.h"
#endif

//****************************************************************************
//  Interfaces should be implemented by PB Extensions
//****************************************************************************
struct IPBX_UserObject
{
    virtual PBXRESULT Invoke(
        IPB_Session *session,
        pbobject obj,
        pbmethodID mid,
        PBCallInfo *ci) = 0;

    virtual void Destroy() = 0;
    virtual ~IPBX_UserObject(){};
};

//****************************************************************************
//  IPBX_NonVisualObject
//  The IPBX_NonVisualObject interface needs to be implemented by the native
//  classes that inherit from nonvisualobject.
//****************************************************************************
struct IPBX_NonVisualObject : public IPBX_UserObject
{
};

// A duplicate of PB internal definition
#define PB_NULL -1
//****************************************************************************
//  IPBX_VisualObject
//  The IPBX_VisualObject interface needs to be implemented by the native
//  classes that inherit from userobject.
//****************************************************************************
struct IPBX_VisualObject : public IPBX_UserObject
{
    virtual LPCTSTR GetWindowClassName() = 0;

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
        ) = 0;

    virtual int GetEventID(
        HWND hWnd,     /* Handle to owner window */
        UINT iMsg,     /* Owner message */
        WPARAM wParam, /* Owner word parm */
        LPARAM lParam  /* Owner long parm */
    )
    {
        return PB_NULL;
    }
};

//****************************************************************************
//  IPBX_Marshaler
//  Each marshaler extension should have at least one class implementing
//  the IPBX_Marshaler interface.
//****************************************************************************
struct IPBX_Marshaler
{
    virtual PBXRESULT InvokeRemoteMethod(
        IPB_Session *session,
        pbproxyObject obj,
        LPCTSTR methodDesc,
        PBCallInfo *ci) = 0;

    virtual pbulong GetModuleHandle() = 0;
    virtual void Destroy() = 0;

protected:
    virtual ~IPBX_Marshaler(){};
};

//****************************************************************************
//  pbborder_style
//****************************************************************************
enum pbborder_style
{
    pbborderstyle_none = 0,
    pbborderstyle_shadowbox = 1,
    pbborderstyle_box = 2,
    pbborderstyle_lowered = 5,
    pbborderstyle_raised = 6
};

//****************************************************************************
//  PBX_DrawItemStruct
//  The structure contains the properties of a external visual control.
//****************************************************************************
struct PBX_DrawItemStruct
{
    int x;
    int y;
    int width;
    int height;
    LPCTSTR objectName;
    LPCTSTR tag;
    BOOL enabled;
    BOOL visible;
    pbborder_style borderStyle;
    DWORD backColor;
};

//****************************************************************************
//  Functions should be exported by an PBExtension DLL
//****************************************************************************
extern "C"
{

    PBXEXPORT LPCTSTR PBXCALL PBX_GetDescription();

    PBXEXPORT PBXRESULT PBXCALL PBX_CreateNonVisualObject(
        IPB_Session *pbsession,
        pbobject pbobj,
        LPCTSTR className,
        IPBX_NonVisualObject **obj);

    PBXEXPORT PBXRESULT PBXCALL PBX_CreateVisualObject(
        IPB_Session *pbsession,
        pbobject pbobj,
        LPCTSTR className,
        IPBX_VisualObject **obj);

    PBXEXPORT PBXRESULT PBXCALL PBX_InvokeGlobalFunction(
        IPB_Session *pbsession,
        LPCTSTR functionName,
        PBCallInfo *ci);

    PBXEXPORT PBXRESULT PBXCALL PBX_DrawVisualObject(
        HDC hDC,
        LPCTSTR className,
        const PBX_DrawItemStruct &property);

    const pbint kAfterDllLoaded = 1;
    const pbint kBeforeDllUnloaded = 2;

    PBXEXPORT PBXRESULT PBXCALL PBX_Notify(
        IPB_Session *pbsession,
        pbint reasonForCall);

    //****************************************************************************
    //  PBX_GetVersion Function indicate the version information of PBExtension DLL
    //****************************************************************************

    const pbint kPBVMVersion = 0x0a00;

#if defined(UNICODE) || defined(_UNICODE)
    const pbint kUnicodeEnabled = 1;
#else
    const pbint kUnicodeEnabled = 0;
#endif

    PBXEXPORT inline long PBXCALL PBX_GetVersion()
    {
        return MAKELONG(kPBVMVersion, kUnicodeEnabled);
    };
}

#endif
