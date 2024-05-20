use std::fmt::Debug;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, PartialEq)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
union ColourData {
    rgba: std::mem::ManuallyDrop<Rgba>,
}

impl Default for ColourData {
    fn default() -> Self {
        ColourData {
            rgba: std::mem::ManuallyDrop::new(Rgba::default()),
        }
    }
}

impl Debug for ColourData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rgba")
            .field("red", unsafe { &self.rgba.r })
            .field("blue", unsafe { &self.rgba.g })
            .field("green", unsafe { &self.rgba.b })
            .field("alpha", unsafe { &self.rgba.a })
            .finish()
    }
}

impl PartialEq for ColourData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.rgba == other.rgba }
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
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

    pub fn from_argb(from: u32) -> Colour {
        Colour {
            data: ColourData {
                rgba: std::mem::ManuallyDrop::new(Rgba {
                    a: ((from & 0xFF000000) >> 24) as u8,
                    r: ((from & 0x00FF0000) >> 16) as u8,
                    g: ((from & 0x0000FF00) >> 8) as u8,
                    b: (from & 0x000000FF) as u8,
                }),
            },
        }
    }

    pub fn rgba(&self) -> u32 {
        unsafe {
            (((self.data.rgba.r as u32) << 24) & 0xFF000000)
                | (((self.data.rgba.g as u32) << 16) & 0x00FF0000)
                | (((self.data.rgba.b as u32) << 8) & 0x0000FF00)
                | ((self.data.rgba.a as u32) & 0x000000FF)
        }
    }

    pub fn argb(&self) -> u32 {
        unsafe {
            (((self.data.rgba.a as u32) << 24) & 0xFF000000)
                | (((self.data.rgba.r as u32) << 16) & 0x00FF0000)
                | (((self.data.rgba.g as u32) << 8) & 0x0000FF00)
                | ((self.data.rgba.b as u32) & 0x000000FF)
        }
    }
}
