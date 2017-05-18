extern crate otama_sys;

use otama_sys::*;
use std::mem;
use std::ffi;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Otama(*const otama_t);

#[derive(Debug)]
pub struct OtamaResult {
    id: String,
    similarity: f32,
}

#[derive(Debug)]
pub enum Value {
    Null,
    Int(i32),
    Float(f32),
    String(String),
    Array(Vec<Value>),
    Hash(HashMap<String, Value>),
}

#[derive(Debug)]
pub enum Status {
    Ok,
    Ng,
}

#[derive(Debug)]
pub enum Error {
    Nodata,
    InvalidArguments,
    AssertionFailure,
    SysError,
    NotImplemented,
    End,
    Unknown,
}

impl Otama {
    pub fn new(config: &str) -> Result<Self, Error> {
        let c = ffi::CString::new(config).unwrap();
        unsafe {
            let mut o = mem::uninitialized();
            match otama_open(&mut o as *mut *mut otama_t, c.as_ptr()) {
                otama_status_t::OTAMA_STATUS_OK => { Ok(Otama(o as *const otama_t)) },
                _ => { Err(Error::Unknown) },
            }
        }
    }

    pub fn create_database(&mut self) -> Result<Status, Error> {
        unsafe {
            match otama_create_database(self.0 as *mut _) {
                otama_status_t::OTAMA_STATUS_OK => Ok(Status::Ok),
                otama_status_t::OTAMA_STATUS_NODATA => Err(Error::Nodata),
                otama_status_t::OTAMA_STATUS_INVALID_ARGUMENTS => Err(Error::InvalidArguments),
                otama_status_t::OTAMA_STATUS_ASSERTION_FAILURE => Err(Error::AssertionFailure),
                otama_status_t::OTAMA_STATUS_SYSERROR => Err(Error::SysError),
                otama_status_t::OTAMA_STATUS_NOT_IMPLEMENTED => Err(Error::NotImplemented),
                _ => Err(Error::Unknown),
            }
        }
    }

    pub fn insert(&mut self, file: &str) -> Result<String, Error> {
        let f = ffi::CString::new(file).unwrap();
        unsafe {
            let mut id = mem::uninitialized();
            let mut c_hexid: Vec<u8> = vec![0 as u8; 41];  // OTAMA_ID_HEXSTR_LEN
            match otama_insert_file(self.0 as *mut _, &mut id, f.as_ptr()) {
                otama_status_t::OTAMA_STATUS_OK => {
                    otama_id_bin2hexstr(c_hexid.as_mut_ptr() as *mut i8, &mut id);
                    c_hexid.truncate(40);
                    let s = ffi::CString::new(c_hexid).unwrap();
                    let hexid = String::from(s.to_str().unwrap());
                    Ok(hexid)
                },
                otama_status_t::OTAMA_STATUS_NODATA => Err(Error::Nodata),
                otama_status_t::OTAMA_STATUS_INVALID_ARGUMENTS => Err(Error::InvalidArguments),
                otama_status_t::OTAMA_STATUS_ASSERTION_FAILURE => Err(Error::AssertionFailure),
                otama_status_t::OTAMA_STATUS_SYSERROR => Err(Error::SysError),
                otama_status_t::OTAMA_STATUS_NOT_IMPLEMENTED => Err(Error::NotImplemented),
                _ => Err(Error::Unknown),
            }
        }
    }

    pub fn pull(&mut self) -> Result<Status, Error> {
        unsafe {
            match otama_pull(self.0 as *mut _) {
                otama_status_t::OTAMA_STATUS_OK => Ok(Status::Ok),
                otama_status_t::OTAMA_STATUS_NODATA => Err(Error::Nodata),
                otama_status_t::OTAMA_STATUS_INVALID_ARGUMENTS => Err(Error::InvalidArguments),
                otama_status_t::OTAMA_STATUS_ASSERTION_FAILURE => Err(Error::AssertionFailure),
                otama_status_t::OTAMA_STATUS_SYSERROR => Err(Error::SysError),
                otama_status_t::OTAMA_STATUS_NOT_IMPLEMENTED => Err(Error::NotImplemented),
                _ => Err(Error::Unknown),
            }
        }
    }

    pub fn search(&mut self, result_num: i32, file: &str) -> Result<Vec<OtamaResult>, Error> {
        let f = ffi::CString::new(file).unwrap();
        unsafe {
            let mut results = mem::uninitialized();
            let ret_code = match otama_search_file(self.0 as *mut _, &mut results as *mut *mut _, result_num, f.as_ptr()) {
                otama_status_t::OTAMA_STATUS_OK => {
                    let mut r: Vec<OtamaResult> = Vec::new();
                    let n = otama_result_count(results as *mut _);
                    for i in 0..n {
                        let mut c_hexid: Vec<u8> = vec![0 as u8; 41];  // OTAMA_ID_HEXSTR_LEN
                        let id = otama_result_id(results as *mut _, i);
                        otama_id_bin2hexstr(c_hexid.as_mut_ptr() as *mut i8, id);
                        c_hexid.truncate(40);
                        let s = ffi::CString::new(c_hexid).unwrap();
                        let hexid = String::from(s.to_str().unwrap());

                        let otama_result = otama_result_value(results as *mut _, i);
                        let similarity = match self.variant2obj(otama_result) {
                            Value::Hash(v) => {
                                let sim = v.get("similarity").unwrap();
                                match *sim {
                                    Value::Float(v) => v,
                                    _ => 0.0
                                }
                            },
                            _ => 0.0,
                        };
                        r.push(OtamaResult{
                            id: hexid,
                            similarity: similarity,
                        });
                    }
                    Ok(r)
                },
                otama_status_t::OTAMA_STATUS_NODATA => Err(Error::Nodata),
                otama_status_t::OTAMA_STATUS_INVALID_ARGUMENTS => Err(Error::InvalidArguments),
                otama_status_t::OTAMA_STATUS_ASSERTION_FAILURE => Err(Error::AssertionFailure),
                otama_status_t::OTAMA_STATUS_SYSERROR => Err(Error::SysError),
                otama_status_t::OTAMA_STATUS_NOT_IMPLEMENTED => Err(Error::NotImplemented),
                _ => Err(Error::Unknown),
            };
            otama_result_free(&mut results as *mut *mut _);
            ret_code
        }
    }

    fn variant2obj(&mut self, result: *mut otama_variant_t) -> Value {
        unsafe {
            match otama_variant_type(result) {
                otama_variant_type_e::OTAMA_VARIANT_TYPE_INT => {
                    let v = otama_variant_to_int(result);
                    Value::Int(v as i32)
                },
                otama_variant_type_e::OTAMA_VARIANT_TYPE_FLOAT => {
                    let v = otama_variant_to_float(result);
                    Value::Float(v as f32)
                },
                otama_variant_type_e::OTAMA_VARIANT_TYPE_STRING => {
                    let v = otama_variant_to_string(result);
                    let s = ffi::CString::from_raw(v as *mut _);
                    Value::String(String::from(s.to_str().unwrap()))
                },
                otama_variant_type_e::OTAMA_VARIANT_TYPE_ARRAY => {
                    let cnt = otama_variant_array_count(result);
                    let mut v: Vec<Value> = Vec::new();
                    for i in 0..cnt {
                        v.push(self.variant2obj(otama_variant_array_at(result, i)));
                    }
                    Value::Array(v)
                },
                otama_variant_type_e::OTAMA_VARIANT_TYPE_HASH => {
                    let keys = otama_variant_hash_keys(result);
                    let cnt = otama_variant_array_count(result);
                    let mut h: HashMap<String, Value> = HashMap::new();
                    for i in 0..cnt {
                        let v = self.variant2obj(otama_variant_hash_at2(result, otama_variant_array_at(keys, i)));
                        let c_k = otama_variant_to_string(otama_variant_array_at(keys, i));
                        let k = ffi::CString::from_raw(c_k as *mut _);
                        h.insert(String::from(k.to_str().unwrap()), v);
                    }
                    Value::Hash(h)
                },
                _ => Value::Null,
            }
        }
    }
}
