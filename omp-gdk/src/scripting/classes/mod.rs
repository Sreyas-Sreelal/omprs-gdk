pub mod events;
pub mod functions;

use std::{ffi::c_void, mem::transmute};

use crate::{players::WeaponSlots, types::vector::Vector3};
pub use functions::{load_functions, Class_Add as Class_Create};

/// Player Class Information
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerClass {
    /// The class's team
    pub team: u8,
    /// The class's skin ID            
    pub skin: i32,
    /// The class's spawn position         
    pub spawn: Vector3,
    /// The class's angle       
    pub angle: f32,
    /// The class's weapons          
    pub weapons: WeaponSlots,
}

pub struct Class {
    handle: *const c_void,
}

impl Class {
    pub fn new(handle: *const c_void) -> Self {
        Class { handle }
    }
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn from_id(classid: i32) -> Option<Class> {
        functions::Class_FromID(classid)
    }
    pub fn get_id(&self) -> i32 {
        functions::Class_GetID(self)
    }

    pub fn add(
        team: u8,
        skin: i32,
        spawn: Vector3,
        angle: f32,
        weapon: WeaponSlots,
    ) -> Option<Class> {
        let mut id = 0;
        functions::Class_Add(
            team,
            skin,
            spawn.x,
            spawn.y,
            spawn.z,
            angle,
            weapon[0].id as u8,
            weapon[0].ammo,
            weapon[1].id as u8,
            weapon[1].ammo,
            weapon[2].id as u8,
            weapon[2].ammo,
            &mut id,
        )
    }
    pub fn count(&self) -> i32 {
        functions::Class_Count()
    }
    pub fn getdata(&self) -> PlayerClass {
        let mut data = PlayerClass::default();
        let (mut weapon1, mut ammo1, mut weapon2, mut ammo2, mut weapon3, mut ammo3): (
            u8,
            u32,
            u8,
            u32,
            u8,
            u32,
        ) = (0, 0, 0, 0, 0, 0);
        functions::Class_GetData(
            self,
            &mut data.team,
            &mut data.skin,
            &mut data.spawn.x,
            &mut data.spawn.y,
            &mut data.spawn.z,
            &mut data.angle,
            &mut weapon1,
            &mut ammo1,
            &mut weapon2,
            &mut ammo2,
            &mut weapon3,
            &mut ammo3,
        );
        data.weapons[0].id = unsafe { transmute(weapon1) };
        data.weapons[0].ammo = ammo1;
        data.weapons[1].id = unsafe { transmute(weapon2) };
        data.weapons[1].ammo = ammo2;
        data.weapons[2].id = unsafe { transmute(weapon3) };
        data.weapons[2].ammo = ammo3;

        data
    }
    pub fn edit(&self, data: PlayerClass) -> bool {
        functions::Class_Edit(
            self,
            data.team,
            data.skin,
            data.spawn.x,
            data.spawn.y,
            data.spawn.z,
            data.angle,
            data.weapons[0].id,
            data.weapons[0].ammo,
            data.weapons[1].id,
            data.weapons[1].ammo,
            data.weapons[2].id,
            data.weapons[2].ammo,
        )
    }
}
