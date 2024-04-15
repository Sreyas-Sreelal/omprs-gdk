#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct StaticArray<T,const SIZE:usize> {
    elements:[T;SIZE]
}