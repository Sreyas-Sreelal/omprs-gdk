#[repr(C)]
#[derive(Default)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
#[repr(C)]
union ColourData {
    rgba: std::mem::ManuallyDrop<Rgba>,
}

impl Default for ColourData {
    fn default() -> Self {
        ColourData {
        rgba: std::mem::ManuallyDrop::new(Rgba::default())
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct Colour {
    data: ColourData,
}

impl Colour {
    pub fn from_rgba(from: u32) -> Colour {
        Colour {
            data: ColourData {
                rgba: std::mem::ManuallyDrop::new(Rgba {
                    r: ((from & 0xFF000000) >> 24) as u8,
                    g: ((from & 0x00FF0000) >> 16) as u8,
                    b: ((from & 0x0000FF00) >> 8) as u8,
                    a: (from & 0x000000FF) as u8,
                }),
            },
        }
    }
}
