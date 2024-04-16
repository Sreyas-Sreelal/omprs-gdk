#[repr(C)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
#[repr(C)]
union ColourData {
    rgba: std::mem::ManuallyDrop<RGBA>,
}

#[repr(C)]
pub struct Colour {
    data: ColourData,
}

impl Colour {
    pub fn from_rgba(from: u32) -> Colour {
        Colour {
            data: ColourData {
                rgba: std::mem::ManuallyDrop::new(RGBA {
                    r: ((from & 0xFF000000) >> 24) as u8,
                    g: ((from & 0x00FF0000) >> 16) as u8,
                    b: ((from & 0x0000FF00) >> 8) as u8,
                    a: (from & 0x000000FF) as u8,
                }),
            },
        }
    }
}
