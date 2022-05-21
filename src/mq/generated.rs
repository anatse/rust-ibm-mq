#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

// Copied from include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const MQAIR_STRUC_ID: &'static [u8; 5usize] = b"AIR \0";
pub const MQAIR_VERSION_1: u32 = 1;
pub const MQAIR_VERSION_2: u32 = 2;
pub const MQAIR_CURRENT_VERSION: u32 = 2;
pub const MQAIR_LENGTH_1: u32 = 328;
pub const MQAIR_LENGTH_2: u32 = 584;
pub const MQAIR_CURRENT_LENGTH: u32 = 584;
pub const MQAIT_ALL: u32 = 0;
pub const MQAIT_CRL_LDAP: u32 = 1;
pub const MQAIT_OCSP: u32 = 2;
pub const MQAIT_IDPW_OS: u32 = 3;
pub const MQAIT_IDPW_LDAP: u32 = 4;
pub const MQBMHO_STRUC_ID: &'static [u8; 5usize] = b"BMHO\0";
pub const MQBMHO_VERSION_1: u32 = 1;
pub const MQBMHO_CURRENT_VERSION: u32 = 1;
pub const MQBMHO_LENGTH_1: u32 = 12;
pub const MQBMHO_CURRENT_LENGTH: u32 = 12;
pub const MQBMHO_NONE: u32 = 0;
pub const MQBMHO_DELETE_PROPERTIES: u32 = 1;
pub const MQBO_STRUC_ID: &'static [u8; 5usize] = b"BO  \0";
pub const MQBO_VERSION_1: u32 = 1;
pub const MQBO_CURRENT_VERSION: u32 = 1;
pub const MQBO_LENGTH_1: u32 = 12;
pub const MQBO_CURRENT_LENGTH: u32 = 12;
pub const MQBO_NONE: u32 = 0;
pub const MQCBC_STRUC_ID: &'static [u8; 5usize] = b"CBC \0";
pub const MQCBC_VERSION_1: u32 = 1;
pub const MQCBC_VERSION_2: u32 = 2;
pub const MQCBC_CURRENT_VERSION: u32 = 2;
pub const MQCBC_LENGTH_1: u32 = 56;
pub const MQCBC_LENGTH_2: u32 = 64;
pub const MQCBC_CURRENT_LENGTH: u32 = 64;
pub const MQCBCF_NONE: u32 = 0;
pub const MQCBCF_READA_BUFFER_EMPTY: u32 = 1;
pub const MQCBCT_START_CALL: u32 = 1;
pub const MQCBCT_STOP_CALL: u32 = 2;
pub const MQCBCT_REGISTER_CALL: u32 = 3;
pub const MQCBCT_DEREGISTER_CALL: u32 = 4;
pub const MQCBCT_EVENT_CALL: u32 = 5;
pub const MQCBCT_MSG_REMOVED: u32 = 6;
pub const MQCBCT_MSG_NOT_REMOVED: u32 = 7;
pub const MQCBCT_MC_EVENT_CALL: u32 = 8;
pub const MQCS_NONE: u32 = 0;
pub const MQCS_SUSPENDED_TEMPORARY: u32 = 1;
pub const MQCS_SUSPENDED_USER_ACTION: u32 = 2;
pub const MQCS_SUSPENDED: u32 = 3;
pub const MQCS_STOPPED: u32 = 4;
pub const MQRD_NO_RECONNECT: i32 = -1;
pub const MQRD_NO_DELAY: u32 = 0;
pub const MQCBD_STRUC_ID: &'static [u8; 5usize] = b"CBD \0";
pub const MQCBD_VERSION_1: u32 = 1;
pub const MQCBD_CURRENT_VERSION: u32 = 1;
pub const MQCBD_LENGTH_1: u32 = 168;
pub const MQCBD_CURRENT_LENGTH: u32 = 168;
pub const MQCBDO_NONE: u32 = 0;
pub const MQCBDO_START_CALL: u32 = 1;
pub const MQCBDO_STOP_CALL: u32 = 4;
pub const MQCBDO_REGISTER_CALL: u32 = 256;
pub const MQCBDO_DEREGISTER_CALL: u32 = 512;
pub const MQCBDO_FAIL_IF_QUIESCING: u32 = 8192;
pub const MQCBDO_EVENT_CALL: u32 = 16384;
pub const MQCBDO_MC_EVENT_CALL: u32 = 32768;
pub const MQCBT_MESSAGE_CONSUMER: u32 = 1;
pub const MQCBT_EVENT_HANDLER: u32 = 2;
pub const MQCBD_FULL_MSG_LENGTH: i32 = -1;
pub const MQVS_NULL_TERMINATED: i32 = -1;
pub const MQCIH_STRUC_ID: &'static [u8; 5usize] = b"CIH \0";
pub const MQCIH_VERSION_1: u32 = 1;
pub const MQCIH_VERSION_2: u32 = 2;
pub const MQCIH_CURRENT_VERSION: u32 = 2;
pub const MQCIH_LENGTH_1: u32 = 164;
pub const MQCIH_LENGTH_2: u32 = 180;
pub const MQCIH_CURRENT_LENGTH: u32 = 180;
pub const MQCIH_NONE: u32 = 0;
pub const MQCIH_PASS_EXPIRATION: u32 = 1;
pub const MQCIH_UNLIMITED_EXPIRATION: u32 = 0;
pub const MQCIH_REPLY_WITHOUT_NULLS: u32 = 2;
pub const MQCIH_REPLY_WITH_NULLS: u32 = 0;
pub const MQCIH_SYNC_ON_RETURN: u32 = 4;
pub const MQCIH_NO_SYNC_ON_RETURN: u32 = 0;
pub const MQCRC_OK: u32 = 0;
pub const MQCRC_CICS_EXEC_ERROR: u32 = 1;
pub const MQCRC_MQ_API_ERROR: u32 = 2;
pub const MQCRC_BRIDGE_ERROR: u32 = 3;
pub const MQCRC_BRIDGE_ABEND: u32 = 4;
pub const MQCRC_APPLICATION_ABEND: u32 = 5;
pub const MQCRC_SECURITY_ERROR: u32 = 6;
pub const MQCRC_PROGRAM_NOT_AVAILABLE: u32 = 7;
pub const MQCRC_BRIDGE_TIMEOUT: u32 = 8;
pub const MQCRC_TRANSID_NOT_AVAILABLE: u32 = 9;
pub const MQCUOWC_ONLY: u32 = 273;
pub const MQCUOWC_CONTINUE: u32 = 65536;
pub const MQCUOWC_FIRST: u32 = 17;
pub const MQCUOWC_MIDDLE: u32 = 16;
pub const MQCUOWC_LAST: u32 = 272;
pub const MQCUOWC_COMMIT: u32 = 256;
pub const MQCUOWC_BACKOUT: u32 = 4352;
pub const MQCGWI_DEFAULT: i32 = -2;
pub const MQCLT_PROGRAM: u32 = 1;
pub const MQCLT_TRANSACTION: u32 = 2;
pub const MQCODL_AS_INPUT: i32 = -1;
pub const MQCADSD_NONE: u32 = 0;
pub const MQCADSD_SEND: u32 = 1;
pub const MQCADSD_RECV: u32 = 16;
pub const MQCADSD_MSGFORMAT: u32 = 256;
pub const MQCCT_YES: u32 = 1;
pub const MQCCT_NO: u32 = 0;
pub const MQCTES_NOSYNC: u32 = 0;
pub const MQCTES_COMMIT: u32 = 256;
pub const MQCTES_BACKOUT: u32 = 4352;
pub const MQCTES_ENDTASK: u32 = 65536;
pub const MQCFAC_NONE: &'static [u8; 9usize] = b"\0\0\0\0\0\0\0\0\0";
pub const MQCFUNC_MQCONN: &'static [u8; 5usize] = b"CONN\0";
pub const MQCFUNC_MQGET: &'static [u8; 5usize] = b"GET \0";
pub const MQCFUNC_MQINQ: &'static [u8; 5usize] = b"INQ \0";
pub const MQCFUNC_MQOPEN: &'static [u8; 5usize] = b"OPEN\0";
pub const MQCFUNC_MQPUT: &'static [u8; 5usize] = b"PUT \0";
pub const MQCFUNC_MQPUT1: &'static [u8; 5usize] = b"PUT1\0";
pub const MQCFUNC_NONE: &'static [u8; 5usize] = b"    \0";
pub const MQCSC_START: &'static [u8; 5usize] = b"S   \0";
pub const MQCSC_STARTDATA: &'static [u8; 5usize] = b"SD  \0";
pub const MQCSC_TERMINPUT: &'static [u8; 5usize] = b"TD  \0";
pub const MQCSC_NONE: &'static [u8; 5usize] = b"    \0";
pub const MQCMHO_STRUC_ID: &'static [u8; 5usize] = b"CMHO\0";
pub const MQCMHO_VERSION_1: u32 = 1;
pub const MQCMHO_CURRENT_VERSION: u32 = 1;
pub const MQCMHO_LENGTH_1: u32 = 12;
pub const MQCMHO_CURRENT_LENGTH: u32 = 12;
pub const MQCMHO_DEFAULT_VALIDATION: u32 = 0;
pub const MQCMHO_NO_VALIDATION: u32 = 1;
pub const MQCMHO_VALIDATE: u32 = 2;
pub const MQCMHO_NONE: u32 = 0;
pub const MQCTLO_STRUC_ID: &'static [u8; 5usize] = b"CTLO\0";
pub const MQCTLO_VERSION_1: u32 = 1;
pub const MQCTLO_CURRENT_VERSION: u32 = 1;
pub const MQCTLO_LENGTH_1: u32 = 24;
pub const MQCTLO_CURRENT_LENGTH: u32 = 24;
pub const MQCTLO_NONE: u32 = 0;
pub const MQCTLO_THREAD_AFFINITY: u32 = 1;
pub const MQCTLO_FAIL_IF_QUIESCING: u32 = 8192;
pub const MQSCO_STRUC_ID: &'static [u8; 5usize] = b"SCO \0";
pub const MQSCO_VERSION_1: u32 = 1;
pub const MQSCO_VERSION_2: u32 = 2;
pub const MQSCO_VERSION_3: u32 = 3;
pub const MQSCO_VERSION_4: u32 = 4;
pub const MQSCO_VERSION_5: u32 = 5;
pub const MQSCO_CURRENT_VERSION: u32 = 5;
pub const MQSCO_LENGTH_1: u32 = 536;
pub const MQSCO_LENGTH_2: u32 = 544;
pub const MQSCO_LENGTH_3: u32 = 560;
pub const MQSCO_LENGTH_4: u32 = 568;
pub const MQSCO_LENGTH_5: u32 = 632;
pub const MQSCO_CURRENT_LENGTH: u32 = 632;
pub const MQ_SUITE_B_NOT_AVAILABLE: u32 = 0;
pub const MQ_SUITE_B_NONE: u32 = 1;
pub const MQ_SUITE_B_128_BIT: u32 = 2;
pub const MQ_SUITE_B_192_BIT: u32 = 4;
pub const MQSCO_RESET_COUNT_DEFAULT: u32 = 0;
pub const MQ_CERT_VAL_POLICY_DEFAULT: u32 = 0;
pub const MQ_CERT_VAL_POLICY_ANY: u32 = 0;
pub const MQ_CERT_VAL_POLICY_RFC5280: u32 = 1;
pub const MQCSP_STRUC_ID: &'static [u8; 5usize] = b"CSP \0";
pub const MQCSP_VERSION_1: u32 = 1;
pub const MQCSP_CURRENT_VERSION: u32 = 1;
pub const MQCSP_LENGTH_1: u32 = 56;
pub const MQCSP_CURRENT_LENGTH: u32 = 56;
pub const MQCSP_AUTH_NONE: u32 = 0;
pub const MQCSP_AUTH_USER_ID_AND_PWD: u32 = 1;
pub const MQCNO_STRUC_ID: &'static [u8; 5usize] = b"CNO \0";
pub const MQCNO_VERSION_1: u32 = 1;
pub const MQCNO_VERSION_2: u32 = 2;
pub const MQCNO_VERSION_3: u32 = 3;
pub const MQCNO_VERSION_4: u32 = 4;
pub const MQCNO_VERSION_5: u32 = 5;
pub const MQCNO_VERSION_6: u32 = 6;
pub const MQCNO_VERSION_7: u32 = 7;
pub const MQCNO_CURRENT_VERSION: u32 = 7;
pub const MQCNO_LENGTH_1: u32 = 12;
pub const MQCNO_LENGTH_2: u32 = 24;
pub const MQCNO_LENGTH_3: u32 = 152;
pub const MQCNO_LENGTH_4: u32 = 168;
pub const MQCNO_LENGTH_5: u32 = 200;
pub const MQCNO_LENGTH_6: u32 = 224;
pub const MQCNO_LENGTH_7: u32 = 256;
pub const MQCNO_CURRENT_LENGTH: u32 = 256;
pub const MQCNO_STANDARD_BINDING: u32 = 0;
pub const MQCNO_FASTPATH_BINDING: u32 = 1;
pub const MQCNO_SERIALIZE_CONN_TAG_Q_MGR: u32 = 2;
pub const MQCNO_SERIALIZE_CONN_TAG_QSG: u32 = 4;
pub const MQCNO_RESTRICT_CONN_TAG_Q_MGR: u32 = 8;
pub const MQCNO_RESTRICT_CONN_TAG_QSG: u32 = 16;
pub const MQCNO_HANDLE_SHARE_NONE: u32 = 32;
pub const MQCNO_HANDLE_SHARE_BLOCK: u32 = 64;
pub const MQCNO_HANDLE_SHARE_NO_BLOCK: u32 = 128;
pub const MQCNO_SHARED_BINDING: u32 = 256;
pub const MQCNO_ISOLATED_BINDING: u32 = 512;
pub const MQCNO_LOCAL_BINDING: u32 = 1024;
pub const MQCNO_CLIENT_BINDING: u32 = 2048;
pub const MQCNO_ACCOUNTING_MQI_ENABLED: u32 = 4096;
pub const MQCNO_ACCOUNTING_MQI_DISABLED: u32 = 8192;
pub const MQCNO_ACCOUNTING_Q_ENABLED: u32 = 16384;
pub const MQCNO_ACCOUNTING_Q_DISABLED: u32 = 32768;
pub const MQCNO_NO_CONV_SHARING: u32 = 65536;
pub const MQCNO_ALL_CONVS_SHARE: u32 = 262144;
pub const MQCNO_CD_FOR_OUTPUT_ONLY: u32 = 524288;
pub const MQCNO_USE_CD_SELECTION: u32 = 1048576;
pub const MQCNO_GENERATE_CONN_TAG: u32 = 2097152;
pub const MQCNO_RECONNECT_AS_DEF: u32 = 0;
pub const MQCNO_RECONNECT: u32 = 16777216;
pub const MQCNO_RECONNECT_DISABLED: u32 = 33554432;
pub const MQCNO_RECONNECT_Q_MGR: u32 = 67108864;
pub const MQCNO_ACTIVITY_TRACE_ENABLED: u32 = 134217728;
pub const MQCNO_ACTIVITY_TRACE_DISABLED: u32 = 268435456;
pub const MQCNO_NONE: u32 = 0;
pub const MQCT_NONE : & 'static [ u8 ; 129usize ] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" ;
pub const MQCONNID_NONE: &'static [u8; 25usize] =
    b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQAN_NONE: &'static [u8; 29usize] = b"                            \0";
pub const MQDH_STRUC_ID: &'static [u8; 5usize] = b"DH  \0";
pub const MQDH_VERSION_1: u32 = 1;
pub const MQDH_CURRENT_VERSION: u32 = 1;
pub const MQDH_LENGTH_1: u32 = 48;
pub const MQDH_CURRENT_LENGTH: u32 = 48;
pub const MQDHF_NEW_MSG_IDS: u32 = 1;
pub const MQDHF_NONE: u32 = 0;
pub const MQDLH_STRUC_ID: &'static [u8; 5usize] = b"DLH \0";
pub const MQDLH_VERSION_1: u32 = 1;
pub const MQDLH_CURRENT_VERSION: u32 = 1;
pub const MQDLH_LENGTH_1: u32 = 172;
pub const MQDLH_CURRENT_LENGTH: u32 = 172;
pub const MQDMHO_STRUC_ID: &'static [u8; 5usize] = b"DMHO\0";
pub const MQDMHO_VERSION_1: u32 = 1;
pub const MQDMHO_CURRENT_VERSION: u32 = 1;
pub const MQDMHO_LENGTH_1: u32 = 12;
pub const MQDMHO_CURRENT_LENGTH: u32 = 12;
pub const MQDMHO_NONE: u32 = 0;
pub const MQDMPO_STRUC_ID: &'static [u8; 5usize] = b"DMPO\0";
pub const MQDMPO_VERSION_1: u32 = 1;
pub const MQDMPO_CURRENT_VERSION: u32 = 1;
pub const MQDMPO_LENGTH_1: u32 = 12;
pub const MQDMPO_CURRENT_LENGTH: u32 = 12;
pub const MQDMPO_DEL_FIRST: u32 = 0;
pub const MQDMPO_DEL_PROP_UNDER_CURSOR: u32 = 1;
pub const MQDMPO_NONE: u32 = 0;
pub const MQGMO_STRUC_ID: &'static [u8; 5usize] = b"GMO \0";
pub const MQGMO_VERSION_1: u32 = 1;
pub const MQGMO_VERSION_2: u32 = 2;
pub const MQGMO_VERSION_3: u32 = 3;
pub const MQGMO_VERSION_4: u32 = 4;
pub const MQGMO_CURRENT_VERSION: u32 = 4;
pub const MQGMO_LENGTH_1: u32 = 72;
pub const MQGMO_LENGTH_2: u32 = 80;
pub const MQGMO_LENGTH_3: u32 = 100;
pub const MQGMO_LENGTH_4: u32 = 112;
pub const MQGMO_CURRENT_LENGTH: u32 = 112;
pub const MQGMO_WAIT: u32 = 1;
pub const MQGMO_NO_WAIT: u32 = 0;
pub const MQGMO_SET_SIGNAL: u32 = 8;
pub const MQGMO_FAIL_IF_QUIESCING: u32 = 8192;
pub const MQGMO_SYNCPOINT: u32 = 2;
pub const MQGMO_SYNCPOINT_IF_PERSISTENT: u32 = 4096;
pub const MQGMO_NO_SYNCPOINT: u32 = 4;
pub const MQGMO_MARK_SKIP_BACKOUT: u32 = 128;
pub const MQGMO_BROWSE_FIRST: u32 = 16;
pub const MQGMO_BROWSE_NEXT: u32 = 32;
pub const MQGMO_BROWSE_MSG_UNDER_CURSOR: u32 = 2048;
pub const MQGMO_MSG_UNDER_CURSOR: u32 = 256;
pub const MQGMO_LOCK: u32 = 512;
pub const MQGMO_UNLOCK: u32 = 1024;
pub const MQGMO_ACCEPT_TRUNCATED_MSG: u32 = 64;
pub const MQGMO_CONVERT: u32 = 16384;
pub const MQGMO_LOGICAL_ORDER: u32 = 32768;
pub const MQGMO_COMPLETE_MSG: u32 = 65536;
pub const MQGMO_ALL_MSGS_AVAILABLE: u32 = 131072;
pub const MQGMO_ALL_SEGMENTS_AVAILABLE: u32 = 262144;
pub const MQGMO_MARK_BROWSE_HANDLE: u32 = 1048576;
pub const MQGMO_MARK_BROWSE_CO_OP: u32 = 2097152;
pub const MQGMO_UNMARK_BROWSE_CO_OP: u32 = 4194304;
pub const MQGMO_UNMARK_BROWSE_HANDLE: u32 = 8388608;
pub const MQGMO_UNMARKED_BROWSE_MSG: u32 = 16777216;
pub const MQGMO_PROPERTIES_FORCE_MQRFH2: u32 = 33554432;
pub const MQGMO_NO_PROPERTIES: u32 = 67108864;
pub const MQGMO_PROPERTIES_IN_HANDLE: u32 = 134217728;
pub const MQGMO_PROPERTIES_COMPATIBILITY: u32 = 268435456;
pub const MQGMO_PROPERTIES_AS_Q_DEF: u32 = 0;
pub const MQGMO_NONE: u32 = 0;
pub const MQGMO_BROWSE_HANDLE: u32 = 17825808;
pub const MQGMO_BROWSE_CO_OP: u32 = 18874384;
pub const MQWI_UNLIMITED: i32 = -1;
pub const MQEC_MSG_ARRIVED: u32 = 2;
pub const MQEC_WAIT_INTERVAL_EXPIRED: u32 = 3;
pub const MQEC_WAIT_CANCELED: u32 = 4;
pub const MQEC_Q_MGR_QUIESCING: u32 = 5;
pub const MQEC_CONNECTION_QUIESCING: u32 = 6;
pub const MQMO_MATCH_MSG_ID: u32 = 1;
pub const MQMO_MATCH_CORREL_ID: u32 = 2;
pub const MQMO_MATCH_GROUP_ID: u32 = 4;
pub const MQMO_MATCH_MSG_SEQ_NUMBER: u32 = 8;
pub const MQMO_MATCH_OFFSET: u32 = 16;
pub const MQMO_MATCH_MSG_TOKEN: u32 = 32;
pub const MQMO_NONE: u32 = 0;
pub const MQGS_NOT_IN_GROUP: u8 = 32u8;
pub const MQGS_MSG_IN_GROUP: u8 = 71u8;
pub const MQGS_LAST_MSG_IN_GROUP: u8 = 76u8;
pub const MQSS_NOT_A_SEGMENT: u8 = 32u8;
pub const MQSS_SEGMENT: u8 = 83u8;
pub const MQSS_LAST_SEGMENT: u8 = 76u8;
pub const MQSEG_INHIBITED: u8 = 32u8;
pub const MQSEG_ALLOWED: u8 = 65u8;
pub const MQMTOK_NONE: &'static [u8; 17usize] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQRL_UNDEFINED: i32 = -1;
pub const MQIIH_STRUC_ID: &'static [u8; 5usize] = b"IIH \0";
pub const MQIIH_VERSION_1: u32 = 1;
pub const MQIIH_CURRENT_VERSION: u32 = 1;
pub const MQIIH_LENGTH_1: u32 = 84;
pub const MQIIH_CURRENT_LENGTH: u32 = 84;
pub const MQIIH_NONE: u32 = 0;
pub const MQIIH_PASS_EXPIRATION: u32 = 1;
pub const MQIIH_UNLIMITED_EXPIRATION: u32 = 0;
pub const MQIIH_REPLY_FORMAT_NONE: u32 = 8;
pub const MQIIH_IGNORE_PURG: u32 = 16;
pub const MQIIH_CM0_REQUEST_RESPONSE: u32 = 32;
pub const MQIAUT_NONE: &'static [u8; 9usize] = b"        \0";
pub const MQITII_NONE: &'static [u8; 17usize] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQITS_IN_CONVERSATION: u8 = 67u8;
pub const MQITS_NOT_IN_CONVERSATION: u8 = 32u8;
pub const MQITS_ARCHITECTED: u8 = 65u8;
pub const MQICM_COMMIT_THEN_SEND: u8 = 48u8;
pub const MQICM_SEND_THEN_COMMIT: u8 = 49u8;
pub const MQISS_CHECK: u8 = 67u8;
pub const MQISS_FULL: u8 = 70u8;
pub const MQIMPO_STRUC_ID: &'static [u8; 5usize] = b"IMPO\0";
pub const MQIMPO_VERSION_1: u32 = 1;
pub const MQIMPO_CURRENT_VERSION: u32 = 1;
pub const MQIMPO_LENGTH_1: u32 = 64;
pub const MQIMPO_CURRENT_LENGTH: u32 = 64;
pub const MQIMPO_CONVERT_TYPE: u32 = 2;
pub const MQIMPO_QUERY_LENGTH: u32 = 4;
pub const MQIMPO_INQ_FIRST: u32 = 0;
pub const MQIMPO_INQ_NEXT: u32 = 8;
pub const MQIMPO_INQ_PROP_UNDER_CURSOR: u32 = 16;
pub const MQIMPO_CONVERT_VALUE: u32 = 32;
pub const MQIMPO_NONE: u32 = 0;
pub const MQMD_STRUC_ID: &'static [u8; 5usize] = b"MD  \0";
pub const MQMD_VERSION_1: u32 = 1;
pub const MQMD_VERSION_2: u32 = 2;
pub const MQMD_CURRENT_VERSION: u32 = 2;
pub const MQMD_LENGTH_1: u32 = 324;
pub const MQMD_LENGTH_2: u32 = 364;
pub const MQMD_CURRENT_LENGTH: u32 = 364;
pub const MQRO_EXCEPTION: u32 = 16777216;
pub const MQRO_EXCEPTION_WITH_DATA: u32 = 50331648;
pub const MQRO_EXCEPTION_WITH_FULL_DATA: u32 = 117440512;
pub const MQRO_EXPIRATION: u32 = 2097152;
pub const MQRO_EXPIRATION_WITH_DATA: u32 = 6291456;
pub const MQRO_EXPIRATION_WITH_FULL_DATA: u32 = 14680064;
pub const MQRO_COA: u32 = 256;
pub const MQRO_COA_WITH_DATA: u32 = 768;
pub const MQRO_COA_WITH_FULL_DATA: u32 = 1792;
pub const MQRO_COD: u32 = 2048;
pub const MQRO_COD_WITH_DATA: u32 = 6144;
pub const MQRO_COD_WITH_FULL_DATA: u32 = 14336;
pub const MQRO_PAN: u32 = 1;
pub const MQRO_NAN: u32 = 2;
pub const MQRO_ACTIVITY: u32 = 4;
pub const MQRO_NEW_MSG_ID: u32 = 0;
pub const MQRO_PASS_MSG_ID: u32 = 128;
pub const MQRO_COPY_MSG_ID_TO_CORREL_ID: u32 = 0;
pub const MQRO_PASS_CORREL_ID: u32 = 64;
pub const MQRO_DEAD_LETTER_Q: u32 = 0;
pub const MQRO_DISCARD_MSG: u32 = 134217728;
pub const MQRO_PASS_DISCARD_AND_EXPIRY: u32 = 16384;
pub const MQRO_NONE: u32 = 0;
pub const MQRO_REJECT_UNSUP_MASK: u32 = 270270464;
pub const MQRO_ACCEPT_UNSUP_MASK: u32 = 4024434943;
pub const MQRO_ACCEPT_UNSUP_IF_XMIT_MASK: u32 = 261888;
pub const MQMT_SYSTEM_FIRST: u32 = 1;
pub const MQMT_REQUEST: u32 = 1;
pub const MQMT_REPLY: u32 = 2;
pub const MQMT_DATAGRAM: u32 = 8;
pub const MQMT_REPORT: u32 = 4;
pub const MQMT_MQE_FIELDS_FROM_MQE: u32 = 112;
pub const MQMT_MQE_FIELDS: u32 = 113;
pub const MQMT_SYSTEM_LAST: u32 = 65535;
pub const MQMT_APPL_FIRST: u32 = 65536;
pub const MQMT_APPL_LAST: u32 = 999999999;
pub const MQEI_UNLIMITED: i32 = -1;
pub const MQFB_NONE: u32 = 0;
pub const MQFB_SYSTEM_FIRST: u32 = 1;
pub const MQFB_QUIT: u32 = 256;
pub const MQFB_EXPIRATION: u32 = 258;
pub const MQFB_COA: u32 = 259;
pub const MQFB_COD: u32 = 260;
pub const MQFB_CHANNEL_COMPLETED: u32 = 262;
pub const MQFB_CHANNEL_FAIL_RETRY: u32 = 263;
pub const MQFB_CHANNEL_FAIL: u32 = 264;
pub const MQFB_APPL_CANNOT_BE_STARTED: u32 = 265;
pub const MQFB_TM_ERROR: u32 = 266;
pub const MQFB_APPL_TYPE_ERROR: u32 = 267;
pub const MQFB_STOPPED_BY_MSG_EXIT: u32 = 268;
pub const MQFB_ACTIVITY: u32 = 269;
pub const MQFB_XMIT_Q_MSG_ERROR: u32 = 271;
pub const MQFB_PAN: u32 = 275;
pub const MQFB_NAN: u32 = 276;
pub const MQFB_STOPPED_BY_CHAD_EXIT: u32 = 277;
pub const MQFB_STOPPED_BY_PUBSUB_EXIT: u32 = 279;
pub const MQFB_NOT_A_REPOSITORY_MSG: u32 = 280;
pub const MQFB_BIND_OPEN_CLUSRCVR_DEL: u32 = 281;
pub const MQFB_MAX_ACTIVITIES: u32 = 282;
pub const MQFB_NOT_FORWARDED: u32 = 283;
pub const MQFB_NOT_DELIVERED: u32 = 284;
pub const MQFB_UNSUPPORTED_FORWARDING: u32 = 285;
pub const MQFB_UNSUPPORTED_DELIVERY: u32 = 286;
pub const MQFB_DATA_LENGTH_ZERO: u32 = 291;
pub const MQFB_DATA_LENGTH_NEGATIVE: u32 = 292;
pub const MQFB_DATA_LENGTH_TOO_BIG: u32 = 293;
pub const MQFB_BUFFER_OVERFLOW: u32 = 294;
pub const MQFB_LENGTH_OFF_BY_ONE: u32 = 295;
pub const MQFB_IIH_ERROR: u32 = 296;
pub const MQFB_NOT_AUTHORIZED_FOR_IMS: u32 = 298;
pub const MQFB_IMS_ERROR: u32 = 300;
pub const MQFB_IMS_FIRST: u32 = 301;
pub const MQFB_IMS_LAST: u32 = 399;
pub const MQFB_CICS_INTERNAL_ERROR: u32 = 401;
pub const MQFB_CICS_NOT_AUTHORIZED: u32 = 402;
pub const MQFB_CICS_BRIDGE_FAILURE: u32 = 403;
pub const MQFB_CICS_CORREL_ID_ERROR: u32 = 404;
pub const MQFB_CICS_CCSID_ERROR: u32 = 405;
pub const MQFB_CICS_ENCODING_ERROR: u32 = 406;
pub const MQFB_CICS_CIH_ERROR: u32 = 407;
pub const MQFB_CICS_UOW_ERROR: u32 = 408;
pub const MQFB_CICS_COMMAREA_ERROR: u32 = 409;
pub const MQFB_CICS_APPL_NOT_STARTED: u32 = 410;
pub const MQFB_CICS_APPL_ABENDED: u32 = 411;
pub const MQFB_CICS_DLQ_ERROR: u32 = 412;
pub const MQFB_CICS_UOW_BACKED_OUT: u32 = 413;
pub const MQFB_PUBLICATIONS_ON_REQUEST: u32 = 501;
pub const MQFB_SUBSCRIBER_IS_PUBLISHER: u32 = 502;
pub const MQFB_MSG_SCOPE_MISMATCH: u32 = 503;
pub const MQFB_SELECTOR_MISMATCH: u32 = 504;
pub const MQFB_NOT_A_GROUPUR_MSG: u32 = 505;
pub const MQFB_IMS_NACK_1A_REASON_FIRST: u32 = 600;
pub const MQFB_IMS_NACK_1A_REASON_LAST: u32 = 855;
pub const MQFB_SYSTEM_LAST: u32 = 65535;
pub const MQFB_APPL_FIRST: u32 = 65536;
pub const MQFB_APPL_LAST: u32 = 999999999;
pub const MQENC_NATIVE: u32 = 546;
pub const MQENC_INTEGER_MASK: u32 = 15;
pub const MQENC_DECIMAL_MASK: u32 = 240;
pub const MQENC_FLOAT_MASK: u32 = 3840;
pub const MQENC_RESERVED_MASK: u32 = 4294963200;
pub const MQENC_INTEGER_UNDEFINED: u32 = 0;
pub const MQENC_INTEGER_NORMAL: u32 = 1;
pub const MQENC_INTEGER_REVERSED: u32 = 2;
pub const MQENC_DECIMAL_UNDEFINED: u32 = 0;
pub const MQENC_DECIMAL_NORMAL: u32 = 16;
pub const MQENC_DECIMAL_REVERSED: u32 = 32;
pub const MQENC_FLOAT_UNDEFINED: u32 = 0;
pub const MQENC_FLOAT_IEEE_NORMAL: u32 = 256;
pub const MQENC_FLOAT_IEEE_REVERSED: u32 = 512;
pub const MQENC_FLOAT_S390: u32 = 768;
pub const MQENC_FLOAT_TNS: u32 = 1024;
pub const MQENC_NORMAL: u32 = 273;
pub const MQENC_REVERSED: u32 = 546;
pub const MQENC_S390: u32 = 785;
pub const MQENC_TNS: u32 = 1041;
pub const MQENC_AS_PUBLISHED: i32 = -1;
pub const MQCCSI_UNDEFINED: u32 = 0;
pub const MQCCSI_DEFAULT: u32 = 0;
pub const MQCCSI_Q_MGR: u32 = 0;
pub const MQCCSI_INHERIT: i32 = -2;
pub const MQCCSI_EMBEDDED: i32 = -1;
pub const MQCCSI_APPL: i32 = -3;
pub const MQCCSI_AS_PUBLISHED: i32 = -4;
pub const MQFMT_NONE: &'static [u8; 9usize] = b"        \0";
pub const MQFMT_ADMIN: &'static [u8; 9usize] = b"MQADMIN \0";
pub const MQFMT_AMQP: &'static [u8; 9usize] = b"MQAMQP  \0";
pub const MQFMT_CHANNEL_COMPLETED: &'static [u8; 9usize] = b"MQCHCOM \0";
pub const MQFMT_CICS: &'static [u8; 9usize] = b"MQCICS  \0";
pub const MQFMT_COMMAND_1: &'static [u8; 9usize] = b"MQCMD1  \0";
pub const MQFMT_COMMAND_2: &'static [u8; 9usize] = b"MQCMD2  \0";
pub const MQFMT_DEAD_LETTER_HEADER: &'static [u8; 9usize] = b"MQDEAD  \0";
pub const MQFMT_DIST_HEADER: &'static [u8; 9usize] = b"MQHDIST \0";
pub const MQFMT_EMBEDDED_PCF: &'static [u8; 9usize] = b"MQHEPCF \0";
pub const MQFMT_EVENT: &'static [u8; 9usize] = b"MQEVENT \0";
pub const MQFMT_IMS: &'static [u8; 9usize] = b"MQIMS   \0";
pub const MQFMT_IMS_VAR_STRING: &'static [u8; 9usize] = b"MQIMSVS \0";
pub const MQFMT_MD_EXTENSION: &'static [u8; 9usize] = b"MQHMDE  \0";
pub const MQFMT_PCF: &'static [u8; 9usize] = b"MQPCF   \0";
pub const MQFMT_REF_MSG_HEADER: &'static [u8; 9usize] = b"MQHREF  \0";
pub const MQFMT_RF_HEADER: &'static [u8; 9usize] = b"MQHRF   \0";
pub const MQFMT_RF_HEADER_1: &'static [u8; 9usize] = b"MQHRF   \0";
pub const MQFMT_RF_HEADER_2: &'static [u8; 9usize] = b"MQHRF2  \0";
pub const MQFMT_STRING: &'static [u8; 9usize] = b"MQSTR   \0";
pub const MQFMT_TRIGGER: &'static [u8; 9usize] = b"MQTRIG  \0";
pub const MQFMT_WORK_INFO_HEADER: &'static [u8; 9usize] = b"MQHWIH  \0";
pub const MQFMT_XMIT_Q_HEADER: &'static [u8; 9usize] = b"MQXMIT  \0";
pub const MQPRI_PRIORITY_AS_Q_DEF: i32 = -1;
pub const MQPRI_PRIORITY_AS_PARENT: i32 = -2;
pub const MQPRI_PRIORITY_AS_PUBLISHED: i32 = -3;
pub const MQPRI_PRIORITY_AS_TOPIC_DEF: i32 = -1;
pub const MQPER_PERSISTENCE_AS_PARENT: i32 = -1;
pub const MQPER_NOT_PERSISTENT: u32 = 0;
pub const MQPER_PERSISTENT: u32 = 1;
pub const MQPER_PERSISTENCE_AS_Q_DEF: u32 = 2;
pub const MQPER_PERSISTENCE_AS_TOPIC_DEF: u32 = 2;
pub const MQPRT_RESPONSE_AS_PARENT: u32 = 0;
pub const MQPRT_SYNC_RESPONSE: u32 = 1;
pub const MQPRT_ASYNC_RESPONSE: u32 = 2;
pub const MQMI_NONE: &'static [u8; 25usize] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQCI_NONE: &'static [u8; 25usize] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQCI_NEW_SESSION: &'static [u8; 25usize] = b"AMQ!NEW_SESSION_CORRELID\0";
pub const MQACT_NONE: &'static [u8; 33usize] =
    b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQACTT_UNKNOWN: u8 = 0u8;
pub const MQACTT_CICS_LUOW_ID: u8 = 1u8;
pub const MQACTT_OS2_DEFAULT: u8 = 4u8;
pub const MQACTT_DOS_DEFAULT: u8 = 5u8;
pub const MQACTT_UNIX_NUMERIC_ID: u8 = 6u8;
pub const MQACTT_OS400_ACCOUNT_TOKEN: u8 = 8u8;
pub const MQACTT_WINDOWS_DEFAULT: u8 = 9u8;
pub const MQACTT_NT_SECURITY_ID: u8 = 11u8;
pub const MQACTT_AZUREAD_SECURITY_ID: u8 = 12u8;
pub const MQACTT_MS_ACC_AUTH_SECURITY_ID: u8 = 13u8;
pub const MQACTT_USER: u8 = 25u8;
pub const MQAT_UNKNOWN: i32 = -1;
pub const MQAT_NO_CONTEXT: u32 = 0;
pub const MQAT_CICS: u32 = 1;
pub const MQAT_MVS: u32 = 2;
pub const MQAT_OS390: u32 = 2;
pub const MQAT_ZOS: u32 = 2;
pub const MQAT_IMS: u32 = 3;
pub const MQAT_OS2: u32 = 4;
pub const MQAT_DOS: u32 = 5;
pub const MQAT_AIX: u32 = 6;
pub const MQAT_UNIX: u32 = 6;
pub const MQAT_QMGR: u32 = 7;
pub const MQAT_OS400: u32 = 8;
pub const MQAT_WINDOWS: u32 = 9;
pub const MQAT_CICS_VSE: u32 = 10;
pub const MQAT_WINDOWS_NT: u32 = 11;
pub const MQAT_VMS: u32 = 12;
pub const MQAT_GUARDIAN: u32 = 13;
pub const MQAT_NSK: u32 = 13;
pub const MQAT_VOS: u32 = 14;
pub const MQAT_OPEN_TP1: u32 = 15;
pub const MQAT_VM: u32 = 18;
pub const MQAT_IMS_BRIDGE: u32 = 19;
pub const MQAT_XCF: u32 = 20;
pub const MQAT_CICS_BRIDGE: u32 = 21;
pub const MQAT_NOTES_AGENT: u32 = 22;
pub const MQAT_TPF: u32 = 23;
pub const MQAT_USER: u32 = 25;
pub const MQAT_BROKER: u32 = 26;
pub const MQAT_QMGR_PUBLISH: u32 = 26;
pub const MQAT_JAVA: u32 = 28;
pub const MQAT_DQM: u32 = 29;
pub const MQAT_CHANNEL_INITIATOR: u32 = 30;
pub const MQAT_WLM: u32 = 31;
pub const MQAT_BATCH: u32 = 32;
pub const MQAT_RRS_BATCH: u32 = 33;
pub const MQAT_SIB: u32 = 34;
pub const MQAT_SYSTEM_EXTENSION: u32 = 35;
pub const MQAT_MCAST_PUBLISH: u32 = 36;
pub const MQAT_AMQP: u32 = 37;
pub const MQAT_DEFAULT: u32 = 6;
pub const MQAT_USER_FIRST: u32 = 65536;
pub const MQAT_USER_LAST: u32 = 999999999;
pub const MQGI_NONE: &'static [u8; 25usize] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQMF_SEGMENTATION_INHIBITED: u32 = 0;
pub const MQMF_SEGMENTATION_ALLOWED: u32 = 1;
pub const MQMF_MSG_IN_GROUP: u32 = 8;
pub const MQMF_LAST_MSG_IN_GROUP: u32 = 16;
pub const MQMF_SEGMENT: u32 = 2;
pub const MQMF_LAST_SEGMENT: u32 = 4;
pub const MQMF_NONE: u32 = 0;
pub const MQMF_REJECT_UNSUP_MASK: u32 = 4095;
pub const MQMF_ACCEPT_UNSUP_MASK: u32 = 4293918720;
pub const MQMF_ACCEPT_UNSUP_IF_XMIT_MASK: u32 = 1044480;
pub const MQOL_UNDEFINED: i32 = -1;
pub const MQMDE_STRUC_ID: &'static [u8; 5usize] = b"MDE \0";
pub const MQMDE_VERSION_2: u32 = 2;
pub const MQMDE_CURRENT_VERSION: u32 = 2;
pub const MQMDE_LENGTH_2: u32 = 72;
pub const MQMDE_CURRENT_LENGTH: u32 = 72;
pub const MQMDEF_NONE: u32 = 0;
pub const MQMD1_LENGTH_1: u32 = 324;
pub const MQMD1_CURRENT_LENGTH: u32 = 324;
pub const MQMD2_LENGTH_1: u32 = 324;
pub const MQMD2_LENGTH_2: u32 = 364;
pub const MQMD2_CURRENT_LENGTH: u32 = 364;
pub const MQMHBO_STRUC_ID: &'static [u8; 5usize] = b"MHBO\0";
pub const MQMHBO_VERSION_1: u32 = 1;
pub const MQMHBO_CURRENT_VERSION: u32 = 1;
pub const MQMHBO_LENGTH_1: u32 = 12;
pub const MQMHBO_CURRENT_LENGTH: u32 = 12;
pub const MQMHBO_PROPERTIES_IN_MQRFH2: u32 = 1;
pub const MQMHBO_DELETE_PROPERTIES: u32 = 2;
pub const MQMHBO_NONE: u32 = 0;
pub const MQOD_STRUC_ID: &'static [u8; 5usize] = b"OD  \0";
pub const MQOD_VERSION_1: u32 = 1;
pub const MQOD_VERSION_2: u32 = 2;
pub const MQOD_VERSION_3: u32 = 3;
pub const MQOD_VERSION_4: u32 = 4;
pub const MQOD_CURRENT_VERSION: u32 = 4;
pub const MQOD_LENGTH_1: u32 = 168;
pub const MQOD_LENGTH_2: u32 = 208;
pub const MQOD_LENGTH_3: u32 = 344;
pub const MQOD_LENGTH_4: u32 = 424;
pub const MQOD_CURRENT_LENGTH: u32 = 424;
pub const MQOM_NO: u32 = 0;
pub const MQOM_YES: u32 = 1;
pub const MQOT_NONE: u32 = 0;
pub const MQOT_Q: u32 = 1;
pub const MQOT_NAMELIST: u32 = 2;
pub const MQOT_PROCESS: u32 = 3;
pub const MQOT_STORAGE_CLASS: u32 = 4;
pub const MQOT_Q_MGR: u32 = 5;
pub const MQOT_CHANNEL: u32 = 6;
pub const MQOT_AUTH_INFO: u32 = 7;
pub const MQOT_TOPIC: u32 = 8;
pub const MQOT_COMM_INFO: u32 = 9;
pub const MQOT_CF_STRUC: u32 = 10;
pub const MQOT_LISTENER: u32 = 11;
pub const MQOT_SERVICE: u32 = 12;
pub const MQOT_RESERVED_1: u32 = 999;
pub const MQOT_ALL: u32 = 1001;
pub const MQOT_ALIAS_Q: u32 = 1002;
pub const MQOT_MODEL_Q: u32 = 1003;
pub const MQOT_LOCAL_Q: u32 = 1004;
pub const MQOT_REMOTE_Q: u32 = 1005;
pub const MQOT_SENDER_CHANNEL: u32 = 1007;
pub const MQOT_SERVER_CHANNEL: u32 = 1008;
pub const MQOT_REQUESTER_CHANNEL: u32 = 1009;
pub const MQOT_RECEIVER_CHANNEL: u32 = 1010;
pub const MQOT_CURRENT_CHANNEL: u32 = 1011;
pub const MQOT_SAVED_CHANNEL: u32 = 1012;
pub const MQOT_SVRCONN_CHANNEL: u32 = 1013;
pub const MQOT_CLNTCONN_CHANNEL: u32 = 1014;
pub const MQOT_SHORT_CHANNEL: u32 = 1015;
pub const MQOT_CHLAUTH: u32 = 1016;
pub const MQOT_REMOTE_Q_MGR_NAME: u32 = 1017;
pub const MQOT_PROT_POLICY: u32 = 1019;
pub const MQOT_TT_CHANNEL: u32 = 1020;
pub const MQOT_AMQP_CHANNEL: u32 = 1021;
pub const MQOT_AUTH_REC: u32 = 1022;
pub const MQPD_STRUC_ID: &'static [u8; 5usize] = b"PD  \0";
pub const MQPD_VERSION_1: u32 = 1;
pub const MQPD_CURRENT_VERSION: u32 = 1;
pub const MQPD_LENGTH_1: u32 = 24;
pub const MQPD_CURRENT_LENGTH: u32 = 24;
pub const MQPD_NONE: u32 = 0;
pub const MQPD_SUPPORT_OPTIONAL: u32 = 1;
pub const MQPD_SUPPORT_REQUIRED: u32 = 1048576;
pub const MQPD_SUPPORT_REQUIRED_IF_LOCAL: u32 = 1024;
pub const MQPD_REJECT_UNSUP_MASK: u32 = 4293918720;
pub const MQPD_ACCEPT_UNSUP_IF_XMIT_MASK: u32 = 1047552;
pub const MQPD_ACCEPT_UNSUP_MASK: u32 = 1023;
pub const MQPD_NO_CONTEXT: u32 = 0;
pub const MQPD_USER_CONTEXT: u32 = 1;
pub const MQCOPY_NONE: u32 = 0;
pub const MQCOPY_ALL: u32 = 1;
pub const MQCOPY_FORWARD: u32 = 2;
pub const MQCOPY_PUBLISH: u32 = 4;
pub const MQCOPY_REPLY: u32 = 8;
pub const MQCOPY_REPORT: u32 = 16;
pub const MQCOPY_DEFAULT: u32 = 22;
pub const MQPMO_STRUC_ID: &'static [u8; 5usize] = b"PMO \0";
pub const MQPMO_VERSION_1: u32 = 1;
pub const MQPMO_VERSION_2: u32 = 2;
pub const MQPMO_VERSION_3: u32 = 3;
pub const MQPMO_CURRENT_VERSION: u32 = 3;
pub const MQPMO_LENGTH_1: u32 = 128;
pub const MQPMO_LENGTH_2: u32 = 160;
pub const MQPMO_LENGTH_3: u32 = 184;
pub const MQPMO_CURRENT_LENGTH: u32 = 184;
pub const MQPMO_SYNCPOINT: u32 = 2;
pub const MQPMO_NO_SYNCPOINT: u32 = 4;
pub const MQPMO_DEFAULT_CONTEXT: u32 = 32;
pub const MQPMO_NEW_MSG_ID: u32 = 64;
pub const MQPMO_NEW_CORREL_ID: u32 = 128;
pub const MQPMO_PASS_IDENTITY_CONTEXT: u32 = 256;
pub const MQPMO_PASS_ALL_CONTEXT: u32 = 512;
pub const MQPMO_SET_IDENTITY_CONTEXT: u32 = 1024;
pub const MQPMO_SET_ALL_CONTEXT: u32 = 2048;
pub const MQPMO_ALTERNATE_USER_AUTHORITY: u32 = 4096;
pub const MQPMO_FAIL_IF_QUIESCING: u32 = 8192;
pub const MQPMO_NO_CONTEXT: u32 = 16384;
pub const MQPMO_LOGICAL_ORDER: u32 = 32768;
pub const MQPMO_ASYNC_RESPONSE: u32 = 65536;
pub const MQPMO_SYNC_RESPONSE: u32 = 131072;
pub const MQPMO_RESOLVE_LOCAL_Q: u32 = 262144;
pub const MQPMO_WARN_IF_NO_SUBS_MATCHED: u32 = 524288;
pub const MQPMO_RETAIN: u32 = 2097152;
pub const MQPMO_MD_FOR_OUTPUT_ONLY: u32 = 8388608;
pub const MQPMO_SCOPE_QMGR: u32 = 67108864;
pub const MQPMO_SUPPRESS_REPLYTO: u32 = 134217728;
pub const MQPMO_NOT_OWN_SUBS: u32 = 268435456;
pub const MQPMO_RESPONSE_AS_Q_DEF: u32 = 0;
pub const MQPMO_RESPONSE_AS_TOPIC_DEF: u32 = 0;
pub const MQPMO_NONE: u32 = 0;
pub const MQPMO_PUB_OPTIONS_MASK: u32 = 2097152;
pub const MQPMRF_MSG_ID: u32 = 1;
pub const MQPMRF_CORREL_ID: u32 = 2;
pub const MQPMRF_GROUP_ID: u32 = 4;
pub const MQPMRF_FEEDBACK: u32 = 8;
pub const MQPMRF_ACCOUNTING_TOKEN: u32 = 16;
pub const MQPMRF_NONE: u32 = 0;
pub const MQACTP_NEW: u32 = 0;
pub const MQACTP_FORWARD: u32 = 1;
pub const MQACTP_REPLY: u32 = 2;
pub const MQACTP_REPORT: u32 = 3;
pub const MQRFH_STRUC_ID: &'static [u8; 5usize] = b"RFH \0";
pub const MQRFH_VERSION_1: u32 = 1;
pub const MQRFH_VERSION_2: u32 = 2;
pub const MQRFH_STRUC_LENGTH_FIXED: u32 = 32;
pub const MQRFH_STRUC_LENGTH_FIXED_2: u32 = 36;
pub const MQRFH_LENGTH_1: u32 = 32;
pub const MQRFH_CURRENT_LENGTH: u32 = 32;
pub const MQRFH_NONE: u32 = 0;
pub const MQRFH_NO_FLAGS: u32 = 0;
pub const MQRFH_FLAGS_RESTRICTED_MASK: u32 = 4294901760;
pub const MQNVS_APPL_TYPE: &'static [u8; 13usize] = b"OPT_APP_GRP \0";
pub const MQNVS_MSG_TYPE: &'static [u8; 14usize] = b"OPT_MSG_TYPE \0";
pub const MQRFH2_LENGTH_2: u32 = 36;
pub const MQRFH2_CURRENT_LENGTH: u32 = 36;
pub const MQRMH_STRUC_ID: &'static [u8; 5usize] = b"RMH \0";
pub const MQRMH_VERSION_1: u32 = 1;
pub const MQRMH_CURRENT_VERSION: u32 = 1;
pub const MQRMH_LENGTH_1: u32 = 108;
pub const MQRMH_CURRENT_LENGTH: u32 = 108;
pub const MQRMHF_LAST: u32 = 1;
pub const MQRMHF_NOT_LAST: u32 = 0;
pub const MQOII_NONE: &'static [u8; 25usize] =
    b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQSD_STRUC_ID: &'static [u8; 5usize] = b"SD  \0";
pub const MQSD_VERSION_1: u32 = 1;
pub const MQSD_CURRENT_VERSION: u32 = 1;
pub const MQSD_LENGTH_1: u32 = 344;
pub const MQSD_CURRENT_LENGTH: u32 = 344;
pub const MQSID_NONE: &'static [u8; 41usize] =
    b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQSIDT_NONE: u8 = 0u8;
pub const MQSIDT_NT_SECURITY_ID: u8 = 1u8;
pub const MQSIDT_WAS_SECURITY_ID: u8 = 2u8;
pub const MQSMPO_STRUC_ID: &'static [u8; 5usize] = b"SMPO\0";
pub const MQSMPO_VERSION_1: u32 = 1;
pub const MQSMPO_CURRENT_VERSION: u32 = 1;
pub const MQSMPO_LENGTH_1: u32 = 20;
pub const MQSMPO_CURRENT_LENGTH: u32 = 20;
pub const MQSMPO_SET_FIRST: u32 = 0;
pub const MQSMPO_SET_PROP_UNDER_CURSOR: u32 = 1;
pub const MQSMPO_SET_PROP_AFTER_CURSOR: u32 = 2;
pub const MQSMPO_APPEND_PROPERTY: u32 = 4;
pub const MQSMPO_SET_PROP_BEFORE_CURSOR: u32 = 8;
pub const MQSMPO_NONE: u32 = 0;
pub const MQSRO_STRUC_ID: &'static [u8; 5usize] = b"SRO \0";
pub const MQSRO_VERSION_1: u32 = 1;
pub const MQSRO_CURRENT_VERSION: u32 = 1;
pub const MQSRO_LENGTH_1: u32 = 16;
pub const MQSRO_CURRENT_LENGTH: u32 = 16;
pub const MQSRO_NONE: u32 = 0;
pub const MQSRO_FAIL_IF_QUIESCING: u32 = 8192;
pub const MQSTS_STRUC_ID: &'static [u8; 5usize] = b"STAT\0";
pub const MQSTS_VERSION_1: u32 = 1;
pub const MQSTS_VERSION_2: u32 = 2;
pub const MQSTS_CURRENT_VERSION: u32 = 2;
pub const MQSTS_LENGTH_1: u32 = 224;
pub const MQSTS_LENGTH_2: u32 = 280;
pub const MQSTS_CURRENT_LENGTH: u32 = 280;
pub const MQTM_STRUC_ID: &'static [u8; 5usize] = b"TM  \0";
pub const MQTM_VERSION_1: u32 = 1;
pub const MQTM_CURRENT_VERSION: u32 = 1;
pub const MQTM_LENGTH_1: u32 = 684;
pub const MQTM_CURRENT_LENGTH: u32 = 684;
pub const MQTMC_STRUC_ID: &'static [u8; 5usize] = b"TMC \0";
pub const MQTMC2_LENGTH_1: u32 = 684;
pub const MQTMC2_LENGTH_2: u32 = 732;
pub const MQTMC2_CURRENT_LENGTH: u32 = 732;
pub const MQTMC_VERSION_1: &'static [u8; 5usize] = b"   1\0";
pub const MQTMC_VERSION_2: &'static [u8; 5usize] = b"   2\0";
pub const MQTMC_CURRENT_VERSION: &'static [u8; 5usize] = b"   2\0";
pub const MQWIH_STRUC_ID: &'static [u8; 5usize] = b"WIH \0";
pub const MQWIH_VERSION_1: u32 = 1;
pub const MQWIH_CURRENT_VERSION: u32 = 1;
pub const MQWIH_LENGTH_1: u32 = 120;
pub const MQWIH_CURRENT_LENGTH: u32 = 120;
pub const MQWIH_NONE: u32 = 0;
pub const MQXQH_STRUC_ID: &'static [u8; 5usize] = b"XQH \0";
pub const MQXQH_VERSION_1: u32 = 1;
pub const MQXQH_CURRENT_VERSION: u32 = 1;
pub const MQXQH_LENGTH_1: u32 = 428;
pub const MQXQH_CURRENT_LENGTH: u32 = 428;
pub const MQHC_DEF_HCONN: u32 = 0;
pub const MQHC_UNUSABLE_HCONN: i32 = -1;
pub const MQHC_UNASSOCIATED_HCONN: i32 = -3;
pub const MQ_OPERATOR_MESSAGE_LENGTH: u32 = 4;
pub const MQ_ABEND_CODE_LENGTH: u32 = 4;
pub const MQ_ACCOUNTING_TOKEN_LENGTH: u32 = 32;
pub const MQ_APPL_DESC_LENGTH: u32 = 64;
pub const MQ_APPL_IDENTITY_DATA_LENGTH: u32 = 32;
pub const MQ_APPL_NAME_LENGTH: u32 = 28;
pub const MQ_APPL_ORIGIN_DATA_LENGTH: u32 = 4;
pub const MQ_APPL_TAG_LENGTH: u32 = 28;
pub const MQ_ARM_SUFFIX_LENGTH: u32 = 2;
pub const MQ_ATTENTION_ID_LENGTH: u32 = 4;
pub const MQ_AUTH_INFO_CONN_NAME_LENGTH: u32 = 264;
pub const MQ_AUTH_INFO_DESC_LENGTH: u32 = 64;
pub const MQ_AUTH_INFO_NAME_LENGTH: u32 = 48;
pub const MQ_AUTH_INFO_OCSP_URL_LENGTH: u32 = 256;
pub const MQ_AUTHENTICATOR_LENGTH: u32 = 8;
pub const MQ_AUTO_REORG_CATALOG_LENGTH: u32 = 44;
pub const MQ_AUTO_REORG_TIME_LENGTH: u32 = 4;
pub const MQ_BATCH_INTERFACE_ID_LENGTH: u32 = 8;
pub const MQ_BRIDGE_NAME_LENGTH: u32 = 24;
pub const MQ_CANCEL_CODE_LENGTH: u32 = 4;
pub const MQ_CF_STRUC_DESC_LENGTH: u32 = 64;
pub const MQ_CF_STRUC_NAME_LENGTH: u32 = 12;
pub const MQ_CHANNEL_DATE_LENGTH: u32 = 12;
pub const MQ_CHANNEL_DESC_LENGTH: u32 = 64;
pub const MQ_CHANNEL_NAME_LENGTH: u32 = 20;
pub const MQ_CHANNEL_TIME_LENGTH: u32 = 8;
pub const MQ_CHINIT_SERVICE_PARM_LENGTH: u32 = 32;
pub const MQ_CICS_FILE_NAME_LENGTH: u32 = 8;
pub const MQ_AMQP_CLIENT_ID_LENGTH: u32 = 256;
pub const MQ_CLIENT_ID_LENGTH: u32 = 23;
pub const MQ_CLIENT_USER_ID_LENGTH: u32 = 1024;
pub const MQ_CLUSTER_NAME_LENGTH: u32 = 48;
pub const MQ_COMM_INFO_DESC_LENGTH: u32 = 64;
pub const MQ_COMM_INFO_NAME_LENGTH: u32 = 48;
pub const MQ_CONN_NAME_LENGTH: u32 = 264;
pub const MQ_CONN_TAG_LENGTH: u32 = 128;
pub const MQ_CONNECTION_ID_LENGTH: u32 = 24;
pub const MQ_CORREL_ID_LENGTH: u32 = 24;
pub const MQ_CREATION_DATE_LENGTH: u32 = 12;
pub const MQ_CREATION_TIME_LENGTH: u32 = 8;
pub const MQ_CSP_PASSWORD_LENGTH: u32 = 256;
pub const MQ_DATE_LENGTH: u32 = 12;
pub const MQ_DISTINGUISHED_NAME_LENGTH: u32 = 1024;
pub const MQ_DNS_GROUP_NAME_LENGTH: u32 = 18;
pub const MQ_EXIT_DATA_LENGTH: u32 = 32;
pub const MQ_EXIT_INFO_NAME_LENGTH: u32 = 48;
pub const MQ_EXIT_NAME_LENGTH: u32 = 128;
pub const MQ_EXIT_PD_AREA_LENGTH: u32 = 48;
pub const MQ_EXIT_USER_AREA_LENGTH: u32 = 16;
pub const MQ_FACILITY_LENGTH: u32 = 8;
pub const MQ_FACILITY_LIKE_LENGTH: u32 = 4;
pub const MQ_FORMAT_LENGTH: u32 = 8;
pub const MQ_FUNCTION_LENGTH: u32 = 4;
pub const MQ_GROUP_ID_LENGTH: u32 = 24;
pub const MQ_APPL_FUNCTION_NAME_LENGTH: u32 = 10;
pub const MQ_INSTALLATION_DESC_LENGTH: u32 = 64;
pub const MQ_INSTALLATION_NAME_LENGTH: u32 = 16;
pub const MQ_INSTALLATION_PATH_LENGTH: u32 = 256;
pub const MQ_JAAS_CONFIG_LENGTH: u32 = 1024;
pub const MQ_LDAP_PASSWORD_LENGTH: u32 = 32;
pub const MQ_LDAP_BASE_DN_LENGTH: u32 = 1024;
pub const MQ_LDAP_FIELD_LENGTH: u32 = 128;
pub const MQ_LDAP_CLASS_LENGTH: u32 = 128;
pub const MQ_LISTENER_NAME_LENGTH: u32 = 48;
pub const MQ_LISTENER_DESC_LENGTH: u32 = 64;
pub const MQ_LOCAL_ADDRESS_LENGTH: u32 = 48;
pub const MQ_LTERM_OVERRIDE_LENGTH: u32 = 8;
pub const MQ_LU_NAME_LENGTH: u32 = 8;
pub const MQ_LUWID_LENGTH: u32 = 16;
pub const MQ_MAX_EXIT_NAME_LENGTH: u32 = 128;
pub const MQ_MAX_MCA_USER_ID_LENGTH: u32 = 64;
pub const MQ_MAX_LDAP_MCA_USER_ID_LENGTH: u32 = 1024;
pub const MQ_MAX_PROPERTY_NAME_LENGTH: u32 = 4095;
pub const MQ_MAX_USER_ID_LENGTH: u32 = 64;
pub const MQ_MCA_JOB_NAME_LENGTH: u32 = 28;
pub const MQ_MCA_NAME_LENGTH: u32 = 20;
pub const MQ_MCA_USER_DATA_LENGTH: u32 = 32;
pub const MQ_MCA_USER_ID_LENGTH: u32 = 12;
pub const MQ_LDAP_MCA_USER_ID_LENGTH: u32 = 1024;
pub const MQ_MFS_MAP_NAME_LENGTH: u32 = 8;
pub const MQ_MODE_NAME_LENGTH: u32 = 8;
pub const MQ_MSG_HEADER_LENGTH: u32 = 4000;
pub const MQ_MSG_ID_LENGTH: u32 = 24;
pub const MQ_MSG_TOKEN_LENGTH: u32 = 16;
pub const MQ_NAMELIST_DESC_LENGTH: u32 = 64;
pub const MQ_NAMELIST_NAME_LENGTH: u32 = 48;
pub const MQ_OBJECT_INSTANCE_ID_LENGTH: u32 = 24;
pub const MQ_OBJECT_NAME_LENGTH: u32 = 48;
pub const MQ_PASS_TICKET_APPL_LENGTH: u32 = 8;
pub const MQ_PASSWORD_LENGTH: u32 = 12;
pub const MQ_PROCESS_APPL_ID_LENGTH: u32 = 256;
pub const MQ_PROCESS_DESC_LENGTH: u32 = 64;
pub const MQ_PROCESS_ENV_DATA_LENGTH: u32 = 128;
pub const MQ_PROCESS_NAME_LENGTH: u32 = 48;
pub const MQ_PROCESS_USER_DATA_LENGTH: u32 = 128;
pub const MQ_PROGRAM_NAME_LENGTH: u32 = 20;
pub const MQ_PUT_APPL_NAME_LENGTH: u32 = 28;
pub const MQ_PUT_DATE_LENGTH: u32 = 8;
pub const MQ_PUT_TIME_LENGTH: u32 = 8;
pub const MQ_Q_DESC_LENGTH: u32 = 64;
pub const MQ_Q_MGR_DESC_LENGTH: u32 = 64;
pub const MQ_Q_MGR_IDENTIFIER_LENGTH: u32 = 48;
pub const MQ_Q_MGR_NAME_LENGTH: u32 = 48;
pub const MQ_Q_NAME_LENGTH: u32 = 48;
pub const MQ_QSG_NAME_LENGTH: u32 = 4;
pub const MQ_REMOTE_SYS_ID_LENGTH: u32 = 4;
pub const MQ_SECURITY_ID_LENGTH: u32 = 40;
pub const MQ_SELECTOR_LENGTH: u32 = 10240;
pub const MQ_SERVICE_ARGS_LENGTH: u32 = 255;
pub const MQ_SERVICE_COMMAND_LENGTH: u32 = 255;
pub const MQ_SERVICE_DESC_LENGTH: u32 = 64;
pub const MQ_SERVICE_NAME_LENGTH: u32 = 32;
pub const MQ_SERVICE_PATH_LENGTH: u32 = 255;
pub const MQ_SERVICE_STEP_LENGTH: u32 = 8;
pub const MQ_SHORT_CONN_NAME_LENGTH: u32 = 20;
pub const MQ_SHORT_DNAME_LENGTH: u32 = 256;
pub const MQ_SSL_CIPHER_SPEC_LENGTH: u32 = 32;
pub const MQ_SSL_CIPHER_SUITE_LENGTH: u32 = 32;
pub const MQ_SSL_CRYPTO_HARDWARE_LENGTH: u32 = 256;
pub const MQ_SSL_HANDSHAKE_STAGE_LENGTH: u32 = 32;
pub const MQ_SSL_KEY_LIBRARY_LENGTH: u32 = 44;
pub const MQ_SSL_KEY_MEMBER_LENGTH: u32 = 8;
pub const MQ_SSL_KEY_REPOSITORY_LENGTH: u32 = 256;
pub const MQ_SSL_PEER_NAME_LENGTH: u32 = 1024;
pub const MQ_SSL_SHORT_PEER_NAME_LENGTH: u32 = 256;
pub const MQ_START_CODE_LENGTH: u32 = 4;
pub const MQ_STORAGE_CLASS_DESC_LENGTH: u32 = 64;
pub const MQ_STORAGE_CLASS_LENGTH: u32 = 8;
pub const MQ_SUB_IDENTITY_LENGTH: u32 = 128;
pub const MQ_SUB_POINT_LENGTH: u32 = 128;
pub const MQ_TCP_NAME_LENGTH: u32 = 8;
pub const MQ_TIME_LENGTH: u32 = 8;
pub const MQ_TOPIC_DESC_LENGTH: u32 = 64;
pub const MQ_TOPIC_NAME_LENGTH: u32 = 48;
pub const MQ_TOPIC_STR_LENGTH: u32 = 10240;
pub const MQ_TOTAL_EXIT_DATA_LENGTH: u32 = 999;
pub const MQ_TOTAL_EXIT_NAME_LENGTH: u32 = 999;
pub const MQ_TP_NAME_LENGTH: u32 = 64;
pub const MQ_TPIPE_NAME_LENGTH: u32 = 8;
pub const MQ_TRAN_INSTANCE_ID_LENGTH: u32 = 16;
pub const MQ_TRANSACTION_ID_LENGTH: u32 = 4;
pub const MQ_TRIGGER_DATA_LENGTH: u32 = 64;
pub const MQ_TRIGGER_PROGRAM_NAME_LENGTH: u32 = 8;
pub const MQ_TRIGGER_TERM_ID_LENGTH: u32 = 4;
pub const MQ_TRIGGER_TRANS_ID_LENGTH: u32 = 4;
pub const MQ_USER_ID_LENGTH: u32 = 12;
pub const MQ_VERSION_LENGTH: u32 = 8;
pub const MQ_XCF_GROUP_NAME_LENGTH: u32 = 8;
pub const MQ_XCF_MEMBER_NAME_LENGTH: u32 = 16;
pub const MQ_SMDS_NAME_LENGTH: u32 = 4;
pub const MQ_CHLAUTH_DESC_LENGTH: u32 = 64;
pub const MQ_CUSTOM_LENGTH: u32 = 128;
pub const MQ_SUITE_B_SIZE: u32 = 4;
pub const MQ_CERT_LABEL_LENGTH: u32 = 64;
pub const MQCC_OK: u32 = 0;
pub const MQCC_WARNING: u32 = 1;
pub const MQCC_FAILED: u32 = 2;
pub const MQCC_UNKNOWN: i32 = -1;
pub const MQRC_NONE: u32 = 0;
pub const MQRC_APPL_FIRST: u32 = 900;
pub const MQRC_APPL_LAST: u32 = 999;
pub const MQRC_ALIAS_BASE_Q_TYPE_ERROR: u32 = 2001;
pub const MQRC_ALREADY_CONNECTED: u32 = 2002;
pub const MQRC_BACKED_OUT: u32 = 2003;
pub const MQRC_BUFFER_ERROR: u32 = 2004;
pub const MQRC_BUFFER_LENGTH_ERROR: u32 = 2005;
pub const MQRC_CHAR_ATTR_LENGTH_ERROR: u32 = 2006;
pub const MQRC_CHAR_ATTRS_ERROR: u32 = 2007;
pub const MQRC_CHAR_ATTRS_TOO_SHORT: u32 = 2008;
pub const MQRC_CONNECTION_BROKEN: u32 = 2009;
pub const MQRC_DATA_LENGTH_ERROR: u32 = 2010;
pub const MQRC_DYNAMIC_Q_NAME_ERROR: u32 = 2011;
pub const MQRC_ENVIRONMENT_ERROR: u32 = 2012;
pub const MQRC_EXPIRY_ERROR: u32 = 2013;
pub const MQRC_FEEDBACK_ERROR: u32 = 2014;
pub const MQRC_GET_INHIBITED: u32 = 2016;
pub const MQRC_HANDLE_NOT_AVAILABLE: u32 = 2017;
pub const MQRC_HCONN_ERROR: u32 = 2018;
pub const MQRC_HOBJ_ERROR: u32 = 2019;
pub const MQRC_INHIBIT_VALUE_ERROR: u32 = 2020;
pub const MQRC_INT_ATTR_COUNT_ERROR: u32 = 2021;
pub const MQRC_INT_ATTR_COUNT_TOO_SMALL: u32 = 2022;
pub const MQRC_INT_ATTRS_ARRAY_ERROR: u32 = 2023;
pub const MQRC_SYNCPOINT_LIMIT_REACHED: u32 = 2024;
pub const MQRC_MAX_CONNS_LIMIT_REACHED: u32 = 2025;
pub const MQRC_MD_ERROR: u32 = 2026;
pub const MQRC_MISSING_REPLY_TO_Q: u32 = 2027;
pub const MQRC_MSG_TYPE_ERROR: u32 = 2029;
pub const MQRC_MSG_TOO_BIG_FOR_Q: u32 = 2030;
pub const MQRC_MSG_TOO_BIG_FOR_Q_MGR: u32 = 2031;
pub const MQRC_NO_MSG_AVAILABLE: u32 = 2033;
pub const MQRC_NO_MSG_UNDER_CURSOR: u32 = 2034;
pub const MQRC_NOT_AUTHORIZED: u32 = 2035;
pub const MQRC_NOT_OPEN_FOR_BROWSE: u32 = 2036;
pub const MQRC_NOT_OPEN_FOR_INPUT: u32 = 2037;
pub const MQRC_NOT_OPEN_FOR_INQUIRE: u32 = 2038;
pub const MQRC_NOT_OPEN_FOR_OUTPUT: u32 = 2039;
pub const MQRC_NOT_OPEN_FOR_SET: u32 = 2040;
pub const MQRC_OBJECT_CHANGED: u32 = 2041;
pub const MQRC_OBJECT_IN_USE: u32 = 2042;
pub const MQRC_OBJECT_TYPE_ERROR: u32 = 2043;
pub const MQRC_OD_ERROR: u32 = 2044;
pub const MQRC_OPTION_NOT_VALID_FOR_TYPE: u32 = 2045;
pub const MQRC_OPTIONS_ERROR: u32 = 2046;
pub const MQRC_PERSISTENCE_ERROR: u32 = 2047;
pub const MQRC_PERSISTENT_NOT_ALLOWED: u32 = 2048;
pub const MQRC_PRIORITY_EXCEEDS_MAXIMUM: u32 = 2049;
pub const MQRC_PRIORITY_ERROR: u32 = 2050;
pub const MQRC_PUT_INHIBITED: u32 = 2051;
pub const MQRC_Q_DELETED: u32 = 2052;
pub const MQRC_Q_FULL: u32 = 2053;
pub const MQRC_Q_NOT_EMPTY: u32 = 2055;
pub const MQRC_Q_SPACE_NOT_AVAILABLE: u32 = 2056;
pub const MQRC_Q_TYPE_ERROR: u32 = 2057;
pub const MQRC_Q_MGR_NAME_ERROR: u32 = 2058;
pub const MQRC_Q_MGR_NOT_AVAILABLE: u32 = 2059;
pub const MQRC_REPORT_OPTIONS_ERROR: u32 = 2061;
pub const MQRC_SECOND_MARK_NOT_ALLOWED: u32 = 2062;
pub const MQRC_SECURITY_ERROR: u32 = 2063;
pub const MQRC_SELECTOR_COUNT_ERROR: u32 = 2065;
pub const MQRC_SELECTOR_LIMIT_EXCEEDED: u32 = 2066;
pub const MQRC_SELECTOR_ERROR: u32 = 2067;
pub const MQRC_SELECTOR_NOT_FOR_TYPE: u32 = 2068;
pub const MQRC_SIGNAL_OUTSTANDING: u32 = 2069;
pub const MQRC_SIGNAL_REQUEST_ACCEPTED: u32 = 2070;
pub const MQRC_STORAGE_NOT_AVAILABLE: u32 = 2071;
pub const MQRC_SYNCPOINT_NOT_AVAILABLE: u32 = 2072;
pub const MQRC_TRIGGER_CONTROL_ERROR: u32 = 2075;
pub const MQRC_TRIGGER_DEPTH_ERROR: u32 = 2076;
pub const MQRC_TRIGGER_MSG_PRIORITY_ERR: u32 = 2077;
pub const MQRC_TRIGGER_TYPE_ERROR: u32 = 2078;
pub const MQRC_TRUNCATED_MSG_ACCEPTED: u32 = 2079;
pub const MQRC_TRUNCATED_MSG_FAILED: u32 = 2080;
pub const MQRC_UNKNOWN_ALIAS_BASE_Q: u32 = 2082;
pub const MQRC_UNKNOWN_OBJECT_NAME: u32 = 2085;
pub const MQRC_UNKNOWN_OBJECT_Q_MGR: u32 = 2086;
pub const MQRC_UNKNOWN_REMOTE_Q_MGR: u32 = 2087;
pub const MQRC_WAIT_INTERVAL_ERROR: u32 = 2090;
pub const MQRC_XMIT_Q_TYPE_ERROR: u32 = 2091;
pub const MQRC_XMIT_Q_USAGE_ERROR: u32 = 2092;
pub const MQRC_NOT_OPEN_FOR_PASS_ALL: u32 = 2093;
pub const MQRC_NOT_OPEN_FOR_PASS_IDENT: u32 = 2094;
pub const MQRC_NOT_OPEN_FOR_SET_ALL: u32 = 2095;
pub const MQRC_NOT_OPEN_FOR_SET_IDENT: u32 = 2096;
pub const MQRC_CONTEXT_HANDLE_ERROR: u32 = 2097;
pub const MQRC_CONTEXT_NOT_AVAILABLE: u32 = 2098;
pub const MQRC_SIGNAL1_ERROR: u32 = 2099;
pub const MQRC_OBJECT_ALREADY_EXISTS: u32 = 2100;
pub const MQRC_OBJECT_DAMAGED: u32 = 2101;
pub const MQRC_RESOURCE_PROBLEM: u32 = 2102;
pub const MQRC_ANOTHER_Q_MGR_CONNECTED: u32 = 2103;
pub const MQRC_UNKNOWN_REPORT_OPTION: u32 = 2104;
pub const MQRC_STORAGE_CLASS_ERROR: u32 = 2105;
pub const MQRC_COD_NOT_VALID_FOR_XCF_Q: u32 = 2106;
pub const MQRC_XWAIT_CANCELED: u32 = 2107;
pub const MQRC_XWAIT_ERROR: u32 = 2108;
pub const MQRC_SUPPRESSED_BY_EXIT: u32 = 2109;
pub const MQRC_FORMAT_ERROR: u32 = 2110;
pub const MQRC_SOURCE_CCSID_ERROR: u32 = 2111;
pub const MQRC_SOURCE_INTEGER_ENC_ERROR: u32 = 2112;
pub const MQRC_SOURCE_DECIMAL_ENC_ERROR: u32 = 2113;
pub const MQRC_SOURCE_FLOAT_ENC_ERROR: u32 = 2114;
pub const MQRC_TARGET_CCSID_ERROR: u32 = 2115;
pub const MQRC_TARGET_INTEGER_ENC_ERROR: u32 = 2116;
pub const MQRC_TARGET_DECIMAL_ENC_ERROR: u32 = 2117;
pub const MQRC_TARGET_FLOAT_ENC_ERROR: u32 = 2118;
pub const MQRC_NOT_CONVERTED: u32 = 2119;
pub const MQRC_CONVERTED_MSG_TOO_BIG: u32 = 2120;
pub const MQRC_TRUNCATED: u32 = 2120;
pub const MQRC_NO_EXTERNAL_PARTICIPANTS: u32 = 2121;
pub const MQRC_PARTICIPANT_NOT_AVAILABLE: u32 = 2122;
pub const MQRC_OUTCOME_MIXED: u32 = 2123;
pub const MQRC_OUTCOME_PENDING: u32 = 2124;
pub const MQRC_BRIDGE_STARTED: u32 = 2125;
pub const MQRC_BRIDGE_STOPPED: u32 = 2126;
pub const MQRC_ADAPTER_STORAGE_SHORTAGE: u32 = 2127;
pub const MQRC_UOW_IN_PROGRESS: u32 = 2128;
pub const MQRC_ADAPTER_CONN_LOAD_ERROR: u32 = 2129;
pub const MQRC_ADAPTER_SERV_LOAD_ERROR: u32 = 2130;
pub const MQRC_ADAPTER_DEFS_ERROR: u32 = 2131;
pub const MQRC_ADAPTER_DEFS_LOAD_ERROR: u32 = 2132;
pub const MQRC_ADAPTER_CONV_LOAD_ERROR: u32 = 2133;
pub const MQRC_BO_ERROR: u32 = 2134;
pub const MQRC_DH_ERROR: u32 = 2135;
pub const MQRC_MULTIPLE_REASONS: u32 = 2136;
pub const MQRC_OPEN_FAILED: u32 = 2137;
pub const MQRC_ADAPTER_DISC_LOAD_ERROR: u32 = 2138;
pub const MQRC_CNO_ERROR: u32 = 2139;
pub const MQRC_CICS_WAIT_FAILED: u32 = 2140;
pub const MQRC_DLH_ERROR: u32 = 2141;
pub const MQRC_HEADER_ERROR: u32 = 2142;
pub const MQRC_SOURCE_LENGTH_ERROR: u32 = 2143;
pub const MQRC_TARGET_LENGTH_ERROR: u32 = 2144;
pub const MQRC_SOURCE_BUFFER_ERROR: u32 = 2145;
pub const MQRC_TARGET_BUFFER_ERROR: u32 = 2146;
pub const MQRC_INCOMPLETE_TRANSACTION: u32 = 2147;
pub const MQRC_IIH_ERROR: u32 = 2148;
pub const MQRC_PCF_ERROR: u32 = 2149;
pub const MQRC_DBCS_ERROR: u32 = 2150;
pub const MQRC_OBJECT_NAME_ERROR: u32 = 2152;
pub const MQRC_OBJECT_Q_MGR_NAME_ERROR: u32 = 2153;
pub const MQRC_RECS_PRESENT_ERROR: u32 = 2154;
pub const MQRC_OBJECT_RECORDS_ERROR: u32 = 2155;
pub const MQRC_RESPONSE_RECORDS_ERROR: u32 = 2156;
pub const MQRC_ASID_MISMATCH: u32 = 2157;
pub const MQRC_PMO_RECORD_FLAGS_ERROR: u32 = 2158;
pub const MQRC_PUT_MSG_RECORDS_ERROR: u32 = 2159;
pub const MQRC_CONN_ID_IN_USE: u32 = 2160;
pub const MQRC_Q_MGR_QUIESCING: u32 = 2161;
pub const MQRC_Q_MGR_STOPPING: u32 = 2162;
pub const MQRC_DUPLICATE_RECOV_COORD: u32 = 2163;
pub const MQRC_PMO_ERROR: u32 = 2173;
pub const MQRC_API_EXIT_NOT_FOUND: u32 = 2182;
pub const MQRC_API_EXIT_LOAD_ERROR: u32 = 2183;
pub const MQRC_REMOTE_Q_NAME_ERROR: u32 = 2184;
pub const MQRC_INCONSISTENT_PERSISTENCE: u32 = 2185;
pub const MQRC_GMO_ERROR: u32 = 2186;
pub const MQRC_CICS_BRIDGE_RESTRICTION: u32 = 2187;
pub const MQRC_STOPPED_BY_CLUSTER_EXIT: u32 = 2188;
pub const MQRC_CLUSTER_RESOLUTION_ERROR: u32 = 2189;
pub const MQRC_CONVERTED_STRING_TOO_BIG: u32 = 2190;
pub const MQRC_TMC_ERROR: u32 = 2191;
pub const MQRC_STORAGE_MEDIUM_FULL: u32 = 2192;
pub const MQRC_PAGESET_FULL: u32 = 2192;
pub const MQRC_PAGESET_ERROR: u32 = 2193;
pub const MQRC_NAME_NOT_VALID_FOR_TYPE: u32 = 2194;
pub const MQRC_UNEXPECTED_ERROR: u32 = 2195;
pub const MQRC_UNKNOWN_XMIT_Q: u32 = 2196;
pub const MQRC_UNKNOWN_DEF_XMIT_Q: u32 = 2197;
pub const MQRC_DEF_XMIT_Q_TYPE_ERROR: u32 = 2198;
pub const MQRC_DEF_XMIT_Q_USAGE_ERROR: u32 = 2199;
pub const MQRC_MSG_MARKED_BROWSE_CO_OP: u32 = 2200;
pub const MQRC_NAME_IN_USE: u32 = 2201;
pub const MQRC_CONNECTION_QUIESCING: u32 = 2202;
pub const MQRC_CONNECTION_STOPPING: u32 = 2203;
pub const MQRC_ADAPTER_NOT_AVAILABLE: u32 = 2204;
pub const MQRC_MSG_ID_ERROR: u32 = 2206;
pub const MQRC_CORREL_ID_ERROR: u32 = 2207;
pub const MQRC_FILE_SYSTEM_ERROR: u32 = 2208;
pub const MQRC_NO_MSG_LOCKED: u32 = 2209;
pub const MQRC_SOAP_DOTNET_ERROR: u32 = 2210;
pub const MQRC_SOAP_AXIS_ERROR: u32 = 2211;
pub const MQRC_SOAP_URL_ERROR: u32 = 2212;
pub const MQRC_FILE_NOT_AUDITED: u32 = 2216;
pub const MQRC_CONNECTION_NOT_AUTHORIZED: u32 = 2217;
pub const MQRC_MSG_TOO_BIG_FOR_CHANNEL: u32 = 2218;
pub const MQRC_CALL_IN_PROGRESS: u32 = 2219;
pub const MQRC_RMH_ERROR: u32 = 2220;
pub const MQRC_Q_MGR_ACTIVE: u32 = 2222;
pub const MQRC_Q_MGR_NOT_ACTIVE: u32 = 2223;
pub const MQRC_Q_DEPTH_HIGH: u32 = 2224;
pub const MQRC_Q_DEPTH_LOW: u32 = 2225;
pub const MQRC_Q_SERVICE_INTERVAL_HIGH: u32 = 2226;
pub const MQRC_Q_SERVICE_INTERVAL_OK: u32 = 2227;
pub const MQRC_RFH_HEADER_FIELD_ERROR: u32 = 2228;
pub const MQRC_RAS_PROPERTY_ERROR: u32 = 2229;
pub const MQRC_UNIT_OF_WORK_NOT_STARTED: u32 = 2232;
pub const MQRC_CHANNEL_AUTO_DEF_OK: u32 = 2233;
pub const MQRC_CHANNEL_AUTO_DEF_ERROR: u32 = 2234;
pub const MQRC_CFH_ERROR: u32 = 2235;
pub const MQRC_CFIL_ERROR: u32 = 2236;
pub const MQRC_CFIN_ERROR: u32 = 2237;
pub const MQRC_CFSL_ERROR: u32 = 2238;
pub const MQRC_CFST_ERROR: u32 = 2239;
pub const MQRC_INCOMPLETE_GROUP: u32 = 2241;
pub const MQRC_INCOMPLETE_MSG: u32 = 2242;
pub const MQRC_INCONSISTENT_CCSIDS: u32 = 2243;
pub const MQRC_INCONSISTENT_ENCODINGS: u32 = 2244;
pub const MQRC_INCONSISTENT_UOW: u32 = 2245;
pub const MQRC_INVALID_MSG_UNDER_CURSOR: u32 = 2246;
pub const MQRC_MATCH_OPTIONS_ERROR: u32 = 2247;
pub const MQRC_MDE_ERROR: u32 = 2248;
pub const MQRC_MSG_FLAGS_ERROR: u32 = 2249;
pub const MQRC_MSG_SEQ_NUMBER_ERROR: u32 = 2250;
pub const MQRC_OFFSET_ERROR: u32 = 2251;
pub const MQRC_ORIGINAL_LENGTH_ERROR: u32 = 2252;
pub const MQRC_SEGMENT_LENGTH_ZERO: u32 = 2253;
pub const MQRC_UOW_NOT_AVAILABLE: u32 = 2255;
pub const MQRC_WRONG_GMO_VERSION: u32 = 2256;
pub const MQRC_WRONG_MD_VERSION: u32 = 2257;
pub const MQRC_GROUP_ID_ERROR: u32 = 2258;
pub const MQRC_INCONSISTENT_BROWSE: u32 = 2259;
pub const MQRC_XQH_ERROR: u32 = 2260;
pub const MQRC_SRC_ENV_ERROR: u32 = 2261;
pub const MQRC_SRC_NAME_ERROR: u32 = 2262;
pub const MQRC_DEST_ENV_ERROR: u32 = 2263;
pub const MQRC_DEST_NAME_ERROR: u32 = 2264;
pub const MQRC_TM_ERROR: u32 = 2265;
pub const MQRC_CLUSTER_EXIT_ERROR: u32 = 2266;
pub const MQRC_CLUSTER_EXIT_LOAD_ERROR: u32 = 2267;
pub const MQRC_CLUSTER_PUT_INHIBITED: u32 = 2268;
pub const MQRC_CLUSTER_RESOURCE_ERROR: u32 = 2269;
pub const MQRC_NO_DESTINATIONS_AVAILABLE: u32 = 2270;
pub const MQRC_CONN_TAG_IN_USE: u32 = 2271;
pub const MQRC_PARTIALLY_CONVERTED: u32 = 2272;
pub const MQRC_CONNECTION_ERROR: u32 = 2273;
pub const MQRC_OPTION_ENVIRONMENT_ERROR: u32 = 2274;
pub const MQRC_CD_ERROR: u32 = 2277;
pub const MQRC_CLIENT_CONN_ERROR: u32 = 2278;
pub const MQRC_CHANNEL_STOPPED_BY_USER: u32 = 2279;
pub const MQRC_HCONFIG_ERROR: u32 = 2280;
pub const MQRC_FUNCTION_ERROR: u32 = 2281;
pub const MQRC_CHANNEL_STARTED: u32 = 2282;
pub const MQRC_CHANNEL_STOPPED: u32 = 2283;
pub const MQRC_CHANNEL_CONV_ERROR: u32 = 2284;
pub const MQRC_SERVICE_NOT_AVAILABLE: u32 = 2285;
pub const MQRC_INITIALIZATION_FAILED: u32 = 2286;
pub const MQRC_TERMINATION_FAILED: u32 = 2287;
pub const MQRC_UNKNOWN_Q_NAME: u32 = 2288;
pub const MQRC_SERVICE_ERROR: u32 = 2289;
pub const MQRC_Q_ALREADY_EXISTS: u32 = 2290;
pub const MQRC_USER_ID_NOT_AVAILABLE: u32 = 2291;
pub const MQRC_UNKNOWN_ENTITY: u32 = 2292;
pub const MQRC_UNKNOWN_AUTH_ENTITY: u32 = 2293;
pub const MQRC_UNKNOWN_REF_OBJECT: u32 = 2294;
pub const MQRC_CHANNEL_ACTIVATED: u32 = 2295;
pub const MQRC_CHANNEL_NOT_ACTIVATED: u32 = 2296;
pub const MQRC_UOW_CANCELED: u32 = 2297;
pub const MQRC_FUNCTION_NOT_SUPPORTED: u32 = 2298;
pub const MQRC_SELECTOR_TYPE_ERROR: u32 = 2299;
pub const MQRC_COMMAND_TYPE_ERROR: u32 = 2300;
pub const MQRC_MULTIPLE_INSTANCE_ERROR: u32 = 2301;
pub const MQRC_SYSTEM_ITEM_NOT_ALTERABLE: u32 = 2302;
pub const MQRC_BAG_CONVERSION_ERROR: u32 = 2303;
pub const MQRC_SELECTOR_OUT_OF_RANGE: u32 = 2304;
pub const MQRC_SELECTOR_NOT_UNIQUE: u32 = 2305;
pub const MQRC_INDEX_NOT_PRESENT: u32 = 2306;
pub const MQRC_STRING_ERROR: u32 = 2307;
pub const MQRC_ENCODING_NOT_SUPPORTED: u32 = 2308;
pub const MQRC_SELECTOR_NOT_PRESENT: u32 = 2309;
pub const MQRC_OUT_SELECTOR_ERROR: u32 = 2310;
pub const MQRC_STRING_TRUNCATED: u32 = 2311;
pub const MQRC_SELECTOR_WRONG_TYPE: u32 = 2312;
pub const MQRC_INCONSISTENT_ITEM_TYPE: u32 = 2313;
pub const MQRC_INDEX_ERROR: u32 = 2314;
pub const MQRC_SYSTEM_BAG_NOT_ALTERABLE: u32 = 2315;
pub const MQRC_ITEM_COUNT_ERROR: u32 = 2316;
pub const MQRC_FORMAT_NOT_SUPPORTED: u32 = 2317;
pub const MQRC_SELECTOR_NOT_SUPPORTED: u32 = 2318;
pub const MQRC_ITEM_VALUE_ERROR: u32 = 2319;
pub const MQRC_HBAG_ERROR: u32 = 2320;
pub const MQRC_PARAMETER_MISSING: u32 = 2321;
pub const MQRC_CMD_SERVER_NOT_AVAILABLE: u32 = 2322;
pub const MQRC_STRING_LENGTH_ERROR: u32 = 2323;
pub const MQRC_INQUIRY_COMMAND_ERROR: u32 = 2324;
pub const MQRC_NESTED_BAG_NOT_SUPPORTED: u32 = 2325;
pub const MQRC_BAG_WRONG_TYPE: u32 = 2326;
pub const MQRC_ITEM_TYPE_ERROR: u32 = 2327;
pub const MQRC_SYSTEM_BAG_NOT_DELETABLE: u32 = 2328;
pub const MQRC_SYSTEM_ITEM_NOT_DELETABLE: u32 = 2329;
pub const MQRC_CODED_CHAR_SET_ID_ERROR: u32 = 2330;
pub const MQRC_MSG_TOKEN_ERROR: u32 = 2331;
pub const MQRC_MISSING_WIH: u32 = 2332;
pub const MQRC_WIH_ERROR: u32 = 2333;
pub const MQRC_RFH_ERROR: u32 = 2334;
pub const MQRC_RFH_STRING_ERROR: u32 = 2335;
pub const MQRC_RFH_COMMAND_ERROR: u32 = 2336;
pub const MQRC_RFH_PARM_ERROR: u32 = 2337;
pub const MQRC_RFH_DUPLICATE_PARM: u32 = 2338;
pub const MQRC_RFH_PARM_MISSING: u32 = 2339;
pub const MQRC_CHAR_CONVERSION_ERROR: u32 = 2340;
pub const MQRC_UCS2_CONVERSION_ERROR: u32 = 2341;
pub const MQRC_DB2_NOT_AVAILABLE: u32 = 2342;
pub const MQRC_OBJECT_NOT_UNIQUE: u32 = 2343;
pub const MQRC_CONN_TAG_NOT_RELEASED: u32 = 2344;
pub const MQRC_CF_NOT_AVAILABLE: u32 = 2345;
pub const MQRC_CF_STRUC_IN_USE: u32 = 2346;
pub const MQRC_CF_STRUC_LIST_HDR_IN_USE: u32 = 2347;
pub const MQRC_CF_STRUC_AUTH_FAILED: u32 = 2348;
pub const MQRC_CF_STRUC_ERROR: u32 = 2349;
pub const MQRC_CONN_TAG_NOT_USABLE: u32 = 2350;
pub const MQRC_GLOBAL_UOW_CONFLICT: u32 = 2351;
pub const MQRC_LOCAL_UOW_CONFLICT: u32 = 2352;
pub const MQRC_HANDLE_IN_USE_FOR_UOW: u32 = 2353;
pub const MQRC_UOW_ENLISTMENT_ERROR: u32 = 2354;
pub const MQRC_UOW_MIX_NOT_SUPPORTED: u32 = 2355;
pub const MQRC_WXP_ERROR: u32 = 2356;
pub const MQRC_CURRENT_RECORD_ERROR: u32 = 2357;
pub const MQRC_NEXT_OFFSET_ERROR: u32 = 2358;
pub const MQRC_NO_RECORD_AVAILABLE: u32 = 2359;
pub const MQRC_OBJECT_LEVEL_INCOMPATIBLE: u32 = 2360;
pub const MQRC_NEXT_RECORD_ERROR: u32 = 2361;
pub const MQRC_BACKOUT_THRESHOLD_REACHED: u32 = 2362;
pub const MQRC_MSG_NOT_MATCHED: u32 = 2363;
pub const MQRC_JMS_FORMAT_ERROR: u32 = 2364;
pub const MQRC_SEGMENTS_NOT_SUPPORTED: u32 = 2365;
pub const MQRC_WRONG_CF_LEVEL: u32 = 2366;
pub const MQRC_CONFIG_CREATE_OBJECT: u32 = 2367;
pub const MQRC_CONFIG_CHANGE_OBJECT: u32 = 2368;
pub const MQRC_CONFIG_DELETE_OBJECT: u32 = 2369;
pub const MQRC_CONFIG_REFRESH_OBJECT: u32 = 2370;
pub const MQRC_CHANNEL_SSL_ERROR: u32 = 2371;
pub const MQRC_PARTICIPANT_NOT_DEFINED: u32 = 2372;
pub const MQRC_CF_STRUC_FAILED: u32 = 2373;
pub const MQRC_API_EXIT_ERROR: u32 = 2374;
pub const MQRC_API_EXIT_INIT_ERROR: u32 = 2375;
pub const MQRC_API_EXIT_TERM_ERROR: u32 = 2376;
pub const MQRC_EXIT_REASON_ERROR: u32 = 2377;
pub const MQRC_RESERVED_VALUE_ERROR: u32 = 2378;
pub const MQRC_NO_DATA_AVAILABLE: u32 = 2379;
pub const MQRC_SCO_ERROR: u32 = 2380;
pub const MQRC_KEY_REPOSITORY_ERROR: u32 = 2381;
pub const MQRC_CRYPTO_HARDWARE_ERROR: u32 = 2382;
pub const MQRC_AUTH_INFO_REC_COUNT_ERROR: u32 = 2383;
pub const MQRC_AUTH_INFO_REC_ERROR: u32 = 2384;
pub const MQRC_AIR_ERROR: u32 = 2385;
pub const MQRC_AUTH_INFO_TYPE_ERROR: u32 = 2386;
pub const MQRC_AUTH_INFO_CONN_NAME_ERROR: u32 = 2387;
pub const MQRC_LDAP_USER_NAME_ERROR: u32 = 2388;
pub const MQRC_LDAP_USER_NAME_LENGTH_ERR: u32 = 2389;
pub const MQRC_LDAP_PASSWORD_ERROR: u32 = 2390;
pub const MQRC_SSL_ALREADY_INITIALIZED: u32 = 2391;
pub const MQRC_SSL_CONFIG_ERROR: u32 = 2392;
pub const MQRC_SSL_INITIALIZATION_ERROR: u32 = 2393;
pub const MQRC_Q_INDEX_TYPE_ERROR: u32 = 2394;
pub const MQRC_CFBS_ERROR: u32 = 2395;
pub const MQRC_SSL_NOT_ALLOWED: u32 = 2396;
pub const MQRC_JSSE_ERROR: u32 = 2397;
pub const MQRC_SSL_PEER_NAME_MISMATCH: u32 = 2398;
pub const MQRC_SSL_PEER_NAME_ERROR: u32 = 2399;
pub const MQRC_UNSUPPORTED_CIPHER_SUITE: u32 = 2400;
pub const MQRC_SSL_CERTIFICATE_REVOKED: u32 = 2401;
pub const MQRC_SSL_CERT_STORE_ERROR: u32 = 2402;
pub const MQRC_CLIENT_EXIT_LOAD_ERROR: u32 = 2406;
pub const MQRC_CLIENT_EXIT_ERROR: u32 = 2407;
pub const MQRC_UOW_COMMITTED: u32 = 2408;
pub const MQRC_SSL_KEY_RESET_ERROR: u32 = 2409;
pub const MQRC_UNKNOWN_COMPONENT_NAME: u32 = 2410;
pub const MQRC_LOGGER_STATUS: u32 = 2411;
pub const MQRC_COMMAND_MQSC: u32 = 2412;
pub const MQRC_COMMAND_PCF: u32 = 2413;
pub const MQRC_CFIF_ERROR: u32 = 2414;
pub const MQRC_CFSF_ERROR: u32 = 2415;
pub const MQRC_CFGR_ERROR: u32 = 2416;
pub const MQRC_MSG_NOT_ALLOWED_IN_GROUP: u32 = 2417;
pub const MQRC_FILTER_OPERATOR_ERROR: u32 = 2418;
pub const MQRC_NESTED_SELECTOR_ERROR: u32 = 2419;
pub const MQRC_EPH_ERROR: u32 = 2420;
pub const MQRC_RFH_FORMAT_ERROR: u32 = 2421;
pub const MQRC_CFBF_ERROR: u32 = 2422;
pub const MQRC_CLIENT_CHANNEL_CONFLICT: u32 = 2423;
pub const MQRC_SD_ERROR: u32 = 2424;
pub const MQRC_TOPIC_STRING_ERROR: u32 = 2425;
pub const MQRC_STS_ERROR: u32 = 2426;
pub const MQRC_NO_SUBSCRIPTION: u32 = 2428;
pub const MQRC_SUBSCRIPTION_IN_USE: u32 = 2429;
pub const MQRC_STAT_TYPE_ERROR: u32 = 2430;
pub const MQRC_SUB_USER_DATA_ERROR: u32 = 2431;
pub const MQRC_SUB_ALREADY_EXISTS: u32 = 2432;
pub const MQRC_IDENTITY_MISMATCH: u32 = 2434;
pub const MQRC_ALTER_SUB_ERROR: u32 = 2435;
pub const MQRC_DURABILITY_NOT_ALLOWED: u32 = 2436;
pub const MQRC_NO_RETAINED_MSG: u32 = 2437;
pub const MQRC_SRO_ERROR: u32 = 2438;
pub const MQRC_SUB_NAME_ERROR: u32 = 2440;
pub const MQRC_OBJECT_STRING_ERROR: u32 = 2441;
pub const MQRC_PROPERTY_NAME_ERROR: u32 = 2442;
pub const MQRC_SEGMENTATION_NOT_ALLOWED: u32 = 2443;
pub const MQRC_CBD_ERROR: u32 = 2444;
pub const MQRC_CTLO_ERROR: u32 = 2445;
pub const MQRC_NO_CALLBACKS_ACTIVE: u32 = 2446;
pub const MQRC_CALLBACK_NOT_REGISTERED: u32 = 2448;
pub const MQRC_OPTIONS_CHANGED: u32 = 2457;
pub const MQRC_READ_AHEAD_MSGS: u32 = 2458;
pub const MQRC_SELECTOR_SYNTAX_ERROR: u32 = 2459;
pub const MQRC_HMSG_ERROR: u32 = 2460;
pub const MQRC_CMHO_ERROR: u32 = 2461;
pub const MQRC_DMHO_ERROR: u32 = 2462;
pub const MQRC_SMPO_ERROR: u32 = 2463;
pub const MQRC_IMPO_ERROR: u32 = 2464;
pub const MQRC_PROPERTY_NAME_TOO_BIG: u32 = 2465;
pub const MQRC_PROP_VALUE_NOT_CONVERTED: u32 = 2466;
pub const MQRC_PROP_TYPE_NOT_SUPPORTED: u32 = 2467;
pub const MQRC_PROPERTY_VALUE_TOO_BIG: u32 = 2469;
pub const MQRC_PROP_CONV_NOT_SUPPORTED: u32 = 2470;
pub const MQRC_PROPERTY_NOT_AVAILABLE: u32 = 2471;
pub const MQRC_PROP_NUMBER_FORMAT_ERROR: u32 = 2472;
pub const MQRC_PROPERTY_TYPE_ERROR: u32 = 2473;
pub const MQRC_PROPERTIES_TOO_BIG: u32 = 2478;
pub const MQRC_PUT_NOT_RETAINED: u32 = 2479;
pub const MQRC_ALIAS_TARGTYPE_CHANGED: u32 = 2480;
pub const MQRC_DMPO_ERROR: u32 = 2481;
pub const MQRC_PD_ERROR: u32 = 2482;
pub const MQRC_CALLBACK_TYPE_ERROR: u32 = 2483;
pub const MQRC_CBD_OPTIONS_ERROR: u32 = 2484;
pub const MQRC_MAX_MSG_LENGTH_ERROR: u32 = 2485;
pub const MQRC_CALLBACK_ROUTINE_ERROR: u32 = 2486;
pub const MQRC_CALLBACK_LINK_ERROR: u32 = 2487;
pub const MQRC_OPERATION_ERROR: u32 = 2488;
pub const MQRC_BMHO_ERROR: u32 = 2489;
pub const MQRC_UNSUPPORTED_PROPERTY: u32 = 2490;
pub const MQRC_PROP_NAME_NOT_CONVERTED: u32 = 2492;
pub const MQRC_GET_ENABLED: u32 = 2494;
pub const MQRC_MODULE_NOT_FOUND: u32 = 2495;
pub const MQRC_MODULE_INVALID: u32 = 2496;
pub const MQRC_MODULE_ENTRY_NOT_FOUND: u32 = 2497;
pub const MQRC_MIXED_CONTENT_NOT_ALLOWED: u32 = 2498;
pub const MQRC_MSG_HANDLE_IN_USE: u32 = 2499;
pub const MQRC_HCONN_ASYNC_ACTIVE: u32 = 2500;
pub const MQRC_MHBO_ERROR: u32 = 2501;
pub const MQRC_PUBLICATION_FAILURE: u32 = 2502;
pub const MQRC_SUB_INHIBITED: u32 = 2503;
pub const MQRC_SELECTOR_ALWAYS_FALSE: u32 = 2504;
pub const MQRC_XEPO_ERROR: u32 = 2507;
pub const MQRC_DURABILITY_NOT_ALTERABLE: u32 = 2509;
pub const MQRC_TOPIC_NOT_ALTERABLE: u32 = 2510;
pub const MQRC_SUBLEVEL_NOT_ALTERABLE: u32 = 2512;
pub const MQRC_PROPERTY_NAME_LENGTH_ERR: u32 = 2513;
pub const MQRC_DUPLICATE_GROUP_SUB: u32 = 2514;
pub const MQRC_GROUPING_NOT_ALTERABLE: u32 = 2515;
pub const MQRC_SELECTOR_INVALID_FOR_TYPE: u32 = 2516;
pub const MQRC_HOBJ_QUIESCED: u32 = 2517;
pub const MQRC_HOBJ_QUIESCED_NO_MSGS: u32 = 2518;
pub const MQRC_SELECTION_STRING_ERROR: u32 = 2519;
pub const MQRC_RES_OBJECT_STRING_ERROR: u32 = 2520;
pub const MQRC_CONNECTION_SUSPENDED: u32 = 2521;
pub const MQRC_INVALID_DESTINATION: u32 = 2522;
pub const MQRC_INVALID_SUBSCRIPTION: u32 = 2523;
pub const MQRC_SELECTOR_NOT_ALTERABLE: u32 = 2524;
pub const MQRC_RETAINED_MSG_Q_ERROR: u32 = 2525;
pub const MQRC_RETAINED_NOT_DELIVERED: u32 = 2526;
pub const MQRC_RFH_RESTRICTED_FORMAT_ERR: u32 = 2527;
pub const MQRC_CONNECTION_STOPPED: u32 = 2528;
pub const MQRC_ASYNC_UOW_CONFLICT: u32 = 2529;
pub const MQRC_ASYNC_XA_CONFLICT: u32 = 2530;
pub const MQRC_PUBSUB_INHIBITED: u32 = 2531;
pub const MQRC_MSG_HANDLE_COPY_FAILURE: u32 = 2532;
pub const MQRC_DEST_CLASS_NOT_ALTERABLE: u32 = 2533;
pub const MQRC_OPERATION_NOT_ALLOWED: u32 = 2534;
pub const MQRC_ACTION_ERROR: u32 = 2535;
pub const MQRC_CHANNEL_NOT_AVAILABLE: u32 = 2537;
pub const MQRC_HOST_NOT_AVAILABLE: u32 = 2538;
pub const MQRC_CHANNEL_CONFIG_ERROR: u32 = 2539;
pub const MQRC_UNKNOWN_CHANNEL_NAME: u32 = 2540;
pub const MQRC_LOOPING_PUBLICATION: u32 = 2541;
pub const MQRC_ALREADY_JOINED: u32 = 2542;
pub const MQRC_STANDBY_Q_MGR: u32 = 2543;
pub const MQRC_RECONNECTING: u32 = 2544;
pub const MQRC_RECONNECTED: u32 = 2545;
pub const MQRC_RECONNECT_QMID_MISMATCH: u32 = 2546;
pub const MQRC_RECONNECT_INCOMPATIBLE: u32 = 2547;
pub const MQRC_RECONNECT_FAILED: u32 = 2548;
pub const MQRC_CALL_INTERRUPTED: u32 = 2549;
pub const MQRC_NO_SUBS_MATCHED: u32 = 2550;
pub const MQRC_SELECTION_NOT_AVAILABLE: u32 = 2551;
pub const MQRC_CHANNEL_SSL_WARNING: u32 = 2552;
pub const MQRC_OCSP_URL_ERROR: u32 = 2553;
pub const MQRC_CONTENT_ERROR: u32 = 2554;
pub const MQRC_RECONNECT_Q_MGR_REQD: u32 = 2555;
pub const MQRC_RECONNECT_TIMED_OUT: u32 = 2556;
pub const MQRC_PUBLISH_EXIT_ERROR: u32 = 2557;
pub const MQRC_COMMINFO_ERROR: u32 = 2558;
pub const MQRC_DEF_SYNCPOINT_INHIBITED: u32 = 2559;
pub const MQRC_MULTICAST_ONLY: u32 = 2560;
pub const MQRC_DATA_SET_NOT_AVAILABLE: u32 = 2561;
pub const MQRC_GROUPING_NOT_ALLOWED: u32 = 2562;
pub const MQRC_GROUP_ADDRESS_ERROR: u32 = 2563;
pub const MQRC_MULTICAST_CONFIG_ERROR: u32 = 2564;
pub const MQRC_MULTICAST_INTERFACE_ERROR: u32 = 2565;
pub const MQRC_MULTICAST_SEND_ERROR: u32 = 2566;
pub const MQRC_MULTICAST_INTERNAL_ERROR: u32 = 2567;
pub const MQRC_CONNECTION_NOT_AVAILABLE: u32 = 2568;
pub const MQRC_SYNCPOINT_NOT_ALLOWED: u32 = 2569;
pub const MQRC_SSL_ALT_PROVIDER_REQUIRED: u32 = 2570;
pub const MQRC_MCAST_PUB_STATUS: u32 = 2571;
pub const MQRC_MCAST_SUB_STATUS: u32 = 2572;
pub const MQRC_PRECONN_EXIT_LOAD_ERROR: u32 = 2573;
pub const MQRC_PRECONN_EXIT_NOT_FOUND: u32 = 2574;
pub const MQRC_PRECONN_EXIT_ERROR: u32 = 2575;
pub const MQRC_CD_ARRAY_ERROR: u32 = 2576;
pub const MQRC_CHANNEL_BLOCKED: u32 = 2577;
pub const MQRC_CHANNEL_BLOCKED_WARNING: u32 = 2578;
pub const MQRC_SUBSCRIPTION_CREATE: u32 = 2579;
pub const MQRC_SUBSCRIPTION_DELETE: u32 = 2580;
pub const MQRC_SUBSCRIPTION_CHANGE: u32 = 2581;
pub const MQRC_SUBSCRIPTION_REFRESH: u32 = 2582;
pub const MQRC_INSTALLATION_MISMATCH: u32 = 2583;
pub const MQRC_NOT_PRIVILEGED: u32 = 2584;
pub const MQRC_PROPERTIES_DISABLED: u32 = 2586;
pub const MQRC_HMSG_NOT_AVAILABLE: u32 = 2587;
pub const MQRC_EXIT_PROPS_NOT_SUPPORTED: u32 = 2588;
pub const MQRC_INSTALLATION_MISSING: u32 = 2589;
pub const MQRC_FASTPATH_NOT_AVAILABLE: u32 = 2590;
pub const MQRC_CIPHER_SPEC_NOT_SUITE_B: u32 = 2591;
pub const MQRC_SUITE_B_ERROR: u32 = 2592;
pub const MQRC_CERT_VAL_POLICY_ERROR: u32 = 2593;
pub const MQRC_PASSWORD_PROTECTION_ERROR: u32 = 2594;
pub const MQRC_CSP_ERROR: u32 = 2595;
pub const MQRC_CERT_LABEL_NOT_ALLOWED: u32 = 2596;
pub const MQRC_ADMIN_TOPIC_STRING_ERROR: u32 = 2598;
pub const MQRC_AMQP_NOT_AVAILABLE: u32 = 2599;
pub const MQRC_CCDT_URL_ERROR: u32 = 2600;
pub const MQRC_Q_MGR_RECONNECT_REQUESTED: u32 = 2601;
pub const MQRC_REOPEN_EXCL_INPUT_ERROR: u32 = 6100;
pub const MQRC_REOPEN_INQUIRE_ERROR: u32 = 6101;
pub const MQRC_REOPEN_SAVED_CONTEXT_ERR: u32 = 6102;
pub const MQRC_REOPEN_TEMPORARY_Q_ERROR: u32 = 6103;
pub const MQRC_ATTRIBUTE_LOCKED: u32 = 6104;
pub const MQRC_CURSOR_NOT_VALID: u32 = 6105;
pub const MQRC_ENCODING_ERROR: u32 = 6106;
pub const MQRC_STRUC_ID_ERROR: u32 = 6107;
pub const MQRC_NULL_POINTER: u32 = 6108;
pub const MQRC_NO_CONNECTION_REFERENCE: u32 = 6109;
pub const MQRC_NO_BUFFER: u32 = 6110;
pub const MQRC_BINARY_DATA_LENGTH_ERROR: u32 = 6111;
pub const MQRC_BUFFER_NOT_AUTOMATIC: u32 = 6112;
pub const MQRC_INSUFFICIENT_BUFFER: u32 = 6113;
pub const MQRC_INSUFFICIENT_DATA: u32 = 6114;
pub const MQRC_DATA_TRUNCATED: u32 = 6115;
pub const MQRC_ZERO_LENGTH: u32 = 6116;
pub const MQRC_NEGATIVE_LENGTH: u32 = 6117;
pub const MQRC_NEGATIVE_OFFSET: u32 = 6118;
pub const MQRC_INCONSISTENT_FORMAT: u32 = 6119;
pub const MQRC_INCONSISTENT_OBJECT_STATE: u32 = 6120;
pub const MQRC_CONTEXT_OBJECT_NOT_VALID: u32 = 6121;
pub const MQRC_CONTEXT_OPEN_ERROR: u32 = 6122;
pub const MQRC_STRUC_LENGTH_ERROR: u32 = 6123;
pub const MQRC_NOT_CONNECTED: u32 = 6124;
pub const MQRC_NOT_OPEN: u32 = 6125;
pub const MQRC_DISTRIBUTION_LIST_EMPTY: u32 = 6126;
pub const MQRC_INCONSISTENT_OPEN_OPTIONS: u32 = 6127;
pub const MQRC_WRONG_VERSION: u32 = 6128;
pub const MQRC_REFERENCE_ERROR: u32 = 6129;
pub const MQRC_XR_NOT_AVAILABLE: u32 = 6130;
pub const MQRC_SUB_JOIN_NOT_ALTERABLE: u32 = 29440;
pub const MQQT_LOCAL: u32 = 1;
pub const MQQT_MODEL: u32 = 2;
pub const MQQT_ALIAS: u32 = 3;
pub const MQQT_REMOTE: u32 = 6;
pub const MQQT_CLUSTER: u32 = 7;
pub const MQCQT_LOCAL_Q: u32 = 1;
pub const MQCQT_ALIAS_Q: u32 = 2;
pub const MQCQT_REMOTE_Q: u32 = 3;
pub const MQCQT_Q_MGR_ALIAS: u32 = 4;
pub const MQQT_ALL: u32 = 1001;
pub const MQQDT_PREDEFINED: u32 = 1;
pub const MQQDT_PERMANENT_DYNAMIC: u32 = 2;
pub const MQQDT_TEMPORARY_DYNAMIC: u32 = 3;
pub const MQQDT_SHARED_DYNAMIC: u32 = 4;
pub const MQQA_GET_INHIBITED: u32 = 1;
pub const MQQA_GET_ALLOWED: u32 = 0;
pub const MQQA_PUT_INHIBITED: u32 = 1;
pub const MQQA_PUT_ALLOWED: u32 = 0;
pub const MQQA_SHAREABLE: u32 = 1;
pub const MQQA_NOT_SHAREABLE: u32 = 0;
pub const MQQA_BACKOUT_HARDENED: u32 = 1;
pub const MQQA_BACKOUT_NOT_HARDENED: u32 = 0;
pub const MQMDS_PRIORITY: u32 = 0;
pub const MQMDS_FIFO: u32 = 1;
pub const MQNPM_CLASS_NORMAL: u32 = 0;
pub const MQNPM_CLASS_HIGH: u32 = 10;
pub const MQTC_OFF: u32 = 0;
pub const MQTC_ON: u32 = 1;
pub const MQTT_NONE: u32 = 0;
pub const MQTT_FIRST: u32 = 1;
pub const MQTT_EVERY: u32 = 2;
pub const MQTT_DEPTH: u32 = 3;
pub const MQTRIGGER_RESTART_NO: u32 = 0;
pub const MQTRIGGER_RESTART_YES: u32 = 1;
pub const MQUS_NORMAL: u32 = 0;
pub const MQUS_TRANSMISSION: u32 = 1;
pub const MQDL_SUPPORTED: u32 = 1;
pub const MQDL_NOT_SUPPORTED: u32 = 0;
pub const MQIT_NONE: u32 = 0;
pub const MQIT_MSG_ID: u32 = 1;
pub const MQIT_CORREL_ID: u32 = 2;
pub const MQIT_MSG_TOKEN: u32 = 4;
pub const MQIT_GROUP_ID: u32 = 5;
pub const MQBND_BIND_ON_OPEN: u32 = 0;
pub const MQBND_BIND_NOT_FIXED: u32 = 1;
pub const MQBND_BIND_ON_GROUP: u32 = 2;
pub const MQQSGD_ALL: i32 = -1;
pub const MQQSGD_Q_MGR: u32 = 0;
pub const MQQSGD_COPY: u32 = 1;
pub const MQQSGD_SHARED: u32 = 2;
pub const MQQSGD_GROUP: u32 = 3;
pub const MQQSGD_PRIVATE: u32 = 4;
pub const MQQSGD_LIVE: u32 = 6;
pub const MQREORG_DISABLED: u32 = 0;
pub const MQREORG_ENABLED: u32 = 1;
pub const MQQFS_DEFAULT: i32 = -1;
pub const MQREADA_NO: u32 = 0;
pub const MQREADA_YES: u32 = 1;
pub const MQREADA_DISABLED: u32 = 2;
pub const MQREADA_INHIBITED: u32 = 3;
pub const MQREADA_BACKLOG: u32 = 4;
pub const MQPROP_COMPATIBILITY: u32 = 0;
pub const MQPROP_NONE: u32 = 1;
pub const MQPROP_ALL: u32 = 2;
pub const MQPROP_FORCE_MQRFH2: u32 = 3;
pub const MQPROP_V6COMPAT: u32 = 4;
pub const MQNC_MAX_NAMELIST_NAME_COUNT: u32 = 256;
pub const MQNT_NONE: u32 = 0;
pub const MQNT_Q: u32 = 1;
pub const MQNT_CLUSTER: u32 = 2;
pub const MQNT_AUTH_INFO: u32 = 4;
pub const MQNT_ALL: u32 = 1001;
pub const MQCFR_YES: u32 = 1;
pub const MQCFR_NO: u32 = 0;
pub const MQRECAUTO_NO: u32 = 0;
pub const MQRECAUTO_YES: u32 = 1;
pub const MQCFCONLOS_TERMINATE: u32 = 0;
pub const MQCFCONLOS_TOLERATE: u32 = 1;
pub const MQCFCONLOS_ASQMGR: u32 = 2;
pub const MQSVC_TYPE_COMMAND: u32 = 0;
pub const MQSVC_TYPE_SERVER: u32 = 1;
pub const MQADOPT_CHECK_NONE: u32 = 0;
pub const MQADOPT_CHECK_ALL: u32 = 1;
pub const MQADOPT_CHECK_Q_MGR_NAME: u32 = 2;
pub const MQADOPT_CHECK_NET_ADDR: u32 = 4;
pub const MQADOPT_CHECK_CHANNEL_NAME: u32 = 8;
pub const MQADOPT_TYPE_NO: u32 = 0;
pub const MQADOPT_TYPE_ALL: u32 = 1;
pub const MQADOPT_TYPE_SVR: u32 = 2;
pub const MQADOPT_TYPE_SDR: u32 = 4;
pub const MQADOPT_TYPE_RCVR: u32 = 8;
pub const MQADOPT_TYPE_CLUSRCVR: u32 = 16;
pub const MQAUTO_START_NO: u32 = 0;
pub const MQAUTO_START_YES: u32 = 1;
pub const MQCHAD_DISABLED: u32 = 0;
pub const MQCHAD_ENABLED: u32 = 1;
pub const MQCLWL_USEQ_LOCAL: u32 = 0;
pub const MQCLWL_USEQ_ANY: u32 = 1;
pub const MQCLWL_USEQ_AS_Q_MGR: i32 = -3;
pub const MQCMDL_LEVEL_1: u32 = 100;
pub const MQCMDL_LEVEL_101: u32 = 101;
pub const MQCMDL_LEVEL_110: u32 = 110;
pub const MQCMDL_LEVEL_114: u32 = 114;
pub const MQCMDL_LEVEL_120: u32 = 120;
pub const MQCMDL_LEVEL_200: u32 = 200;
pub const MQCMDL_LEVEL_201: u32 = 201;
pub const MQCMDL_LEVEL_210: u32 = 210;
pub const MQCMDL_LEVEL_211: u32 = 211;
pub const MQCMDL_LEVEL_220: u32 = 220;
pub const MQCMDL_LEVEL_221: u32 = 221;
pub const MQCMDL_LEVEL_230: u32 = 230;
pub const MQCMDL_LEVEL_320: u32 = 320;
pub const MQCMDL_LEVEL_420: u32 = 420;
pub const MQCMDL_LEVEL_500: u32 = 500;
pub const MQCMDL_LEVEL_510: u32 = 510;
pub const MQCMDL_LEVEL_520: u32 = 520;
pub const MQCMDL_LEVEL_530: u32 = 530;
pub const MQCMDL_LEVEL_531: u32 = 531;
pub const MQCMDL_LEVEL_600: u32 = 600;
pub const MQCMDL_LEVEL_700: u32 = 700;
pub const MQCMDL_LEVEL_701: u32 = 701;
pub const MQCMDL_LEVEL_710: u32 = 710;
pub const MQCMDL_LEVEL_711: u32 = 711;
pub const MQCMDL_LEVEL_750: u32 = 750;
pub const MQCMDL_LEVEL_800: u32 = 800;
pub const MQCMDL_LEVEL_801: u32 = 801;
pub const MQCMDL_LEVEL_802: u32 = 802;
pub const MQCMDL_LEVEL_900: u32 = 900;
pub const MQCMDL_LEVEL_901: u32 = 901;
pub const MQCMDL_LEVEL_902: u32 = 902;
pub const MQCMDL_LEVEL_903: u32 = 903;
pub const MQCMDL_LEVEL_904: u32 = 904;
pub const MQCMDL_LEVEL_905: u32 = 905;
pub const MQCMDL_LEVEL_910: u32 = 910;
pub const MQCMDL_LEVEL_911: u32 = 911;
pub const MQCMDL_LEVEL_912: u32 = 912;
pub const MQCMDL_LEVEL_913: u32 = 913;
pub const MQCMDL_LEVEL_914: u32 = 914;
pub const MQCMDL_LEVEL_915: u32 = 915;
pub const MQCMDL_CURRENT_LEVEL: u32 = 915;
pub const MQCSRV_CONVERT_NO: u32 = 0;
pub const MQCSRV_CONVERT_YES: u32 = 1;
pub const MQCSRV_DLQ_NO: u32 = 0;
pub const MQCSRV_DLQ_YES: u32 = 1;
pub const MQDNSWLM_NO: u32 = 0;
pub const MQDNSWLM_YES: u32 = 1;
pub const MQEXPI_OFF: u32 = 0;
pub const MQIGQ_DISABLED: u32 = 0;
pub const MQIGQ_ENABLED: u32 = 1;
pub const MQIGQPA_DEFAULT: u32 = 1;
pub const MQIGQPA_CONTEXT: u32 = 2;
pub const MQIGQPA_ONLY_IGQ: u32 = 3;
pub const MQIGQPA_ALTERNATE_OR_IGQ: u32 = 4;
pub const MQIPADDR_IPV4: u32 = 0;
pub const MQIPADDR_IPV6: u32 = 1;
pub const MQMMBI_UNLIMITED: i32 = -1;
pub const MQMON_NOT_AVAILABLE: i32 = -1;
pub const MQMON_NONE: i32 = -1;
pub const MQMON_Q_MGR: i32 = -3;
pub const MQMON_OFF: u32 = 0;
pub const MQMON_ON: u32 = 1;
pub const MQMON_DISABLED: u32 = 0;
pub const MQMON_ENABLED: u32 = 1;
pub const MQMON_LOW: u32 = 17;
pub const MQMON_MEDIUM: u32 = 33;
pub const MQMON_HIGH: u32 = 65;
pub const MQFUN_TYPE_UNKNOWN: u32 = 0;
pub const MQFUN_TYPE_JVM: u32 = 1;
pub const MQFUN_TYPE_PROGRAM: u32 = 2;
pub const MQFUN_TYPE_PROCEDURE: u32 = 3;
pub const MQFUN_TYPE_USERDEF: u32 = 4;
pub const MQFUN_TYPE_COMMAND: u32 = 5;
pub const MQACTV_DETAIL_LOW: u32 = 1;
pub const MQACTV_DETAIL_MEDIUM: u32 = 2;
pub const MQACTV_DETAIL_HIGH: u32 = 3;
pub const MQPL_MVS: u32 = 1;
pub const MQPL_OS390: u32 = 1;
pub const MQPL_ZOS: u32 = 1;
pub const MQPL_OS2: u32 = 2;
pub const MQPL_AIX: u32 = 3;
pub const MQPL_UNIX: u32 = 3;
pub const MQPL_OS400: u32 = 4;
pub const MQPL_WINDOWS: u32 = 5;
pub const MQPL_WINDOWS_NT: u32 = 11;
pub const MQPL_VMS: u32 = 12;
pub const MQPL_NSK: u32 = 13;
pub const MQPL_NSS: u32 = 13;
pub const MQPL_OPEN_TP1: u32 = 15;
pub const MQPL_VM: u32 = 18;
pub const MQPL_TPF: u32 = 23;
pub const MQPL_VSE: u32 = 27;
pub const MQPL_APPLIANCE: u32 = 28;
pub const MQPL_NATIVE: u32 = 3;
pub const MQPROP_UNRESTRICTED_LENGTH: i32 = -1;
pub const MQPSM_DISABLED: u32 = 0;
pub const MQPSM_COMPAT: u32 = 1;
pub const MQPSM_ENABLED: u32 = 2;
pub const MQPSCLUS_DISABLED: u32 = 0;
pub const MQPSCLUS_ENABLED: u32 = 1;
pub const MQQMOPT_DISABLED: u32 = 0;
pub const MQQMOPT_ENABLED: u32 = 1;
pub const MQQMOPT_REPLY: u32 = 2;
pub const MQRCVTIME_MULTIPLY: u32 = 0;
pub const MQRCVTIME_ADD: u32 = 1;
pub const MQRCVTIME_EQUAL: u32 = 2;
pub const MQRECORDING_DISABLED: u32 = 0;
pub const MQRECORDING_Q: u32 = 1;
pub const MQRECORDING_MSG: u32 = 2;
pub const MQSCYC_UPPER: u32 = 0;
pub const MQSCYC_MIXED: u32 = 1;
pub const MQSQQM_USE: u32 = 0;
pub const MQSQQM_IGNORE: u32 = 1;
pub const MQSSL_FIPS_NO: u32 = 0;
pub const MQSSL_FIPS_YES: u32 = 1;
pub const MQSP_AVAILABLE: u32 = 1;
pub const MQSP_NOT_AVAILABLE: u32 = 0;
pub const MQSVC_CONTROL_Q_MGR: u32 = 0;
pub const MQSVC_CONTROL_Q_MGR_START: u32 = 1;
pub const MQSVC_CONTROL_MANUAL: u32 = 2;
pub const MQSVC_STATUS_STOPPED: u32 = 0;
pub const MQSVC_STATUS_STARTING: u32 = 1;
pub const MQSVC_STATUS_RUNNING: u32 = 2;
pub const MQSVC_STATUS_STOPPING: u32 = 3;
pub const MQSVC_STATUS_RETRYING: u32 = 4;
pub const MQTCPKEEP_NO: u32 = 0;
pub const MQTCPKEEP_YES: u32 = 1;
pub const MQTCPSTACK_SINGLE: u32 = 0;
pub const MQTCPSTACK_MULTIPLE: u32 = 1;
pub const MQTRAXSTR_NO: u32 = 0;
pub const MQTRAXSTR_YES: u32 = 1;
pub const MQCAP_NOT_SUPPORTED: u32 = 0;
pub const MQCAP_SUPPORTED: u32 = 1;
pub const MQCAP_EXPIRED: u32 = 2;
pub const MQMEDIMGSCHED_MANUAL: u32 = 0;
pub const MQMEDIMGSCHED_AUTO: u32 = 1;
pub const MQMEDIMGINTVL_OFF: u32 = 0;
pub const MQMEDIMGLOGLN_OFF: u32 = 0;
pub const MQIMGRCOV_NO: u32 = 0;
pub const MQIMGRCOV_YES: u32 = 1;
pub const MQIMGRCOV_AS_Q_MGR: u32 = 2;
pub const MQDLV_AS_PARENT: u32 = 0;
pub const MQDLV_ALL: u32 = 1;
pub const MQDLV_ALL_DUR: u32 = 2;
pub const MQDLV_ALL_AVAIL: u32 = 3;
pub const MQMASTER_NO: u32 = 0;
pub const MQMASTER_YES: u32 = 1;
pub const MQSCOPE_ALL: u32 = 0;
pub const MQSCOPE_AS_PARENT: u32 = 1;
pub const MQSCOPE_QMGR: u32 = 4;
pub const MQSUB_DURABLE_AS_PARENT: u32 = 0;
pub const MQSUB_DURABLE_ALLOWED: u32 = 1;
pub const MQSUB_DURABLE_INHIBITED: u32 = 2;
pub const MQTA_BLOCK: u32 = 1;
pub const MQTA_PASSTHRU: u32 = 2;
pub const MQTA_SUB_AS_PARENT: u32 = 0;
pub const MQTA_SUB_INHIBITED: u32 = 1;
pub const MQTA_SUB_ALLOWED: u32 = 2;
pub const MQTA_PROXY_SUB_FORCE: u32 = 1;
pub const MQTA_PROXY_SUB_FIRSTUSE: u32 = 2;
pub const MQTA_PUB_AS_PARENT: u32 = 0;
pub const MQTA_PUB_INHIBITED: u32 = 1;
pub const MQTA_PUB_ALLOWED: u32 = 2;
pub const MQTOPT_LOCAL: u32 = 0;
pub const MQTOPT_CLUSTER: u32 = 1;
pub const MQTOPT_ALL: u32 = 2;
pub const MQMC_AS_PARENT: u32 = 0;
pub const MQMC_ENABLED: u32 = 1;
pub const MQMC_DISABLED: u32 = 2;
pub const MQMC_ONLY: u32 = 3;
pub const MQCIT_MULTICAST: u32 = 1;
pub const MQDC_MANAGED: u32 = 1;
pub const MQDC_PROVIDED: u32 = 2;
pub const MQPSPROP_NONE: u32 = 0;
pub const MQPSPROP_COMPAT: u32 = 1;
pub const MQPSPROP_RFH2: u32 = 2;
pub const MQPSPROP_MSGPROP: u32 = 3;
pub const MQRU_PUBLISH_ON_REQUEST: u32 = 1;
pub const MQRU_PUBLISH_ALL: u32 = 2;
pub const MQSUB_DURABLE_ALL: i32 = -1;
pub const MQSUB_DURABLE_YES: u32 = 1;
pub const MQSUB_DURABLE_NO: u32 = 2;
pub const MQTSCOPE_QMGR: u32 = 1;
pub const MQTSCOPE_ALL: u32 = 2;
pub const MQVU_FIXED_USER: u32 = 1;
pub const MQVU_ANY_USER: u32 = 2;
pub const MQWS_DEFAULT: u32 = 0;
pub const MQWS_CHAR: u32 = 1;
pub const MQWS_TOPIC: u32 = 2;
pub const MQUSRC_MAP: u32 = 0;
pub const MQUSRC_NOACCESS: u32 = 1;
pub const MQUSRC_CHANNEL: u32 = 2;
pub const MQWARN_YES: u32 = 1;
pub const MQWARN_NO: u32 = 0;
pub const MQDSB_DEFAULT: u32 = 0;
pub const MQDSB_8K: u32 = 1;
pub const MQDSB_16K: u32 = 2;
pub const MQDSB_32K: u32 = 3;
pub const MQDSB_64K: u32 = 4;
pub const MQDSB_128K: u32 = 5;
pub const MQDSB_256K: u32 = 6;
pub const MQDSB_512K: u32 = 7;
pub const MQDSB_1024K: u32 = 8;
pub const MQDSB_1M: u32 = 8;
pub const MQDSE_DEFAULT: u32 = 0;
pub const MQDSE_YES: u32 = 1;
pub const MQDSE_NO: u32 = 2;
pub const MQCFOFFLD_NONE: u32 = 0;
pub const MQCFOFFLD_SMDS: u32 = 1;
pub const MQCFOFFLD_DB2: u32 = 2;
pub const MQCFOFFLD_BOTH: u32 = 3;
pub const MQUSEDLQ_AS_PARENT: u32 = 0;
pub const MQUSEDLQ_NO: u32 = 1;
pub const MQUSEDLQ_YES: u32 = 2;
pub const MQ_MQTT_MAX_KEEP_ALIVE: u32 = 65536;
pub const MQ_SSL_KEY_PASSPHRASE_LENGTH: u32 = 1024;
pub const MQHO_UNUSABLE_HOBJ: i32 = -1;
pub const MQHO_NONE: u32 = 0;
pub const MQCO_IMMEDIATE: u32 = 0;
pub const MQCO_NONE: u32 = 0;
pub const MQCO_DELETE: u32 = 1;
pub const MQCO_DELETE_PURGE: u32 = 2;
pub const MQCO_KEEP_SUB: u32 = 4;
pub const MQCO_REMOVE_SUB: u32 = 8;
pub const MQCO_QUIESCE: u32 = 32;
pub const MQOP_START: u32 = 1;
pub const MQOP_START_WAIT: u32 = 2;
pub const MQOP_STOP: u32 = 4;
pub const MQOP_REGISTER: u32 = 256;
pub const MQOP_DEREGISTER: u32 = 512;
pub const MQOP_SUSPEND: u32 = 65536;
pub const MQOP_RESUME: u32 = 131072;
pub const MQHM_UNUSABLE_HMSG: i32 = -1;
pub const MQHM_NONE: u32 = 0;
pub const MQBA_FIRST: u32 = 6001;
pub const MQBA_LAST: u32 = 8000;
pub const MQCA_ADMIN_TOPIC_NAME: u32 = 2105;
pub const MQCA_ALTERATION_DATE: u32 = 2027;
pub const MQCA_ALTERATION_TIME: u32 = 2028;
pub const MQCA_AMQP_SSL_CIPHER_SUITES: u32 = 2137;
pub const MQCA_AMQP_VERSION: u32 = 2136;
pub const MQCA_APPL_ID: u32 = 2001;
pub const MQCA_AUTH_INFO_CONN_NAME: u32 = 2053;
pub const MQCA_AUTH_INFO_DESC: u32 = 2046;
pub const MQCA_AUTH_INFO_NAME: u32 = 2045;
pub const MQCA_AUTH_INFO_OCSP_URL: u32 = 2109;
pub const MQCA_AUTO_REORG_CATALOG: u32 = 2091;
pub const MQCA_AUTO_REORG_START_TIME: u32 = 2090;
pub const MQCA_BACKOUT_REQ_Q_NAME: u32 = 2019;
pub const MQCA_BASE_OBJECT_NAME: u32 = 2002;
pub const MQCA_BASE_Q_NAME: u32 = 2002;
pub const MQCA_BATCH_INTERFACE_ID: u32 = 2068;
pub const MQCA_CERT_LABEL: u32 = 2121;
pub const MQCA_CF_STRUC_DESC: u32 = 2052;
pub const MQCA_CF_STRUC_NAME: u32 = 2039;
pub const MQCA_CHANNEL_AUTO_DEF_EXIT: u32 = 2026;
pub const MQCA_CHILD: u32 = 2101;
pub const MQCA_CHINIT_SERVICE_PARM: u32 = 2076;
pub const MQCA_CHLAUTH_DESC: u32 = 2118;
pub const MQCA_CICS_FILE_NAME: u32 = 2060;
pub const MQCA_CLUSTER_DATE: u32 = 2037;
pub const MQCA_CLUSTER_NAME: u32 = 2029;
pub const MQCA_CLUSTER_NAMELIST: u32 = 2030;
pub const MQCA_CLUSTER_Q_MGR_NAME: u32 = 2031;
pub const MQCA_CLUSTER_TIME: u32 = 2038;
pub const MQCA_CLUSTER_WORKLOAD_DATA: u32 = 2034;
pub const MQCA_CLUSTER_WORKLOAD_EXIT: u32 = 2033;
pub const MQCA_CLUS_CHL_NAME: u32 = 2124;
pub const MQCA_COMMAND_INPUT_Q_NAME: u32 = 2003;
pub const MQCA_COMMAND_REPLY_Q_NAME: u32 = 2067;
pub const MQCA_COMM_INFO_DESC: u32 = 2111;
pub const MQCA_COMM_INFO_NAME: u32 = 2110;
pub const MQCA_CONN_AUTH: u32 = 2125;
pub const MQCA_CREATION_DATE: u32 = 2004;
pub const MQCA_CREATION_TIME: u32 = 2005;
pub const MQCA_CUSTOM: u32 = 2119;
pub const MQCA_DEAD_LETTER_Q_NAME: u32 = 2006;
pub const MQCA_DEF_XMIT_Q_NAME: u32 = 2025;
pub const MQCA_DNS_GROUP: u32 = 2071;
pub const MQCA_ENV_DATA: u32 = 2007;
pub const MQCA_FIRST: u32 = 2001;
pub const MQCA_IGQ_USER_ID: u32 = 2041;
pub const MQCA_INITIATION_Q_NAME: u32 = 2008;
pub const MQCA_INSTALLATION_DESC: u32 = 2115;
pub const MQCA_INSTALLATION_NAME: u32 = 2116;
pub const MQCA_INSTALLATION_PATH: u32 = 2117;
pub const MQCA_LAST: u32 = 4000;
pub const MQCA_LAST_USED: u32 = 2137;
pub const MQCA_LDAP_BASE_DN_GROUPS: u32 = 2132;
pub const MQCA_LDAP_BASE_DN_USERS: u32 = 2126;
pub const MQCA_LDAP_FIND_GROUP_FIELD: u32 = 2135;
pub const MQCA_LDAP_GROUP_ATTR_FIELD: u32 = 2134;
pub const MQCA_LDAP_GROUP_OBJECT_CLASS: u32 = 2133;
pub const MQCA_LDAP_PASSWORD: u32 = 2048;
pub const MQCA_LDAP_SHORT_USER_FIELD: u32 = 2127;
pub const MQCA_LDAP_USER_ATTR_FIELD: u32 = 2129;
pub const MQCA_LDAP_USER_NAME: u32 = 2047;
pub const MQCA_LDAP_USER_OBJECT_CLASS: u32 = 2128;
pub const MQCA_LU62_ARM_SUFFIX: u32 = 2074;
pub const MQCA_LU_GROUP_NAME: u32 = 2072;
pub const MQCA_LU_NAME: u32 = 2073;
pub const MQCA_MODEL_DURABLE_Q: u32 = 2096;
pub const MQCA_MODEL_NON_DURABLE_Q: u32 = 2097;
pub const MQCA_MONITOR_Q_NAME: u32 = 2066;
pub const MQCA_NAMELIST_DESC: u32 = 2009;
pub const MQCA_NAMELIST_NAME: u32 = 2010;
pub const MQCA_NAMES: u32 = 2020;
pub const MQCA_PARENT: u32 = 2102;
pub const MQCA_PASS_TICKET_APPL: u32 = 2086;
pub const MQCA_POLICY_NAME: u32 = 2112;
pub const MQCA_PROCESS_DESC: u32 = 2011;
pub const MQCA_PROCESS_NAME: u32 = 2012;
pub const MQCA_QSG_CERT_LABEL: u32 = 2131;
pub const MQCA_QSG_NAME: u32 = 2040;
pub const MQCA_Q_DESC: u32 = 2013;
pub const MQCA_Q_MGR_DESC: u32 = 2014;
pub const MQCA_Q_MGR_IDENTIFIER: u32 = 2032;
pub const MQCA_Q_MGR_NAME: u32 = 2015;
pub const MQCA_Q_NAME: u32 = 2016;
pub const MQCA_RECIPIENT_DN: u32 = 2114;
pub const MQCA_REMOTE_Q_MGR_NAME: u32 = 2017;
pub const MQCA_REMOTE_Q_NAME: u32 = 2018;
pub const MQCA_REPOSITORY_NAME: u32 = 2035;
pub const MQCA_REPOSITORY_NAMELIST: u32 = 2036;
pub const MQCA_RESUME_DATE: u32 = 2098;
pub const MQCA_RESUME_TIME: u32 = 2099;
pub const MQCA_SERVICE_DESC: u32 = 2078;
pub const MQCA_SERVICE_NAME: u32 = 2077;
pub const MQCA_SERVICE_START_ARGS: u32 = 2080;
pub const MQCA_SERVICE_START_COMMAND: u32 = 2079;
pub const MQCA_SERVICE_STOP_ARGS: u32 = 2082;
pub const MQCA_SERVICE_STOP_COMMAND: u32 = 2081;
pub const MQCA_SIGNER_DN: u32 = 2113;
pub const MQCA_SSL_CERT_ISSUER_NAME: u32 = 2130;
pub const MQCA_SSL_CRL_NAMELIST: u32 = 2050;
pub const MQCA_SSL_CRYPTO_HARDWARE: u32 = 2051;
pub const MQCA_SSL_KEY_LIBRARY: u32 = 2069;
pub const MQCA_SSL_KEY_MEMBER: u32 = 2070;
pub const MQCA_SSL_KEY_REPOSITORY: u32 = 2049;
pub const MQCA_STDERR_DESTINATION: u32 = 2084;
pub const MQCA_STDOUT_DESTINATION: u32 = 2083;
pub const MQCA_STORAGE_CLASS: u32 = 2022;
pub const MQCA_STORAGE_CLASS_DESC: u32 = 2042;
pub const MQCA_SYSTEM_LOG_Q_NAME: u32 = 2065;
pub const MQCA_TCP_NAME: u32 = 2075;
pub const MQCA_TOPIC_DESC: u32 = 2093;
pub const MQCA_TOPIC_NAME: u32 = 2092;
pub const MQCA_TOPIC_STRING: u32 = 2094;
pub const MQCA_TOPIC_STRING_FILTER: u32 = 2108;
pub const MQCA_TPIPE_NAME: u32 = 2085;
pub const MQCA_TRIGGER_CHANNEL_NAME: u32 = 2064;
pub const MQCA_TRIGGER_DATA: u32 = 2023;
pub const MQCA_TRIGGER_PROGRAM_NAME: u32 = 2062;
pub const MQCA_TRIGGER_TERM_ID: u32 = 2063;
pub const MQCA_TRIGGER_TRANS_ID: u32 = 2061;
pub const MQCA_USER_DATA: u32 = 2021;
pub const MQCA_USER_LIST: u32 = 4000;
pub const MQCA_VERSION: u32 = 2120;
pub const MQCA_XCF_GROUP_NAME: u32 = 2043;
pub const MQCA_XCF_MEMBER_NAME: u32 = 2044;
pub const MQCA_XMIT_Q_NAME: u32 = 2024;
pub const MQCA_XR_SSL_CIPHER_SUITES: u32 = 2123;
pub const MQCA_XR_VERSION: u32 = 2122;
pub const MQIA_ACCOUNTING_CONN_OVERRIDE: u32 = 136;
pub const MQIA_ACCOUNTING_INTERVAL: u32 = 135;
pub const MQIA_ACCOUNTING_MQI: u32 = 133;
pub const MQIA_ACCOUNTING_Q: u32 = 134;
pub const MQIA_ACTIVE_CHANNELS: u32 = 100;
pub const MQIA_ACTIVITY_CONN_OVERRIDE: u32 = 239;
pub const MQIA_ACTIVITY_RECORDING: u32 = 138;
pub const MQIA_ACTIVITY_TRACE: u32 = 240;
pub const MQIA_ADOPTNEWMCA_CHECK: u32 = 102;
pub const MQIA_ADOPTNEWMCA_INTERVAL: u32 = 104;
pub const MQIA_ADOPTNEWMCA_TYPE: u32 = 103;
pub const MQIA_ADOPT_CONTEXT: u32 = 260;
pub const MQIA_ADVANCED_CAPABILITY: u32 = 273;
pub const MQIA_AMQP_CAPABILITY: u32 = 265;
pub const MQIA_APPL_TYPE: u32 = 1;
pub const MQIA_ARCHIVE: u32 = 60;
pub const MQIA_AUTHENTICATION_FAIL_DELAY: u32 = 259;
pub const MQIA_AUTHENTICATION_METHOD: u32 = 266;
pub const MQIA_AUTHORITY_EVENT: u32 = 47;
pub const MQIA_AUTH_INFO_TYPE: u32 = 66;
pub const MQIA_AUTO_REORGANIZATION: u32 = 173;
pub const MQIA_AUTO_REORG_INTERVAL: u32 = 174;
pub const MQIA_BACKOUT_THRESHOLD: u32 = 22;
pub const MQIA_BASE_TYPE: u32 = 193;
pub const MQIA_BATCH_INTERFACE_AUTO: u32 = 86;
pub const MQIA_BRIDGE_EVENT: u32 = 74;
pub const MQIA_CERT_VAL_POLICY: u32 = 252;
pub const MQIA_CF_CFCONLOS: u32 = 246;
pub const MQIA_CF_LEVEL: u32 = 70;
pub const MQIA_CF_OFFLDUSE: u32 = 229;
pub const MQIA_CF_OFFLOAD: u32 = 224;
pub const MQIA_CF_OFFLOAD_THRESHOLD1: u32 = 225;
pub const MQIA_CF_OFFLOAD_THRESHOLD2: u32 = 226;
pub const MQIA_CF_OFFLOAD_THRESHOLD3: u32 = 227;
pub const MQIA_CF_RECAUTO: u32 = 244;
pub const MQIA_CF_RECOVER: u32 = 71;
pub const MQIA_CF_SMDS_BUFFERS: u32 = 228;
pub const MQIA_CHANNEL_AUTO_DEF: u32 = 55;
pub const MQIA_CHANNEL_AUTO_DEF_EVENT: u32 = 56;
pub const MQIA_CHANNEL_EVENT: u32 = 73;
pub const MQIA_CHECK_CLIENT_BINDING: u32 = 258;
pub const MQIA_CHECK_LOCAL_BINDING: u32 = 257;
pub const MQIA_CHINIT_ADAPTERS: u32 = 101;
pub const MQIA_CHINIT_CONTROL: u32 = 119;
pub const MQIA_CHINIT_DISPATCHERS: u32 = 105;
pub const MQIA_CHINIT_TRACE_AUTO_START: u32 = 117;
pub const MQIA_CHINIT_TRACE_TABLE_SIZE: u32 = 118;
pub const MQIA_CHLAUTH_RECORDS: u32 = 248;
pub const MQIA_CLUSTER_OBJECT_STATE: u32 = 256;
pub const MQIA_CLUSTER_PUB_ROUTE: u32 = 255;
pub const MQIA_CLUSTER_Q_TYPE: u32 = 59;
pub const MQIA_CLUSTER_WORKLOAD_LENGTH: u32 = 58;
pub const MQIA_CLWL_MRU_CHANNELS: u32 = 97;
pub const MQIA_CLWL_Q_PRIORITY: u32 = 96;
pub const MQIA_CLWL_Q_RANK: u32 = 95;
pub const MQIA_CLWL_USEQ: u32 = 98;
pub const MQIA_CMD_SERVER_AUTO: u32 = 87;
pub const MQIA_CMD_SERVER_CONTROL: u32 = 120;
pub const MQIA_CMD_SERVER_CONVERT_MSG: u32 = 88;
pub const MQIA_CMD_SERVER_DLQ_MSG: u32 = 89;
pub const MQIA_CODED_CHAR_SET_ID: u32 = 2;
pub const MQIA_COMMAND_EVENT: u32 = 99;
pub const MQIA_COMMAND_LEVEL: u32 = 31;
pub const MQIA_COMM_EVENT: u32 = 232;
pub const MQIA_COMM_INFO_TYPE: u32 = 223;
pub const MQIA_CONFIGURATION_EVENT: u32 = 51;
pub const MQIA_CPI_LEVEL: u32 = 27;
pub const MQIA_CURRENT_Q_DEPTH: u32 = 3;
pub const MQIA_DEFINITION_TYPE: u32 = 7;
pub const MQIA_DEF_BIND: u32 = 61;
pub const MQIA_DEF_CLUSTER_XMIT_Q_TYPE: u32 = 250;
pub const MQIA_DEF_INPUT_OPEN_OPTION: u32 = 4;
pub const MQIA_DEF_PERSISTENCE: u32 = 5;
pub const MQIA_DEF_PRIORITY: u32 = 6;
pub const MQIA_DEF_PUT_RESPONSE_TYPE: u32 = 184;
pub const MQIA_DEF_READ_AHEAD: u32 = 188;
pub const MQIA_DISPLAY_TYPE: u32 = 262;
pub const MQIA_DIST_LISTS: u32 = 34;
pub const MQIA_DNS_WLM: u32 = 106;
pub const MQIA_DURABLE_SUB: u32 = 175;
pub const MQIA_ENCRYPTION_ALGORITHM: u32 = 237;
pub const MQIA_EXPIRY_INTERVAL: u32 = 39;
pub const MQIA_FIRST: u32 = 1;
pub const MQIA_GROUP_UR: u32 = 221;
pub const MQIA_HARDEN_GET_BACKOUT: u32 = 8;
pub const MQIA_HIGH_Q_DEPTH: u32 = 36;
pub const MQIA_IGQ_PUT_AUTHORITY: u32 = 65;
pub const MQIA_INDEX_TYPE: u32 = 57;
pub const MQIA_INHIBIT_EVENT: u32 = 48;
pub const MQIA_INHIBIT_GET: u32 = 9;
pub const MQIA_INHIBIT_PUB: u32 = 181;
pub const MQIA_INHIBIT_PUT: u32 = 10;
pub const MQIA_INHIBIT_SUB: u32 = 182;
pub const MQIA_INTRA_GROUP_QUEUING: u32 = 64;
pub const MQIA_IP_ADDRESS_VERSION: u32 = 93;
pub const MQIA_KEY_REUSE_COUNT: u32 = 267;
pub const MQIA_LAST: u32 = 2000;
pub const MQIA_LAST_USED: u32 = 274;
pub const MQIA_LDAP_AUTHORMD: u32 = 263;
pub const MQIA_LDAP_NESTGRP: u32 = 264;
pub const MQIA_LDAP_SECURE_COMM: u32 = 261;
pub const MQIA_LISTENER_PORT_NUMBER: u32 = 85;
pub const MQIA_LISTENER_TIMER: u32 = 107;
pub const MQIA_LOCAL_EVENT: u32 = 49;
pub const MQIA_LOGGER_EVENT: u32 = 94;
pub const MQIA_LU62_CHANNELS: u32 = 108;
pub const MQIA_MASTER_ADMIN: u32 = 186;
pub const MQIA_MAX_CHANNELS: u32 = 109;
pub const MQIA_MAX_CLIENTS: u32 = 172;
pub const MQIA_MAX_GLOBAL_LOCKS: u32 = 83;
pub const MQIA_MAX_HANDLES: u32 = 11;
pub const MQIA_MAX_LOCAL_LOCKS: u32 = 84;
pub const MQIA_MAX_MSG_LENGTH: u32 = 13;
pub const MQIA_MAX_OPEN_Q: u32 = 80;
pub const MQIA_MAX_PRIORITY: u32 = 14;
pub const MQIA_MAX_PROPERTIES_LENGTH: u32 = 192;
pub const MQIA_MAX_Q_DEPTH: u32 = 15;
pub const MQIA_MAX_Q_FILE_SIZE: u32 = 274;
pub const MQIA_MAX_Q_TRIGGERS: u32 = 90;
pub const MQIA_MAX_RECOVERY_TASKS: u32 = 171;
pub const MQIA_MAX_RESPONSES: u32 = 230;
pub const MQIA_MAX_UNCOMMITTED_MSGS: u32 = 33;
pub const MQIA_MCAST_BRIDGE: u32 = 233;
pub const MQIA_MEDIA_IMAGE_INTERVAL: u32 = 269;
pub const MQIA_MEDIA_IMAGE_LOG_LENGTH: u32 = 270;
pub const MQIA_MEDIA_IMAGE_RECOVER_OBJ: u32 = 271;
pub const MQIA_MEDIA_IMAGE_RECOVER_Q: u32 = 272;
pub const MQIA_MEDIA_IMAGE_SCHEDULING: u32 = 268;
pub const MQIA_MONITORING_AUTO_CLUSSDR: u32 = 124;
pub const MQIA_MONITORING_CHANNEL: u32 = 122;
pub const MQIA_MONITORING_Q: u32 = 123;
pub const MQIA_MONITOR_INTERVAL: u32 = 81;
pub const MQIA_MSG_DELIVERY_SEQUENCE: u32 = 16;
pub const MQIA_MSG_DEQ_COUNT: u32 = 38;
pub const MQIA_MSG_ENQ_COUNT: u32 = 37;
pub const MQIA_MSG_MARK_BROWSE_INTERVAL: u32 = 68;
pub const MQIA_MULTICAST: u32 = 176;
pub const MQIA_NAMELIST_TYPE: u32 = 72;
pub const MQIA_NAME_COUNT: u32 = 19;
pub const MQIA_NPM_CLASS: u32 = 78;
pub const MQIA_NPM_DELIVERY: u32 = 196;
pub const MQIA_OPEN_INPUT_COUNT: u32 = 17;
pub const MQIA_OPEN_OUTPUT_COUNT: u32 = 18;
pub const MQIA_OUTBOUND_PORT_MAX: u32 = 140;
pub const MQIA_OUTBOUND_PORT_MIN: u32 = 110;
pub const MQIA_PAGESET_ID: u32 = 62;
pub const MQIA_PERFORMANCE_EVENT: u32 = 53;
pub const MQIA_PLATFORM: u32 = 32;
pub const MQIA_PM_DELIVERY: u32 = 195;
pub const MQIA_POLICY_VERSION: u32 = 238;
pub const MQIA_PROPERTY_CONTROL: u32 = 190;
pub const MQIA_PROT_POLICY_CAPABILITY: u32 = 251;
pub const MQIA_PROXY_SUB: u32 = 199;
pub const MQIA_PUBSUB_CLUSTER: u32 = 249;
pub const MQIA_PUBSUB_MAXMSG_RETRY_COUNT: u32 = 206;
pub const MQIA_PUBSUB_MODE: u32 = 187;
pub const MQIA_PUBSUB_NP_MSG: u32 = 203;
pub const MQIA_PUBSUB_NP_RESP: u32 = 205;
pub const MQIA_PUBSUB_SYNC_PT: u32 = 207;
pub const MQIA_PUB_COUNT: u32 = 215;
pub const MQIA_PUB_SCOPE: u32 = 219;
pub const MQIA_QMGR_CFCONLOS: u32 = 245;
pub const MQIA_QMOPT_CONS_COMMS_MSGS: u32 = 155;
pub const MQIA_QMOPT_CONS_CRITICAL_MSGS: u32 = 154;
pub const MQIA_QMOPT_CONS_ERROR_MSGS: u32 = 153;
pub const MQIA_QMOPT_CONS_INFO_MSGS: u32 = 151;
pub const MQIA_QMOPT_CONS_REORG_MSGS: u32 = 156;
pub const MQIA_QMOPT_CONS_SYSTEM_MSGS: u32 = 157;
pub const MQIA_QMOPT_CONS_WARNING_MSGS: u32 = 152;
pub const MQIA_QMOPT_CSMT_ON_ERROR: u32 = 150;
pub const MQIA_QMOPT_INTERNAL_DUMP: u32 = 170;
pub const MQIA_QMOPT_LOG_COMMS_MSGS: u32 = 162;
pub const MQIA_QMOPT_LOG_CRITICAL_MSGS: u32 = 161;
pub const MQIA_QMOPT_LOG_ERROR_MSGS: u32 = 160;
pub const MQIA_QMOPT_LOG_INFO_MSGS: u32 = 158;
pub const MQIA_QMOPT_LOG_REORG_MSGS: u32 = 163;
pub const MQIA_QMOPT_LOG_SYSTEM_MSGS: u32 = 164;
pub const MQIA_QMOPT_LOG_WARNING_MSGS: u32 = 159;
pub const MQIA_QMOPT_TRACE_COMMS: u32 = 166;
pub const MQIA_QMOPT_TRACE_CONVERSION: u32 = 168;
pub const MQIA_QMOPT_TRACE_MQI_CALLS: u32 = 165;
pub const MQIA_QMOPT_TRACE_REORG: u32 = 167;
pub const MQIA_QMOPT_TRACE_SYSTEM: u32 = 169;
pub const MQIA_QSG_DISP: u32 = 63;
pub const MQIA_Q_DEPTH_HIGH_EVENT: u32 = 43;
pub const MQIA_Q_DEPTH_HIGH_LIMIT: u32 = 40;
pub const MQIA_Q_DEPTH_LOW_EVENT: u32 = 44;
pub const MQIA_Q_DEPTH_LOW_LIMIT: u32 = 41;
pub const MQIA_Q_DEPTH_MAX_EVENT: u32 = 42;
pub const MQIA_Q_SERVICE_INTERVAL: u32 = 54;
pub const MQIA_Q_SERVICE_INTERVAL_EVENT: u32 = 46;
pub const MQIA_Q_TYPE: u32 = 20;
pub const MQIA_Q_USERS: u32 = 82;
pub const MQIA_READ_AHEAD: u32 = 189;
pub const MQIA_RECEIVE_TIMEOUT: u32 = 111;
pub const MQIA_RECEIVE_TIMEOUT_MIN: u32 = 113;
pub const MQIA_RECEIVE_TIMEOUT_TYPE: u32 = 112;
pub const MQIA_REMOTE_EVENT: u32 = 50;
pub const MQIA_RESPONSE_RESTART_POINT: u32 = 231;
pub const MQIA_RETENTION_INTERVAL: u32 = 21;
pub const MQIA_REVERSE_DNS_LOOKUP: u32 = 254;
pub const MQIA_SCOPE: u32 = 45;
pub const MQIA_SECURITY_CASE: u32 = 141;
pub const MQIA_SERVICE_CONTROL: u32 = 139;
pub const MQIA_SERVICE_TYPE: u32 = 121;
pub const MQIA_SHAREABILITY: u32 = 23;
pub const MQIA_SHARED_Q_Q_MGR_NAME: u32 = 77;
pub const MQIA_SIGNATURE_ALGORITHM: u32 = 236;
pub const MQIA_SSL_EVENT: u32 = 75;
pub const MQIA_SSL_FIPS_REQUIRED: u32 = 92;
pub const MQIA_SSL_RESET_COUNT: u32 = 76;
pub const MQIA_SSL_TASKS: u32 = 69;
pub const MQIA_START_STOP_EVENT: u32 = 52;
pub const MQIA_STATISTICS_AUTO_CLUSSDR: u32 = 130;
pub const MQIA_STATISTICS_CHANNEL: u32 = 129;
pub const MQIA_STATISTICS_INTERVAL: u32 = 131;
pub const MQIA_STATISTICS_MQI: u32 = 127;
pub const MQIA_STATISTICS_Q: u32 = 128;
pub const MQIA_SUB_CONFIGURATION_EVENT: u32 = 242;
pub const MQIA_SUB_COUNT: u32 = 204;
pub const MQIA_SUB_SCOPE: u32 = 218;
pub const MQIA_SUITE_B_STRENGTH: u32 = 247;
pub const MQIA_SYNCPOINT: u32 = 30;
pub const MQIA_TCP_CHANNELS: u32 = 114;
pub const MQIA_TCP_KEEP_ALIVE: u32 = 115;
pub const MQIA_TCP_STACK_TYPE: u32 = 116;
pub const MQIA_TIME_SINCE_RESET: u32 = 35;
pub const MQIA_TOLERATE_UNPROTECTED: u32 = 235;
pub const MQIA_TOPIC_DEF_PERSISTENCE: u32 = 185;
pub const MQIA_TOPIC_NODE_COUNT: u32 = 253;
pub const MQIA_TOPIC_TYPE: u32 = 208;
pub const MQIA_TRACE_ROUTE_RECORDING: u32 = 137;
pub const MQIA_TREE_LIFE_TIME: u32 = 183;
pub const MQIA_TRIGGER_CONTROL: u32 = 24;
pub const MQIA_TRIGGER_DEPTH: u32 = 29;
pub const MQIA_TRIGGER_INTERVAL: u32 = 25;
pub const MQIA_TRIGGER_MSG_PRIORITY: u32 = 26;
pub const MQIA_TRIGGER_RESTART: u32 = 91;
pub const MQIA_TRIGGER_TYPE: u32 = 28;
pub const MQIA_UR_DISP: u32 = 222;
pub const MQIA_USAGE: u32 = 12;
pub const MQIA_USER_LIST: u32 = 2000;
pub const MQIA_USE_DEAD_LETTER_Q: u32 = 234;
pub const MQIA_WILDCARD_OPERATION: u32 = 216;
pub const MQIA_XR_CAPABILITY: u32 = 243;
pub const MQIAV_NOT_APPLICABLE: i32 = -1;
pub const MQIAV_UNDEFINED: i32 = -2;
pub const MQMCB_DISABLED: u32 = 0;
pub const MQMCB_ENABLED: u32 = 1;
pub const MQKEY_REUSE_DISABLED: u32 = 0;
pub const MQKEY_REUSE_UNLIMITED: i32 = -1;
pub const MQGA_FIRST: u32 = 8001;
pub const MQGA_LAST: u32 = 9000;
pub const MQOO_BIND_AS_Q_DEF: u32 = 0;
pub const MQOO_READ_AHEAD_AS_Q_DEF: u32 = 0;
pub const MQOO_INPUT_AS_Q_DEF: u32 = 1;
pub const MQOO_INPUT_SHARED: u32 = 2;
pub const MQOO_INPUT_EXCLUSIVE: u32 = 4;
pub const MQOO_BROWSE: u32 = 8;
pub const MQOO_OUTPUT: u32 = 16;
pub const MQOO_INQUIRE: u32 = 32;
pub const MQOO_SET: u32 = 64;
pub const MQOO_SAVE_ALL_CONTEXT: u32 = 128;
pub const MQOO_PASS_IDENTITY_CONTEXT: u32 = 256;
pub const MQOO_PASS_ALL_CONTEXT: u32 = 512;
pub const MQOO_SET_IDENTITY_CONTEXT: u32 = 1024;
pub const MQOO_SET_ALL_CONTEXT: u32 = 2048;
pub const MQOO_ALTERNATE_USER_AUTHORITY: u32 = 4096;
pub const MQOO_FAIL_IF_QUIESCING: u32 = 8192;
pub const MQOO_BIND_ON_OPEN: u32 = 16384;
pub const MQOO_BIND_ON_GROUP: u32 = 4194304;
pub const MQOO_BIND_NOT_FIXED: u32 = 32768;
pub const MQOO_CO_OP: u32 = 131072;
pub const MQOO_NO_READ_AHEAD: u32 = 524288;
pub const MQOO_READ_AHEAD: u32 = 1048576;
pub const MQOO_NO_MULTICAST: u32 = 2097152;
pub const MQOO_RESOLVE_LOCAL_Q: u32 = 262144;
pub const MQOO_RESOLVE_LOCAL_TOPIC: u32 = 262144;
pub const MQOO_RESOLVE_NAMES: u32 = 65536;
pub const MQTYPE_AS_SET: u32 = 0;
pub const MQTYPE_NULL: u32 = 2;
pub const MQTYPE_BOOLEAN: u32 = 4;
pub const MQTYPE_BYTE_STRING: u32 = 8;
pub const MQTYPE_INT8: u32 = 16;
pub const MQTYPE_INT16: u32 = 32;
pub const MQTYPE_INT32: u32 = 64;
pub const MQTYPE_LONG: u32 = 64;
pub const MQTYPE_INT64: u32 = 128;
pub const MQTYPE_FLOAT32: u32 = 256;
pub const MQTYPE_FLOAT64: u32 = 512;
pub const MQTYPE_STRING: u32 = 1024;
pub const MQVL_NULL_TERMINATED: i32 = -1;
pub const MQVL_EMPTY_STRING: u32 = 0;
pub const MQSTAT_TYPE_ASYNC_ERROR: u32 = 0;
pub const MQSTAT_TYPE_RECONNECTION: u32 = 1;
pub const MQSTAT_TYPE_RECONNECTION_ERROR: u32 = 2;
pub const MQSO_NONE: u32 = 0;
pub const MQSO_NON_DURABLE: u32 = 0;
pub const MQSO_READ_AHEAD_AS_Q_DEF: u32 = 0;
pub const MQSO_ALTER: u32 = 1;
pub const MQSO_CREATE: u32 = 2;
pub const MQSO_RESUME: u32 = 4;
pub const MQSO_DURABLE: u32 = 8;
pub const MQSO_GROUP_SUB: u32 = 16;
pub const MQSO_MANAGED: u32 = 32;
pub const MQSO_SET_IDENTITY_CONTEXT: u32 = 64;
pub const MQSO_NO_MULTICAST: u32 = 128;
pub const MQSO_FIXED_USERID: u32 = 256;
pub const MQSO_ANY_USERID: u32 = 512;
pub const MQSO_PUBLICATIONS_ON_REQUEST: u32 = 2048;
pub const MQSO_NEW_PUBLICATIONS_ONLY: u32 = 4096;
pub const MQSO_FAIL_IF_QUIESCING: u32 = 8192;
pub const MQSO_ALTERNATE_USER_AUTHORITY: u32 = 262144;
pub const MQSO_WILDCARD_CHAR: u32 = 1048576;
pub const MQSO_WILDCARD_TOPIC: u32 = 2097152;
pub const MQSO_SET_CORREL_ID: u32 = 4194304;
pub const MQSO_SCOPE_QMGR: u32 = 67108864;
pub const MQSO_NO_READ_AHEAD: u32 = 134217728;
pub const MQSO_READ_AHEAD: u32 = 268435456;
pub const MQSR_ACTION_PUBLICATION: u32 = 1;
pub const MQCD_VERSION_1: u32 = 1;
pub const MQCD_VERSION_2: u32 = 2;
pub const MQCD_VERSION_3: u32 = 3;
pub const MQCD_VERSION_4: u32 = 4;
pub const MQCD_VERSION_5: u32 = 5;
pub const MQCD_VERSION_6: u32 = 6;
pub const MQCD_VERSION_7: u32 = 7;
pub const MQCD_VERSION_8: u32 = 8;
pub const MQCD_VERSION_9: u32 = 9;
pub const MQCD_VERSION_10: u32 = 10;
pub const MQCD_VERSION_11: u32 = 11;
pub const MQCD_VERSION_12: u32 = 12;
pub const MQCD_CURRENT_VERSION: u32 = 12;
pub const MQCD_LENGTH_1: u32 = 984;
pub const MQCD_LENGTH_2: u32 = 1312;
pub const MQCD_LENGTH_3: u32 = 1480;
pub const MQCD_LENGTH_4: u32 = 1568;
pub const MQCD_LENGTH_5: u32 = 1584;
pub const MQCD_LENGTH_6: u32 = 1688;
pub const MQCD_LENGTH_7: u32 = 1792;
pub const MQCD_LENGTH_8: u32 = 1888;
pub const MQCD_LENGTH_9: u32 = 1912;
pub const MQCD_LENGTH_10: u32 = 1920;
pub const MQCD_LENGTH_11: u32 = 1984;
pub const MQCD_LENGTH_12: u32 = 1992;
pub const MQCD_CURRENT_LENGTH: u32 = 1992;
pub const MQCHT_SENDER: u32 = 1;
pub const MQCHT_SERVER: u32 = 2;
pub const MQCHT_RECEIVER: u32 = 3;
pub const MQCHT_REQUESTER: u32 = 4;
pub const MQCHT_ALL: u32 = 5;
pub const MQCHT_CLNTCONN: u32 = 6;
pub const MQCHT_SVRCONN: u32 = 7;
pub const MQCHT_CLUSRCVR: u32 = 8;
pub const MQCHT_CLUSSDR: u32 = 9;
pub const MQCHT_MQTT: u32 = 10;
pub const MQCHT_AMQP: u32 = 11;
pub const MQCOMPRESS_NOT_AVAILABLE: i32 = -1;
pub const MQCOMPRESS_NONE: u32 = 0;
pub const MQCOMPRESS_RLE: u32 = 1;
pub const MQCOMPRESS_ZLIBFAST: u32 = 2;
pub const MQCOMPRESS_ZLIBHIGH: u32 = 4;
pub const MQCOMPRESS_SYSTEM: u32 = 8;
pub const MQCOMPRESS_ANY: u32 = 268435455;
pub const MQXPT_ALL: i32 = -1;
pub const MQXPT_LOCAL: u32 = 0;
pub const MQXPT_LU62: u32 = 1;
pub const MQXPT_TCP: u32 = 2;
pub const MQXPT_NETBIOS: u32 = 3;
pub const MQXPT_SPX: u32 = 4;
pub const MQXPT_DECNET: u32 = 5;
pub const MQXPT_UDP: u32 = 6;
pub const MQPA_DEFAULT: u32 = 1;
pub const MQPA_CONTEXT: u32 = 2;
pub const MQPA_ONLY_MCA: u32 = 3;
pub const MQPA_ALTERNATE_OR_MCA: u32 = 4;
pub const MQCDC_SENDER_CONVERSION: u32 = 1;
pub const MQCDC_NO_SENDER_CONVERSION: u32 = 0;
pub const MQMCAT_PROCESS: u32 = 1;
pub const MQMCAT_THREAD: u32 = 2;
pub const MQNPMS_NORMAL: u32 = 1;
pub const MQNPMS_FAST: u32 = 2;
pub const MQSCA_REQUIRED: u32 = 0;
pub const MQSCA_OPTIONAL: u32 = 1;
pub const MQSCA_NEVER_REQUIRED: u32 = 2;
pub const MQKAI_AUTO: i32 = -1;
pub const MQCAFTY_NONE: u32 = 0;
pub const MQCAFTY_PREFERRED: u32 = 1;
pub const MQRCN_NO: u32 = 0;
pub const MQRCN_YES: u32 = 1;
pub const MQRCN_Q_MGR: u32 = 2;
pub const MQRCN_DISABLED: u32 = 3;
pub const MQPROTO_MQTTV3: u32 = 1;
pub const MQPROTO_HTTP: u32 = 2;
pub const MQPROTO_AMQP: u32 = 3;
pub const MQPROTO_MQTTV311: u32 = 4;
pub const MQSECPROT_NONE: u32 = 0;
pub const MQSECPROT_SSLV30: u32 = 1;
pub const MQSECPROT_TLSV10: u32 = 2;
pub const MQSECPROT_TLSV12: u32 = 4;
pub const MQSECPROT_TLSV13: u32 = 8;
pub const MQSPL_PASSTHRU: u32 = 0;
pub const MQSPL_REMOVE: u32 = 1;
pub const MQSPL_AS_POLICY: u32 = 2;
pub const MQACH_STRUC_ID: &'static [u8; 5usize] = b"ACH \0";
pub const MQACH_VERSION_1: u32 = 1;
pub const MQACH_CURRENT_VERSION: u32 = 1;
pub const MQACH_LENGTH_1: u32 = 72;
pub const MQACH_CURRENT_LENGTH: u32 = 72;
pub const MQAXC_STRUC_ID: &'static [u8; 5usize] = b"AXC \0";
pub const MQAXC_VERSION_1: u32 = 1;
pub const MQAXC_VERSION_2: u32 = 2;
pub const MQAXC_CURRENT_VERSION: u32 = 2;
pub const MQAXC_LENGTH_1: u32 = 392;
pub const MQAXC_LENGTH_2: u32 = 424;
pub const MQAXC_CURRENT_LENGTH: u32 = 424;
pub const MQXE_OTHER: u32 = 0;
pub const MQXE_MCA: u32 = 1;
pub const MQXE_MCA_SVRCONN: u32 = 2;
pub const MQXE_COMMAND_SERVER: u32 = 3;
pub const MQXE_MQSC: u32 = 4;
pub const MQXE_MCA_CLNTCONN: u32 = 5;
pub const MQAXP_STRUC_ID: &'static [u8; 5usize] = b"AXP \0";
pub const MQAXP_VERSION_1: u32 = 1;
pub const MQAXP_VERSION_2: u32 = 2;
pub const MQAXP_CURRENT_VERSION: u32 = 2;
pub const MQAXP_LENGTH_1: u32 = 256;
pub const MQAXP_CURRENT_LENGTH: u32 = 256;
pub const MQXACT_EXTERNAL: u32 = 1;
pub const MQXACT_INTERNAL: u32 = 2;
pub const MQXPDA_NONE : & 'static [ u8 ; 49usize ] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" ;
pub const MQXF_INIT: u32 = 1;
pub const MQXF_TERM: u32 = 2;
pub const MQXF_CONN: u32 = 3;
pub const MQXF_CONNX: u32 = 4;
pub const MQXF_DISC: u32 = 5;
pub const MQXF_OPEN: u32 = 6;
pub const MQXF_CLOSE: u32 = 7;
pub const MQXF_PUT1: u32 = 8;
pub const MQXF_PUT: u32 = 9;
pub const MQXF_GET: u32 = 10;
pub const MQXF_DATA_CONV_ON_GET: u32 = 11;
pub const MQXF_INQ: u32 = 12;
pub const MQXF_SET: u32 = 13;
pub const MQXF_BEGIN: u32 = 14;
pub const MQXF_CMIT: u32 = 15;
pub const MQXF_BACK: u32 = 16;
pub const MQXF_STAT: u32 = 18;
pub const MQXF_CB: u32 = 19;
pub const MQXF_CTL: u32 = 20;
pub const MQXF_CALLBACK: u32 = 21;
pub const MQXF_SUB: u32 = 22;
pub const MQXF_SUBRQ: u32 = 23;
pub const MQXF_XACLOSE: u32 = 24;
pub const MQXF_XACOMMIT: u32 = 25;
pub const MQXF_XACOMPLETE: u32 = 26;
pub const MQXF_XAEND: u32 = 27;
pub const MQXF_XAFORGET: u32 = 28;
pub const MQXF_XAOPEN: u32 = 29;
pub const MQXF_XAPREPARE: u32 = 30;
pub const MQXF_XARECOVER: u32 = 31;
pub const MQXF_XAROLLBACK: u32 = 32;
pub const MQXF_XASTART: u32 = 33;
pub const MQXF_AXREG: u32 = 34;
pub const MQXF_AXUNREG: u32 = 35;
pub const MQCXP_STRUC_ID: &'static [u8; 5usize] = b"CXP \0";
pub const MQCXP_VERSION_1: u32 = 1;
pub const MQCXP_VERSION_2: u32 = 2;
pub const MQCXP_VERSION_3: u32 = 3;
pub const MQCXP_VERSION_4: u32 = 4;
pub const MQCXP_VERSION_5: u32 = 5;
pub const MQCXP_VERSION_6: u32 = 6;
pub const MQCXP_VERSION_7: u32 = 7;
pub const MQCXP_VERSION_8: u32 = 8;
pub const MQCXP_VERSION_9: u32 = 9;
pub const MQCXP_CURRENT_VERSION: u32 = 9;
pub const MQCXP_LENGTH_3: u32 = 156;
pub const MQCXP_LENGTH_4: u32 = 156;
pub const MQCXP_LENGTH_5: u32 = 160;
pub const MQCXP_LENGTH_6: u32 = 200;
pub const MQCXP_LENGTH_7: u32 = 208;
pub const MQCXP_LENGTH_8: u32 = 224;
pub const MQCXP_LENGTH_9: u32 = 240;
pub const MQCXP_CURRENT_LENGTH: u32 = 240;
pub const MQXR2_PUT_WITH_DEF_ACTION: u32 = 0;
pub const MQXR2_PUT_WITH_DEF_USERID: u32 = 1;
pub const MQXR2_PUT_WITH_MSG_USERID: u32 = 2;
pub const MQXR2_USE_AGENT_BUFFER: u32 = 0;
pub const MQXR2_USE_EXIT_BUFFER: u32 = 4;
pub const MQXR2_DEFAULT_CONTINUATION: u32 = 0;
pub const MQXR2_CONTINUE_CHAIN: u32 = 8;
pub const MQXR2_SUPPRESS_CHAIN: u32 = 16;
pub const MQXR2_STATIC_CACHE: u32 = 0;
pub const MQXR2_DYNAMIC_CACHE: u32 = 32;
pub const MQCF_NONE: u32 = 0;
pub const MQCF_DIST_LISTS: u32 = 1;
pub const MQDXP_STRUC_ID: &'static [u8; 5usize] = b"DXP \0";
pub const MQDXP_VERSION_1: u32 = 1;
pub const MQDXP_VERSION_2: u32 = 2;
pub const MQDXP_CURRENT_VERSION: u32 = 2;
pub const MQDXP_LENGTH_1: u32 = 44;
pub const MQDXP_LENGTH_2: u32 = 56;
pub const MQDXP_CURRENT_LENGTH: u32 = 56;
pub const MQXDR_OK: u32 = 0;
pub const MQXDR_CONVERSION_FAILED: u32 = 1;
pub const MQNXP_STRUC_ID: &'static [u8; 5usize] = b"NXP \0";
pub const MQNXP_VERSION_1: u32 = 1;
pub const MQNXP_VERSION_2: u32 = 2;
pub const MQNXP_CURRENT_VERSION: u32 = 2;
pub const MQNXP_LENGTH_1: u32 = 64;
pub const MQNXP_LENGTH_2: u32 = 72;
pub const MQNXP_CURRENT_LENGTH: u32 = 72;
pub const MQPBC_STRUC_ID: &'static [u8; 5usize] = b"PBC \0";
pub const MQPBC_VERSION_1: u32 = 1;
pub const MQPBC_VERSION_2: u32 = 2;
pub const MQPBC_CURRENT_VERSION: u32 = 2;
pub const MQPBC_LENGTH_1: u32 = 32;
pub const MQPBC_LENGTH_2: u32 = 40;
pub const MQPBC_CURRENT_LENGTH: u32 = 40;
pub const MQPSXP_STRUC_ID: &'static [u8; 5usize] = b"PSXP\0";
pub const MQPSXP_VERSION_1: u32 = 1;
pub const MQPSXP_VERSION_2: u32 = 2;
pub const MQPSXP_CURRENT_VERSION: u32 = 2;
pub const MQPSXP_LENGTH_1: u32 = 176;
pub const MQPSXP_LENGTH_2: u32 = 184;
pub const MQPSXP_CURRENT_LENGTH: u32 = 184;
pub const MQSBC_STRUC_ID: &'static [u8; 5usize] = b"SBC \0";
pub const MQSBC_VERSION_1: u32 = 1;
pub const MQSBC_CURRENT_VERSION: u32 = 1;
pub const MQSBC_LENGTH_1: u32 = 288;
pub const MQSBC_CURRENT_LENGTH: u32 = 288;
pub const MQWDR_STRUC_ID: &'static [u8; 5usize] = b"WDR \0";
pub const MQWDR_VERSION_1: u32 = 1;
pub const MQWDR_VERSION_2: u32 = 2;
pub const MQWDR_CURRENT_VERSION: u32 = 2;
pub const MQWDR_LENGTH_1: u32 = 124;
pub const MQWDR_LENGTH_2: u32 = 136;
pub const MQWDR_CURRENT_LENGTH: u32 = 136;
pub const MQQMF_REPOSITORY_Q_MGR: u32 = 2;
pub const MQQMF_CLUSSDR_USER_DEFINED: u32 = 8;
pub const MQQMF_CLUSSDR_AUTO_DEFINED: u32 = 16;
pub const MQQMF_AVAILABLE: u32 = 32;
pub const MQWDR1_LENGTH_1: u32 = 124;
pub const MQWDR1_CURRENT_LENGTH: u32 = 124;
pub const MQWDR2_LENGTH_1: u32 = 124;
pub const MQWDR2_LENGTH_2: u32 = 136;
pub const MQWDR2_CURRENT_LENGTH: u32 = 136;
pub const MQWQR_STRUC_ID: &'static [u8; 5usize] = b"WQR \0";
pub const MQWQR_VERSION_1: u32 = 1;
pub const MQWQR_VERSION_2: u32 = 2;
pub const MQWQR_VERSION_3: u32 = 3;
pub const MQWQR_CURRENT_VERSION: u32 = 3;
pub const MQWQR_LENGTH_1: u32 = 200;
pub const MQWQR_LENGTH_2: u32 = 208;
pub const MQWQR_LENGTH_3: u32 = 212;
pub const MQWQR_CURRENT_LENGTH: u32 = 212;
pub const MQQF_LOCAL_Q: u32 = 1;
pub const MQQF_CLWL_USEQ_ANY: u32 = 64;
pub const MQQF_CLWL_USEQ_LOCAL: u32 = 128;
pub const MQWQR1_LENGTH_1: u32 = 200;
pub const MQWQR1_CURRENT_LENGTH: u32 = 200;
pub const MQWQR2_LENGTH_1: u32 = 200;
pub const MQWQR2_LENGTH_2: u32 = 208;
pub const MQWQR2_CURRENT_LENGTH: u32 = 208;
pub const MQWQR3_LENGTH_1: u32 = 200;
pub const MQWQR3_LENGTH_2: u32 = 208;
pub const MQWQR3_LENGTH_3: u32 = 212;
pub const MQWQR3_CURRENT_LENGTH: u32 = 212;
pub const MQWXP_STRUC_ID: &'static [u8; 5usize] = b"WXP \0";
pub const MQWXP_VERSION_1: u32 = 1;
pub const MQWXP_VERSION_2: u32 = 2;
pub const MQWXP_VERSION_3: u32 = 3;
pub const MQWXP_VERSION_4: u32 = 4;
pub const MQWXP_CURRENT_VERSION: u32 = 4;
pub const MQWXP_LENGTH_1: u32 = 224;
pub const MQWXP_LENGTH_2: u32 = 240;
pub const MQWXP_LENGTH_3: u32 = 240;
pub const MQWXP_LENGTH_4: u32 = 248;
pub const MQWXP_CURRENT_LENGTH: u32 = 248;
pub const MQWXP_PUT_BY_CLUSTER_CHL: u32 = 2;
pub const MQWXP1_LENGTH_1: u32 = 224;
pub const MQWXP1_CURRENT_LENGTH: u32 = 224;
pub const MQWXP2_LENGTH_1: u32 = 224;
pub const MQWXP2_LENGTH_2: u32 = 240;
pub const MQWXP2_CURRENT_LENGTH: u32 = 240;
pub const MQWXP3_LENGTH_1: u32 = 224;
pub const MQWXP3_LENGTH_2: u32 = 240;
pub const MQWXP3_LENGTH_3: u32 = 240;
pub const MQWXP3_CURRENT_LENGTH: u32 = 240;
pub const MQWXP4_LENGTH_1: u32 = 224;
pub const MQWXP4_LENGTH_2: u32 = 240;
pub const MQWXP4_LENGTH_3: u32 = 240;
pub const MQWXP4_LENGTH_4: u32 = 248;
pub const MQWXP4_CURRENT_LENGTH: u32 = 248;
pub const MQXEPO_STRUC_ID: &'static [u8; 5usize] = b"XEPO\0";
pub const MQXEPO_VERSION_1: u32 = 1;
pub const MQXEPO_CURRENT_VERSION: u32 = 1;
pub const MQXEPO_LENGTH_1: u32 = 40;
pub const MQXEPO_CURRENT_LENGTH: u32 = 40;
pub const MQXEPO_NONE: u32 = 0;
pub const MQXT_API_CROSSING_EXIT: u32 = 1;
pub const MQXT_API_EXIT: u32 = 2;
pub const MQXT_CHANNEL_SEC_EXIT: u32 = 11;
pub const MQXT_CHANNEL_MSG_EXIT: u32 = 12;
pub const MQXT_CHANNEL_SEND_EXIT: u32 = 13;
pub const MQXT_CHANNEL_RCV_EXIT: u32 = 14;
pub const MQXT_CHANNEL_MSG_RETRY_EXIT: u32 = 15;
pub const MQXT_CHANNEL_AUTO_DEF_EXIT: u32 = 16;
pub const MQXT_CLUSTER_WORKLOAD_EXIT: u32 = 20;
pub const MQXT_PUBSUB_ROUTING_EXIT: u32 = 21;
pub const MQXT_PUBLISH_EXIT: u32 = 22;
pub const MQXT_PRECONNECT_EXIT: u32 = 23;
pub const MQXR_BEFORE: u32 = 1;
pub const MQXR_AFTER: u32 = 2;
pub const MQXR_CONNECTION: u32 = 3;
pub const MQXR_BEFORE_CONVERT: u32 = 4;
pub const MQXR_INIT: u32 = 11;
pub const MQXR_TERM: u32 = 12;
pub const MQXR_MSG: u32 = 13;
pub const MQXR_XMIT: u32 = 14;
pub const MQXR_SEC_MSG: u32 = 15;
pub const MQXR_INIT_SEC: u32 = 16;
pub const MQXR_RETRY: u32 = 17;
pub const MQXR_AUTO_CLUSSDR: u32 = 18;
pub const MQXR_AUTO_RECEIVER: u32 = 19;
pub const MQXR_CLWL_OPEN: u32 = 20;
pub const MQXR_CLWL_PUT: u32 = 21;
pub const MQXR_CLWL_MOVE: u32 = 22;
pub const MQXR_CLWL_REPOS: u32 = 23;
pub const MQXR_CLWL_REPOS_MOVE: u32 = 24;
pub const MQXR_END_BATCH: u32 = 25;
pub const MQXR_ACK_RECEIVED: u32 = 26;
pub const MQXR_AUTO_SVRCONN: u32 = 27;
pub const MQXR_AUTO_CLUSRCVR: u32 = 28;
pub const MQXR_SEC_PARMS: u32 = 29;
pub const MQXR_PUBLICATION: u32 = 30;
pub const MQXR_PRECONNECT: u32 = 31;
pub const MQXCC_OK: u32 = 0;
pub const MQXCC_SUPPRESS_FUNCTION: i32 = -1;
pub const MQXCC_SKIP_FUNCTION: i32 = -2;
pub const MQXCC_SEND_AND_REQUEST_SEC_MSG: i32 = -3;
pub const MQXCC_SEND_SEC_MSG: i32 = -4;
pub const MQXCC_SUPPRESS_EXIT: i32 = -5;
pub const MQXCC_CLOSE_CHANNEL: i32 = -6;
pub const MQXCC_REQUEST_ACK: i32 = -7;
pub const MQXCC_FAILED: i32 = -8;
pub const MQXUA_NONE: &'static [u8; 17usize] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQCLCT_STATIC: u32 = 0;
pub const MQCLCT_DYNAMIC: u32 = 1;
pub const MQMCEV_PACKET_LOSS: u32 = 1;
pub const MQMCEV_HEARTBEAT_TIMEOUT: u32 = 2;
pub const MQMCEV_VERSION_CONFLICT: u32 = 3;
pub const MQMCEV_RELIABILITY: u32 = 4;
pub const MQMCEV_CLOSED_TRANS: u32 = 5;
pub const MQMCEV_STREAM_ERROR: u32 = 6;
pub const MQMCEV_NEW_SOURCE: u32 = 10;
pub const MQMCEV_RECEIVE_QUEUE_TRIMMED: u32 = 11;
pub const MQMCEV_PACKET_LOSS_NACK_EXPIRE: u32 = 12;
pub const MQMCEV_ACK_RETRIES_EXCEEDED: u32 = 13;
pub const MQMCEV_STREAM_SUSPEND_NACK: u32 = 14;
pub const MQMCEV_STREAM_RESUME_NACK: u32 = 15;
pub const MQMCEV_STREAM_EXPELLED: u32 = 16;
pub const MQMCEV_FIRST_MESSAGE: u32 = 20;
pub const MQMCEV_LATE_JOIN_FAILURE: u32 = 21;
pub const MQMCEV_MESSAGE_LOSS: u32 = 22;
pub const MQMCEV_SEND_PACKET_FAILURE: u32 = 23;
pub const MQMCEV_REPAIR_DELAY: u32 = 24;
pub const MQMCEV_MEMORY_ALERT_ON: u32 = 25;
pub const MQMCEV_MEMORY_ALERT_OFF: u32 = 26;
pub const MQMCEV_NACK_ALERT_ON: u32 = 27;
pub const MQMCEV_NACK_ALERT_OFF: u32 = 28;
pub const MQMCEV_REPAIR_ALERT_ON: u32 = 29;
pub const MQMCEV_REPAIR_ALERT_OFF: u32 = 30;
pub const MQMCEV_RELIABILITY_CHANGED: u32 = 31;
pub const MQMCEV_SHM_DEST_UNUSABLE: u32 = 80;
pub const MQMCEV_SHM_PORT_UNUSABLE: u32 = 81;
pub const MQMCEV_CCT_GETTIME_FAILED: u32 = 110;
pub const MQMCEV_DEST_INTERFACE_FAILURE: u32 = 120;
pub const MQMCEV_DEST_INTERFACE_FAILOVER: u32 = 121;
pub const MQMCEV_PORT_INTERFACE_FAILURE: u32 = 122;
pub const MQMCEV_PORT_INTERFACE_FAILOVER: u32 = 123;
pub const MQDCC_DEFAULT_CONVERSION: u32 = 1;
pub const MQDCC_FILL_TARGET_BUFFER: u32 = 2;
pub const MQDCC_INT_DEFAULT_CONVERSION: u32 = 4;
pub const MQDCC_SOURCE_ENC_NATIVE: u32 = 32;
pub const MQDCC_SOURCE_ENC_NORMAL: u32 = 16;
pub const MQDCC_SOURCE_ENC_REVERSED: u32 = 32;
pub const MQDCC_SOURCE_ENC_UNDEFINED: u32 = 0;
pub const MQDCC_TARGET_ENC_NATIVE: u32 = 512;
pub const MQDCC_TARGET_ENC_NORMAL: u32 = 256;
pub const MQDCC_TARGET_ENC_REVERSED: u32 = 512;
pub const MQDCC_TARGET_ENC_UNDEFINED: u32 = 0;
pub const MQDCC_NONE: u32 = 0;
pub const MQDCC_SOURCE_ENC_MASK: u32 = 240;
pub const MQDCC_TARGET_ENC_MASK: u32 = 3840;
pub const MQDCC_SOURCE_ENC_FACTOR: u32 = 16;
pub const MQDCC_TARGET_ENC_FACTOR: u32 = 256;
pub type MQBYTE = ::std::os::raw::c_uchar;
pub type PMQBYTE = *mut MQBYTE;
pub type PPMQBYTE = *mut PMQBYTE;
pub type MQBYTE4 = [MQBYTE; 4usize];
pub type PMQBYTE4 = *mut MQBYTE4;
pub type MQBYTE8 = [MQBYTE; 8usize];
pub type PMQBYTE8 = *mut MQBYTE8;
pub type MQBYTE16 = [MQBYTE; 16usize];
pub type PMQBYTE16 = *mut MQBYTE16;
pub type MQBYTE24 = [MQBYTE; 24usize];
pub type PMQBYTE24 = *mut MQBYTE24;
pub type MQBYTE32 = [MQBYTE; 32usize];
pub type PMQBYTE32 = *mut MQBYTE32;
pub type MQBYTE40 = [MQBYTE; 40usize];
pub type PMQBYTE40 = *mut MQBYTE40;
pub type MQBYTE48 = [MQBYTE; 48usize];
pub type PMQBYTE48 = *mut MQBYTE48;
pub type MQBYTE128 = [MQBYTE; 128usize];
pub type PMQBYTE128 = *mut MQBYTE128;
pub type MQCHAR = ::std::os::raw::c_char;
pub type PMQCHAR = *mut MQCHAR;
pub type PPMQCHAR = *mut PMQCHAR;
pub type MQCHAR4 = [MQCHAR; 4usize];
pub type PMQCHAR4 = *mut MQCHAR4;
pub type MQCHAR8 = [MQCHAR; 8usize];
pub type PMQCHAR8 = *mut MQCHAR8;
pub type MQCHAR12 = [MQCHAR; 12usize];
pub type PMQCHAR12 = *mut MQCHAR12;
pub type MQCHAR16 = [MQCHAR; 16usize];
pub type PMQCHAR16 = *mut MQCHAR16;
pub type MQCHAR20 = [MQCHAR; 20usize];
pub type PMQCHAR20 = *mut MQCHAR20;
pub type MQCHAR28 = [MQCHAR; 28usize];
pub type PMQCHAR28 = *mut MQCHAR28;
pub type MQCHAR32 = [MQCHAR; 32usize];
pub type PMQCHAR32 = *mut MQCHAR32;
pub type MQCHAR48 = [MQCHAR; 48usize];
pub type PMQCHAR48 = *mut MQCHAR48;
pub type MQCHAR64 = [MQCHAR; 64usize];
pub type PMQCHAR64 = *mut MQCHAR64;
pub type MQCHAR128 = [MQCHAR; 128usize];
pub type PMQCHAR128 = *mut MQCHAR128;
pub type MQCHAR256 = [MQCHAR; 256usize];
pub type PMQCHAR256 = *mut MQCHAR256;
pub type MQCHAR264 = [MQCHAR; 264usize];
pub type PMQCHAR264 = *mut MQCHAR264;
pub type MQLONG = ::std::os::raw::c_int;
pub type MQULONG = ::std::os::raw::c_uint;
pub type MQINT64 = ::std::os::raw::c_long;
pub type MQUINT64 = ::std::os::raw::c_ulong;
pub type PMQLONG = *mut MQLONG;
pub type PPMQLONG = *mut PMQLONG;
pub type MQINT8 = ::std::os::raw::c_schar;
pub type PMQINT8 = *mut MQINT8;
pub type PPMQINT8 = *mut PMQINT8;
pub type MQUINT8 = ::std::os::raw::c_uchar;
pub type PMQUINT8 = *mut MQUINT8;
pub type PPMQUINT8 = *mut PMQUINT8;
pub type MQINT16 = ::std::os::raw::c_short;
pub type PMQINT16 = *mut MQINT16;
pub type PPMQINT16 = *mut PMQINT16;
pub type MQUINT16 = ::std::os::raw::c_ushort;
pub type PMQUINT16 = *mut MQUINT16;
pub type PPMQUINT16 = *mut PMQUINT16;
pub type MQINT32 = MQLONG;
pub type PMQINT32 = PMQLONG;
pub type PPMQINT32 = PPMQLONG;
pub type PMQINT64 = *mut MQINT64;
pub type PPMQINT64 = *mut PMQINT64;
pub type PMQULONG = *mut MQULONG;
pub type PPMQULONG = *mut PMQULONG;
pub type MQUINT32 = MQULONG;
pub type PMQUINT32 = PMQULONG;
pub type PPMQUINT32 = PPMQULONG;
pub type PMQUINT64 = *mut MQUINT64;
pub type PPMQUINT64 = *mut PMQUINT64;
pub type MQFLOAT32 = f32;
pub type PMQFLOAT32 = *mut MQFLOAT32;
pub type PPMQFLOAT32 = *mut PMQFLOAT32;
pub type MQFLOAT64 = f64;
pub type PMQFLOAT64 = *mut MQFLOAT64;
pub type PPMQFLOAT64 = *mut PMQFLOAT64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQIEP {
    _unused: [u8; 0],
}
pub type MQIEP = tagMQIEP;
pub type PMQIEP = *mut MQIEP;
pub type PPMQIEP = *mut PMQIEP;
pub type MQHCONFIG = PMQIEP;
pub type PMQHCONFIG = *mut MQHCONFIG;
pub type MQHCONN = MQLONG;
pub type PMQHCONN = *mut MQHCONN;
pub type PPMQHCONN = *mut PMQHCONN;
pub type MQHOBJ = MQLONG;
pub type PMQHOBJ = *mut MQHOBJ;
pub type PPMQHOBJ = *mut PMQHOBJ;
pub type MQPTR = *mut ::std::os::raw::c_void;
pub type PMQPTR = *mut MQPTR;
pub type PMQFUNC = *mut ::std::os::raw::c_void;
pub type PMQVOID = *mut ::std::os::raw::c_void;
pub type PPMQVOID = *mut PMQVOID;
pub type MQBOOL = MQLONG;
pub type PMQBOOL = *mut MQBOOL;
pub type PPMQBOOL = *mut PMQBOOL;
pub type MQHMSG = MQINT64;
pub type PMQHMSG = *mut MQHMSG;
pub type PPMQHMSG = *mut PMQHMSG;
pub type MQPID = MQLONG;
pub type PMQPID = *mut MQPID;
pub type MQTID = MQLONG;
pub type PMQTID = *mut MQTID;
pub type MQAIR = tagMQAIR;
pub type PMQAIR = *mut MQAIR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQAIR {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub AuthInfoType: MQLONG,
    pub AuthInfoConnName: [MQCHAR; 264usize],
    pub LDAPUserNamePtr: PMQCHAR,
    pub LDAPUserNameOffset: MQLONG,
    pub LDAPUserNameLength: MQLONG,
    pub LDAPPassword: MQCHAR32,
    pub OCSPResponderURL: MQCHAR256,
}
#[test]
fn bindgen_test_layout_tagMQAIR() {
    assert_eq!(
        ::std::mem::size_of::<tagMQAIR>(),
        584usize,
        concat!("Size of: ", stringify!(tagMQAIR))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQAIR>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQAIR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).AuthInfoType as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(AuthInfoType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).AuthInfoConnName as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(AuthInfoConnName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).LDAPUserNamePtr as *const _ as usize },
        280usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(LDAPUserNamePtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).LDAPUserNameOffset as *const _ as usize },
        288usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(LDAPUserNameOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).LDAPUserNameLength as *const _ as usize },
        292usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(LDAPUserNameLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).LDAPPassword as *const _ as usize },
        296usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(LDAPPassword)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAIR>())).OCSPResponderURL as *const _ as usize },
        328usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAIR),
        "::",
        stringify!(OCSPResponderURL)
        )
    );
}
pub type MQBMHO = tagMQBMHO;
pub type PMQBMHO = *mut MQBMHO;
pub type PPMQBMHO = *mut PMQBMHO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQBMHO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQBMHO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQBMHO>(),
        12usize,
        concat!("Size of: ", stringify!(tagMQBMHO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQBMHO>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQBMHO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQBMHO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQBMHO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQBMHO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQBMHO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQBMHO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQBMHO),
        "::",
        stringify!(Options)
        )
    );
}
pub type MQBO = tagMQBO;
pub type PMQBO = *mut MQBO;
pub type PPMQBO = *mut PMQBO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQBO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQBO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQBO>(),
        12usize,
        concat!("Size of: ", stringify!(tagMQBO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQBO>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQBO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQBO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQBO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQBO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQBO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQBO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQBO),
        "::",
        stringify!(Options)
        )
    );
}
pub type MQCBC = tagMQCBC;
pub type PMQCBC = *mut MQCBC;
pub type PPMQCBC = *mut PMQCBC;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCBC {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub CallType: MQLONG,
    pub Hobj: MQHOBJ,
    pub CallbackArea: MQPTR,
    pub ConnectionArea: MQPTR,
    pub CompCode: MQLONG,
    pub Reason: MQLONG,
    pub State: MQLONG,
    pub DataLength: MQLONG,
    pub BufferLength: MQLONG,
    pub Flags: MQLONG,
    pub ReconnectDelay: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQCBC() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCBC>(),
        64usize,
        concat!("Size of: ", stringify!(tagMQCBC))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCBC>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQCBC))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).CallType as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(CallType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).Hobj as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(Hobj)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).CallbackArea as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(CallbackArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).ConnectionArea as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(ConnectionArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).CompCode as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(CompCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).Reason as *const _ as usize },
        36usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(Reason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).State as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(State)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).DataLength as *const _ as usize },
        44usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(DataLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).BufferLength as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(BufferLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).Flags as *const _ as usize },
        52usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBC>())).ReconnectDelay as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBC),
        "::",
        stringify!(ReconnectDelay)
        )
    );
}
pub type MQCBD = tagMQCBD;
pub type PMQCBD = *mut MQCBD;
pub type PPMQCBD = *mut PMQCBD;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQCBD {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub CallbackType: MQLONG,
    pub Options: MQLONG,
    pub CallbackArea: MQPTR,
    pub CallbackFunction: MQPTR,
    pub CallbackName: MQCHAR128,
    pub MaxMsgLength: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQCBD() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCBD>(),
        168usize,
        concat!("Size of: ", stringify!(tagMQCBD))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCBD>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQCBD))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBD>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBD),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBD>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBD),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBD>())).CallbackType as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBD),
        "::",
        stringify!(CallbackType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBD>())).Options as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBD),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBD>())).CallbackArea as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBD),
        "::",
        stringify!(CallbackArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBD>())).CallbackFunction as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBD),
        "::",
        stringify!(CallbackFunction)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBD>())).CallbackName as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBD),
        "::",
        stringify!(CallbackName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCBD>())).MaxMsgLength as *const _ as usize },
        160usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCBD),
        "::",
        stringify!(MaxMsgLength)
        )
    );
}
pub type MQCHARV = tagMQCHARV;
pub type PMQCHARV = *mut MQCHARV;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCHARV {
    pub VSPtr: MQPTR,
    pub VSOffset: MQLONG,
    pub VSBufSize: MQLONG,
    pub VSLength: MQLONG,
    pub VSCCSID: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQCHARV() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCHARV>(),
        24usize,
        concat!("Size of: ", stringify!(tagMQCHARV))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCHARV>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQCHARV))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCHARV>())).VSPtr as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCHARV),
        "::",
        stringify!(VSPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCHARV>())).VSOffset as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCHARV),
        "::",
        stringify!(VSOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCHARV>())).VSBufSize as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCHARV),
        "::",
        stringify!(VSBufSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCHARV>())).VSLength as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCHARV),
        "::",
        stringify!(VSLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCHARV>())).VSCCSID as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCHARV),
        "::",
        stringify!(VSCCSID)
        )
    );
}
pub type MQCIH = tagMQCIH;
pub type PMQCIH = *mut MQCIH;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCIH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Flags: MQLONG,
    pub ReturnCode: MQLONG,
    pub CompCode: MQLONG,
    pub Reason: MQLONG,
    pub UOWControl: MQLONG,
    pub GetWaitInterval: MQLONG,
    pub LinkType: MQLONG,
    pub OutputDataLength: MQLONG,
    pub FacilityKeepTime: MQLONG,
    pub ADSDescriptor: MQLONG,
    pub ConversationalTask: MQLONG,
    pub TaskEndStatus: MQLONG,
    pub Facility: MQBYTE8,
    pub Function: MQCHAR4,
    pub AbendCode: MQCHAR4,
    pub Authenticator: MQCHAR8,
    pub Reserved1: MQCHAR8,
    pub ReplyToFormat: MQCHAR8,
    pub RemoteSysId: MQCHAR4,
    pub RemoteTransId: MQCHAR4,
    pub TransactionId: MQCHAR4,
    pub FacilityLike: MQCHAR4,
    pub AttentionId: MQCHAR4,
    pub StartCode: MQCHAR4,
    pub CancelCode: MQCHAR4,
    pub NextTransactionId: MQCHAR4,
    pub Reserved2: MQCHAR8,
    pub Reserved3: MQCHAR8,
    pub CursorPosition: MQLONG,
    pub ErrorOffset: MQLONG,
    pub InputItem: MQLONG,
    pub Reserved4: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQCIH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCIH>(),
        180usize,
        concat!("Size of: ", stringify!(tagMQCIH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCIH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQCIH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Encoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).CodedCharSetId as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Format as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).ReturnCode as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(ReturnCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).CompCode as *const _ as usize },
        36usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(CompCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Reason as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Reason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).UOWControl as *const _ as usize },
        44usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(UOWControl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).GetWaitInterval as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(GetWaitInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).LinkType as *const _ as usize },
        52usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(LinkType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).OutputDataLength as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(OutputDataLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).FacilityKeepTime as *const _ as usize },
        60usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(FacilityKeepTime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).ADSDescriptor as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(ADSDescriptor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).ConversationalTask as *const _ as usize },
        68usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(ConversationalTask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).TaskEndStatus as *const _ as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(TaskEndStatus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Facility as *const _ as usize },
        76usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Facility)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Function as *const _ as usize },
        84usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Function)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).AbendCode as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(AbendCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Authenticator as *const _ as usize },
        92usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Authenticator)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Reserved1 as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Reserved1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).ReplyToFormat as *const _ as usize },
        108usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(ReplyToFormat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).RemoteSysId as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(RemoteSysId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).RemoteTransId as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(RemoteTransId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).TransactionId as *const _ as usize },
        124usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(TransactionId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).FacilityLike as *const _ as usize },
        128usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(FacilityLike)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).AttentionId as *const _ as usize },
        132usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(AttentionId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).StartCode as *const _ as usize },
        136usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(StartCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).CancelCode as *const _ as usize },
        140usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(CancelCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).NextTransactionId as *const _ as usize },
        144usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(NextTransactionId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Reserved2 as *const _ as usize },
        148usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Reserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Reserved3 as *const _ as usize },
        156usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Reserved3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).CursorPosition as *const _ as usize },
        164usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(CursorPosition)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).ErrorOffset as *const _ as usize },
        168usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(ErrorOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).InputItem as *const _ as usize },
        172usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(InputItem)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCIH>())).Reserved4 as *const _ as usize },
        176usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCIH),
        "::",
        stringify!(Reserved4)
        )
    );
}
pub type MQCMHO = tagMQCMHO;
pub type PMQCMHO = *mut MQCMHO;
pub type PPMQCMHO = *mut PMQCMHO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCMHO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQCMHO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCMHO>(),
        12usize,
        concat!("Size of: ", stringify!(tagMQCMHO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCMHO>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQCMHO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCMHO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCMHO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCMHO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCMHO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCMHO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCMHO),
        "::",
        stringify!(Options)
        )
    );
}
pub type MQCTLO = tagMQCTLO;
pub type PMQCTLO = *mut MQCTLO;
pub type PPMQCTLO = *mut PMQCTLO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCTLO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub Reserved: MQLONG,
    pub ConnectionArea: MQPTR,
}
#[test]
fn bindgen_test_layout_tagMQCTLO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCTLO>(),
        24usize,
        concat!("Size of: ", stringify!(tagMQCTLO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCTLO>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQCTLO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCTLO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCTLO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCTLO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCTLO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCTLO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCTLO),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCTLO>())).Reserved as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCTLO),
        "::",
        stringify!(Reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCTLO>())).ConnectionArea as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCTLO),
        "::",
        stringify!(ConnectionArea)
        )
    );
}
pub type MQSCO = tagMQSCO;
pub type PMQSCO = *mut MQSCO;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQSCO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub KeyRepository: MQCHAR256,
    pub CryptoHardware: MQCHAR256,
    pub AuthInfoRecCount: MQLONG,
    pub AuthInfoRecOffset: MQLONG,
    pub AuthInfoRecPtr: PMQAIR,
    pub KeyResetCount: MQLONG,
    pub FipsRequired: MQLONG,
    pub EncryptionPolicySuiteB: [MQLONG; 4usize],
    pub CertificateValPolicy: MQLONG,
    pub CertificateLabel: MQCHAR64,
}
#[test]
fn bindgen_test_layout_tagMQSCO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQSCO>(),
        632usize,
        concat!("Size of: ", stringify!(tagMQSCO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQSCO>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQSCO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).KeyRepository as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(KeyRepository)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).CryptoHardware as *const _ as usize },
        264usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(CryptoHardware)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).AuthInfoRecCount as *const _ as usize },
        520usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(AuthInfoRecCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).AuthInfoRecOffset as *const _ as usize },
        524usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(AuthInfoRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).AuthInfoRecPtr as *const _ as usize },
        528usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(AuthInfoRecPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).KeyResetCount as *const _ as usize },
        536usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(KeyResetCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).FipsRequired as *const _ as usize },
        540usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(FipsRequired)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).EncryptionPolicySuiteB as *const _ as usize },
        544usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(EncryptionPolicySuiteB)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).CertificateValPolicy as *const _ as usize },
        560usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(CertificateValPolicy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSCO>())).CertificateLabel as *const _ as usize },
        564usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSCO),
        "::",
        stringify!(CertificateLabel)
        )
    );
}
pub type MQCSP = tagMQCSP;
pub type PMQCSP = *mut MQCSP;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCSP {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub AuthenticationType: MQLONG,
    pub Reserved1: MQBYTE4,
    pub CSPUserIdPtr: MQPTR,
    pub CSPUserIdOffset: MQLONG,
    pub CSPUserIdLength: MQLONG,
    pub Reserved2: MQBYTE8,
    pub CSPPasswordPtr: MQPTR,
    pub CSPPasswordOffset: MQLONG,
    pub CSPPasswordLength: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQCSP() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCSP>(),
        56usize,
        concat!("Size of: ", stringify!(tagMQCSP))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCSP>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQCSP))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).AuthenticationType as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(AuthenticationType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).Reserved1 as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(Reserved1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).CSPUserIdPtr as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(CSPUserIdPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).CSPUserIdOffset as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(CSPUserIdOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).CSPUserIdLength as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(CSPUserIdLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).Reserved2 as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(Reserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).CSPPasswordPtr as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(CSPPasswordPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).CSPPasswordOffset as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(CSPPasswordOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCSP>())).CSPPasswordLength as *const _ as usize },
        52usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCSP),
        "::",
        stringify!(CSPPasswordLength)
        )
    );
}
pub type MQCNO = tagMQCNO;
pub type PMQCNO = *mut MQCNO;
pub type PPMQCNO = *mut PMQCNO;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQCNO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub ClientConnOffset: MQLONG,
    pub ClientConnPtr: MQPTR,
    pub ConnTag: MQBYTE128,
    pub SSLConfigPtr: PMQSCO,
    pub SSLConfigOffset: MQLONG,
    pub ConnectionId: MQBYTE24,
    pub SecurityParmsOffset: MQLONG,
    pub SecurityParmsPtr: PMQCSP,
    pub CCDTUrlPtr: PMQCHAR,
    pub CCDTUrlOffset: MQLONG,
    pub CCDTUrlLength: MQLONG,
    pub Reserved: MQBYTE8,
    pub ApplName: MQCHAR28,
    pub Reserved2: MQBYTE4,
}
#[test]
fn bindgen_test_layout_tagMQCNO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCNO>(),
        256usize,
        concat!("Size of: ", stringify!(tagMQCNO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCNO>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQCNO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).ClientConnOffset as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(ClientConnOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).ClientConnPtr as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(ClientConnPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).ConnTag as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(ConnTag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).SSLConfigPtr as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(SSLConfigPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).SSLConfigOffset as *const _ as usize },
        160usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(SSLConfigOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).ConnectionId as *const _ as usize },
        164usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(ConnectionId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).SecurityParmsOffset as *const _ as usize },
        188usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(SecurityParmsOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).SecurityParmsPtr as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(SecurityParmsPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).CCDTUrlPtr as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(CCDTUrlPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).CCDTUrlOffset as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(CCDTUrlOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).CCDTUrlLength as *const _ as usize },
        212usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(CCDTUrlLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).Reserved as *const _ as usize },
        216usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(Reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).ApplName as *const _ as usize },
        224usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(ApplName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCNO>())).Reserved2 as *const _ as usize },
        252usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCNO),
        "::",
        stringify!(Reserved2)
        )
    );
}
pub type MQDH = tagMQDH;
pub type PMQDH = *mut MQDH;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Flags: MQLONG,
    pub PutMsgRecFields: MQLONG,
    pub RecsPresent: MQLONG,
    pub ObjectRecOffset: MQLONG,
    pub PutMsgRecOffset: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQDH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQDH>(),
        48usize,
        concat!("Size of: ", stringify!(tagMQDH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQDH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQDH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).Encoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).CodedCharSetId as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).Format as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).PutMsgRecFields as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(PutMsgRecFields)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).RecsPresent as *const _ as usize },
        36usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(RecsPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).ObjectRecOffset as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(ObjectRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDH>())).PutMsgRecOffset as *const _ as usize },
        44usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDH),
        "::",
        stringify!(PutMsgRecOffset)
        )
    );
}
pub type MQDLH = tagMQDLH;
pub type PMQDLH = *mut MQDLH;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQDLH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Reason: MQLONG,
    pub DestQName: MQCHAR48,
    pub DestQMgrName: MQCHAR48,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub PutApplType: MQLONG,
    pub PutApplName: MQCHAR28,
    pub PutDate: MQCHAR8,
    pub PutTime: MQCHAR8,
}
#[test]
fn bindgen_test_layout_tagMQDLH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQDLH>(),
        172usize,
        concat!("Size of: ", stringify!(tagMQDLH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQDLH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQDLH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).Reason as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(Reason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).DestQName as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(DestQName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).DestQMgrName as *const _ as usize },
        60usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(DestQMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).Encoding as *const _ as usize },
        108usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).CodedCharSetId as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).Format as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).PutApplType as *const _ as usize },
        124usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(PutApplType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).PutApplName as *const _ as usize },
        128usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(PutApplName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).PutDate as *const _ as usize },
        156usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(PutDate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDLH>())).PutTime as *const _ as usize },
        164usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDLH),
        "::",
        stringify!(PutTime)
        )
    );
}
pub type MQDMHO = tagMQDMHO;
pub type PMQDMHO = *mut MQDMHO;
pub type PPMQDMHO = *mut PMQDMHO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDMHO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQDMHO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQDMHO>(),
        12usize,
        concat!("Size of: ", stringify!(tagMQDMHO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQDMHO>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQDMHO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDMHO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDMHO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDMHO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDMHO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDMHO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDMHO),
        "::",
        stringify!(Options)
        )
    );
}
pub type MQDMPO = tagMQDMPO;
pub type PMQDMPO = *mut MQDMPO;
pub type PPMQDMPO = *mut PMQDMPO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDMPO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQDMPO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQDMPO>(),
        12usize,
        concat!("Size of: ", stringify!(tagMQDMPO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQDMPO>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQDMPO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDMPO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDMPO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDMPO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDMPO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDMPO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDMPO),
        "::",
        stringify!(Options)
        )
    );
}
pub type MQGMO = tagMQGMO;
pub type PMQGMO = *mut MQGMO;
pub type PPMQGMO = *mut PMQGMO;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQGMO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub WaitInterval: MQLONG,
    pub Signal1: MQLONG,
    pub Signal2: MQLONG,
    pub ResolvedQName: MQCHAR48,
    pub MatchOptions: MQLONG,
    pub GroupStatus: MQCHAR,
    pub SegmentStatus: MQCHAR,
    pub Segmentation: MQCHAR,
    pub Reserved1: MQCHAR,
    pub MsgToken: MQBYTE16,
    pub ReturnedLength: MQLONG,
    pub Reserved2: MQLONG,
    pub MsgHandle: MQHMSG,
}
#[test]
fn bindgen_test_layout_tagMQGMO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQGMO>(),
        112usize,
        concat!("Size of: ", stringify!(tagMQGMO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQGMO>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQGMO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).WaitInterval as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(WaitInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).Signal1 as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(Signal1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).Signal2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(Signal2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).ResolvedQName as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(ResolvedQName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).MatchOptions as *const _ as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(MatchOptions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).GroupStatus as *const _ as usize },
        76usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(GroupStatus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).SegmentStatus as *const _ as usize },
        77usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(SegmentStatus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).Segmentation as *const _ as usize },
        78usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(Segmentation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).Reserved1 as *const _ as usize },
        79usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(Reserved1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).MsgToken as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(MsgToken)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).ReturnedLength as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(ReturnedLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).Reserved2 as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(Reserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQGMO>())).MsgHandle as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQGMO),
        "::",
        stringify!(MsgHandle)
        )
    );
}
pub type MQIIH = tagMQIIH;
pub type PMQIIH = *mut MQIIH;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQIIH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Flags: MQLONG,
    pub LTermOverride: MQCHAR8,
    pub MFSMapName: MQCHAR8,
    pub ReplyToFormat: MQCHAR8,
    pub Authenticator: MQCHAR8,
    pub TranInstanceId: MQBYTE16,
    pub TranState: MQCHAR,
    pub CommitMode: MQCHAR,
    pub SecurityScope: MQCHAR,
    pub Reserved: MQCHAR,
}
#[test]
fn bindgen_test_layout_tagMQIIH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQIIH>(),
        84usize,
        concat!("Size of: ", stringify!(tagMQIIH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQIIH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQIIH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).Encoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).CodedCharSetId as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).Format as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).LTermOverride as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(LTermOverride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).MFSMapName as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(MFSMapName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).ReplyToFormat as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(ReplyToFormat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).Authenticator as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(Authenticator)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).TranInstanceId as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(TranInstanceId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).TranState as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(TranState)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).CommitMode as *const _ as usize },
        81usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(CommitMode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).SecurityScope as *const _ as usize },
        82usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(SecurityScope)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIIH>())).Reserved as *const _ as usize },
        83usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIIH),
        "::",
        stringify!(Reserved)
        )
    );
}
pub type MQIMPO = tagMQIMPO;
pub type PMQIMPO = *mut MQIMPO;
pub type PPMQIMPO = *mut PMQIMPO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQIMPO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub RequestedEncoding: MQLONG,
    pub RequestedCCSID: MQLONG,
    pub ReturnedEncoding: MQLONG,
    pub ReturnedCCSID: MQLONG,
    pub Reserved1: MQLONG,
    pub ReturnedName: MQCHARV,
    pub TypeString: MQCHAR8,
}
#[test]
fn bindgen_test_layout_tagMQIMPO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQIMPO>(),
        64usize,
        concat!("Size of: ", stringify!(tagMQIMPO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQIMPO>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQIMPO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).RequestedEncoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(RequestedEncoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).RequestedCCSID as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(RequestedCCSID)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).ReturnedEncoding as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(ReturnedEncoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).ReturnedCCSID as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(ReturnedCCSID)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).Reserved1 as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(Reserved1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).ReturnedName as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(ReturnedName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQIMPO>())).TypeString as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQIMPO),
        "::",
        stringify!(TypeString)
        )
    );
}
pub type MQMD = tagMQMD;
pub type PMQMD = *mut MQMD;
pub type PPMQMD = *mut PMQMD;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQMD {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Report: MQLONG,
    pub MsgType: MQLONG,
    pub Expiry: MQLONG,
    pub Feedback: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Priority: MQLONG,
    pub Persistence: MQLONG,
    pub MsgId: MQBYTE24,
    pub CorrelId: MQBYTE24,
    pub BackoutCount: MQLONG,
    pub ReplyToQ: MQCHAR48,
    pub ReplyToQMgr: MQCHAR48,
    pub UserIdentifier: MQCHAR12,
    pub AccountingToken: MQBYTE32,
    pub ApplIdentityData: MQCHAR32,
    pub PutApplType: MQLONG,
    pub PutApplName: MQCHAR28,
    pub PutDate: MQCHAR8,
    pub PutTime: MQCHAR8,
    pub ApplOriginData: MQCHAR4,
    pub GroupId: MQBYTE24,
    pub MsgSeqNumber: MQLONG,
    pub Offset: MQLONG,
    pub MsgFlags: MQLONG,
    pub OriginalLength: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQMD() {
    assert_eq!(
        ::std::mem::size_of::<tagMQMD>(),
        364usize,
        concat!("Size of: ", stringify!(tagMQMD))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQMD>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQMD))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Report as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Report)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).MsgType as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(MsgType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Expiry as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Expiry)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Feedback as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Encoding as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).CodedCharSetId as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Format as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Priority as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Priority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Persistence as *const _ as usize },
        44usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Persistence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).MsgId as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(MsgId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).CorrelId as *const _ as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(CorrelId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).BackoutCount as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(BackoutCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).ReplyToQ as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(ReplyToQ)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).ReplyToQMgr as *const _ as usize },
        148usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(ReplyToQMgr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).UserIdentifier as *const _ as usize },
        196usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(UserIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).AccountingToken as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(AccountingToken)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).ApplIdentityData as *const _ as usize },
        240usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(ApplIdentityData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).PutApplType as *const _ as usize },
        272usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(PutApplType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).PutApplName as *const _ as usize },
        276usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(PutApplName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).PutDate as *const _ as usize },
        304usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(PutDate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).PutTime as *const _ as usize },
        312usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(PutTime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).ApplOriginData as *const _ as usize },
        320usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(ApplOriginData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).GroupId as *const _ as usize },
        324usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(GroupId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).MsgSeqNumber as *const _ as usize },
        348usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(MsgSeqNumber)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).Offset as *const _ as usize },
        352usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(Offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).MsgFlags as *const _ as usize },
        356usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(MsgFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD>())).OriginalLength as *const _ as usize },
        360usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD),
        "::",
        stringify!(OriginalLength)
        )
    );
}
pub type MQMDE = tagMQMDE;
pub type PMQMDE = *mut MQMDE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQMDE {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Flags: MQLONG,
    pub GroupId: MQBYTE24,
    pub MsgSeqNumber: MQLONG,
    pub Offset: MQLONG,
    pub MsgFlags: MQLONG,
    pub OriginalLength: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQMDE() {
    assert_eq!(
        ::std::mem::size_of::<tagMQMDE>(),
        72usize,
        concat!("Size of: ", stringify!(tagMQMDE))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQMDE>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQMDE))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).Encoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).CodedCharSetId as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).Format as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).GroupId as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(GroupId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).MsgSeqNumber as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(MsgSeqNumber)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).Offset as *const _ as usize },
        60usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(Offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).MsgFlags as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(MsgFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMDE>())).OriginalLength as *const _ as usize },
        68usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMDE),
        "::",
        stringify!(OriginalLength)
        )
    );
}
pub type MQMD1 = tagMQMD1;
pub type PMQMD1 = *mut MQMD1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQMD1 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Report: MQLONG,
    pub MsgType: MQLONG,
    pub Expiry: MQLONG,
    pub Feedback: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Priority: MQLONG,
    pub Persistence: MQLONG,
    pub MsgId: MQBYTE24,
    pub CorrelId: MQBYTE24,
    pub BackoutCount: MQLONG,
    pub ReplyToQ: MQCHAR48,
    pub ReplyToQMgr: MQCHAR48,
    pub UserIdentifier: MQCHAR12,
    pub AccountingToken: MQBYTE32,
    pub ApplIdentityData: MQCHAR32,
    pub PutApplType: MQLONG,
    pub PutApplName: MQCHAR28,
    pub PutDate: MQCHAR8,
    pub PutTime: MQCHAR8,
    pub ApplOriginData: MQCHAR4,
}
#[test]
fn bindgen_test_layout_tagMQMD1() {
    assert_eq!(
        ::std::mem::size_of::<tagMQMD1>(),
        324usize,
        concat!("Size of: ", stringify!(tagMQMD1))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQMD1>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQMD1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).Report as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(Report)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).MsgType as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(MsgType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).Expiry as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(Expiry)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).Feedback as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).Encoding as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).CodedCharSetId as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).Format as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).Priority as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(Priority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).Persistence as *const _ as usize },
        44usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(Persistence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).MsgId as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(MsgId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).CorrelId as *const _ as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(CorrelId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).BackoutCount as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(BackoutCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).ReplyToQ as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(ReplyToQ)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).ReplyToQMgr as *const _ as usize },
        148usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(ReplyToQMgr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).UserIdentifier as *const _ as usize },
        196usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(UserIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).AccountingToken as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(AccountingToken)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).ApplIdentityData as *const _ as usize },
        240usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(ApplIdentityData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).PutApplType as *const _ as usize },
        272usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(PutApplType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).PutApplName as *const _ as usize },
        276usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(PutApplName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).PutDate as *const _ as usize },
        304usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(PutDate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).PutTime as *const _ as usize },
        312usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(PutTime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD1>())).ApplOriginData as *const _ as usize },
        320usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD1),
        "::",
        stringify!(ApplOriginData)
        )
    );
}
pub type MQMD2 = tagMQMD2;
pub type PMQMD2 = *mut MQMD2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQMD2 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Report: MQLONG,
    pub MsgType: MQLONG,
    pub Expiry: MQLONG,
    pub Feedback: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Priority: MQLONG,
    pub Persistence: MQLONG,
    pub MsgId: MQBYTE24,
    pub CorrelId: MQBYTE24,
    pub BackoutCount: MQLONG,
    pub ReplyToQ: MQCHAR48,
    pub ReplyToQMgr: MQCHAR48,
    pub UserIdentifier: MQCHAR12,
    pub AccountingToken: MQBYTE32,
    pub ApplIdentityData: MQCHAR32,
    pub PutApplType: MQLONG,
    pub PutApplName: MQCHAR28,
    pub PutDate: MQCHAR8,
    pub PutTime: MQCHAR8,
    pub ApplOriginData: MQCHAR4,
    pub GroupId: MQBYTE24,
    pub MsgSeqNumber: MQLONG,
    pub Offset: MQLONG,
    pub MsgFlags: MQLONG,
    pub OriginalLength: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQMD2() {
    assert_eq!(
        ::std::mem::size_of::<tagMQMD2>(),
        364usize,
        concat!("Size of: ", stringify!(tagMQMD2))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQMD2>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQMD2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Report as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Report)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).MsgType as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(MsgType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Expiry as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Expiry)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Feedback as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Encoding as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).CodedCharSetId as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Format as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Priority as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Priority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Persistence as *const _ as usize },
        44usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Persistence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).MsgId as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(MsgId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).CorrelId as *const _ as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(CorrelId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).BackoutCount as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(BackoutCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).ReplyToQ as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(ReplyToQ)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).ReplyToQMgr as *const _ as usize },
        148usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(ReplyToQMgr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).UserIdentifier as *const _ as usize },
        196usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(UserIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).AccountingToken as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(AccountingToken)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).ApplIdentityData as *const _ as usize },
        240usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(ApplIdentityData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).PutApplType as *const _ as usize },
        272usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(PutApplType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).PutApplName as *const _ as usize },
        276usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(PutApplName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).PutDate as *const _ as usize },
        304usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(PutDate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).PutTime as *const _ as usize },
        312usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(PutTime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).ApplOriginData as *const _ as usize },
        320usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(ApplOriginData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).GroupId as *const _ as usize },
        324usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(GroupId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).MsgSeqNumber as *const _ as usize },
        348usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(MsgSeqNumber)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).Offset as *const _ as usize },
        352usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(Offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).MsgFlags as *const _ as usize },
        356usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(MsgFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMD2>())).OriginalLength as *const _ as usize },
        360usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMD2),
        "::",
        stringify!(OriginalLength)
        )
    );
}
pub type MQMHBO = tagMQMHBO;
pub type PMQMHBO = *mut MQMHBO;
pub type PPMQMHBO = *mut PMQMHBO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQMHBO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQMHBO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQMHBO>(),
        12usize,
        concat!("Size of: ", stringify!(tagMQMHBO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQMHBO>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQMHBO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMHBO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMHBO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMHBO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMHBO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQMHBO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQMHBO),
        "::",
        stringify!(Options)
        )
    );
}
pub type MQOD = tagMQOD;
pub type PMQOD = *mut MQOD;
pub type PPMQOD = *mut PMQOD;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQOD {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ObjectType: MQLONG,
    pub ObjectName: MQCHAR48,
    pub ObjectQMgrName: MQCHAR48,
    pub DynamicQName: MQCHAR48,
    pub AlternateUserId: MQCHAR12,
    pub RecsPresent: MQLONG,
    pub KnownDestCount: MQLONG,
    pub UnknownDestCount: MQLONG,
    pub InvalidDestCount: MQLONG,
    pub ObjectRecOffset: MQLONG,
    pub ResponseRecOffset: MQLONG,
    pub ObjectRecPtr: MQPTR,
    pub ResponseRecPtr: MQPTR,
    pub AlternateSecurityId: MQBYTE40,
    pub ResolvedQName: MQCHAR48,
    pub ResolvedQMgrName: MQCHAR48,
    pub ObjectString: MQCHARV,
    pub SelectionString: MQCHARV,
    pub ResObjectString: MQCHARV,
    pub ResolvedType: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQOD() {
    assert_eq!(
        ::std::mem::size_of::<tagMQOD>(),
        424usize,
        concat!("Size of: ", stringify!(tagMQOD))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQOD>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQOD))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ObjectType as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ObjectType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ObjectName as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ObjectName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ObjectQMgrName as *const _ as usize },
        60usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ObjectQMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).DynamicQName as *const _ as usize },
        108usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(DynamicQName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).AlternateUserId as *const _ as usize },
        156usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(AlternateUserId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).RecsPresent as *const _ as usize },
        168usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(RecsPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).KnownDestCount as *const _ as usize },
        172usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(KnownDestCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).UnknownDestCount as *const _ as usize },
        176usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(UnknownDestCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).InvalidDestCount as *const _ as usize },
        180usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(InvalidDestCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ObjectRecOffset as *const _ as usize },
        184usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ObjectRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ResponseRecOffset as *const _ as usize },
        188usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ResponseRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ObjectRecPtr as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ObjectRecPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ResponseRecPtr as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ResponseRecPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).AlternateSecurityId as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(AlternateSecurityId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ResolvedQName as *const _ as usize },
        248usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ResolvedQName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ResolvedQMgrName as *const _ as usize },
        296usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ResolvedQMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ObjectString as *const _ as usize },
        344usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ObjectString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).SelectionString as *const _ as usize },
        368usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(SelectionString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ResObjectString as *const _ as usize },
        392usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ResObjectString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOD>())).ResolvedType as *const _ as usize },
        416usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOD),
        "::",
        stringify!(ResolvedType)
        )
    );
}
pub type MQOR = tagMQOR;
pub type PMQOR = *mut MQOR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQOR {
    pub ObjectName: MQCHAR48,
    pub ObjectQMgrName: MQCHAR48,
}
#[test]
fn bindgen_test_layout_tagMQOR() {
    assert_eq!(
        ::std::mem::size_of::<tagMQOR>(),
        96usize,
        concat!("Size of: ", stringify!(tagMQOR))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQOR>(),
        1usize,
        concat!("Alignment of ", stringify!(tagMQOR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOR>())).ObjectName as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOR),
        "::",
        stringify!(ObjectName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQOR>())).ObjectQMgrName as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQOR),
        "::",
        stringify!(ObjectQMgrName)
        )
    );
}
pub type MQPD = tagMQPD;
pub type PMQPD = *mut MQPD;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQPD {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub Support: MQLONG,
    pub Context: MQLONG,
    pub CopyOptions: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQPD() {
    assert_eq!(
        ::std::mem::size_of::<tagMQPD>(),
        24usize,
        concat!("Size of: ", stringify!(tagMQPD))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQPD>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQPD))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPD>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPD),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPD>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPD),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPD>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPD),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPD>())).Support as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPD),
        "::",
        stringify!(Support)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPD>())).Context as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPD),
        "::",
        stringify!(Context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPD>())).CopyOptions as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPD),
        "::",
        stringify!(CopyOptions)
        )
    );
}
pub type MQPMO = tagMQPMO;
pub type PMQPMO = *mut MQPMO;
pub type PPMQPMO = *mut PMQPMO;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQPMO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub Timeout: MQLONG,
    pub Context: MQHOBJ,
    pub KnownDestCount: MQLONG,
    pub UnknownDestCount: MQLONG,
    pub InvalidDestCount: MQLONG,
    pub ResolvedQName: MQCHAR48,
    pub ResolvedQMgrName: MQCHAR48,
    pub RecsPresent: MQLONG,
    pub PutMsgRecFields: MQLONG,
    pub PutMsgRecOffset: MQLONG,
    pub ResponseRecOffset: MQLONG,
    pub PutMsgRecPtr: MQPTR,
    pub ResponseRecPtr: MQPTR,
    pub OriginalMsgHandle: MQHMSG,
    pub NewMsgHandle: MQHMSG,
    pub Action: MQLONG,
    pub PubLevel: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQPMO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQPMO>(),
        184usize,
        concat!("Size of: ", stringify!(tagMQPMO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQPMO>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQPMO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).Timeout as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(Timeout)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).Context as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(Context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).KnownDestCount as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(KnownDestCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).UnknownDestCount as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(UnknownDestCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).InvalidDestCount as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(InvalidDestCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).ResolvedQName as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(ResolvedQName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).ResolvedQMgrName as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(ResolvedQMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).RecsPresent as *const _ as usize },
        128usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(RecsPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).PutMsgRecFields as *const _ as usize },
        132usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(PutMsgRecFields)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).PutMsgRecOffset as *const _ as usize },
        136usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(PutMsgRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).ResponseRecOffset as *const _ as usize },
        140usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(ResponseRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).PutMsgRecPtr as *const _ as usize },
        144usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(PutMsgRecPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).ResponseRecPtr as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(ResponseRecPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).OriginalMsgHandle as *const _ as usize },
        160usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(OriginalMsgHandle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).NewMsgHandle as *const _ as usize },
        168usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(NewMsgHandle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).Action as *const _ as usize },
        176usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(Action)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPMO>())).PubLevel as *const _ as usize },
        180usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPMO),
        "::",
        stringify!(PubLevel)
        )
    );
}
pub type MQRFH = tagMQRFH;
pub type PMQRFH = *mut MQRFH;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQRFH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Flags: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQRFH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQRFH>(),
        32usize,
        concat!("Size of: ", stringify!(tagMQRFH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQRFH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQRFH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH>())).Encoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH>())).CodedCharSetId as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH>())).Format as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH),
        "::",
        stringify!(Flags)
        )
    );
}
pub type MQRFH2 = tagMQRFH2;
pub type PMQRFH2 = *mut MQRFH2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQRFH2 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Flags: MQLONG,
    pub NameValueCCSID: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQRFH2() {
    assert_eq!(
        ::std::mem::size_of::<tagMQRFH2>(),
        36usize,
        concat!("Size of: ", stringify!(tagMQRFH2))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQRFH2>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQRFH2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH2>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH2),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH2>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH2),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH2>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH2),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH2>())).Encoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH2),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH2>())).CodedCharSetId as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH2),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH2>())).Format as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH2),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH2>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH2),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRFH2>())).NameValueCCSID as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRFH2),
        "::",
        stringify!(NameValueCCSID)
        )
    );
}
pub type MQRMH = tagMQRMH;
pub type PMQRMH = *mut MQRMH;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQRMH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Flags: MQLONG,
    pub ObjectType: MQCHAR8,
    pub ObjectInstanceId: MQBYTE24,
    pub SrcEnvLength: MQLONG,
    pub SrcEnvOffset: MQLONG,
    pub SrcNameLength: MQLONG,
    pub SrcNameOffset: MQLONG,
    pub DestEnvLength: MQLONG,
    pub DestEnvOffset: MQLONG,
    pub DestNameLength: MQLONG,
    pub DestNameOffset: MQLONG,
    pub DataLogicalLength: MQLONG,
    pub DataLogicalOffset: MQLONG,
    pub DataLogicalOffset2: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQRMH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQRMH>(),
        108usize,
        concat!("Size of: ", stringify!(tagMQRMH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQRMH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQRMH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).Encoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).CodedCharSetId as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).Format as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).ObjectType as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(ObjectType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).ObjectInstanceId as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(ObjectInstanceId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).SrcEnvLength as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(SrcEnvLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).SrcEnvOffset as *const _ as usize },
        68usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(SrcEnvOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).SrcNameLength as *const _ as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(SrcNameLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).SrcNameOffset as *const _ as usize },
        76usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(SrcNameOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).DestEnvLength as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(DestEnvLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).DestEnvOffset as *const _ as usize },
        84usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(DestEnvOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).DestNameLength as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(DestNameLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).DestNameOffset as *const _ as usize },
        92usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(DestNameOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).DataLogicalLength as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(DataLogicalLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).DataLogicalOffset as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(DataLogicalOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRMH>())).DataLogicalOffset2 as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRMH),
        "::",
        stringify!(DataLogicalOffset2)
        )
    );
}
pub type MQRR = tagMQRR;
pub type PMQRR = *mut MQRR;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQRR {
    pub CompCode: MQLONG,
    pub Reason: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQRR() {
    assert_eq!(
        ::std::mem::size_of::<tagMQRR>(),
        8usize,
        concat!("Size of: ", stringify!(tagMQRR))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQRR>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQRR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRR>())).CompCode as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRR),
        "::",
        stringify!(CompCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQRR>())).Reason as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQRR),
        "::",
        stringify!(Reason)
        )
    );
}
pub type MQSD = tagMQSD;
pub type PMQSD = *mut MQSD;
pub type PPMQSD = *mut PMQSD;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQSD {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub ObjectName: MQCHAR48,
    pub AlternateUserId: MQCHAR12,
    pub AlternateSecurityId: MQBYTE40,
    pub SubExpiry: MQLONG,
    pub ObjectString: MQCHARV,
    pub SubName: MQCHARV,
    pub SubUserData: MQCHARV,
    pub SubCorrelId: MQBYTE24,
    pub PubPriority: MQLONG,
    pub PubAccountingToken: MQBYTE32,
    pub PubApplIdentityData: MQCHAR32,
    pub SelectionString: MQCHARV,
    pub SubLevel: MQLONG,
    pub ResObjectString: MQCHARV,
}
#[test]
fn bindgen_test_layout_tagMQSD() {
    assert_eq!(
        ::std::mem::size_of::<tagMQSD>(),
        344usize,
        concat!("Size of: ", stringify!(tagMQSD))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQSD>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQSD))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).ObjectName as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(ObjectName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).AlternateUserId as *const _ as usize },
        60usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(AlternateUserId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).AlternateSecurityId as *const _ as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(AlternateSecurityId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).SubExpiry as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(SubExpiry)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).ObjectString as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(ObjectString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).SubName as *const _ as usize },
        144usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(SubName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).SubUserData as *const _ as usize },
        168usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(SubUserData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).SubCorrelId as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(SubCorrelId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).PubPriority as *const _ as usize },
        216usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(PubPriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).PubAccountingToken as *const _ as usize },
        220usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(PubAccountingToken)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).PubApplIdentityData as *const _ as usize },
        252usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(PubApplIdentityData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).SelectionString as *const _ as usize },
        288usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(SelectionString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).SubLevel as *const _ as usize },
        312usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(SubLevel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSD>())).ResObjectString as *const _ as usize },
        320usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSD),
        "::",
        stringify!(ResObjectString)
        )
    );
}
pub type MQSMPO = tagMQSMPO;
pub type PMQSMPO = *mut MQSMPO;
pub type PPMQSMPO = *mut PMQSMPO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQSMPO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub ValueEncoding: MQLONG,
    pub ValueCCSID: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQSMPO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQSMPO>(),
        20usize,
        concat!("Size of: ", stringify!(tagMQSMPO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQSMPO>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQSMPO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSMPO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSMPO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSMPO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSMPO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSMPO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSMPO),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSMPO>())).ValueEncoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSMPO),
        "::",
        stringify!(ValueEncoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSMPO>())).ValueCCSID as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSMPO),
        "::",
        stringify!(ValueCCSID)
        )
    );
}
pub type MQSRO = tagMQSRO;
pub type PMQSRO = *mut MQSRO;
pub type PPMQSRO = *mut PMQSRO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQSRO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub NumPubs: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQSRO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQSRO>(),
        16usize,
        concat!("Size of: ", stringify!(tagMQSRO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQSRO>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQSRO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSRO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSRO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSRO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSRO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSRO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSRO),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSRO>())).NumPubs as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSRO),
        "::",
        stringify!(NumPubs)
        )
    );
}
pub type MQSTS = tagMQSTS;
pub type PMQSTS = *mut MQSTS;
pub type PPMQSTS = *mut PMQSTS;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQSTS {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub CompCode: MQLONG,
    pub Reason: MQLONG,
    pub PutSuccessCount: MQLONG,
    pub PutWarningCount: MQLONG,
    pub PutFailureCount: MQLONG,
    pub ObjectType: MQLONG,
    pub ObjectName: MQCHAR48,
    pub ObjectQMgrName: MQCHAR48,
    pub ResolvedObjectName: MQCHAR48,
    pub ResolvedQMgrName: MQCHAR48,
    pub ObjectString: MQCHARV,
    pub SubName: MQCHARV,
    pub OpenOptions: MQLONG,
    pub SubOptions: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQSTS() {
    assert_eq!(
        ::std::mem::size_of::<tagMQSTS>(),
        280usize,
        concat!("Size of: ", stringify!(tagMQSTS))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQSTS>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQSTS))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).CompCode as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(CompCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).Reason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(Reason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).PutSuccessCount as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(PutSuccessCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).PutWarningCount as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(PutWarningCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).PutFailureCount as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(PutFailureCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).ObjectType as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(ObjectType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).ObjectName as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(ObjectName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).ObjectQMgrName as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(ObjectQMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).ResolvedObjectName as *const _ as usize },
        128usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(ResolvedObjectName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).ResolvedQMgrName as *const _ as usize },
        176usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(ResolvedQMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).ObjectString as *const _ as usize },
        224usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(ObjectString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).SubName as *const _ as usize },
        248usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(SubName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).OpenOptions as *const _ as usize },
        272usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(OpenOptions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSTS>())).SubOptions as *const _ as usize },
        276usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSTS),
        "::",
        stringify!(SubOptions)
        )
    );
}
pub type MQTM = tagMQTM;
pub type PMQTM = *mut MQTM;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQTM {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub QName: MQCHAR48,
    pub ProcessName: MQCHAR48,
    pub TriggerData: MQCHAR64,
    pub ApplType: MQLONG,
    pub ApplId: MQCHAR256,
    pub EnvData: MQCHAR128,
    pub UserData: MQCHAR128,
}
#[test]
fn bindgen_test_layout_tagMQTM() {
    assert_eq!(
        ::std::mem::size_of::<tagMQTM>(),
        684usize,
        concat!("Size of: ", stringify!(tagMQTM))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQTM>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQTM))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).QName as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).ProcessName as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(ProcessName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).TriggerData as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(TriggerData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).ApplType as *const _ as usize },
        168usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(ApplType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).ApplId as *const _ as usize },
        172usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(ApplId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).EnvData as *const _ as usize },
        428usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(EnvData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTM>())).UserData as *const _ as usize },
        556usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTM),
        "::",
        stringify!(UserData)
        )
    );
}
pub type MQTMC2 = tagMQTMC2;
pub type PMQTMC2 = *mut MQTMC2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQTMC2 {
    pub StrucId: MQCHAR4,
    pub Version: MQCHAR4,
    pub QName: MQCHAR48,
    pub ProcessName: MQCHAR48,
    pub TriggerData: MQCHAR64,
    pub ApplType: MQCHAR4,
    pub ApplId: MQCHAR256,
    pub EnvData: MQCHAR128,
    pub UserData: MQCHAR128,
    pub QMgrName: MQCHAR48,
}
#[test]
fn bindgen_test_layout_tagMQTMC2() {
    assert_eq!(
        ::std::mem::size_of::<tagMQTMC2>(),
        732usize,
        concat!("Size of: ", stringify!(tagMQTMC2))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQTMC2>(),
        1usize,
        concat!("Alignment of ", stringify!(tagMQTMC2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).QName as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).ProcessName as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(ProcessName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).TriggerData as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(TriggerData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).ApplType as *const _ as usize },
        168usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(ApplType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).ApplId as *const _ as usize },
        172usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(ApplId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).EnvData as *const _ as usize },
        428usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(EnvData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).UserData as *const _ as usize },
        556usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(UserData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQTMC2>())).QMgrName as *const _ as usize },
        684usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQTMC2),
        "::",
        stringify!(QMgrName)
        )
    );
}
pub type MQWIH = tagMQWIH;
pub type PMQWIH = *mut MQWIH;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWIH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub Format: MQCHAR8,
    pub Flags: MQLONG,
    pub ServiceName: MQCHAR32,
    pub ServiceStep: MQCHAR8,
    pub MsgToken: MQBYTE16,
    pub Reserved: MQCHAR32,
}
#[test]
fn bindgen_test_layout_tagMQWIH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWIH>(),
        120usize,
        concat!("Size of: ", stringify!(tagMQWIH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWIH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQWIH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).Encoding as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).CodedCharSetId as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).Format as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(Format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).ServiceName as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(ServiceName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).ServiceStep as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(ServiceStep)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).MsgToken as *const _ as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(MsgToken)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWIH>())).Reserved as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWIH),
        "::",
        stringify!(Reserved)
        )
    );
}
pub type MQXQH = tagMQXQH;
pub type PMQXQH = *mut MQXQH;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQXQH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub RemoteQName: MQCHAR48,
    pub RemoteQMgrName: MQCHAR48,
    pub MsgDesc: MQMD1,
}
#[test]
fn bindgen_test_layout_tagMQXQH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQXQH>(),
        428usize,
        concat!("Size of: ", stringify!(tagMQXQH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQXQH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQXQH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXQH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXQH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXQH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXQH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXQH>())).RemoteQName as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXQH),
        "::",
        stringify!(RemoteQName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXQH>())).RemoteQMgrName as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXQH),
        "::",
        stringify!(RemoteQMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXQH>())).MsgDesc as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXQH),
        "::",
        stringify!(MsgDesc)
        )
    );
}
extern "C" {
    pub fn MQBACK(Hconn: MQHCONN, pCompCode: PMQLONG, pReason: PMQLONG);
}
pub type MQ_BACK_CALL = ::std::option::Option<
    unsafe extern "C" fn(Hconn: MQHCONN, pCompCode: PMQLONG, pReason: PMQLONG),
>;
pub type PMQ_BACK_CALL = MQ_BACK_CALL;
extern "C" {
    pub fn MQBEGIN(Hconn: MQHCONN, pBeginOptions: PMQVOID, pCompCode: PMQLONG, pReason: PMQLONG);
}
pub type MQ_BEGIN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pBeginOptions: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_BEGIN_CALL = MQ_BEGIN_CALL;
extern "C" {
    pub fn MQBUFMH(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pBufMsgHOpts: PMQVOID,
        pMsgDesc: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_BUFMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pBufMsgHOpts: PMQVOID,
        pMsgDesc: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_BUFMH_CALL = MQ_BUFMH_CALL;
extern "C" {
    pub fn MQCB(
        Hconn: MQHCONN,
        Operation: MQLONG,
        pCallbackDesc: PMQVOID,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_CB_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Operation: MQLONG,
        pCallbackDesc: PMQVOID,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CB_CALL = MQ_CB_CALL;
extern "C" {
    pub fn MQCLOSE(
        Hconn: MQHCONN,
        pHobj: PMQHOBJ,
        Options: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_CLOSE_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pHobj: PMQHOBJ,
        Options: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CLOSE_CALL = MQ_CLOSE_CALL;
extern "C" {
    pub fn MQCMIT(Hconn: MQHCONN, pCompCode: PMQLONG, pReason: PMQLONG);
}
pub type MQ_CMIT_CALL = ::std::option::Option<
    unsafe extern "C" fn(Hconn: MQHCONN, pCompCode: PMQLONG, pReason: PMQLONG),
>;
pub type PMQ_CMIT_CALL = MQ_CMIT_CALL;
extern "C" {
    pub fn MQCONN(pQMgrName: PMQCHAR, pHconn: PMQHCONN, pCompCode: PMQLONG, pReason: PMQLONG);
}
pub type MQ_CONN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CONN_CALL = MQ_CONN_CALL;
extern "C" {
    pub fn MQCONNX(
        pQMgrName: PMQCHAR,
        pConnectOpts: PMQCNO,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_CONNX_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pConnectOpts: PMQCNO,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CONNX_CALL = MQ_CONNX_CALL;
extern "C" {
    pub fn MQCRTMH(
        Hconn: MQHCONN,
        pCrtMsgHOpts: PMQVOID,
        pHmsg: PMQHMSG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_CRTMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pCrtMsgHOpts: PMQVOID,
        pHmsg: PMQHMSG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CRTMH_CALL = MQ_CRTMH_CALL;
extern "C" {
    pub fn MQCTL(
        Hconn: MQHCONN,
        Operation: MQLONG,
        pControlOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_CTL_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Operation: MQLONG,
        pControlOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CTL_CALL = MQ_CTL_CALL;
extern "C" {
    pub fn MQDISC(pHconn: PMQHCONN, pCompCode: PMQLONG, pReason: PMQLONG);
}
pub type MQ_DISC_CALL = ::std::option::Option<
    unsafe extern "C" fn(pHconn: PMQHCONN, pCompCode: PMQLONG, pReason: PMQLONG),
>;
pub type PMQ_DISC_CALL = MQ_DISC_CALL;
extern "C" {
    pub fn MQDLTMH(
        Hconn: MQHCONN,
        pHmsg: PMQHMSG,
        pDltMsgHOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_DLTMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pHmsg: PMQHMSG,
        pDltMsgHOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_DLTMH_CALL = MQ_DLTMH_CALL;
extern "C" {
    pub fn MQDLTMP(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pDltPropOpts: PMQVOID,
        pName: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_DLTMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pDltPropOpts: PMQVOID,
        pName: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_DLTMP_CALL = MQ_DLTMP_CALL;
extern "C" {
    pub fn MQGET(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_GET_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_GET_CALL = MQ_GET_CALL;
extern "C" {
    pub fn MQINQ(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_INQ_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_INQ_CALL = MQ_INQ_CALL;
extern "C" {
    pub fn MQINQMP(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pInqPropOpts: PMQVOID,
        pName: PMQVOID,
        pPropDesc: PMQVOID,
        pType: PMQLONG,
        ValueLength: MQLONG,
        pValue: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_INQMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pInqPropOpts: PMQVOID,
        pName: PMQVOID,
        pPropDesc: PMQVOID,
        pType: PMQLONG,
        ValueLength: MQLONG,
        pValue: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_INQMP_CALL = MQ_INQMP_CALL;
extern "C" {
    pub fn MQMHBUF(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pMsgHBufOpts: PMQVOID,
        pName: PMQVOID,
        pMsgDesc: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_MHBUF_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pMsgHBufOpts: PMQVOID,
        pName: PMQVOID,
        pMsgDesc: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_MHBUF_CALL = MQ_MHBUF_CALL;
extern "C" {
    pub fn MQOPEN(
        Hconn: MQHCONN,
        pObjDesc: PMQVOID,
        Options: MQLONG,
        pHobj: PMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_OPEN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pObjDesc: PMQVOID,
        Options: MQLONG,
        pHobj: PMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_OPEN_CALL = MQ_OPEN_CALL;
extern "C" {
    pub fn MQPUT(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_PUT_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PUT_CALL = MQ_PUT_CALL;
extern "C" {
    pub fn MQPUT1(
        Hconn: MQHCONN,
        pObjDesc: PMQVOID,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_PUT1_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pObjDesc: PMQVOID,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PUT1_CALL = MQ_PUT1_CALL;
extern "C" {
    pub fn MQSET(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_SET_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SET_CALL = MQ_SET_CALL;
extern "C" {
    pub fn MQSETMP(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pSetPropOpts: PMQVOID,
        pName: PMQVOID,
        pPropDesc: PMQVOID,
        Type: MQLONG,
        ValueLength: MQLONG,
        pValue: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_SETMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pSetPropOpts: PMQVOID,
        pName: PMQVOID,
        pPropDesc: PMQVOID,
        Type: MQLONG,
        ValueLength: MQLONG,
        pValue: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SETMP_CALL = MQ_SETMP_CALL;
extern "C" {
    pub fn MQSTAT(
        Hconn: MQHCONN,
        Type: MQLONG,
        pStatus: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_STAT_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Type: MQLONG,
        pStatus: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_STAT_CALL = MQ_STAT_CALL;
extern "C" {
    pub fn MQSUB(
        Hconn: MQHCONN,
        pSubDesc: PMQVOID,
        pHobj: PMQHOBJ,
        pHsub: PMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_SUB_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pSubDesc: PMQVOID,
        pHobj: PMQHOBJ,
        pHsub: PMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SUB_CALL = MQ_SUB_CALL;
extern "C" {
    pub fn MQSUBRQ(
        Hconn: MQHCONN,
        Hsub: MQHOBJ,
        Action: MQLONG,
        pSubRqOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_SUBRQ_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hsub: MQHOBJ,
        Action: MQLONG,
        pSubRqOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SUBRQ_CALL = MQ_SUBRQ_CALL;
pub type MQCB_FUNCTION = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        pBuffer: PMQVOID,
        pContext: PMQCBC,
    ),
>;
pub type PMQCB_FUNCTION = MQCB_FUNCTION;
pub type MQCD = tagMQCD;
pub type PMQCD = *mut MQCD;
pub type PPMQCD = *mut PMQCD;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQCD {
    pub ChannelName: [MQCHAR; 20usize],
    pub Version: MQLONG,
    pub ChannelType: MQLONG,
    pub TransportType: MQLONG,
    pub Desc: [MQCHAR; 64usize],
    pub QMgrName: [MQCHAR; 48usize],
    pub XmitQName: [MQCHAR; 48usize],
    pub ShortConnectionName: [MQCHAR; 20usize],
    pub MCAName: [MQCHAR; 20usize],
    pub ModeName: [MQCHAR; 8usize],
    pub TpName: [MQCHAR; 64usize],
    pub BatchSize: MQLONG,
    pub DiscInterval: MQLONG,
    pub ShortRetryCount: MQLONG,
    pub ShortRetryInterval: MQLONG,
    pub LongRetryCount: MQLONG,
    pub LongRetryInterval: MQLONG,
    pub SecurityExit: [MQCHAR; 128usize],
    pub MsgExit: [MQCHAR; 128usize],
    pub SendExit: [MQCHAR; 128usize],
    pub ReceiveExit: [MQCHAR; 128usize],
    pub SeqNumberWrap: MQLONG,
    pub MaxMsgLength: MQLONG,
    pub PutAuthority: MQLONG,
    pub DataConversion: MQLONG,
    pub SecurityUserData: [MQCHAR; 32usize],
    pub MsgUserData: [MQCHAR; 32usize],
    pub SendUserData: [MQCHAR; 32usize],
    pub ReceiveUserData: [MQCHAR; 32usize],
    pub UserIdentifier: [MQCHAR; 12usize],
    pub Password: [MQCHAR; 12usize],
    pub MCAUserIdentifier: [MQCHAR; 12usize],
    pub MCAType: MQLONG,
    pub ConnectionName: [MQCHAR; 264usize],
    pub RemoteUserIdentifier: [MQCHAR; 12usize],
    pub RemotePassword: [MQCHAR; 12usize],
    pub MsgRetryExit: [MQCHAR; 128usize],
    pub MsgRetryUserData: [MQCHAR; 32usize],
    pub MsgRetryCount: MQLONG,
    pub MsgRetryInterval: MQLONG,
    pub HeartbeatInterval: MQLONG,
    pub BatchInterval: MQLONG,
    pub NonPersistentMsgSpeed: MQLONG,
    pub StrucLength: MQLONG,
    pub ExitNameLength: MQLONG,
    pub ExitDataLength: MQLONG,
    pub MsgExitsDefined: MQLONG,
    pub SendExitsDefined: MQLONG,
    pub ReceiveExitsDefined: MQLONG,
    pub MsgExitPtr: MQPTR,
    pub MsgUserDataPtr: MQPTR,
    pub SendExitPtr: MQPTR,
    pub SendUserDataPtr: MQPTR,
    pub ReceiveExitPtr: MQPTR,
    pub ReceiveUserDataPtr: MQPTR,
    pub ClusterPtr: MQPTR,
    pub ClustersDefined: MQLONG,
    pub NetworkPriority: MQLONG,
    pub LongMCAUserIdLength: MQLONG,
    pub LongRemoteUserIdLength: MQLONG,
    pub LongMCAUserIdPtr: MQPTR,
    pub LongRemoteUserIdPtr: MQPTR,
    pub MCASecurityId: MQBYTE40,
    pub RemoteSecurityId: MQBYTE40,
    pub SSLCipherSpec: [MQCHAR; 32usize],
    pub SSLPeerNamePtr: MQPTR,
    pub SSLPeerNameLength: MQLONG,
    pub SSLClientAuth: MQLONG,
    pub KeepAliveInterval: MQLONG,
    pub LocalAddress: [MQCHAR; 48usize],
    pub BatchHeartbeat: MQLONG,
    pub HdrCompList: [MQLONG; 2usize],
    pub MsgCompList: [MQLONG; 16usize],
    pub CLWLChannelRank: MQLONG,
    pub CLWLChannelPriority: MQLONG,
    pub CLWLChannelWeight: MQLONG,
    pub ChannelMonitoring: MQLONG,
    pub ChannelStatistics: MQLONG,
    pub SharingConversations: MQLONG,
    pub PropertyControl: MQLONG,
    pub MaxInstances: MQLONG,
    pub MaxInstancesPerClient: MQLONG,
    pub ClientChannelWeight: MQLONG,
    pub ConnectionAffinity: MQLONG,
    pub BatchDataLimit: MQLONG,
    pub UseDLQ: MQLONG,
    pub DefReconnect: MQLONG,
    pub CertificateLabel: [MQCHAR; 64usize],
    pub SPLProtection: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQCD() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCD>(),
        1992usize,
        concat!("Size of: ", stringify!(tagMQCD))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCD>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQCD))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ChannelName as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ChannelName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).Version as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ChannelType as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ChannelType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).TransportType as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(TransportType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).Desc as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(Desc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).QMgrName as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).XmitQName as *const _ as usize },
        144usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(XmitQName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ShortConnectionName as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ShortConnectionName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MCAName as *const _ as usize },
        212usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MCAName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ModeName as *const _ as usize },
        232usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ModeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).TpName as *const _ as usize },
        240usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(TpName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).BatchSize as *const _ as usize },
        304usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(BatchSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).DiscInterval as *const _ as usize },
        308usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(DiscInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ShortRetryCount as *const _ as usize },
        312usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ShortRetryCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ShortRetryInterval as *const _ as usize },
        316usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ShortRetryInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).LongRetryCount as *const _ as usize },
        320usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(LongRetryCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).LongRetryInterval as *const _ as usize },
        324usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(LongRetryInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SecurityExit as *const _ as usize },
        328usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SecurityExit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgExit as *const _ as usize },
        456usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgExit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SendExit as *const _ as usize },
        584usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SendExit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ReceiveExit as *const _ as usize },
        712usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ReceiveExit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SeqNumberWrap as *const _ as usize },
        840usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SeqNumberWrap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MaxMsgLength as *const _ as usize },
        844usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MaxMsgLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).PutAuthority as *const _ as usize },
        848usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(PutAuthority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).DataConversion as *const _ as usize },
        852usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(DataConversion)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SecurityUserData as *const _ as usize },
        856usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SecurityUserData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgUserData as *const _ as usize },
        888usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgUserData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SendUserData as *const _ as usize },
        920usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SendUserData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ReceiveUserData as *const _ as usize },
        952usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ReceiveUserData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).UserIdentifier as *const _ as usize },
        984usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(UserIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).Password as *const _ as usize },
        996usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(Password)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MCAUserIdentifier as *const _ as usize },
        1008usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MCAUserIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MCAType as *const _ as usize },
        1020usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MCAType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ConnectionName as *const _ as usize },
        1024usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ConnectionName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).RemoteUserIdentifier as *const _ as usize },
        1288usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(RemoteUserIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).RemotePassword as *const _ as usize },
        1300usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(RemotePassword)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgRetryExit as *const _ as usize },
        1312usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgRetryExit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgRetryUserData as *const _ as usize },
        1440usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgRetryUserData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgRetryCount as *const _ as usize },
        1472usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgRetryCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgRetryInterval as *const _ as usize },
        1476usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgRetryInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).HeartbeatInterval as *const _ as usize },
        1480usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(HeartbeatInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).BatchInterval as *const _ as usize },
        1484usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(BatchInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).NonPersistentMsgSpeed as *const _ as usize },
        1488usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(NonPersistentMsgSpeed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).StrucLength as *const _ as usize },
        1492usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ExitNameLength as *const _ as usize },
        1496usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ExitNameLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ExitDataLength as *const _ as usize },
        1500usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ExitDataLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgExitsDefined as *const _ as usize },
        1504usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgExitsDefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SendExitsDefined as *const _ as usize },
        1508usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SendExitsDefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ReceiveExitsDefined as *const _ as usize },
        1512usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ReceiveExitsDefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgExitPtr as *const _ as usize },
        1520usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgExitPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgUserDataPtr as *const _ as usize },
        1528usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgUserDataPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SendExitPtr as *const _ as usize },
        1536usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SendExitPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SendUserDataPtr as *const _ as usize },
        1544usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SendUserDataPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ReceiveExitPtr as *const _ as usize },
        1552usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ReceiveExitPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ReceiveUserDataPtr as *const _ as usize },
        1560usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ReceiveUserDataPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ClusterPtr as *const _ as usize },
        1568usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ClusterPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ClustersDefined as *const _ as usize },
        1576usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ClustersDefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).NetworkPriority as *const _ as usize },
        1580usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(NetworkPriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).LongMCAUserIdLength as *const _ as usize },
        1584usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(LongMCAUserIdLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).LongRemoteUserIdLength as *const _ as usize },
        1588usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(LongRemoteUserIdLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).LongMCAUserIdPtr as *const _ as usize },
        1592usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(LongMCAUserIdPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).LongRemoteUserIdPtr as *const _ as usize },
        1600usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(LongRemoteUserIdPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MCASecurityId as *const _ as usize },
        1608usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MCASecurityId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).RemoteSecurityId as *const _ as usize },
        1648usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(RemoteSecurityId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SSLCipherSpec as *const _ as usize },
        1688usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SSLCipherSpec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SSLPeerNamePtr as *const _ as usize },
        1720usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SSLPeerNamePtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SSLPeerNameLength as *const _ as usize },
        1728usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SSLPeerNameLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SSLClientAuth as *const _ as usize },
        1732usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SSLClientAuth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).KeepAliveInterval as *const _ as usize },
        1736usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(KeepAliveInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).LocalAddress as *const _ as usize },
        1740usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(LocalAddress)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).BatchHeartbeat as *const _ as usize },
        1788usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(BatchHeartbeat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).HdrCompList as *const _ as usize },
        1792usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(HdrCompList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MsgCompList as *const _ as usize },
        1800usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MsgCompList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).CLWLChannelRank as *const _ as usize },
        1864usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(CLWLChannelRank)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).CLWLChannelPriority as *const _ as usize },
        1868usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(CLWLChannelPriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).CLWLChannelWeight as *const _ as usize },
        1872usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(CLWLChannelWeight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ChannelMonitoring as *const _ as usize },
        1876usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ChannelMonitoring)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ChannelStatistics as *const _ as usize },
        1880usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ChannelStatistics)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SharingConversations as *const _ as usize },
        1884usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SharingConversations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).PropertyControl as *const _ as usize },
        1888usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(PropertyControl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MaxInstances as *const _ as usize },
        1892usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MaxInstances)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).MaxInstancesPerClient as *const _ as usize },
        1896usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(MaxInstancesPerClient)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ClientChannelWeight as *const _ as usize },
        1900usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ClientChannelWeight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).ConnectionAffinity as *const _ as usize },
        1904usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(ConnectionAffinity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).BatchDataLimit as *const _ as usize },
        1908usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(BatchDataLimit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).UseDLQ as *const _ as usize },
        1912usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(UseDLQ)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).DefReconnect as *const _ as usize },
        1916usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(DefReconnect)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).CertificateLabel as *const _ as usize },
        1920usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(CertificateLabel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCD>())).SPLProtection as *const _ as usize },
        1984usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCD),
        "::",
        stringify!(SPLProtection)
        )
    );
}
pub type MQACH = tagMQACH;
pub type PMQACH = *mut MQACH;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQACH {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub ChainAreaLength: MQLONG,
    pub ExitInfoName: MQCHAR48,
    pub NextChainAreaPtr: PMQACH,
}
#[test]
fn bindgen_test_layout_tagMQACH() {
    assert_eq!(
        ::std::mem::size_of::<tagMQACH>(),
        72usize,
        concat!("Size of: ", stringify!(tagMQACH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQACH>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQACH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQACH>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQACH),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQACH>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQACH),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQACH>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQACH),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQACH>())).ChainAreaLength as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQACH),
        "::",
        stringify!(ChainAreaLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQACH>())).ExitInfoName as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQACH),
        "::",
        stringify!(ExitInfoName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQACH>())).NextChainAreaPtr as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQACH),
        "::",
        stringify!(NextChainAreaPtr)
        )
    );
}
pub type MQAXC = tagMQAXC;
pub type PMQAXC = *mut MQAXC;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQAXC {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Environment: MQLONG,
    pub UserId: MQCHAR12,
    pub SecurityId: MQBYTE40,
    pub ConnectionName: [MQCHAR; 264usize],
    pub LongMCAUserIdLength: MQLONG,
    pub LongRemoteUserIdLength: MQLONG,
    pub LongMCAUserIdPtr: MQPTR,
    pub LongRemoteUserIdPtr: MQPTR,
    pub ApplName: MQCHAR28,
    pub ApplType: MQLONG,
    pub ProcessId: MQPID,
    pub ThreadId: MQTID,
    pub ChannelName: [MQCHAR; 20usize],
    pub Reserved1: MQBYTE4,
    pub pChannelDefinition: PMQCD,
}
#[test]
fn bindgen_test_layout_tagMQAXC() {
    assert_eq!(
        ::std::mem::size_of::<tagMQAXC>(),
        424usize,
        concat!("Size of: ", stringify!(tagMQAXC))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQAXC>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQAXC))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).Environment as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(Environment)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).UserId as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(UserId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).SecurityId as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(SecurityId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).ConnectionName as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(ConnectionName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).LongMCAUserIdLength as *const _ as usize },
        328usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(LongMCAUserIdLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).LongRemoteUserIdLength as *const _ as usize },
        332usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(LongRemoteUserIdLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).LongMCAUserIdPtr as *const _ as usize },
        336usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(LongMCAUserIdPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).LongRemoteUserIdPtr as *const _ as usize },
        344usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(LongRemoteUserIdPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).ApplName as *const _ as usize },
        352usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(ApplName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).ApplType as *const _ as usize },
        380usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(ApplType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).ProcessId as *const _ as usize },
        384usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(ProcessId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).ThreadId as *const _ as usize },
        388usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(ThreadId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).ChannelName as *const _ as usize },
        392usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(ChannelName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).Reserved1 as *const _ as usize },
        412usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(Reserved1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXC>())).pChannelDefinition as *const _ as usize },
        416usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXC),
        "::",
        stringify!(pChannelDefinition)
        )
    );
}
pub type MQAXP = tagMQAXP;
pub type PMQAXP = *mut MQAXP;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQAXP {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub APICallerType: MQLONG,
    pub ExitUserArea: MQBYTE16,
    pub ExitData: MQCHAR32,
    pub ExitInfoName: MQCHAR48,
    pub ExitPDArea: MQBYTE48,
    pub QMgrName: MQCHAR48,
    pub ExitChainAreaPtr: PMQACH,
    pub Hconfig: MQHCONFIG,
    pub Function: MQLONG,
    pub ExitMsgHandle: MQHMSG,
}
#[test]
fn bindgen_test_layout_tagMQAXP() {
    assert_eq!(
        ::std::mem::size_of::<tagMQAXP>(),
        256usize,
        concat!("Size of: ", stringify!(tagMQAXP))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQAXP>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQAXP))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).APICallerType as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(APICallerType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitUserArea as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitUserArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitData as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitInfoName as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitInfoName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitPDArea as *const _ as usize },
        128usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitPDArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).QMgrName as *const _ as usize },
        176usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitChainAreaPtr as *const _ as usize },
        224usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitChainAreaPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).Hconfig as *const _ as usize },
        232usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(Hconfig)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).Function as *const _ as usize },
        240usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(Function)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQAXP>())).ExitMsgHandle as *const _ as usize },
        248usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQAXP),
        "::",
        stringify!(ExitMsgHandle)
        )
    );
}
pub type MQCXP = tagMQCXP;
pub type PMQCXP = *mut MQCXP;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQCXP {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub MaxSegmentLength: MQLONG,
    pub ExitUserArea: MQBYTE16,
    pub ExitData: MQCHAR32,
    pub MsgRetryCount: MQLONG,
    pub MsgRetryInterval: MQLONG,
    pub MsgRetryReason: MQLONG,
    pub HeaderLength: MQLONG,
    pub PartnerName: MQCHAR48,
    pub FAPLevel: MQLONG,
    pub CapabilityFlags: MQLONG,
    pub ExitNumber: MQLONG,
    pub ExitSpace: MQLONG,
    pub SSLCertUserid: MQCHAR12,
    pub SSLRemCertIssNameLength: MQLONG,
    pub SSLRemCertIssNamePtr: MQPTR,
    pub SecurityParms: PMQCSP,
    pub CurHdrCompression: MQLONG,
    pub CurMsgCompression: MQLONG,
    pub Hconn: MQHCONN,
    pub SharingConversations: MQBOOL,
    pub MCAUserSource: MQLONG,
    pub pEntryPoints: PMQIEP,
    pub RemoteProduct: MQCHAR4,
    pub RemoteVersion: MQCHAR8,
}
#[test]
fn bindgen_test_layout_tagMQCXP() {
    assert_eq!(
        ::std::mem::size_of::<tagMQCXP>(),
        240usize,
        concat!("Size of: ", stringify!(tagMQCXP))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQCXP>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQCXP))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).MaxSegmentLength as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(MaxSegmentLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).ExitUserArea as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(ExitUserArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).ExitData as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(ExitData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).MsgRetryCount as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(MsgRetryCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).MsgRetryInterval as *const _ as usize },
        84usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(MsgRetryInterval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).MsgRetryReason as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(MsgRetryReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).HeaderLength as *const _ as usize },
        92usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(HeaderLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).PartnerName as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(PartnerName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).FAPLevel as *const _ as usize },
        144usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(FAPLevel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).CapabilityFlags as *const _ as usize },
        148usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(CapabilityFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).ExitNumber as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(ExitNumber)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).ExitSpace as *const _ as usize },
        156usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(ExitSpace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).SSLCertUserid as *const _ as usize },
        160usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(SSLCertUserid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagMQCXP>())).SSLRemCertIssNameLength as *const _ as usize
        },
        172usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(SSLRemCertIssNameLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).SSLRemCertIssNamePtr as *const _ as usize },
        176usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(SSLRemCertIssNamePtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).SecurityParms as *const _ as usize },
        184usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(SecurityParms)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).CurHdrCompression as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(CurHdrCompression)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).CurMsgCompression as *const _ as usize },
        196usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(CurMsgCompression)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).Hconn as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(Hconn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).SharingConversations as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(SharingConversations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).MCAUserSource as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(MCAUserSource)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).pEntryPoints as *const _ as usize },
        216usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(pEntryPoints)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).RemoteProduct as *const _ as usize },
        224usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(RemoteProduct)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQCXP>())).RemoteVersion as *const _ as usize },
        228usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQCXP),
        "::",
        stringify!(RemoteVersion)
        )
    );
}
pub type MQDXP = tagMQDXP;
pub type PMQDXP = *mut MQDXP;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDXP {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitOptions: MQLONG,
    pub AppOptions: MQLONG,
    pub Encoding: MQLONG,
    pub CodedCharSetId: MQLONG,
    pub DataLength: MQLONG,
    pub CompCode: MQLONG,
    pub Reason: MQLONG,
    pub ExitResponse: MQLONG,
    pub Hconn: MQHCONN,
    pub pEntryPoints: PMQIEP,
}
#[test]
fn bindgen_test_layout_tagMQDXP() {
    assert_eq!(
        ::std::mem::size_of::<tagMQDXP>(),
        56usize,
        concat!("Size of: ", stringify!(tagMQDXP))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQDXP>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQDXP))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).ExitOptions as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(ExitOptions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).AppOptions as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(AppOptions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).Encoding as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(Encoding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).CodedCharSetId as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(CodedCharSetId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).DataLength as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(DataLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).CompCode as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(CompCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).Reason as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(Reason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).ExitResponse as *const _ as usize },
        36usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).Hconn as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(Hconn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQDXP>())).pEntryPoints as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQDXP),
        "::",
        stringify!(pEntryPoints)
        )
    );
}
pub type MQNXP = tagMQNXP;
pub type PMQNXP = *mut MQNXP;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQNXP {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub ExitDataLength: MQLONG,
    pub pExitDataPtr: PMQCHAR,
    pub pExitUserAreaPtr: MQPTR,
    pub ppMQCDArrayPtr: PPMQCD,
    pub MQCDArrayCount: MQLONG,
    pub MaxMQCDVersion: MQLONG,
    pub pEntryPoints: PMQIEP,
}
#[test]
fn bindgen_test_layout_tagMQNXP() {
    assert_eq!(
        ::std::mem::size_of::<tagMQNXP>(),
        72usize,
        concat!("Size of: ", stringify!(tagMQNXP))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQNXP>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQNXP))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).ExitDataLength as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(ExitDataLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).pExitDataPtr as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(pExitDataPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).pExitUserAreaPtr as *const _ as usize },
        40usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(pExitUserAreaPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).ppMQCDArrayPtr as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(ppMQCDArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).MQCDArrayCount as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(MQCDArrayCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).MaxMQCDVersion as *const _ as usize },
        60usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(MaxMQCDVersion)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQNXP>())).pEntryPoints as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQNXP),
        "::",
        stringify!(pEntryPoints)
        )
    );
}
pub type MQPBC = tagMQPBC;
pub type PMQPBC = *mut MQPBC;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQPBC {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub PubTopicString: MQCHARV,
    pub MsgDescPtr: PMQMD,
}
#[test]
fn bindgen_test_layout_tagMQPBC() {
    assert_eq!(
        ::std::mem::size_of::<tagMQPBC>(),
        40usize,
        concat!("Size of: ", stringify!(tagMQPBC))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQPBC>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQPBC))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPBC>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPBC),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPBC>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPBC),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPBC>())).PubTopicString as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPBC),
        "::",
        stringify!(PubTopicString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPBC>())).MsgDescPtr as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPBC),
        "::",
        stringify!(MsgDescPtr)
        )
    );
}
pub type MQPSXP = tagMQPSXP;
pub type PMQPSXP = *mut MQPSXP;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQPSXP {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub Hconn: MQHCONN,
    pub ExitUserArea: MQBYTE16,
    pub ExitData: MQCHAR32,
    pub QMgrName: MQCHAR48,
    pub MsgHandle: MQHMSG,
    pub MsgDescPtr: PMQMD,
    pub MsgInPtr: PMQVOID,
    pub MsgInLength: MQLONG,
    pub MsgOutPtr: PMQVOID,
    pub MsgOutLength: MQLONG,
    pub pEntryPoints: PMQIEP,
}
#[test]
fn bindgen_test_layout_tagMQPSXP() {
    assert_eq!(
        ::std::mem::size_of::<tagMQPSXP>(),
        184usize,
        concat!("Size of: ", stringify!(tagMQPSXP))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQPSXP>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQPSXP))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).Hconn as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(Hconn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).ExitUserArea as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(ExitUserArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).ExitData as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(ExitData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).QMgrName as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).MsgHandle as *const _ as usize },
        128usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(MsgHandle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).MsgDescPtr as *const _ as usize },
        136usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(MsgDescPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).MsgInPtr as *const _ as usize },
        144usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(MsgInPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).MsgInLength as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(MsgInLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).MsgOutPtr as *const _ as usize },
        160usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(MsgOutPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).MsgOutLength as *const _ as usize },
        168usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(MsgOutLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQPSXP>())).pEntryPoints as *const _ as usize },
        176usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQPSXP),
        "::",
        stringify!(pEntryPoints)
        )
    );
}
pub type MQSBC = tagMQSBC;
pub type PMQSBC = *mut MQSBC;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQSBC {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub DestinationQMgrName: MQCHAR48,
    pub DestinationQName: MQCHAR48,
    pub SubType: MQLONG,
    pub SubOptions: MQLONG,
    pub ObjectName: MQCHAR48,
    pub ObjectString: MQCHARV,
    pub SubTopicString: MQCHARV,
    pub SubName: MQCHARV,
    pub SubId: MQBYTE24,
    pub SelectionString: MQCHARV,
    pub SubLevel: MQLONG,
    pub PSProperties: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQSBC() {
    assert_eq!(
        ::std::mem::size_of::<tagMQSBC>(),
        288usize,
        concat!("Size of: ", stringify!(tagMQSBC))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQSBC>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQSBC))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).DestinationQMgrName as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(DestinationQMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).DestinationQName as *const _ as usize },
        56usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(DestinationQName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).SubType as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(SubType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).SubOptions as *const _ as usize },
        108usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(SubOptions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).ObjectName as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(ObjectName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).ObjectString as *const _ as usize },
        160usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(ObjectString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).SubTopicString as *const _ as usize },
        184usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(SubTopicString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).SubName as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(SubName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).SubId as *const _ as usize },
        232usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(SubId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).SelectionString as *const _ as usize },
        256usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(SelectionString)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).SubLevel as *const _ as usize },
        280usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(SubLevel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQSBC>())).PSProperties as *const _ as usize },
        284usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQSBC),
        "::",
        stringify!(PSProperties)
        )
    );
}
pub type MQWCR = tagMQWCR;
pub type PMQWCR = *mut MQWCR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWCR {
    pub ClusterName: MQCHAR48,
    pub ClusterRecOffset: MQLONG,
    pub ClusterFlags: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQWCR() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWCR>(),
        56usize,
        concat!("Size of: ", stringify!(tagMQWCR))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWCR>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQWCR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWCR>())).ClusterName as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWCR),
        "::",
        stringify!(ClusterName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWCR>())).ClusterRecOffset as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWCR),
        "::",
        stringify!(ClusterRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWCR>())).ClusterFlags as *const _ as usize },
        52usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWCR),
        "::",
        stringify!(ClusterFlags)
        )
    );
}
pub type MQWDR = tagMQWDR;
pub type PMQWDR = *mut MQWDR;
pub type PPMQWDR = *mut PMQWDR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWDR {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub QMgrFlags: MQLONG,
    pub QMgrIdentifier: MQCHAR48,
    pub QMgrName: MQCHAR48,
    pub ClusterRecOffset: MQLONG,
    pub ChannelState: MQLONG,
    pub ChannelDefOffset: MQLONG,
    pub DestSeqNumber: MQLONG,
    pub DestSeqFactor: MQINT64,
}
#[test]
fn bindgen_test_layout_tagMQWDR() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWDR>(),
        136usize,
        concat!("Size of: ", stringify!(tagMQWDR))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWDR>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQWDR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).QMgrFlags as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(QMgrFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).QMgrIdentifier as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(QMgrIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).QMgrName as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).ClusterRecOffset as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(ClusterRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).ChannelState as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(ChannelState)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).ChannelDefOffset as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(ChannelDefOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).DestSeqNumber as *const _ as usize },
        124usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(DestSeqNumber)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR>())).DestSeqFactor as *const _ as usize },
        128usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR),
        "::",
        stringify!(DestSeqFactor)
        )
    );
}
pub type MQWDR1 = tagMQWDR1;
pub type PMQWDR1 = *mut MQWDR1;
pub type PPMQWDR1 = *mut PMQWDR1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWDR1 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub QMgrFlags: MQLONG,
    pub QMgrIdentifier: MQCHAR48,
    pub QMgrName: MQCHAR48,
    pub ClusterRecOffset: MQLONG,
    pub ChannelState: MQLONG,
    pub ChannelDefOffset: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQWDR1() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWDR1>(),
        124usize,
        concat!("Size of: ", stringify!(tagMQWDR1))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWDR1>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQWDR1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).QMgrFlags as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(QMgrFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).QMgrIdentifier as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(QMgrIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).QMgrName as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).ClusterRecOffset as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(ClusterRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).ChannelState as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(ChannelState)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR1>())).ChannelDefOffset as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR1),
        "::",
        stringify!(ChannelDefOffset)
        )
    );
}
pub type MQWDR2 = tagMQWDR2;
pub type PMQWDR2 = *mut MQWDR2;
pub type PPMQWDR2 = *mut PMQWDR2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWDR2 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub QMgrFlags: MQLONG,
    pub QMgrIdentifier: MQCHAR48,
    pub QMgrName: MQCHAR48,
    pub ClusterRecOffset: MQLONG,
    pub ChannelState: MQLONG,
    pub ChannelDefOffset: MQLONG,
    pub DestSeqNumber: MQLONG,
    pub DestSeqFactor: MQINT64,
}
#[test]
fn bindgen_test_layout_tagMQWDR2() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWDR2>(),
        136usize,
        concat!("Size of: ", stringify!(tagMQWDR2))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWDR2>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQWDR2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).QMgrFlags as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(QMgrFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).QMgrIdentifier as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(QMgrIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).QMgrName as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).ClusterRecOffset as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(ClusterRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).ChannelState as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(ChannelState)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).ChannelDefOffset as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(ChannelDefOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).DestSeqNumber as *const _ as usize },
        124usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(DestSeqNumber)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWDR2>())).DestSeqFactor as *const _ as usize },
        128usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWDR2),
        "::",
        stringify!(DestSeqFactor)
        )
    );
}
pub type MQWQR = tagMQWQR;
pub type PMQWQR = *mut MQWQR;
pub type PPMQWQR = *mut PMQWQR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWQR {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub QFlags: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrIdentifier: MQCHAR48,
    pub ClusterRecOffset: MQLONG,
    pub QType: MQLONG,
    pub QDesc: MQCHAR64,
    pub DefBind: MQLONG,
    pub DefPersistence: MQLONG,
    pub DefPriority: MQLONG,
    pub InhibitPut: MQLONG,
    pub CLWLQueuePriority: MQLONG,
    pub CLWLQueueRank: MQLONG,
    pub DefPutResponse: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQWQR() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWQR>(),
        212usize,
        concat!("Size of: ", stringify!(tagMQWQR))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWQR>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQWQR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).QFlags as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(QFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).QName as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).QMgrIdentifier as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(QMgrIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).ClusterRecOffset as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(ClusterRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).QType as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(QType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).QDesc as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(QDesc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).DefBind as *const _ as usize },
        184usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(DefBind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).DefPersistence as *const _ as usize },
        188usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(DefPersistence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).DefPriority as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(DefPriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).InhibitPut as *const _ as usize },
        196usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(InhibitPut)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).CLWLQueuePriority as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(CLWLQueuePriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).CLWLQueueRank as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(CLWLQueueRank)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR>())).DefPutResponse as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR),
        "::",
        stringify!(DefPutResponse)
        )
    );
}
pub type MQWQR1 = tagMQWQR1;
pub type PMQWQR1 = *mut MQWQR1;
pub type PPMQWQR1 = *mut PMQWQR1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWQR1 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub QFlags: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrIdentifier: MQCHAR48,
    pub ClusterRecOffset: MQLONG,
    pub QType: MQLONG,
    pub QDesc: MQCHAR64,
    pub DefBind: MQLONG,
    pub DefPersistence: MQLONG,
    pub DefPriority: MQLONG,
    pub InhibitPut: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQWQR1() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWQR1>(),
        200usize,
        concat!("Size of: ", stringify!(tagMQWQR1))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWQR1>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQWQR1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).QFlags as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(QFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).QName as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).QMgrIdentifier as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(QMgrIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).ClusterRecOffset as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(ClusterRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).QType as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(QType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).QDesc as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(QDesc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).DefBind as *const _ as usize },
        184usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(DefBind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).DefPersistence as *const _ as usize },
        188usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(DefPersistence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).DefPriority as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(DefPriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR1>())).InhibitPut as *const _ as usize },
        196usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR1),
        "::",
        stringify!(InhibitPut)
        )
    );
}
pub type MQWQR2 = tagMQWQR2;
pub type PMQWQR2 = *mut MQWQR2;
pub type PPMQWQR2 = *mut PMQWQR2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWQR2 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub QFlags: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrIdentifier: MQCHAR48,
    pub ClusterRecOffset: MQLONG,
    pub QType: MQLONG,
    pub QDesc: MQCHAR64,
    pub DefBind: MQLONG,
    pub DefPersistence: MQLONG,
    pub DefPriority: MQLONG,
    pub InhibitPut: MQLONG,
    pub CLWLQueuePriority: MQLONG,
    pub CLWLQueueRank: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQWQR2() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWQR2>(),
        208usize,
        concat!("Size of: ", stringify!(tagMQWQR2))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWQR2>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQWQR2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).QFlags as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(QFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).QName as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).QMgrIdentifier as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(QMgrIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).ClusterRecOffset as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(ClusterRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).QType as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(QType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).QDesc as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(QDesc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).DefBind as *const _ as usize },
        184usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(DefBind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).DefPersistence as *const _ as usize },
        188usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(DefPersistence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).DefPriority as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(DefPriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).InhibitPut as *const _ as usize },
        196usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(InhibitPut)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).CLWLQueuePriority as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(CLWLQueuePriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR2>())).CLWLQueueRank as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR2),
        "::",
        stringify!(CLWLQueueRank)
        )
    );
}
pub type MQWQR3 = tagMQWQR3;
pub type PMQWQR3 = *mut MQWQR3;
pub type PPMQWQR3 = *mut PMQWQR3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWQR3 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub StrucLength: MQLONG,
    pub QFlags: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrIdentifier: MQCHAR48,
    pub ClusterRecOffset: MQLONG,
    pub QType: MQLONG,
    pub QDesc: MQCHAR64,
    pub DefBind: MQLONG,
    pub DefPersistence: MQLONG,
    pub DefPriority: MQLONG,
    pub InhibitPut: MQLONG,
    pub CLWLQueuePriority: MQLONG,
    pub CLWLQueueRank: MQLONG,
    pub DefPutResponse: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQWQR3() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWQR3>(),
        212usize,
        concat!("Size of: ", stringify!(tagMQWQR3))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWQR3>(),
        4usize,
        concat!("Alignment of ", stringify!(tagMQWQR3))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).StrucLength as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(StrucLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).QFlags as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(QFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).QName as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).QMgrIdentifier as *const _ as usize },
        64usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(QMgrIdentifier)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).ClusterRecOffset as *const _ as usize },
        112usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(ClusterRecOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).QType as *const _ as usize },
        116usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(QType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).QDesc as *const _ as usize },
        120usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(QDesc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).DefBind as *const _ as usize },
        184usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(DefBind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).DefPersistence as *const _ as usize },
        188usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(DefPersistence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).DefPriority as *const _ as usize },
        192usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(DefPriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).InhibitPut as *const _ as usize },
        196usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(InhibitPut)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).CLWLQueuePriority as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(CLWLQueuePriority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).CLWLQueueRank as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(CLWLQueueRank)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWQR3>())).DefPutResponse as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWQR3),
        "::",
        stringify!(DefPutResponse)
        )
    );
}
pub type MQWXP = tagMQWXP;
pub type PMQWXP = *mut MQWXP;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWXP {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub Flags: MQLONG,
    pub ExitUserArea: MQBYTE16,
    pub ExitData: MQCHAR32,
    pub MsgDescPtr: PMQMD,
    pub MsgBufferPtr: PMQVOID,
    pub MsgBufferLength: MQLONG,
    pub MsgLength: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrName: MQCHAR48,
    pub DestinationCount: MQLONG,
    pub DestinationChosen: MQLONG,
    pub DestinationArrayPtr: PPMQWDR,
    pub QArrayPtr: PPMQWQR,
    pub CacheContext: MQPTR,
    pub CacheType: MQLONG,
    pub CLWLMRUChannels: MQLONG,
    pub pEntryPoints: PMQIEP,
}
#[test]
fn bindgen_test_layout_tagMQWXP() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWXP>(),
        248usize,
        concat!("Size of: ", stringify!(tagMQWXP))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWXP>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQWXP))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).ExitUserArea as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(ExitUserArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).ExitData as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(ExitData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).MsgDescPtr as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(MsgDescPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).MsgBufferPtr as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(MsgBufferPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).MsgBufferLength as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(MsgBufferLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).MsgLength as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(MsgLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).QName as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).QMgrName as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).DestinationCount as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(DestinationCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).DestinationChosen as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(DestinationChosen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).DestinationArrayPtr as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(DestinationArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).QArrayPtr as *const _ as usize },
        216usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(QArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).CacheContext as *const _ as usize },
        224usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(CacheContext)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).CacheType as *const _ as usize },
        232usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(CacheType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).CLWLMRUChannels as *const _ as usize },
        236usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(CLWLMRUChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP>())).pEntryPoints as *const _ as usize },
        240usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP),
        "::",
        stringify!(pEntryPoints)
        )
    );
}
pub type MQWXP1 = tagMQWXP1;
pub type PMQWXP1 = *mut MQWXP1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWXP1 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub Flags: MQLONG,
    pub ExitUserArea: MQBYTE16,
    pub ExitData: MQCHAR32,
    pub MsgDescPtr: PMQMD,
    pub MsgBufferPtr: PMQVOID,
    pub MsgBufferLength: MQLONG,
    pub MsgLength: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrName: MQCHAR48,
    pub DestinationCount: MQLONG,
    pub DestinationChosen: MQLONG,
    pub DestinationArrayPtr: PPMQWDR,
    pub QArrayPtr: PPMQWQR,
}
#[test]
fn bindgen_test_layout_tagMQWXP1() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWXP1>(),
        224usize,
        concat!("Size of: ", stringify!(tagMQWXP1))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWXP1>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQWXP1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).ExitUserArea as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(ExitUserArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).ExitData as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(ExitData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).MsgDescPtr as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(MsgDescPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).MsgBufferPtr as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(MsgBufferPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).MsgBufferLength as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(MsgBufferLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).MsgLength as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(MsgLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).QName as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).QMgrName as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).DestinationCount as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(DestinationCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).DestinationChosen as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(DestinationChosen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).DestinationArrayPtr as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(DestinationArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP1>())).QArrayPtr as *const _ as usize },
        216usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP1),
        "::",
        stringify!(QArrayPtr)
        )
    );
}
pub type MQWXP2 = tagMQWXP2;
pub type PMQWXP2 = *mut MQWXP2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWXP2 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub Flags: MQLONG,
    pub ExitUserArea: MQBYTE16,
    pub ExitData: MQCHAR32,
    pub MsgDescPtr: PMQMD,
    pub MsgBufferPtr: PMQVOID,
    pub MsgBufferLength: MQLONG,
    pub MsgLength: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrName: MQCHAR48,
    pub DestinationCount: MQLONG,
    pub DestinationChosen: MQLONG,
    pub DestinationArrayPtr: PPMQWDR,
    pub QArrayPtr: PPMQWQR,
    pub CacheContext: MQPTR,
    pub CacheType: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQWXP2() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWXP2>(),
        240usize,
        concat!("Size of: ", stringify!(tagMQWXP2))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWXP2>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQWXP2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).ExitUserArea as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(ExitUserArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).ExitData as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(ExitData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).MsgDescPtr as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(MsgDescPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).MsgBufferPtr as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(MsgBufferPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).MsgBufferLength as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(MsgBufferLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).MsgLength as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(MsgLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).QName as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).QMgrName as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).DestinationCount as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(DestinationCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).DestinationChosen as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(DestinationChosen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).DestinationArrayPtr as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(DestinationArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).QArrayPtr as *const _ as usize },
        216usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(QArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).CacheContext as *const _ as usize },
        224usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(CacheContext)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP2>())).CacheType as *const _ as usize },
        232usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP2),
        "::",
        stringify!(CacheType)
        )
    );
}
pub type MQWXP3 = tagMQWXP3;
pub type PMQWXP3 = *mut MQWXP3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWXP3 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub Flags: MQLONG,
    pub ExitUserArea: MQBYTE16,
    pub ExitData: MQCHAR32,
    pub MsgDescPtr: PMQMD,
    pub MsgBufferPtr: PMQVOID,
    pub MsgBufferLength: MQLONG,
    pub MsgLength: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrName: MQCHAR48,
    pub DestinationCount: MQLONG,
    pub DestinationChosen: MQLONG,
    pub DestinationArrayPtr: PPMQWDR,
    pub QArrayPtr: PPMQWQR,
    pub CacheContext: MQPTR,
    pub CacheType: MQLONG,
    pub CLWLMRUChannels: MQLONG,
}
#[test]
fn bindgen_test_layout_tagMQWXP3() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWXP3>(),
        240usize,
        concat!("Size of: ", stringify!(tagMQWXP3))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWXP3>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQWXP3))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).ExitUserArea as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(ExitUserArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).ExitData as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(ExitData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).MsgDescPtr as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(MsgDescPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).MsgBufferPtr as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(MsgBufferPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).MsgBufferLength as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(MsgBufferLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).MsgLength as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(MsgLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).QName as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).QMgrName as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).DestinationCount as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(DestinationCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).DestinationChosen as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(DestinationChosen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).DestinationArrayPtr as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(DestinationArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).QArrayPtr as *const _ as usize },
        216usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(QArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).CacheContext as *const _ as usize },
        224usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(CacheContext)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).CacheType as *const _ as usize },
        232usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(CacheType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP3>())).CLWLMRUChannels as *const _ as usize },
        236usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP3),
        "::",
        stringify!(CLWLMRUChannels)
        )
    );
}
pub type MQWXP4 = tagMQWXP4;
pub type PMQWXP4 = *mut MQWXP4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagMQWXP4 {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub ExitId: MQLONG,
    pub ExitReason: MQLONG,
    pub ExitResponse: MQLONG,
    pub ExitResponse2: MQLONG,
    pub Feedback: MQLONG,
    pub Flags: MQLONG,
    pub ExitUserArea: MQBYTE16,
    pub ExitData: MQCHAR32,
    pub MsgDescPtr: PMQMD,
    pub MsgBufferPtr: PMQVOID,
    pub MsgBufferLength: MQLONG,
    pub MsgLength: MQLONG,
    pub QName: MQCHAR48,
    pub QMgrName: MQCHAR48,
    pub DestinationCount: MQLONG,
    pub DestinationChosen: MQLONG,
    pub DestinationArrayPtr: PPMQWDR,
    pub QArrayPtr: PPMQWQR,
    pub CacheContext: MQPTR,
    pub CacheType: MQLONG,
    pub CLWLMRUChannels: MQLONG,
    pub pEntryPoints: PMQIEP,
}
#[test]
fn bindgen_test_layout_tagMQWXP4() {
    assert_eq!(
        ::std::mem::size_of::<tagMQWXP4>(),
        248usize,
        concat!("Size of: ", stringify!(tagMQWXP4))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQWXP4>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQWXP4))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).ExitId as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(ExitId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).ExitReason as *const _ as usize },
        12usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(ExitReason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).ExitResponse as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(ExitResponse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).ExitResponse2 as *const _ as usize },
        20usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(ExitResponse2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).Feedback as *const _ as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(Feedback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).Flags as *const _ as usize },
        28usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).ExitUserArea as *const _ as usize },
        32usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(ExitUserArea)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).ExitData as *const _ as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(ExitData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).MsgDescPtr as *const _ as usize },
        80usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(MsgDescPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).MsgBufferPtr as *const _ as usize },
        88usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(MsgBufferPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).MsgBufferLength as *const _ as usize },
        96usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(MsgBufferLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).MsgLength as *const _ as usize },
        100usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(MsgLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).QName as *const _ as usize },
        104usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(QName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).QMgrName as *const _ as usize },
        152usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(QMgrName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).DestinationCount as *const _ as usize },
        200usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(DestinationCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).DestinationChosen as *const _ as usize },
        204usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(DestinationChosen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).DestinationArrayPtr as *const _ as usize },
        208usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(DestinationArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).QArrayPtr as *const _ as usize },
        216usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(QArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).CacheContext as *const _ as usize },
        224usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(CacheContext)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).CacheType as *const _ as usize },
        232usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(CacheType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).CLWLMRUChannels as *const _ as usize },
        236usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(CLWLMRUChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQWXP4>())).pEntryPoints as *const _ as usize },
        240usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQWXP4),
        "::",
        stringify!(pEntryPoints)
        )
    );
}
pub type MQXEPO = tagMQXEPO;
pub type PMQXEPO = *mut MQXEPO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQXEPO {
    pub StrucId: MQCHAR4,
    pub Version: MQLONG,
    pub Options: MQLONG,
    pub ExitProperties: MQCHARV,
}
#[test]
fn bindgen_test_layout_tagMQXEPO() {
    assert_eq!(
        ::std::mem::size_of::<tagMQXEPO>(),
        40usize,
        concat!("Size of: ", stringify!(tagMQXEPO))
    );
    assert_eq!(
        ::std::mem::align_of::<tagMQXEPO>(),
        8usize,
        concat!("Alignment of ", stringify!(tagMQXEPO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXEPO>())).StrucId as *const _ as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXEPO),
        "::",
        stringify!(StrucId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXEPO>())).Version as *const _ as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXEPO),
        "::",
        stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXEPO>())).Options as *const _ as usize },
        8usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXEPO),
        "::",
        stringify!(Options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagMQXEPO>())).ExitProperties as *const _ as usize },
        16usize,
        concat!(
        "Offset of field: ",
        stringify!(tagMQXEPO),
        "::",
        stringify!(ExitProperties)
        )
    );
}
extern "C" {
    pub fn MQXEP(
        Hconfig: MQHCONFIG,
        ExitReason: MQLONG,
        Function: MQLONG,
        pEntryPoint: PMQFUNC,
        pExitOpts: PMQXEPO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_XEP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        ExitReason: MQLONG,
        Function: MQLONG,
        pEntryPoint: PMQFUNC,
        pExitOpts: PMQXEPO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_XEP_CALL = MQ_XEP_CALL;
pub type MQ_BACK_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_BACK_EXIT = MQ_BACK_EXIT;
pub type MQ_BEGIN_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppBeginOptions: PPMQBO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_BEGIN_EXIT = MQ_BEGIN_EXIT;
pub type MQ_CALLBACK_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppMsgDesc: PPMQMD,
        ppGetMsgOpts: PPMQGMO,
        ppBuffer: PPMQVOID,
        ppMQCBContext: PPMQCBC,
    ),
>;
pub type PMQ_CALLBACK_EXIT = MQ_CALLBACK_EXIT;
pub type MQ_CB_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pOperation: PMQLONG,
        ppCallbackDesc: PPMQCBD,
        pHobj: PMQHOBJ,
        ppMsgDesc: PPMQMD,
        ppGetMsgOpts: PPMQGMO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CB_EXIT = MQ_CB_EXIT;
pub type MQ_CLOSE_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppHobj: PPMQHOBJ,
        pOptions: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CLOSE_EXIT = MQ_CLOSE_EXIT;
pub type MQ_CMIT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CMIT_EXIT = MQ_CMIT_EXIT;
pub type MQ_CONNX_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pQMgrName: PMQCHAR,
        ppConnectOpts: PPMQCNO,
        ppHconn: PPMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CONNX_EXIT = MQ_CONNX_EXIT;
pub type MQ_CTL_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pOperation: PMQLONG,
        ppCtlOpts: PPMQCTLO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CTL_EXIT = MQ_CTL_EXIT;
pub type MQ_DISC_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        ppHconn: PPMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_DISC_EXIT = MQ_DISC_EXIT;
pub type MQ_GET_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHobj: PMQHOBJ,
        ppMsgDesc: PPMQMD,
        ppGetMsgOpts: PPMQGMO,
        pBufferLength: PMQLONG,
        ppBuffer: PPMQVOID,
        ppDataLength: PPMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_GET_EXIT = MQ_GET_EXIT;
pub type MQ_INIT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_INIT_EXIT = MQ_INIT_EXIT;
pub type MQ_INQ_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHobj: PMQHOBJ,
        pSelectorCount: PMQLONG,
        ppSelectors: PPMQLONG,
        pIntAttrCount: PMQLONG,
        ppIntAttrs: PPMQLONG,
        pCharAttrLength: PMQLONG,
        ppCharAttrs: PPMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_INQ_EXIT = MQ_INQ_EXIT;
pub type MQ_OPEN_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppObjDesc: PPMQOD,
        pOptions: PMQLONG,
        ppHobj: PPMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_OPEN_EXIT = MQ_OPEN_EXIT;
pub type MQ_PUT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHobj: PMQHOBJ,
        ppMsgDesc: PPMQMD,
        ppPutMsgOpts: PPMQPMO,
        pBufferLength: PMQLONG,
        ppBuffer: PPMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PUT_EXIT = MQ_PUT_EXIT;
pub type MQ_PUT1_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppObjDesc: PPMQOD,
        ppMsgDesc: PPMQMD,
        ppPutMsgOpts: PPMQPMO,
        pBufferLength: PMQLONG,
        ppBuffer: PPMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PUT1_EXIT = MQ_PUT1_EXIT;
pub type MQ_SET_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHobj: PMQHOBJ,
        pSelectorCount: PMQLONG,
        ppSelectors: PPMQLONG,
        pIntAttrCount: PMQLONG,
        ppIntAttrs: PPMQLONG,
        pCharAttrLength: PMQLONG,
        ppCharAttrs: PPMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SET_EXIT = MQ_SET_EXIT;
pub type MQ_STAT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pType: PMQLONG,
        ppStatus: PPMQSTS,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_STAT_EXIT = MQ_STAT_EXIT;
pub type MQ_SUBRQ_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHsub: PMQHOBJ,
        pAction: PMQLONG,
        ppSubRqOpts: PPMQSRO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SUBRQ_EXIT = MQ_SUBRQ_EXIT;
pub type MQ_SUB_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppSubDesc: PPMQSD,
        ppHobj: PPMQHOBJ,
        ppHsub: PPMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SUB_EXIT = MQ_SUB_EXIT;
pub type MQ_TERM_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_TERM_EXIT = MQ_TERM_EXIT;
pub type XA_CLOSE_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXa_info: PPMQCHAR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_CLOSE_EXIT = XA_CLOSE_EXIT;
pub type XA_COMMIT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXID: PMQPTR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_COMMIT_EXIT = XA_COMMIT_EXIT;
pub type XA_COMPLETE_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppHandle: PPMQLONG,
        ppRetval: PPMQLONG,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_COMPLETE_EXIT = XA_COMPLETE_EXIT;
pub type XA_END_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXID: PMQPTR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_END_EXIT = XA_END_EXIT;
pub type XA_FORGET_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXID: PMQPTR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_FORGET_EXIT = XA_FORGET_EXIT;
pub type XA_OPEN_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXa_info: PPMQCHAR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_OPEN_EXIT = XA_OPEN_EXIT;
pub type XA_PREPARE_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXID: PMQPTR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_PREPARE_EXIT = XA_PREPARE_EXIT;
pub type XA_RECOVER_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXID: PMQPTR,
        pCount: PMQLONG,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_RECOVER_EXIT = XA_RECOVER_EXIT;
pub type XA_ROLLBACK_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXID: PMQPTR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_ROLLBACK_EXIT = XA_ROLLBACK_EXIT;
pub type XA_START_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppXID: PMQPTR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PXA_START_EXIT = XA_START_EXIT;
pub type AX_REG_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        ppXID: PMQPTR,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PAX_REG_EXIT = AX_REG_EXIT;
pub type AX_UNREG_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pRmid: PMQLONG,
        pFlags: PMQLONG,
        pXARetCode: PMQLONG,
    ),
>;
pub type PAX_UNREG_EXIT = AX_UNREG_EXIT;
pub type MQ_CHANNEL_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pChannelExitParms: PMQVOID,
        pChannelDefinition: PMQVOID,
        pDataLength: PMQLONG,
        pAgentBufferLength: PMQLONG,
        pAgentBuffer: PMQVOID,
        pExitBufferLength: PMQLONG,
        pExitBufferAddr: PMQPTR,
    ),
>;
pub type PMQ_CHANNEL_EXIT = MQ_CHANNEL_EXIT;
pub type MQ_CHANNEL_AUTO_DEF_EXIT = ::std::option::Option<
    unsafe extern "C" fn(pChannelExitParms: PMQVOID, pChannelDefinition: PMQVOID),
>;
pub type PMQ_CHANNEL_AUTO_DEF_EXIT = MQ_CHANNEL_AUTO_DEF_EXIT;
pub type MQ_CLUSTER_WORKLOAD_EXIT = ::std::option::Option<unsafe extern "C" fn(pExitParms: PMQWXP)>;
pub type PMQ_CLUSTER_WORKLOAD_EXIT = MQ_CLUSTER_WORKLOAD_EXIT;
pub type MQ_DATA_CONV_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pDataConvExitParms: PMQDXP,
        pMsgDesc: PMQMD,
        InBufferLength: MQLONG,
        pInBuffer: PMQVOID,
        OutBufferLength: MQLONG,
        pOutBuffer: PMQVOID,
    ),
>;
pub type PMQ_DATA_CONV_EXIT = MQ_DATA_CONV_EXIT;
pub type MQ_PUBLISH_EXIT = ::std::option::Option<
    unsafe extern "C" fn(pExitParms: PMQPSXP, pPubContext: PMQPBC, pSubContext: PMQSBC),
>;
pub type PMQ_PUBLISH_EXIT = MQ_PUBLISH_EXIT;
pub type MQ_TRANSPORT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(pExitParms: PMQVOID, DestAddressLength: MQLONG, pDestAddress: PMQCHAR),
>;
pub type PMQ_TRANSPORT_EXIT = MQ_TRANSPORT_EXIT;
pub type MQ_PRECONNECT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQNXP,
        pQMgrName: PMQCHAR,
        ppConnectOpts: PPMQCNO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PRECONNECT_EXIT = MQ_PRECONNECT_EXIT;
extern "C" {
    pub fn MQXCLWLN(
        pExitParms: PMQWXP,
        CurrentRecord: MQPTR,
        NextOffset: MQLONG,
        pNextRecord: PMQPTR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_XCLWLN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQWXP,
        CurrentRecord: MQPTR,
        NextOffset: MQLONG,
        pNextRecord: PMQPTR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_XCLWLN_CALL = MQ_XCLWLN_CALL;
extern "C" {
    pub fn MQXCNVC(
        Hconn: MQHCONN,
        Options: MQLONG,
        SourceCCSID: MQLONG,
        SourceLength: MQLONG,
        pSourceBuffer: PMQCHAR,
        TargetCCSID: MQLONG,
        TargetLength: MQLONG,
        pTargetBuffer: PMQCHAR,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
pub type MQ_XCNVC_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Options: MQLONG,
        SourceCCSID: MQLONG,
        SourceLength: MQLONG,
        pSourceBuffer: PMQCHAR,
        TargetCCSID: MQLONG,
        TargetLength: MQLONG,
        pTargetBuffer: PMQCHAR,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_XCNVC_CALL = MQ_XCNVC_CALL;
extern "C" {
    pub fn MQXDX(
        pDataConvExitParms: PMQDXP,
        pMsgDesc: PMQMD,
        InBufferLength: MQLONG,
        pInBuffer: PMQVOID,
        OutBufferLength: MQLONG,
        pOutBuffer: PMQVOID,
    );
}
pub type MQ_XDX_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        pDataConvExitParms: PMQDXP,
        pMsgDesc: PMQMD,
        InBufferLength: MQLONG,
        pInBuffer: PMQVOID,
        OutBufferLength: MQLONG,
        pOutBuffer: PMQVOID,
    ),
>;
pub type PMQ_XDX_CALL = MQ_XDX_CALL;
