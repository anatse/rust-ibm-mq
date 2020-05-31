#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use hocon::HoconLoader;
use serde::Deserialize;
use std::ptr::{null_mut};
use std::ffi::{CStr};
use ::std::os::raw::c_char;

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Deserialize, Clone)]
pub struct MqConfig {
    pub server_name: String,
    pub server_port: u16,
    pub queue_manager: String,
    pub channel_name: String,
    pub target_queue: String,
    pub user_id: Option<String>,
    pub password: Option<String>,
    pub ssl_key_repos_stem: Option<String>,
    pub cipher_spec: Option<String>,
    pub certificate_label: Option<String>,
    pub oscp_url: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Mq {
    mq: MqConfig
}

impl MqConfig {
    pub fn load_config(configName: &str) -> Mq {
        let doc: Mq = HoconLoader::new()
            .load_file(configName).expect(format!("Unable to load config from file {}", configName).as_str())
            .hocon().and_then(move |conf| {
                conf.resolve()
        }).expect("Unable to resolve configuration as MqConfig");

        doc
    }
}

impl MQCD {
    /**
    * Function used to create MQCD structure with default values
    */
    pub fn default() -> MQCD {
        MQCD {
            ChannelName: [0; 20],
            Version: MQCD_CURRENT_VERSION as i32,
            ChannelType: MQCHT_SENDER as i32,
            TransportType: MQXPT_LU62 as i32,
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
            HeartbeatInterval: 300,
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

impl MQCSP {
    pub fn default() -> MQCSP {
        MQCSP {
            StrucId: ['C' as i8, 'S' as i8, 'P' as i8, ' ' as i8],
            Version: MQCSP_VERSION_1 as i32,
            AuthenticationType: MQCSP_AUTH_USER_ID_AND_PWD as i32,
            Reserved1: [0; 4],
            CSPUserIdPtr: null_mut(),
            CSPUserIdOffset: 0,
            CSPUserIdLength: 0,
            Reserved2: [0; 8],
            CSPPasswordPtr: null_mut(),
            CSPPasswordOffset: 0,
            CSPPasswordLength: 0,
        }
    }
}

impl MQCNO {
    pub fn default() -> MQCNO {
        MQCNO {
            StrucId: ['C' as i8, 'N' as i8, 'O' as i8, ' ' as i8],
            Version: MQCNO_CURRENT_VERSION as i32,
            Options: 0,
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

impl MQSCO {
    pub fn default() -> MQSCO {
        MQSCO {
            StrucId: ['S' as i8,'C' as i8,'O' as i8,' ' as i8],
            Version: MQSCO_VERSION_1 as i32,
            KeyRepository: [0; 256],
            CryptoHardware: [0; 256],
            AuthInfoRecCount: 0,
            AuthInfoRecOffset: 0,
            AuthInfoRecPtr: null_mut(),
            KeyResetCount: MQSCO_RESET_COUNT_DEFAULT as i32,
            FipsRequired: MQSSL_FIPS_NO as i32,
            EncryptionPolicySuiteB: [
                MQ_SUITE_B_NONE as i32,
                MQ_SUITE_B_NOT_AVAILABLE as i32,
                MQ_SUITE_B_NOT_AVAILABLE as i32,
                MQ_SUITE_B_NOT_AVAILABLE as i32,
            ],
            CertificateValPolicy: MQ_CERT_VAL_POLICY_DEFAULT as i32,
            CertificateLabel: [0; 64],
        }
    }
}

impl MQAIR {
    pub fn default() -> MQAIR {
        MQAIR {
            StrucId: ['A' as i8,'I' as i8,'R' as i8,' ' as i8],
            Version: MQAIR_VERSION_1 as i32,
            AuthInfoType: MQAIT_CRL_LDAP as i32,
            AuthInfoConnName: [0; 264],
            LDAPUserNamePtr: null_mut(),
            LDAPUserNameOffset: 0,
            LDAPUserNameLength: 0,
            LDAPPassword: [0; 32],
            OCSPResponderURL: [0; 256],
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
            VSCCSID: MQCCSI_APPL as i32,
        }
    }
}

impl MQOD {
    pub fn default() -> MQOD {
        let mut dqn = [0 as MQCHAR; 48];
        let dyn_q_name = b"AMQ.*";

        for i in 0..dyn_q_name.len() {
            dqn[i] = dyn_q_name[i] as i8;
        }

        MQOD {
            StrucId: ['O' as i8,'D' as i8,' ' as i8,' ' as i8],
            Version: MQOD_VERSION_1 as i32,
            ObjectType: MQOT_Q as i32,
            ObjectName: [0; 48],
            ObjectQMgrName: [0; 48],
            DynamicQName: dqn,
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

impl MQPMO {
    pub fn default() -> MQPMO {
        MQPMO {
            StrucId: [ 'P' as i8, 'M' as i8, 'O' as i8, ' ' as i8],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mq = MqConfig::load_config("config/application.conf");

        assert_eq!(mq.mq.channel_name, "channel_ukofl");
        assert_eq!(mq.mq.server_port, 1414);
        assert_eq!(mq.mq.user_id, Some(String::from("mqm")));
    }

    #[test]
    fn test_MQOD() {
        let mqod = MQOD::default();

        assert_eq!(mqod.Version, MQOD_VERSION_1 as i32);
    }
}