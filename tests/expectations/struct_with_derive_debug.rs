/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct LittleArray {
    pub a: [::std::os::raw::c_int; 32usize],
}
#[test]
fn bindgen_test_layout_LittleArray() {
    assert_eq!(::std::mem::size_of::<LittleArray>() , 128usize);
    assert_eq!(::std::mem::align_of::<LittleArray>() , 4usize);
}
impl Clone for LittleArray {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
pub struct BigArray {
    pub a: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_BigArray() {
    assert_eq!(::std::mem::size_of::<BigArray>() , 132usize);
    assert_eq!(::std::mem::align_of::<BigArray>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct WithLittleArray {
    pub a: LittleArray,
}
#[test]
fn bindgen_test_layout_WithLittleArray() {
    assert_eq!(::std::mem::size_of::<WithLittleArray>() , 128usize);
    assert_eq!(::std::mem::align_of::<WithLittleArray>() , 4usize);
}
impl Clone for WithLittleArray {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
pub struct WithBigArray {
    pub a: BigArray,
}
#[test]
fn bindgen_test_layout_WithBigArray() {
    assert_eq!(::std::mem::size_of::<WithBigArray>() , 132usize);
    assert_eq!(::std::mem::align_of::<WithBigArray>() , 4usize);
}
