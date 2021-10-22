use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::os::raw;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_void};

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/route.rs"));

fn test_cpp_route_function() {
  unsafe {
    let mut path = String::from("../wrapper.h\0");
    let ret2 = init(path.as_ptr() as *const std::os::raw::c_char);
    println!("ret2={}", ret2);
    let mut path2 = std::ffi::CString::new("../wrapper.h").expect("sdsd");
    let ret2 = init(path2.as_ptr());

    let condition = CString::new("sdf").expect("sdf");
    let conditionSize: c_uint = 3;
    let result: *mut *mut c_uchar = std::mem::zeroed();
    let size: *mut c_uint = std::mem::zeroed();
    let id: *mut *mut c_char = std::mem::zeroed();
    let id_size: *mut c_uint = std::mem::zeroed();
    let format: c_uint = 1;
    let ret3: c_int = findPath(
      condition.as_ptr() as *const u8,
      conditionSize,
      result,
      size,
      id,
      id_size,
      format,
    );
    println!("{:?}", ret3);
  }
}

#[test]
fn test_c() {
  test_cpp_route_function();
}

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
  pub fn find_path(condition: String) -> Result<(String, String), RouteError> {
    let condition = CString::new(condition).expect("condition");
    let condition_size: c_uint = 3;
    let format: c_uint = 1;
    let mut ret = 0;
    let mut id: String;
    let mut result;
    unsafe {
      let mut unsafe_result: Vec<c_uchar> = vec![0];
      let mut size = std::mem::zeroed::<c_uint>();
      let mut unsafe_id: Vec<c_char> = vec![0];
      let mut id_size = std::mem::zeroed::<c_uint>();
      // let mut id_size_rust = 1;
      // id_size_rust = *id_size;
      // std::mem::drop(id_size);
      let mut result_step1 = unsafe_result.as_mut_ptr();
      let result_step2 = &mut result_step1 as *mut *mut c_uchar;
      ret = findPath(
        condition.as_ptr() as *const u8,
        condition_size,
        result_step2,
        &mut size as *mut _,
        &mut unsafe_id.as_mut_ptr() as *mut *mut c_char,
        &mut id_size as *mut _,
        format,
      );
      if 0 != ret {
        return Err(RouteError { code: ret as i32 });
      }
      //  let sdfsdf = unsafe_id.as_mut_ptr() as *mut u8;
      //  let length = unsafe_id.len();
      //  let output = String::from_raw_parts(sdfsdf, length, length);
      id = String::from_raw_parts(
        unsafe_id.as_mut_ptr() as *mut u8,
        unsafe_id.len(),
        unsafe_id.len(),
      );
      result = String::from_raw_parts(
        unsafe_result.as_mut_ptr() as *mut u8,
        unsafe_result.len(),
        unsafe_result.len(),
      );

      // libc::free(unsafe_id.as_mut_ptr() as *mut c_char as *mut c_void);
      // libc::free(unsafe_result.as_mut_ptr() as *mut c_uchar as *mut c_void);

      std::mem::drop(unsafe_id);
      std::mem::drop(unsafe_result);
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
    assert_eq!(2, ret);
  }
  #[test]
  fn test_find_path() {
    use super::*;
    let condition = String::new();
    let ret = RouteWrapper::find_path(condition);
    println!("test_find_path->{:#?}", ret);
  }
}
