extern crate otama_sys;

use otama_sys::*;
use std::mem;
use std::ffi;

#[derive(Debug)]
pub struct Otama(*const otama_t);

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

    pub fn search(&mut self) -> Result<Status, Error> {
        /*
        unsafe {
            match otama_search_filee(self.0 as *mut _, ) {
        }
        */
        unimplemented!();
    }
}
