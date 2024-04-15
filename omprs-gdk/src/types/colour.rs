
#[repr(C)]
struct RGBA{
    r:u8,
    g:u8,
    b:u8,
    a:u8,
}
#[repr(C)]
union  ColourData{
    rgba:std::mem::ManuallyDrop<RGBA>
}
#[repr(C)]
pub struct Colour {
    data:ColourData 
}