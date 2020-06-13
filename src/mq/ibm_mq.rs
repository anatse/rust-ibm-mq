#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::ptr::{null_mut, null};
use std::ffi::CStr;
use ::std::os::raw::c_char;
use std::ffi::CString;
use std::option::Option;

use std::fs::copy;
use std::convert::Infallible;
use std::os::raw::c_void;
use std::collections::HashMap;

use log::{debug, info, warn, error, log_enabled};

use quick_xml::Reader;
use quick_xml::events::Event;
use crate::config::config::MqConfig;
use crate::mq::mqdef::*;
use crate::mq::generated::*;
use std::sync::Arc;
use std::future::Future;

extern crate libc;

#[derive(Debug)]
pub struct MqError {
    pub compCode: MQLONG,
    pub cReason: MQLONG,
    pub text: Option<String>,
}

impl MqError {
    pub fn new_text(compCode: MQLONG, cReason: MQLONG, text: String) -> Self {
        MqError {
            compCode,
            cReason,
            text: Some(text),
        }
    }

    pub fn new(compCode: MQLONG, cReason: MQLONG) -> Self {
        MqError {
            compCode,
            cReason,
            text: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MqMessage {
    pub body: String,
    pub headers: HashMap<String, String>,
}

pub type MqMessageCallback = fn(message: MqMessage) -> ();
pub type MqQueueSubscriber = extern "C" fn(
                                            Hconn: MQHCONN,
                                            pMsgDesc: PMQVOID,
                                            pGetMsgOpts: PMQVOID,
                                            pBuffer: PMQVOID,
                                            pContext: PMQCBC
                                        );

/// Struct to work with MQ. This is wrapper for standart "C" IBM MQ library
///
/// # Example usage for send messages
///
/// ```
/// // Create factory instance
/// use ibm_mq_snd::IbmMqFactory;
/// use ibm_mq_snd::mqconfig::MqConfig;
///
/// let config = MqConfig::load_config("config/application.conf");
/// let mut sendFactory = IbmMqFactory::new(&config);
/// // Connect to MQ manager
/// let _ = sendFactory.connect().unwrap();
/// // Open MQ queue
/// let _ = sendFactory.open_queue().unwrap();
/// // Send 100 messages with header: MsgType -> RustMessage
/// for x in 1..100 {
///     let _ = sendFactory.send_message_with_props(
///         format!("Test Message {}", x).to_string(),
///         &[("MsgType".to_string(), "RustMessage".to_string())].iter().cloned().collect()
///     ).unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct IbmMqFactory {
    hCon: MQHCONN,
    hObj: MQHOBJ,
    config: Arc<MqConfig>,
}

impl Drop for IbmMqFactory {
    /// Implemented destructor for IbmFactory struct
    /// Disconnect from MQ if it already connected
    fn drop(&mut self) {
        debug!("Call drop for IbmMqFactory");

        if self.hCon > 0 {
            let _ = self.stop_consumption().unwrap();
            self.close_queue();
            self.disconnect();
        }

        debug!("disconnected");
    }
}

impl IbmMqFactory {
    pub fn config(&self) -> &Arc<MqConfig> {
        &self.config
    }

    /// Function closes opening queues
    fn close_queue(&mut self) {
        if self.hObj > 0 {
            let mut compCode: MQLONG = 0;
            let mut cReason: MQLONG = 0;

            unsafe {
                MQCLOSE(self.hCon,
                        &mut self.hObj,
                        MQCO_NONE as i32,
                        &mut compCode,
                        &mut cReason);
            }

            if log_enabled!(log::Level::Debug) {
                debug!("Successfully close queue: {}/{}, hObj: {}",
                       compCode,
                       cReason,
                       self.hObj);
            }
        }
    }

    /// Function closes MQ connection
    fn disconnect(&mut self) {
        if self.hCon > 0 {
            let mut compCode: MQLONG = 0;
            let mut cReason: MQLONG = 0;
            unsafe {
                MQDISC(
                    &mut self.hCon,
                    &mut compCode,
                    &mut cReason,
                );
            }

            if log_enabled!(log::Level::Debug) {
                debug!("Successfully disconnect: {}/{}, hcon: {}",
                       compCode,
                       cReason,
                       self.hCon);
            }
        }
    }

    /// Creates new IbmMqFactory object
    /// # Arguments
    ///
    /// * host - hostname for MQ manager
    /// * port - port
    /// * channelName - channel name
    /// * managerName - manager name
    /// * queueName - queue name
    ///
    /// # Example
    ///
    /// ```
    /// use ibm_mq_snd::IbmMqFactory;
    /// use ibm_mq_snd::mqconfig::MqConfig;
    ///
    /// let config = MqConfig::load_config("config/application.conf");
    /// let mut factory = IbmMqFactory::new(&config);
    /// ```
    pub fn new(config: Arc<MqConfig>) -> Self {
        IbmMqFactory {
            hCon: 0,
            hObj: 0,
            config,
        }
    }

    /// Function copies string to char array
    /// # Arguments
    /// * value - string to copy from
    /// * res - array of chars to copy to
    ///
    /// # Example
    ///
    /// This example prints array of bytes contains words `this is copy`
    /// ```
    /// use ibm_mq_snd::IbmMqFactory;
    /// let mut res:[i8; 20] = [0; 20];
    /// IbmMqFactory::string_copy(&"This is copy".to_string(), &mut res);
    /// println!("Result is: {:?}", res);
    /// ```
    pub fn string_copy(value: &String, res: &mut [c_char]) -> *mut [c_char] {
        let mut index = 0;
        value.bytes().for_each(|ch| {
            res[index] = ch as i8;
            index += 1;
        });

        if res.len() > index {
            res[index] = 0;
        }
        res
    }

    #[inline(always)]
    fn make_void_ptr<T>(v: &mut T) -> *mut ::std::os::raw::c_void {
        v as *mut _ as *mut ::std::os::raw::c_void
    }

    #[inline]
    fn make_result<T>(&self, function: String, code: MQLONG, reason: MQLONG, result: T) -> Result<T, MqError> {
        if code == MQCC_FAILED as i32 || code == MQCC_WARNING as i32 {
            if reason != MQRC_NO_MSG_AVAILABLE as i32 && log_enabled!(log::Level::Debug) {
                debug!("{} ended with reason code {}/{}, hCon: {}",
                       function,
                       reason,
                       code,
                       self.hCon);
            }

            Err(
                MqError::new_text(code,
                                  reason,
                                  format!("{} ended with reason code {}/{}",
                                          function,
                                          reason,
                                          code))
            )
        } else {
            if log_enabled!(log::Level::Debug) {
                debug!("{} ended with reason code {}/{}, hCon: {}",
                       function,
                       reason,
                       code,
                       self.hCon);
            }

            Ok(result)
        }
    }

    /// Function initialize SSL parameters fro MQ connection
    /// # Arguments
    /// * cno - MQ structure to provide connection parameters
    /// * sslConnOptions - MQ structure to provide SSL configuration parameters
    /// * authRec - MQ structure to provide authentication parameters
    fn init_ssl(&self, cno: &mut MQCNO, sslConnOptions: &mut MQSCO, authRec: &mut MQAIR) {
        self.config.ssl_key_repos_stem.to_owned().map(|repos| {
            debug!("Using SSL key repository stem {}", repos);
            IbmMqFactory::string_copy(&repos, &mut sslConnOptions.KeyRepository);
            if self.config.fips_required {
                debug!("FIPS required");
                sslConnOptions.FipsRequired = MQSSL_FIPS_YES as i32;
                sslConnOptions.Version = MQSCO_VERSION_2 as i32;
            }

            if self.config.suite_b.len() > 0 {
                if log_enabled!(log::Level::Debug) {
                    debug!("Suite B Encryption Policy {:?}", &self.config.suite_b);
                }

                let mut index = 0;
                for suite in self.config.suite_b.to_owned() {
                    sslConnOptions.EncryptionPolicySuiteB[index] = suite;

                    index += 1;
                    if index == 4 {
                        break;
                    }
                }

                sslConnOptions.Version = MQSCO_VERSION_3 as i32;
            }

            self.config.cert_val_policy.to_owned().map(|policy| {
                if log_enabled!(log::Level::Debug) {
                    debug!("Certificate Validation Policy: {}", &policy);
                }
                sslConnOptions.CertificateValPolicy = policy;
                sslConnOptions.Version = MQSCO_VERSION_4 as i32;
            });

            self.config.certificate_label.to_owned().map(|label| {
                if log_enabled!(log::Level::Debug) {
                    debug!("Using certificate label: {}", &label);
                }
                IbmMqFactory::string_copy(&label, &mut sslConnOptions.CertificateLabel);
                sslConnOptions.Version = MQSCO_VERSION_5 as i32;
            });

            self.config.oscp_url.to_owned().map(|url| {
                if log_enabled!(log::Level::Debug) {
                    debug!("Using OCSP responder URL {}", url);
                }
                authRec.Version = MQAIR_VERSION_2 as i32;
                authRec.AuthInfoType = MQAIT_OCSP as i32;
                IbmMqFactory::string_copy(&url, &mut authRec.OCSPResponderURL);
                sslConnOptions.AuthInfoRecCount = 1;
                sslConnOptions.AuthInfoRecPtr = authRec;
            });

            cno.Version = MQCNO_VERSION_4 as i32;
            cno.SSLConfigPtr = sslConnOptions;
        });
    }

    /// Connect to MQ channel
    /// Function using connectionName and channelName from IbmMqFactory.
    /// Function using MQCONNX with MQCNO_HANDLE_SHARE_BLOCK option. This option allow to use several
    /// connections in one thread and allow to share connection handles between threads.
    ///
    /// ## See also
    /// * https://www.ibm.com/support/knowledgecenter/SSFKSJ_9.1.0/com.ibm.mq.dev.doc/q025940_.htm
    pub fn connect(&mut self) -> Result<bool, MqError> {
        let mut connectOptions = MQCNO::default();
        let mut clientConn = MQCD::default_client();
        let mut sslConnOptions = MQSCO::default();
        let mut authRec = MQAIR::default();
        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;
        let mut qmName: [i8; MQ_Q_MGR_NAME_LENGTH as usize] = [0; MQ_Q_MGR_NAME_LENGTH as usize];

        IbmMqFactory::string_copy(&self.config.queue_manager.to_owned().unwrap(), &mut qmName);
        let address = format!("{}({})",
                                          self.config.server_name.to_owned().unwrap(),
                                          self.config.server_port.to_owned().unwrap());
        IbmMqFactory::string_copy(&address, &mut clientConn.ConnectionName);
        IbmMqFactory::string_copy(&self.config.channel_name.to_owned().unwrap(), &mut clientConn.ChannelName);

        // Instead of using match {None => (), Some(xxx) => {}} we are use if let construction
        if let Some(cipher) = self.config.cipher_spec.to_owned() {
            clientConn.Version = MQCD_VERSION_7 as i32;
            IbmMqFactory::string_copy(&cipher, &mut clientConn.SSLCipherSpec);
            if log_enabled!(log::Level::Debug) {
                debug!("Using SSL CipherSpec {}", cipher);
            }
        }

        self.init_ssl(&mut connectOptions, &mut sslConnOptions, &mut authRec);

        let mut csp = MQCSP::default();

        // User information
        if let Some(usr) = self.config.user_id.to_owned() {
            let mut user = usr.clone();
            connectOptions.Version = MQCNO_VERSION_5 as i32;
            csp.AuthenticationType = MQCSP_AUTH_USER_ID_AND_PWD as i32;
            csp.CSPUserIdPtr = IbmMqFactory::make_void_ptr(&mut user);
            csp.CSPUserIdLength = user.len() as i32;
            connectOptions.SecurityParmsPtr = &mut csp;
        }

        if let Some(pwd) = self.config.password.to_owned() {
            let mut p = pwd;
            csp.CSPPasswordPtr = IbmMqFactory::make_void_ptr(&mut p);
            csp.CSPPasswordLength = p.len() as i32;
        }

        connectOptions.ClientConnPtr = IbmMqFactory::make_void_ptr(&mut clientConn);
        connectOptions.Version = MQCNO_CURRENT_VERSION as i32;
        connectOptions.Options |= MQCNO_HANDLE_SHARE_BLOCK as i32;
        // connectOptions.Options |= MQCNO_HANDLE_SHARE_NONE as i32;
        connectOptions.Options |= MQCNO_ALL_CONVS_SHARE as i32;

        unsafe {
            MQCONNX(
                qmName.as_mut_ptr(),
                &mut connectOptions,
                &mut self.hCon,
                &mut compCode,
                &mut cReason,
            );
        }

        self.make_result("MQCONNX".to_string(), compCode, cReason, true)
    }

    /// Function opens queue with name IbmMqFactory::queueName
    /// for input and output
    pub fn open_queue(&mut self) -> Result<bool, MqError> {
        let mut od = MQOD::default();
        let mut openCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;

        IbmMqFactory::string_copy(&self.config.queue_manager.to_owned().unwrap(), &mut od.ObjectQMgrName);
        IbmMqFactory::string_copy(&self.config.target_queue.to_owned().unwrap(), &mut od.ObjectName);

        unsafe {
            MQOPEN(self.hCon,
                   IbmMqFactory::make_void_ptr(&mut od),
                   (MQOO_INPUT_AS_Q_DEF + MQOO_OUTPUT + MQOO_FAIL_IF_QUIESCING) as i32,
                   &mut self.hObj,
                   &mut openCode,
                   &mut cReason);
        }

        self.make_result("MQOPEN".to_string(), openCode, cReason, true)
    }

    /// Function used to inquiry information about defined MQ managers
    /// Manager name will be printed
    pub fn query_manager(&mut self) -> Result<bool, MqError> {
        let mut od = MQOD::default();
        let mut openCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;
        let mut selector: MQLONG = MQCA_Q_MGR_NAME as i32;
        let mut qmName: [i8; 48] = [0; 48];

        od.ObjectType = MQOT_Q_MGR as i32;
        od.ObjectQMgrName[0] = 0;

        unsafe {
            MQOPEN(self.hCon,
                   IbmMqFactory::make_void_ptr(&mut od),
                   (MQOO_INQUIRE + MQOO_FAIL_IF_QUIESCING) as i32,
                   &mut self.hObj,
                   &mut openCode,
                   &mut cReason);
        }

        if openCode != MQCC_FAILED as i32 {
            unsafe {
                MQINQ(self.hCon,
                      self.hObj,
                      1,
                      &mut selector,
                      0,
                      null_mut(),
                      MQ_Q_MGR_NAME_LENGTH as i32,
                      &mut qmName as *mut _ as PMQCHAR,
                      &mut openCode,
                      &mut cReason);
            }

            if openCode != MQCC_FAILED as i32 {
                error!("qmName {:?}", qmName.to_vec());
            }
        }

        self.make_result("MQOPEN".to_string(), openCode, cReason, true)
    }

    fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {
        let mut v = std::mem::ManuallyDrop::new(v);
        let p = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();
        unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
    }

    pub fn parse_xml(xml: &String) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        let mut reader = Reader::from_str(xml.as_str());
        reader.trim_text(true);

        // Temporary buffer
        let mut buf = Vec::new();
        let mut lastName = String::new();
        loop {
            // Parsing events (SAX)
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let headerName = std::str::from_utf8(e.name()).unwrap().to_string();
                    lastName = headerName.clone();
                },
                Ok(Event::Text(e)) => {
                    let text = e.unescape_and_decode(&reader).unwrap();
                    headers.insert(lastName.clone(), text);
                },
                Ok(Event::Eof) => break,
                Err(e) => error!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        headers
    }

    #[inline]
    pub fn convert_big_endian(value: u32) -> i32 {
        let leftmost_byte = ((value & 0x000000FF) >> 0) << 24;
        let left_middle_byle = ((value & 0x0000FF00) >> 8) << 16;
        let right_middle_byte = ((value & 0x00FF0000) >> 16) << 8;
        let rightmost_byte = (value & 0xFF000000) >> 24;

        // Result is the concatenation of all these values.
        (leftmost_byte | left_middle_byle | right_middle_byte | rightmost_byte) as i32
    }

    pub fn parse_message(&self, buffer: &Vec<i8>, buffer_length: usize) -> MqMessage {
        if log_enabled!(log::Level::Debug) {
            debug!("Parse message buffer length: {} of {}", buffer_length, buffer.len());
        }

        let structSize = std::mem::size_of::<MQRFH2>();
        let (is_big_endian, hdr) = unsafe {
            let hdr = *(buffer.as_ptr() as * const MQRFH2);
            let offs = buffer.as_ptr() as u64 + std::mem::size_of::<MQLONG>() as u64;
            (*(offs as *const i8) == 0, hdr)
        };

        debug!("Process numbers with converting to BigEndian");
        let hdrSize = if is_big_endian { IbmMqFactory::convert_big_endian(hdr.StrucLength as u32) } else { hdr.StrucLength };
        let mut offs = buffer.as_ptr() as u64;
        let mut inc = structSize as u64;

        let mut xml_headers= Vec::<String>::new();
        while inc < hdrSize as u64 {
            let size_offset = offs + inc;
            inc += std::mem::size_of::<MQLONG>() as u64;

            let (xml_data, size) = unsafe {
                let mut size = *(size_offset as *const MQLONG);
                size = if is_big_endian { IbmMqFactory::convert_big_endian(size as u32) as i32 } else { size };
                inc += size as u64;

                // Read NameValueData
                let data_offset = size_offset + std::mem::size_of::<MQLONG>() as u64;
                (std::slice::from_raw_parts(data_offset as *const i8, size as usize), size)
            };

            // Read NameValueData XML
            let xml = IbmMqFactory::char_to_string(xml_data.to_vec());
            // Increment inc variable
            // debug!("xml: {}", xml);
            xml_headers.push(xml);
        }

        let headers = xml_headers.iter()
            .map(|xml| IbmMqFactory::parse_xml(xml))
            .fold(HashMap::<String, String>::new(), |a, xml| a.into_iter().chain(xml).collect());

        // Read body
        offs += hdrSize as u64;
        let body_size = buffer_length - hdrSize as usize;
        if log_enabled!(log::Level::Debug) {
            debug!("body size: {}", body_size);
        }

        let body_data = unsafe {
            std::slice::from_raw_parts(offs as *const i8, body_size)
        };

        let body = IbmMqFactory::char_to_string(body_data.to_vec());

        if log_enabled!(log::Level::Debug) {
            debug!("Message: {} with headers {:?}", body, headers);
        }

        MqMessage {
            body,
            headers,
        }
    }

    /// Function used to register consumer using MQCB call
    /// messageConsumer used as callback function
    pub fn register_consumer(&self, callback: MqQueueSubscriber, callbackArea: *mut c_void) -> Result<bool, MqError> {
        let mut cbd = MQCBD::default();
        let mut gmo = MQGMO::default();
        let mut md = MQMD::default();

        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;

        cbd.CallbackType = MQCBT_MESSAGE_CONSUMER as i32;
        cbd.CallbackFunction = callback as MQPTR;
        cbd.CallbackArea = callbackArea;

        gmo.Options = MQGMO_NO_SYNCPOINT as i32;
        gmo.Options |= MQGMO_NO_WAIT as i32;

        unsafe {
            MQCB(self.hCon,
                 MQOP_REGISTER as i32,
                 IbmMqFactory::make_void_ptr(&mut cbd),
                 self.hObj,
                 IbmMqFactory::make_void_ptr(&mut md),
                 IbmMqFactory::make_void_ptr(&mut gmo),
                 &mut compCode,
                 &mut cReason);
        }

        self.make_result("MQCB".to_string(), compCode, cReason, true)
    }

    /// Function starts consumption MQ message.
    /// Messages will process by callback function registered using
    /// function _register_consumer_.
    pub fn start_consumption(&self) -> Result<bool, MqError> {
        let mut ctlo = MQCTLO::default();

        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;

        unsafe {
            MQCTL(self.hCon,
                  MQOP_START as i32,
                  IbmMqFactory::make_void_ptr(&mut ctlo),
                  &mut compCode,
                  &mut cReason);
        }

        self.make_result("MQCTL START".to_string(), compCode, cReason, true)
    }

    /// Function stops consumption MQ messages previously started
    /// using function _start_consumption_.
    pub fn stop_consumption(&self) -> Result<bool, MqError> {
        let mut ctlo = MQCTLO::default();
        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;

        unsafe {
            MQCTL(self.hCon,
                  MQOP_STOP as i32,
                  IbmMqFactory::make_void_ptr(&mut ctlo),
                  &mut compCode,
                  &mut cReason);
        }

        self.make_result("MQCTL STOP".to_string(), compCode, cReason, true)
    }

    /// Internal function used to create MQ message handler
    fn create_msg_handle(&self) -> Result<MQHMSG, MqError> {
        let mut hMsg: MQHMSG = 0;
        let mut cmho = MQCMHO::default();
        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;

        cmho.Options = MQCMHO_VALIDATE as i32;
        cmho.Version = MQCMHO_CURRENT_VERSION as i32;

        unsafe {
            MQCRTMH(self.hCon,
                    IbmMqFactory::make_void_ptr(&mut cmho),
                    &mut hMsg,
                    &mut compCode,
                    &mut cReason);
        }

        self.make_result(
            "MQCRTMH".to_string(),
            compCode,
            cReason,
            hMsg)
    }

    /// Function used to set message property
    /// # Arguments
    /// * hMsg - message handle, created by function _create_msg_handle_
    /// * header - tuple with property name and property value
    fn set_msg_prop(&self,
                    hMsg: MQHMSG,
                    header: (&String, &String)) -> Result<MQHMSG, MqError> {
        let mut smpo = MQSMPO::default();
        let mut pd = MQPD::default();
        let mut vs = MQCHARV::default();
        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;

        let mut headerN = header.0.clone();
        let mut headerV = header.1.clone();
        let hvLen = headerV.len();

        vs.VSPtr = headerN.as_mut_ptr() as * mut _ as * mut c_void;
        vs.VSLength = headerN.len() as i32;
        vs.VSOffset = 0;
        vs.VSBufSize = 0;
        vs.VSCCSID = MQCCSI_APPL;

        unsafe {
            MQSETMP(self.hCon,
                    hMsg,
                    IbmMqFactory::make_void_ptr(&mut smpo),
                    IbmMqFactory::make_void_ptr(&mut vs),
                    IbmMqFactory::make_void_ptr(&mut pd),
                    MQTYPE_STRING as i32,
                    hvLen as i32,
                    headerV.as_mut_ptr() as * mut _ as * mut c_void,
                    &mut compCode,
                    &mut cReason);
        }

        self.make_result(
            format!("MQSETMP {}", headerN).to_string(),
            compCode,
            cReason,
            hMsg)
    }

    /// Function close message handle
    fn close_msg_handle(&self, hMsg: MQHMSG) -> Result<bool, MqError> {
        let mut dmho = MQDMHO::default();
        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;
        let mut hMsgMut = hMsg;

        unsafe {
            MQDLTMH(self.hCon,
                    &mut hMsgMut,
                    IbmMqFactory::make_void_ptr(&mut dmho),
                    &mut compCode,
                    &mut cReason);
        }

        self.make_result(
            "MQDLTMH".to_string(),
            compCode,
            cReason,
            true)
    }

    /// Function send message with additional properties
    pub fn send_message_with_props(&self, text: String, headers: &HashMap<String, String>) -> Result<bool, MqError> {
        self.create_msg_handle()
            .and_then(|hMsg| {
                    for header in headers {
                        let res = self.set_msg_prop(hMsg, header);
                        if res.is_err() {
                            return res;
                        }
                    }

                    Ok(hMsg)
                }
            ).and_then(|hMsg|
                self.set_msg_prop(
                    hMsg,
                    (
                        & "Root.MQMD.Format".to_string(),
                        & CStr::from_bytes_with_nul(MQFMT_STRING).unwrap().to_str().unwrap().to_string()
                    ))
            ).and_then(|hMsg| {
                let mut md = MQMD::default();
                md.Persistence = MQPER_NOT_PERSISTENT as i32;

                let mut pmo = MQPMO::default();
                pmo.Version = MQPMO_CURRENT_VERSION as i32;
                pmo.Options = MQPMO_NO_SYNCPOINT as i32;
                pmo.OriginalMsgHandle = hMsg;
                pmo.NewMsgHandle = hMsg;

                let mut compCode: MQLONG = 0;
                let mut cReason: MQLONG = 0;

                let mut buffer = text.clone();
                let len = buffer.len() as i32;

                unsafe {
                    MQPUT(self.hCon,
                          self.hObj,
                          null_mut(),
                          IbmMqFactory::make_void_ptr(&mut pmo),
                          len,
                          buffer.as_mut_ptr() as *mut _ as *mut c_void,
                          &mut compCode,
                          &mut cReason);
                }

                self.make_result("MQPUT".to_string(), compCode, cReason, hMsg)
            }).and_then(|hMsg| self.close_msg_handle(hMsg)
        )
    }

    pub fn send_message(&self, text: String) -> Result<bool, MqError> {
        let mut md = MQMD::default();
        let mut pmo = MQPMO::default();
        let mut buffer: [c_char; 100] = [0; 100];
        let len = text.len() as i32;

        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;

        md.Persistence = MQPER_NOT_PERSISTENT as i32;

        IbmMqFactory::string_copy(&CStr::from_bytes_with_nul(MQFMT_STRING).unwrap().to_str().unwrap().to_string(),
                                  &mut md.Format);
        IbmMqFactory::string_copy(
            &text,
            &mut buffer,
        );

        pmo.Options |= MQPMO_NEW_MSG_ID as i32;
        pmo.Options |= MQPMO_NEW_CORREL_ID as i32;

        unsafe {
            MQPUT(self.hCon,
                  self.hObj,
                  IbmMqFactory::make_void_ptr(&mut md),
                  IbmMqFactory::make_void_ptr(&mut pmo),
                  len+1,
                  buffer.as_mut_ptr() as *mut _ as *mut c_void,
                  &mut compCode,
                  &mut cReason);
        }

        self.make_result("MQPUT".to_string(), compCode, cReason, true)
    }

    /// Get message function
    pub async fn get_message<F, Fut>(&self, callback: F) -> Result<bool, MqError>
    where F: Fn(MqMessage) -> Fut,
        Fut: Future<Output = bool>, {
        let mut gmo = MQGMO::default();
        let mut md = MQMD::default();

        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;
        let mut buffer_length: MQLONG = 0;
        let mut current_length: MQLONG = 8192; // 8k by default
        let mut buffer = vec![0; current_length as usize];

        gmo.Options = MQGMO_NO_SYNCPOINT as i32;
        gmo.Options |= MQGMO_NO_WAIT as i32;
        gmo.Options |= MQGMO_COMPLETE_MSG as i32;

        loop {
            unsafe {
                MQGET(self.hCon,
                      self.hObj,
                      IbmMqFactory::make_void_ptr(&mut md),
                      IbmMqFactory::make_void_ptr(&mut gmo),
                      current_length as i32,
                      buffer.as_mut_ptr() as *mut _ as *mut ::std::os::raw::c_void,
                      &mut buffer_length,
                      &mut compCode,
                      &mut cReason,
                )
            }

            match compCode as u32 {
                MQCC_OK => {
                    callback(self.parse_message(&buffer, buffer_length as usize)).await;
                    break;
                },
                MQCC_WARNING => {
                    match cReason as u32 {
                        MQRC_TRUNCATED_MSG_FAILED => {
                            // Message truncated
                            if log_enabled!(log::Level::Debug) {
                                debug!("Change buffer length to {}", buffer_length);
                            }
                            buffer = vec![0; buffer_length as usize];
                            current_length = buffer_length;
                            continue;
                        },
                        _ => {
                            error!("MD: {}, {}, {}\n{:?}",
                                   IbmMqFactory::char_to_string(md.ApplOriginData.to_vec()),
                                   IbmMqFactory::char_to_string(md.PutDate.to_vec()),
                                   md.MsgFlags,
                                   gmo.MatchOptions,
                            );

                            break;
                        },
                    }
                },
                MQRC_NO_MSG_AVAILABLE => {
                    break;
                },
                MQCC_FAILED => match cReason as u32 {
                    MQRC_NO_MSG_AVAILABLE => {
                        break;
                    },
                    _ => if log_enabled!(log::Level::Debug) {
                        debug!("get message code: {}/{}", cReason, compCode)
                    },
                },
                _ => if log_enabled!(log::Level::Debug) {
                    debug!("get message code: {}/{}", cReason, compCode)
                },
            }
        }

        self.make_result("MQGET".to_string(), compCode, cReason, true)
    }

    fn char_to_string(buffer: Vec<i8>) -> String {
        match String::from_utf8(buffer.iter().map(|&c| c as u8).collect()) {
            Ok(v) => v,
            Err(e) => {
                error!("Invalid UTF-8 sequence: {}", e);
                "".to_string()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IbmMqFactory;
    use std::{thread, time};
    use std::ffi::c_void;
    use log::{debug, info, warn, error};
    use crate::config::config::Config;
    use crate::mq::ibm_mq::MqMessage;
    use std::sync::Arc;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn send_msg() {}
}
