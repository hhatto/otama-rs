#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::ffi::CString;

    #[test]
    fn test_otama_open() {
        let config = CString::new("path.yaml").unwrap();
        unsafe {
            let &mut o = &mut mem::zeroed();
            otama_open(&mut o as *mut *mut otama_t, config.as_ptr());
        }
    }
}
