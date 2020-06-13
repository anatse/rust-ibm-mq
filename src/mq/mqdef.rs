#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use super::generated::*;
use std::ptr::{null_mut};

impl MQCD {
    /// Function used to create MQCD structure with
    /// MQCD_CLIENT_CONN_DEFAULT values
    pub fn default_client() -> MQCD {
        MQCD {
            ChannelName: [0; 20],
            Version: MQCD_CURRENT_VERSION as i32,
            ChannelType: MQCHT_CLNTCONN as i32,
            TransportType: MQXPT_TCP as i32,
            Desc: [0; 64],
            QMgrName: [0; 48],
            XmitQName: [0; 48],
            ShortConnectionName: [0; 20],
            MCAName: [0; 20],
            ModeName: [0; 8],
            TpName: [0; 64],
            BatchSize: 50,
            DiscInterval: 6000,
            ShortRetryCount: 10,
            ShortRetryInterval: 60,
            LongRetryCount: 999999999,
            LongRetryInterval: 1200,
            SecurityExit: [0; 128],
            MsgExit: [0; 128],
            SendExit: [0; 128],
            ReceiveExit: [0; 128],
            SeqNumberWrap: 999999999,
            MaxMsgLength: 4194304,
            PutAuthority: MQPA_DEFAULT as i32,
            DataConversion: MQCDC_NO_SENDER_CONVERSION as i32,
            SecurityUserData: [0; 32],
            MsgUserData: [0; 32],
            SendUserData: [0; 32],
            ReceiveUserData: [0; 32],
            UserIdentifier: [0; 12],
            Password: [0; 12],
            MCAUserIdentifier: [0; 12],
            MCAType: MQMCAT_PROCESS as i32,
            ConnectionName: [0; 264],
            RemoteUserIdentifier: [0; 12],
            RemotePassword: [0; 12],
            MsgRetryExit: [0; 128],
            MsgRetryUserData: [0; 32],
            MsgRetryCount: 10,
            MsgRetryInterval: 1000,
            HeartbeatInterval: 1,
            BatchInterval: 0,
            NonPersistentMsgSpeed: MQNPMS_FAST as i32,
            StrucLength: MQCD_CURRENT_LENGTH as i32,
            ExitNameLength: MQ_EXIT_NAME_LENGTH as i32,
            ExitDataLength: MQ_EXIT_DATA_LENGTH as i32,
            MsgExitsDefined: 0,
            SendExitsDefined: 0,
            ReceiveExitsDefined: 0,
            MsgExitPtr: null_mut(),
            MsgUserDataPtr: null_mut(),
            SendExitPtr: null_mut(),
            SendUserDataPtr: null_mut(),
            ReceiveExitPtr: null_mut(),
            ReceiveUserDataPtr: null_mut(),
            ClusterPtr: null_mut(),
            ClustersDefined: 0,
            NetworkPriority: 0,
            LongMCAUserIdLength: 0,
            LongRemoteUserIdLength: 0,
            LongMCAUserIdPtr: null_mut(),
            LongRemoteUserIdPtr: null_mut(),
            MCASecurityId: [0; 40],
            RemoteSecurityId: [0; 40],
            SSLCipherSpec: [0; 32],
            SSLPeerNamePtr: null_mut(),
            SSLPeerNameLength: 0,
            SSLClientAuth: MQSCA_REQUIRED as i32,
            KeepAliveInterval: MQKAI_AUTO as i32,
            LocalAddress: [0; 48],
            BatchHeartbeat: 0,
            HdrCompList: [
                MQCOMPRESS_NONE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
            ],
            MsgCompList: [
                MQCOMPRESS_NONE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
                MQCOMPRESS_NOT_AVAILABLE as i32,
            ],
            CLWLChannelRank: 0,
            CLWLChannelPriority: 0,
            CLWLChannelWeight: 50,
            ChannelMonitoring: MQMON_OFF as i32,
            ChannelStatistics: MQMON_OFF as i32,
            SharingConversations: 10,
            PropertyControl: MQPROP_COMPATIBILITY as i32,
            MaxInstances: 999999999,
            MaxInstancesPerClient: 999999999,
            ClientChannelWeight: 0,
            ConnectionAffinity: MQCAFTY_PREFERRED as i32,
            BatchDataLimit: 5000,
            UseDLQ: MQUSEDLQ_YES as i32,
            DefReconnect: MQRCN_NO as i32,
            CertificateLabel: [0; 64],
            SPLProtection: MQSPL_PASSTHRU as i32,
        }
    }
}

impl MQCNO {
    pub fn default() -> MQCNO {
        MQCNO {
            StrucId: ['C' as i8, 'N' as i8, 'O' as i8, ' ' as i8],
            Version: MQCNO_CURRENT_VERSION as i32,
            Options: MQCNO_NONE as i32,
            ClientConnOffset: 0,
            ClientConnPtr: null_mut(),
            ConnTag: [0; 128],
            SSLConfigPtr: null_mut(),
            SSLConfigOffset: 0,
            ConnectionId: [0; 24],
            SecurityParmsOffset: 0,
            SecurityParmsPtr: null_mut(),
            CCDTUrlPtr: null_mut(),
            CCDTUrlOffset: 0,
            CCDTUrlLength: 0,
            Reserved: [0; 8],
            ApplName: [0; 28],
            Reserved2: [0; 4],
        }
    }
}

impl MQCHARV {
    pub fn default() -> MQCHARV {
        MQCHARV {
            VSPtr: null_mut(),
            VSOffset: 0,
            VSBufSize: 0,
            VSLength: 0,
            VSCCSID: -3,
        }
    }
}

impl MQOD {
    pub fn default() -> MQOD {
        MQOD {
            StrucId: ['O' as i8, 'D' as i8, ' ' as i8, ' ' as i8],
            Version: MQOD_CURRENT_VERSION as i32,
            ObjectType: MQOT_Q as i32,
            ObjectName: [0; 48],
            ObjectQMgrName: {
                let mut array: [i8; 48] = [0; 48];
                array[0] = 'A' as i8;
                array[1] = 'M' as i8;
                array[2] = 'Q' as i8;
                array[3] = '.' as i8;
                array[4] = '*' as i8;
                array
            },
            DynamicQName: [0; 48],
            AlternateUserId: [0; 12],
            RecsPresent: 0,
            KnownDestCount: 0,
            UnknownDestCount: 0,
            InvalidDestCount: 0,
            ObjectRecOffset: 0,
            ResponseRecOffset: 0,
            ObjectRecPtr: null_mut(),
            ResponseRecPtr: null_mut(),
            AlternateSecurityId: [0; 40],
            ResolvedQName: [0; 48],
            ResolvedQMgrName: [0; 48],
            ObjectString: MQCHARV::default(),
            SelectionString: MQCHARV::default(),
            ResObjectString: MQCHARV::default(),
            ResolvedType: MQOT_NONE as i32,
        }
    }
}

impl MQMD {
    pub fn default() -> MQMD {
        MQMD {
            StrucId: ['M' as i8,'D' as i8, ' ' as i8, ' ' as i8],
            Version: MQMD_VERSION_1 as i32,
            Report: MQRO_NONE as i32,
            MsgType: MQMT_DATAGRAM as i32,
            Expiry: MQEI_UNLIMITED as i32,
            Feedback: MQRO_NONE as i32,
            Encoding: MQENC_NATIVE as i32,
            CodedCharSetId: MQCCSI_Q_MGR as i32,
            Format: [0; 8],
            Priority: MQPRI_PRIORITY_AS_Q_DEF as i32,
            Persistence: MQPER_PERSISTENCE_AS_Q_DEF as i32,
            MsgId: [0; 24],
            CorrelId: [0; 24],
            BackoutCount: 0,
            ReplyToQ: [0; 48],
            ReplyToQMgr: [0; 48],
            UserIdentifier: [0; 12],
            AccountingToken: [0; 32],
            ApplIdentityData: [0; 32],
            PutApplType: 0,
            PutApplName: [0; 28],
            PutDate: [0; 8],
            PutTime: [0; 8],
            ApplOriginData: [0; 4],
            GroupId: [0; 24],
            MsgSeqNumber: 1,
            Offset: 0,
            MsgFlags: MQMF_NONE as i32,
            OriginalLength: MQOL_UNDEFINED as i32,
        }
    }
}

impl MQGMO {
    pub fn default() -> MQGMO {
        MQGMO {
            StrucId: ['G' as i8, 'M' as i8, 'O' as i8, ' ' as i8],
            Version: MQGMO_CURRENT_VERSION as i32,
            Options: (MQGMO_NO_WAIT + MQGMO_PROPERTIES_AS_Q_DEF) as i32,
            WaitInterval: 0,
            Signal1: 0,
            Signal2: 0,
            ResolvedQName: [0; 48],
            MatchOptions: (MQMO_MATCH_MSG_ID + MQMO_MATCH_CORREL_ID) as i32,
            GroupStatus: MQGS_NOT_IN_GROUP as i8,
            SegmentStatus: MQSS_NOT_A_SEGMENT as i8,
            Segmentation: MQSEG_INHIBITED as i8,
            Reserved1: 0,
            MsgToken: [0; 16],
            ReturnedLength: 0,
            Reserved2: 0,
            MsgHandle: MQHM_NONE as i64,
        }
    }
}

impl MQCBD {
    pub fn default() -> MQCBD {
        MQCBD {
            StrucId: ['C' as i8, 'B' as i8, 'D' as i8, ' ' as i8],
            Version: MQCBD_CURRENT_VERSION as i32,
            CallbackType: MQCBT_MESSAGE_CONSUMER as i32,
            Options: MQCBDO_NONE as i32,
            CallbackArea: null_mut(),
            CallbackFunction: null_mut(),
            CallbackName: [0; 128],
            MaxMsgLength: MQCBD_FULL_MSG_LENGTH as i32,
        }
    }
}

impl MQCTLO {
    pub fn default() -> MQCTLO {
        MQCTLO {
            StrucId: ['C' as i8, 'T' as i8, 'L' as i8, 'O' as i8],
            Version: MQCTLO_CURRENT_VERSION as i32,
            Options: MQCTLO_NONE as i32,
            Reserved: MQWI_UNLIMITED as i32,
            ConnectionArea: null_mut(),
        }
    }
}

impl MQCSP {
    pub fn default() -> MQCSP {
        MQCSP {
            StrucId: ['C' as i8, 'S' as i8, 'P' as i8, ' ' as i8],
            Version: MQCSP_VERSION_1 as i32,
            AuthenticationType: MQCSP_AUTH_NONE as i32,
            Reserved1: [0; 4],
            CSPUserIdPtr: null_mut(),
            CSPUserIdOffset: 0,
            CSPUserIdLength: 3,
            Reserved2: [0; 8],
            CSPPasswordPtr: null_mut(),
            CSPPasswordOffset: 0,
            CSPPasswordLength: 0,
        }
    }
}

impl MQPMO {
    pub fn default() -> MQPMO {
        MQPMO {
            StrucId: ['P' as i8, 'M' as i8, 'O' as i8, ' ' as i8],
            Version: MQPMO_VERSION_1 as i32,
            Options: MQPMO_NONE as i32,
            Timeout: -1,
            Context: 0,
            KnownDestCount: 0,
            UnknownDestCount: 0,
            InvalidDestCount: 0,
            ResolvedQName: [0; 48],
            ResolvedQMgrName: [0; 48],
            RecsPresent: 0,
            PutMsgRecFields: MQPMRF_NONE as i32,
            PutMsgRecOffset: 0,
            ResponseRecOffset: 0,
            PutMsgRecPtr: null_mut(),
            ResponseRecPtr: null_mut(),
            OriginalMsgHandle: MQHM_NONE as i64,
            NewMsgHandle: MQHM_NONE as i64,
            Action: MQACTP_NEW as i32,
            PubLevel: 9,
        }
    }
}

impl MQSTS {
    pub fn default() -> MQSTS {
        MQSTS {
            StrucId: ['S' as i8, 'T' as i8, 'S' as i8, ' ' as i8],
            Version: MQSTS_VERSION_1 as i32,
            CompCode: 0,
            Reason: 0,
            PutSuccessCount: 0,
            PutWarningCount: MQCC_OK as i32,
            PutFailureCount: MQRC_NONE as i32,
            ObjectType: MQOT_Q as i32,
            ObjectName: [0; 48],
            ObjectQMgrName: [0; 48],
            ResolvedObjectName: [0; 48],
            ResolvedQMgrName: [0; 48],
            ObjectString: MQCHARV::default(),
            SubName: MQCHARV::default(),
            OpenOptions: 0,
            SubOptions: 0,
        }
    }
}

impl MQCMHO {
    pub fn default() -> MQCMHO {
        MQCMHO {
            StrucId: ['C' as i8, 'M' as i8, 'H' as i8, 'O' as i8],
            Version: MQCMHO_VERSION_1 as i32,
            Options: MQCMHO_DEFAULT_VALIDATION as i32,
        }
    }
}

impl MQSMPO {
    pub fn default() -> MQSMPO{
        MQSMPO {
            StrucId: ['S' as i8, 'M' as i8, 'P' as i8, 'O' as i8],
            Version: MQSMPO_VERSION_1 as i32,
            Options: MQSMPO_SET_FIRST as i32,
            ValueEncoding: MQENC_NATIVE as i32,
            ValueCCSID: MQCCSI_APPL as i32,
        }
    }
}

impl MQDMHO {
    pub fn default() -> MQDMHO {
        MQDMHO {
            StrucId: ['D' as i8, 'M' as i8, 'H' as i8, 'O' as i8],
            Version: MQDMHO_VERSION_1 as i32,
            Options: MQDMHO_NONE as i32,
        }
    }
}

impl MQPD {
    pub fn default() -> MQPD {
        MQPD {
            StrucId: ['P' as i8, 'D' as i8, ' ' as i8, ' ' as i8],
            Version: MQPD_VERSION_1 as i32,
            Options: MQPD_NONE as i32,
            Support: MQPD_SUPPORT_OPTIONAL as i32,
            Context: MQPD_NO_CONTEXT as i32,
            CopyOptions: MQCOPY_DEFAULT as i32,
        }
    }
}

impl MQSCO {
    pub fn default() -> MQSCO {
        MQSCO {
            StrucId: ['S' as i8, 'C' as i8, 'O' as i8, ' ' as i8],
            Version: MQSCO_VERSION_1 as i32,
            KeyRepository: [0; 256],
            CryptoHardware: [0; 256],
            AuthInfoRecCount: 0,
            AuthInfoRecOffset: 0,
            AuthInfoRecPtr: null_mut(),
            KeyResetCount: MQSCO_RESET_COUNT_DEFAULT as i32,
            FipsRequired:MQSSL_FIPS_NO as i32,
            EncryptionPolicySuiteB: [MQ_SUITE_B_NONE as i32,
                                    MQ_SUITE_B_NOT_AVAILABLE as i32,
                                    MQ_SUITE_B_NOT_AVAILABLE as i32,
                                    MQ_SUITE_B_NOT_AVAILABLE as i32],
            CertificateValPolicy: MQ_CERT_VAL_POLICY_DEFAULT as i32,
            CertificateLabel: [0; 64],
        }
    }
}

impl MQAIR {
    pub fn default() -> MQAIR {
        MQAIR {
            StrucId: ['A' as i8, 'I' as i8, 'R' as i8, ' ' as i8],
            Version: MQAIR_VERSION_1 as i32,
            AuthInfoType: MQAIT_CRL_LDAP as i32,
            AuthInfoConnName: [0; 264],
            LDAPUserNamePtr: null_mut(),
            LDAPUserNameOffset: 0,
            LDAPUserNameLength: 0,
            LDAPPassword: [0; 32],
            OCSPResponderURL: [0; 256]
        }
    }
}

impl MQRFH2 {
    pub fn default() -> MQRFH2 {
        MQRFH2 {
            StrucId: ['R' as i8, 'F' as i8, 'H' as i8, ' ' as i8],
            Version: MQRFH_VERSION_2 as i32,
            StrucLength: MQRFH_STRUC_LENGTH_FIXED_2 as i32,
            Encoding: MQENC_NATIVE as i32,
            CodedCharSetId: MQCCSI_INHERIT,
            Format: [0; 8],
            Flags: MQRFH_NONE as i32,
            NameValueCCSID: 1208
        }
    }
}