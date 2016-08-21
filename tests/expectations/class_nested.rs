/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct A {
    pub member_a: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct A_B {
    pub member_b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_B() {
    assert_eq!(::std::mem::size_of::<A_B>() , 4usize);
    assert_eq!(::std::mem::align_of::<A_B>() , 4usize);
}
impl Clone for A_B {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(::std::mem::size_of::<A>() , 4usize);
    assert_eq!(::std::mem::align_of::<A>() , 4usize);
}
impl Clone for A {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "var"]
    pub static mut var: A_B;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct D {
    pub member: A_B,
}
#[test]
fn bindgen_test_layout_D() {
    assert_eq!(::std::mem::size_of::<D>() , 4usize);
    assert_eq!(::std::mem::align_of::<D>() , 4usize);
}
impl Clone for D {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Templated<T> {
    pub member: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Templated_Templated_inner<T> {
    pub member_ptr: *mut T,
}
