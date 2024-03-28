use std::num::TryFromIntError;

pub fn u32_i32(x: u32) -> Result<i32, TryFromIntError> {
  x.try_into()
}

pub fn u64_u32(x: u64) -> Result<u32, TryFromIntError> {
  x.try_into()
}

pub fn i32_u32(x: i32) -> Result<u32, TryFromIntError> {
  x.try_into()
}

pub fn i32_usize(x: i32) -> Result<usize, TryFromIntError> {
  x.try_into()
}

pub fn f32_i32(x: f32) -> i32 {
  x.round() as i32
}
