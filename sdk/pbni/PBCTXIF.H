//**************************************************************************
//
//                            Copyright 2003
//                              Sybase Inc.
//
//								Copyright © 2003
//						Sybase, Inc. and its subsidiaries.
//	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-
//	Sybase, Inc.("Sybase") claims copyright in this program and documentation
//	as an unpublished work, versions of which were first licensed on the date
//	indicated in the foregoing notice. This claim of copyright does not imply
//	waiver of Sybase's other rights.
//	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-	-
//
//    Filename :	pbctxif.h
//
//    Author   :	PB kernel team
//
//    Purpose  : 	structures and interfaces for PowerBuilder Native Interface
//
//****************************************************************************

#ifndef __PB_CTX_IF_H__
#define __PB_CTX_IF_H__

#ifdef PBOS_UNIX
#include "oswindow.h"
#endif

#include "pbnimd.h"

#ifndef PBOS_UNIX
#define INTERFACE_FUN_CALL 	__cdecl
#define EXPORT_FUN_CALL 	__stdcall
#else
#define INTERFACE_FUN_CALL
#define EXPORT_FUN_CALL
#define INT			int
#define LONG			long
#endif

#ifndef PB_TXNSERVER_TYPE_DEFINED
#define PB_TXNSERVER_TYPE_DEFINED
typedef enum
{
	PB_TXNSERVER_NONE		= 0,
	PB_TXNSERVER_JAGUAR		= 1,
	PB_TXNSERVER_MTS		= 2,
	PB_TXNSERVER_EJB 		= 3,
	PB_TXNSERVER_NORMAL_APP = 4,
	PB_TXNSERVER_SERVER 	= 5
} PB_TXNSERVER_TYPE;
#endif

enum pbcontextservice_type
{
	PB_TRANSACTION_SERVICE = 1,
	PB_SECURITY_SERVICE,
	PB_LOGGING_SERVICE,
	PB_CONNECTION_CACHE_SERVICE,
	PB_CONNECTION_CACHE_C_SERVICE,
	PB_DEBUG_SERVICE
};

struct PBServiceVersionInfo
{
	LPTSTR provider;
	short	majorVersion;
	short	minorVersion;
};

struct IPBX_ContextService
{
public:
	virtual INT	INTERFACE_FUN_CALL Which() = 0;
	virtual const PBServiceVersionInfo*	INTERFACE_FUN_CALL GetVersion() = 0;

public: // leave the destructor public to permit delete operation on base interface
	virtual ~IPBX_ContextService(){};
};

extern "C"
{
PBXEXPORT IPBX_ContextService* EXPORT_FUN_CALL PB_SetContextService
(
	pbcontextservice_type type,
	IPBX_ContextService*	service
);

PBXEXPORT IPBX_ContextService* EXPORT_FUN_CALL PB_GetContextService
(
	pbcontextservice_type type
);
}


inline INT PB_GetServiceProviderType
(
	pbcontextservice_type type
)
{
	IPBX_ContextService* pService = PB_GetContextService(type);
	if (pService)
		return pService->Which();
	else
		return (INT)PB_TXNSERVER_NONE;
}

inline bool PB_HasExternalTransactionService()
{
	return PB_GetServiceProviderType(PB_TRANSACTION_SERVICE) > (INT)PB_TXNSERVER_NONE;
}
inline bool PB_HasExternalConnectionCacheService()
{
	return PB_GetServiceProviderType(PB_CONNECTION_CACHE_SERVICE) > (INT)PB_TXNSERVER_NONE;
}
inline bool PB_HasExternalLoggingService()
{
	return PB_GetServiceProviderType(PB_LOGGING_SERVICE) > (INT)PB_TXNSERVER_NONE;
}
inline bool PB_HasExternalSecurityService()
{
	return PB_GetServiceProviderType(PB_SECURITY_SERVICE) > (INT)PB_TXNSERVER_NONE;
}
inline bool PB_NoUserInteractionNeeded()
{
	PB_TXNSERVER_TYPE nType = (PB_TXNSERVER_TYPE)PB_GetServiceProviderType(PB_TRANSACTION_SERVICE);

	return ((nType == PB_TXNSERVER_JAGUAR) ||
		 	(nType == PB_TXNSERVER_MTS) ||
		 	(nType == PB_TXNSERVER_EJB) ||
		 	(nType == PB_TXNSERVER_SERVER));
}



struct IPBX_TransactionService : public IPBX_ContextService
{
	virtual INT	INTERFACE_FUN_CALL CommitWork() = 0;
	virtual INT	INTERFACE_FUN_CALL AbortWork() = 0;
	virtual INT	INTERFACE_FUN_CALL EnableCommit() = 0;
	virtual INT	INTERFACE_FUN_CALL DisableCommit() = 0;
	virtual BOOL INTERFACE_FUN_CALL IsInTransaction() = 0;
	virtual BOOL INTERFACE_FUN_CALL IsTransactionAborted() = 0;
};

struct IPBX_LoggingService : public IPBX_ContextService
{
	virtual INT INTERFACE_FUN_CALL Log(LPCTSTR message) = 0;
};


struct IPB_Session;
struct _pbobject;
typedef struct _pbobject *pbobject;

struct IPBX_SecurityService : public IPBX_ContextService
{
	virtual BOOL INTERFACE_FUN_CALL IsCallerInRole(LPCTSTR roleName) = 0;
	virtual INT INTERFACE_FUN_CALL ImpersonateClient() = 0;
	virtual INT INTERFACE_FUN_CALL RevertToSelf() = 0;
	virtual BOOL INTERFACE_FUN_CALL IsImpersonating() = 0;
	virtual BOOL INTERFACE_FUN_CALL IsSecurityEnabled() = 0;

	virtual LONG INTERFACE_FUN_CALL CreateInstance(IPB_Session* session, pbobject* objRef, LPCTSTR clsName) = 0;
	virtual LONG INTERFACE_FUN_CALL Lookup(IPB_Session* session, pbobject* objRef, LPCTSTR clsName) = 0;
};


struct PBConnectionCacheProperty
{
	LPTSTR d_key, d_value;

	PBConnectionCacheProperty():d_key(0), d_value(0){};
	~PBConnectionCacheProperty(){delete d_key; delete d_value;};
};

struct _pb_db_connection {};
typedef struct _pb_db_connection *pb_db_connection;
const pb_db_connection INVALID_CONNECTION = 0;

struct IPBX_ConnectionCacheService : public IPBX_ContextService
{
	virtual pb_db_connection INTERFACE_FUN_CALL GetConnection(LPCTSTR dbms, const PBConnectionCacheProperty* props, int numProps) = 0;
	virtual int INTERFACE_FUN_CALL ReleaseConnection(pb_db_connection dbConn, const PBConnectionCacheProperty* props = 0, int numProps = 0) = 0;
	virtual LPCTSTR INTERFACE_FUN_CALL GetError() = 0;
	virtual BOOL IsConnectionAvailable(LPCTSTR dbms, BOOL isCache) = 0;
};

#endif // __PB_CTX_IF_H__
