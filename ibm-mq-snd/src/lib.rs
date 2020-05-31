#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ptr::{null_mut, null};
use std::ffi::CStr;
use ::std::os::raw::c_char;
use std::ffi::CString;
use std::option::Option;

mod mqdef;
mod generated;

use generated::*;
use mqdef::*;
use std::fs::copy;
use std::convert::Infallible;
use std::os::raw::c_void;
use std::collections::HashMap;

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
/// ```
/// //use ibm_mq_snd::IbmMqFactory;
///
/// // Create factory instance
/// use ibm_mq_snd::IbmMqFactory;
/// let mut sendFactory = IbmMqFactory::new(
///     "localhost".to_string(),
///     1414,
///     "DEV.APP.SVRCONN".to_string(),
///     "QM1".to_string(),
///     "DEV.QUEUE.1".to_string(),
/// );
/// // Connect to MQ manager
/// let _ = sendFactory.connect().unwrap();
/// // Open MQ queue
/// let _ = sendFactory.open_queue().unwrap();
/// // Send 100 messages with header: MsgType -> RustMessage
/// for x in 1..100 {
///     let _ = sendFactory.send_message_with_props(
///         format!("Test Message {}", x).to_string(),
///         "MsgType".to_string(),
///         "RustMessage".to_string()
///     ).unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct IbmMqFactory {
    hCon: MQHCONN,
    hObj: MQHOBJ,
    managerName: String,
    queueName: String,
    connectionName: String,
    channelName: String,
}

impl Drop for IbmMqFactory {
    /// Implemented destructor for IbmFactory struct
    /// Disconnect from MQ if it already connected
    fn drop(&mut self) {
        if self.hCon > 0 {
            self.close_queue();
            self.disconnect();
        }
    }
}

impl IbmMqFactory {
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

            println!("Successfully close queue: {}/{}, hObj: {}",
                     compCode,
                     cReason,
                     self.hObj);
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

            println!("Successfully disconnect: {}/{}, hcon: {}",
                     compCode,
                     cReason,
                     self.hCon);
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
    /// let mut factory = IbmMqFactory::new(
    ///     "localhost".to_string(),
    ///     1414,
    ///     "DEV.APP.SVRCONN".to_string(),
    ///     "QM1".to_string(),
    ///     "DEV.QUEUE.1".to_string(),
    /// );
    /// ```
    pub fn new(host: String,
               port: u16,
               channelName: String,
               managerName: String,
               queueName: String) -> Self {
        IbmMqFactory {
            hCon: 0,
            hObj: 0,
            managerName,
            queueName,
            channelName,
            connectionName: format!("{}({})", host, port),
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
    /// IbmMqFactory::string_copy("This is copy".to_string(), &mut res);
    /// println!("Result is: {:?}", res);
    /// ```
    pub fn string_copy(value: String, res: &mut [c_char]) -> *mut [c_char] {
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
            println!("{} ended with reason code {}/{}, hCon: {}",
                     function,
                     reason,
                     code,
                     self.hCon);

            Err(
                MqError::new_text(code,
                                  reason,
                                  format!("{} ended with reason code {}/{}",
                                          function,
                                          reason,
                                          code))
            )
        } else {
            // println!("{} ended with reason code {}/{}, hCon: {}",
            //          function,
            //          reason,
            //          code,
            //          self.hCon);

            Ok(result)
        }
    }

    /// Connect to MQ channel
    /// Function using connectionName and channelName from
    /// IbmMqFactory
    pub fn connect(&mut self) -> Result<bool, MqError> {
        let mut connectOptions = MQCNO::default();
        let mut clientConn = MQCD::default_client();
        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;

        let mut qmName: [i8; MQ_Q_MGR_NAME_LENGTH as usize] = [0; MQ_Q_MGR_NAME_LENGTH as usize];
        IbmMqFactory::string_copy(self.managerName.to_owned(), &mut qmName);
        IbmMqFactory::string_copy(self.connectionName.to_owned(), &mut clientConn.ConnectionName);
        IbmMqFactory::string_copy(self.channelName.to_owned(), &mut clientConn.ChannelName);

        connectOptions.ClientConnPtr = IbmMqFactory::make_void_ptr(&mut clientConn);
        connectOptions.Version = MQCNO_CURRENT_VERSION as i32;

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

        // od.ObjectType = MQOT_Q_MGR as i32;
        // od.ObjectQMgrName[0] = 0;
        IbmMqFactory::string_copy(self.managerName.to_owned(), &mut od.ObjectQMgrName);
        IbmMqFactory::string_copy(self.queueName.to_owned(), &mut od.ObjectName);

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
                println!("qmName {:?}", qmName.to_vec());
            }
        }

        self.make_result("MQOPEN".to_string(), openCode, cReason, true)
    }

    /// Function used to create callback for subscribe on MQ message
    /// # Arguments
    /// * closure - Closure will be called with parsed message
    ///
    /// # Result
    /// Tuple2 (callbackArea, callback)
    ///
    /// # Usage
    /// ```
    /// use ibm_mq_snd::{IbmMqFactory, MqMessage};
    /// let mut on_message = |msg: MqMessage| {
    ///     println!("Message: {:?}", msg);
    /// };
    /// let (closure, callback) = unsafe {IbmMqFactory::unpack_closure(& mut on_message)};
    /// // Next step pass it to IbmMqFactory::register_consumer(callback, closure);
    /// ```
    pub unsafe fn unpack_closure<F>(closure: &mut F) -> (*mut c_void, MqQueueSubscriber)
        where
            F: FnMut(MqMessage),
    {
        extern "C" fn trampoline<F>(hConn: MQHCONN,
                                    pMsgDesc: PMQVOID,
                                    pGetMsgOpts: PMQVOID,
                                    pBuffer: PMQVOID,
                                    pContext: PMQCBC)
            where
                F: FnMut(MqMessage),
        {
            unsafe {
                let gmo = *(pGetMsgOpts as *const MQGMO);
                let md = *(pMsgDesc as *const MQMD);
                let cbc = *(pContext as *mut MQCBC);
                let rfc = *(pBuffer as *const MQRFH2);
                let structSize = std::mem::size_of::<MQRFH2>();
                let callback: &mut F = &mut *(cbc.CallbackArea as *mut F);

                // Start offset of read buffer
                let mut offset = pBuffer as u64;
                let mut inc = structSize as u64;
                let mut headers = HashMap::new();
                while inc < rfc.StrucLength as u64 {
                    // Read NameValueLength
                    let size_offset = offset + inc;
                    let size = *(size_offset as *const MQLONG);
                    // Read NameValueData
                    let data_offset = size_offset + std::mem::size_of::<MQLONG>() as u64;
                    let xml_data = std::slice::from_raw_parts(data_offset as *const i8, size as usize);
                    // Read NameValueData XML
                    let xml = IbmMqFactory::char_to_string(xml_data.to_vec());
                    // Increment inc variable
                    inc += size as u64 + std::mem::size_of::<MQLONG>() as u64;
                    println!("XML: {}, inc: {}, length: {}", xml, inc, gmo.ReturnedLength);

                    headers.insert("XML".to_string(), xml);
                }

                // Read remains data as message body
                offset += inc;
                let body_data = std::slice::from_raw_parts(offset as *const i8, (gmo.ReturnedLength as u64 - inc) as usize);
                let body = IbmMqFactory::char_to_string(body_data.to_vec());
                println!("body: {}", body);

                (*callback)(MqMessage{
                    body,
                    headers,
                });
            }
        }

        (closure as *mut F as *mut c_void, trampoline::<F>)
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

    fn set_msg_prop(&self,
                    hMsg: MQHMSG,
                    header: (String, String)) -> Result<MQHMSG, MqError> {
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

    pub fn send_message_with_props(&self, text: String,
                                   headerName: String,
                                   headerValue: String) -> Result<bool, MqError> {

        self.create_msg_handle()
            .and_then(|hMsg|
                self.set_msg_prop(
                    hMsg,
                    (headerName, headerValue))
            ).and_then(|hMsg|
                self.set_msg_prop(
                    hMsg,
                    (
                        "Root.MQMD.Format".to_string(),
                        CStr::from_bytes_with_nul(MQFMT_STRING).unwrap().to_str().unwrap().to_string()
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

        IbmMqFactory::string_copy(CStr::from_bytes_with_nul(MQFMT_STRING).unwrap().to_str().unwrap().to_string(),
                                  &mut md.Format);
        IbmMqFactory::string_copy(
            text,
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

    pub fn get_message(&self) -> Result<bool, MqError> {
        let mut gmo = MQGMO::default();
        let mut md = MQMD::default();

        let mut compCode: MQLONG = 0;
        let mut cReason: MQLONG = 0;
        let mut buffer: [i8; 100] = [0; 100];
        let mut bufLen: MQLONG = 0;

        gmo.Options = MQGMO_NO_SYNCPOINT as i32;
        gmo.Options |= MQGMO_NO_WAIT as i32;

        unsafe {
            MQGET(self.hCon,
                  self.hObj,
                  IbmMqFactory::make_void_ptr(&mut md),
                  IbmMqFactory::make_void_ptr(&mut gmo),
                  100,
                  IbmMqFactory::make_void_ptr(&mut buffer),
                  &mut bufLen,
                  &mut compCode,
                  &mut cReason,
            )
        }

        if compCode != MQCC_FAILED as i32 {
            // let s = IbmMqFactory::char_to_string(buffer.to_vec());
            println!("MD: {}, {}, {}",
                IbmMqFactory::char_to_string(md.ApplOriginData.to_vec()),
                IbmMqFactory::char_to_string(md.PutDate.to_vec()),
                md.MsgFlags
            );

            println!("GMO: {}",
                gmo.MatchOptions
            );

            println!("Buffer length: {}/ msg: {:?}", bufLen, buffer.to_vec());
        }

        println!("get message code: {}/{}", cReason, compCode);

        self.make_result("MQGET".to_string(), compCode, cReason, true)
    }

    fn char_to_string(buffer: Vec<i8>) -> String {
        match String::from_utf8(buffer.iter().map(|&c| c as u8).collect()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IbmMqFactory;
    use std::{thread, time};
    use crate::{MqMessage, MqMessageCallback, MqQueueSubscriber};
    use std::ffi::c_void;
    use super::generated::*;

    #[test]
    fn connect_test() {
        let mut factory = IbmMqFactory::new(
            "localhost".to_string(),
            1414,
            "DEV.APP.SVRCONN".to_string(),
            "QM1".to_string(),
            "DEV.QUEUE.2".to_string(),
        );

        let _ = factory.connect().unwrap();
        let _ = factory.open_queue().unwrap();

        let mut cb = |msg: MqMessage| {
            println!("On message: {:?}", msg);
        };

        let (closure, callback) = unsafe {
            IbmMqFactory::unpack_closure(&mut cb)
        };

        let _ = factory.register_consumer(callback, closure).unwrap();
        let _ = factory.start_consumption().unwrap();

        // Put messages to MQ in separate thread
        thread::spawn(|| {
            println!("Hello from thread: {:?}", thread::current().id());

            let mut sendFactory = IbmMqFactory::new(
                "localhost".to_string(),
                1414,
                "DEV.APP.SVRCONN".to_string(),
                "QM1".to_string(),
                "DEV.QUEUE.2".to_string(),
            );

            let _ = sendFactory.connect().unwrap();
            let _ = sendFactory.open_queue().unwrap();

            for x in 1..5 {
                let _ = sendFactory.send_message_with_props(
                    format!("Send mega long message from the createst hot peper mint closure trusted bank {}", x).to_string(),
                    "LongTermHeader".to_string(),
                    "HelloWorldHeader value".to_string()
                ).unwrap();
            }
        });

        let ten_millis = time::Duration::from_millis(1000);
        let now = time::Instant::now();
        thread::sleep(ten_millis);

        let _ = factory.stop_consumption().unwrap();

        // for x in 1..100 {
        //     let _ = factory.get_message();
        // }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn send_msg() {}
}
