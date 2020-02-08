#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ptr::{null_mut};
use std::ffi::{CString, CStr};
use ::std::os::raw::c_char;

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn mq_cd_default() -> MQCD {
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

pub fn send_message(message: String) {
    let mut user_id = CStr::from_bytes_with_nul(b"admin\0")
        .unwrap().as_ptr();

    let mut password = CStr::from_bytes_with_nul(b"123456\0")
        .unwrap().as_ptr();

    let mut csp: MQCSP = MQCSP {
        StrucId: ['C' as i8, 'S' as i8, 'P' as i8, ' ' as i8],
        Version: MQCSP_VERSION_1 as i32,
        AuthenticationType: MQCSP_AUTH_USER_ID_AND_PWD as i32,
        Reserved1: [0; 4],
        CSPUserIdPtr: &mut user_id as * mut _ as *mut ::std::os::raw::c_void,
        CSPUserIdOffset: 0,
        CSPUserIdLength: 3,
        Reserved2: [0; 8],
        CSPPasswordPtr: &mut password as * mut _ as *mut ::std::os::raw::c_void,
        CSPPasswordOffset: 0,
        CSPPasswordLength: 6,
    };

//    let cd: MQCD = MQCD {
//
//    };

    let mut cno: MQCNO = MQCNO {
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
        SecurityParmsPtr: &mut csp,
        CCDTUrlPtr: null_mut(),
        CCDTUrlOffset: 0,
        CCDTUrlLength: 0,
        Reserved: [0; 8],
        ApplName: [0; 28],
        Reserved2: [0; 4]
    };

    let mut hcon: MQHCONN = 0;
    let mut comp_code: MQLONG = 0;
    let mut reason: MQLONG = 0;
    let mut name: [c_char; 50] = [0; 50];

    unsafe {
        libc::strcpy(name.as_mut_ptr(),
                     CStr::from_bytes_with_nul(b"MGR\0").unwrap().as_ptr());

        MQCONNX(
            name.as_mut_ptr(),
            &mut cno,
            &mut hcon,
            &mut comp_code,
            &mut reason
        );
    }

    match comp_code as u32 {
        MQCC_FAILED => println!("MQCONNX failed with reason code: {}", reason),
        MQCC_WARNING => println!("MQCONNX generated a warning with reason code: {}", reason),
        _ => println!("Unknown state: {}", reason),
    }

    println!("Connected");
}

#[cfg(test)]
mod tests {
    use super::send_message;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn send_msg() {
        send_message("Hello world".to_string());
    }
}
