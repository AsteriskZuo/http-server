use std::ffi::{CStr, CString};
use std::os::raw;
use std::os::raw::{c_char, c_int, c_uchar, c_uint};

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

#[derive(Debug, Default, Clone)]
pub struct RouteWrapper {}

impl RouteWrapper {
  pub fn init() -> i32 {
    todo!()
  }
  pub fn find_path() {}
}
