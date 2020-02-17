#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod mqconfig;
extern crate libc;

use std::ptr::{null_mut};
use std::ffi::{CStr};
use std::sync::{Arc, Mutex};
use std::default::Default;
use ::std::os::raw::c_char;
use mqconfig::*;


#[derive(Default, Clone)]
pub struct MqWriter {

    pub con: MQHCONN,
    pub obj: MQHOBJ,
}

impl MqWriter {
    pub fn connect(&self, config: &MqConfig) -> MQHCONN {
//        self.con.

        0
    }
}

pub fn send_message(_message: String) {
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

    let cd: MQCD = MQCD::default();

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
