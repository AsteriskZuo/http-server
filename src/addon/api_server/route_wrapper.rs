#![allow(unused_assignments)]

use std::error::Error;
use std::ffi::CString;
use std::fmt::Display;
use std::os::raw::{c_char, c_uchar, c_uint};

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/route.rs"));

type RouteErrorCode = i32;

#[derive(Debug)]
pub struct RouteError {
  pub code: RouteErrorCode,
}

impl Error for RouteError {}

impl Display for RouteError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.code)
  }
}

#[derive(Debug)]
pub struct RouteWrapper {}

impl RouteWrapper {
  pub fn init(rout_config_path: String) -> i32 {
    let c_rout_config_path = CString::new(rout_config_path);
    let mut ret = 0;
    unsafe {
      let tmp = c_rout_config_path.expect("c_rout_config_path");
      ret = init(tmp.as_ptr());
    }
    return ret;
  }
  pub fn find_path(condition: Vec<u8>) -> Result<(String, String), RouteError> {
    // let ss = condition.as_ptr();
    // let sss = condition.len();

    // let condition = CString::new(condition).expect("condition");
    // let condition_size: c_uint = 3;
    let format: c_uint = 1;
    let mut ret = 0;
    let mut id: String = String::new();
    let mut result = String::new();
    unsafe {
      let mut size = std::mem::zeroed::<c_uint>();
      let mut id_size = std::mem::zeroed::<c_uint>();
      let mut test_unsafe_result: *mut c_uchar = std::mem::zeroed();
      std::mem::forget(test_unsafe_result);
      let mut test_unsafe_id: *mut c_char = std::mem::zeroed();
      std::mem::forget(test_unsafe_id);

      ret = findPath(
        condition.as_ptr() as *const u8,
        condition.len() as u32,
        &mut test_unsafe_result as *mut *mut c_uchar,
        &mut size as *mut _,
        &mut test_unsafe_id as *mut *mut c_char,
        &mut id_size as *mut _,
        format,
      );
      if 0 != ret {
        return Err(RouteError { code: ret as i32 });
      }
      id = String::from_raw_parts(
        test_unsafe_id as *mut u8,
        id_size as usize,
        id_size as usize,
      );
      result = String::from_raw_parts(test_unsafe_result as *mut u8, size as usize, size as usize);
    }
    println!("find_path->{}", ret);
    return Ok((id, result));
  }
}

#[cfg(test)]
pub mod tests {
  #[test]
  fn test_init() {
    use super::*;
    let ret = RouteWrapper::init(String::from("routinglib"));
    println!("test_init->{}", ret);
    assert_eq!(0, ret);
  }
  #[test]
  fn test_find_path() {
    use super::*;
    let condition = Vec::<u8>::new();
    let ret = RouteWrapper::find_path(condition);
    println!("test_find_path->{:#?}", ret);
  }
}
