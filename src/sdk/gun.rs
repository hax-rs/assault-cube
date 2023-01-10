use std::borrow::Cow;
use std::ffi::{CStr, CString};

use hax::memlib::MemoryReadExt;
use hax::ExternalMemory;
use memlib::{MemoryRead, MemoryWriteExt};

use super::Gun;

// https://github.com/assaultcube/AC/blob/da5cb69c009b4c8fafbb2498787bd4b05d0274e7/source/src/entity.h#LL90C16-L90C20
// It's set like this:
// ```
// player->dword15C = guns[0x1CB]; // akimbo
// ```
const GUN_INFO_SIZE: u64 = 0x68;
const GUN_INFO_ARRAY: u64 = 0x5EA0C0;

const TITLE: u64 = 23;
const SOUND: u64 = 23 + 42;
const RELOAD: u64 = SOUND + 0x2;
const RELOAD_TIME: u64 = RELOAD + 0x2;
const ATTACK_DELAY: u64 = RELOAD_TIME + 0x2;
const DAMAGE: u64 = ATTACK_DELAY + 0x2;
const PIERCING: u64 = DAMAGE + 0x2;
const PROJ_SPEED: u64 = PIERCING + 0x2;
const PART: u64 = PROJ_SPEED + 0x2;
const SPREAD: u64 = PART + 0x2;
const RECOIL: u64 = SPREAD + 0x2;

// reload,
// reloadtime, attackdelay, damage, piercing, projspeed, part, spread, recoil, magsize, mdl_kick_rot, mdl_kick_back, recoilincrease, recoilbase, maxrecoil, recoilbackfade, pushfactor;

pub struct GunInfo {
    mem: ExternalMemory,
    address: u64,
}

impl GunInfo {
    pub fn new(mem: ExternalMemory, gun: Gun) -> Self {
        let offset = (gun as u64) * GUN_INFO_SIZE;
        Self {
            mem,
            address: GUN_INFO_ARRAY + offset,
        }
    }

    pub fn name(&self) -> String {
        let bytes = self.mem.try_read_bytes(self.address, 23).unwrap();

        CStr::from_bytes_until_nul(&bytes)
            .unwrap()
            .to_string_lossy()
            .to_string()
    }

    pub fn title(&self) -> String {
        let bytes = self.mem.try_read_bytes(self.address + TITLE, 42).unwrap();

        CStr::from_bytes_until_nul(&bytes)
            .unwrap()
            .to_string_lossy()
            .to_string()
    }

    pub fn sound(&self) -> u16 {
        self.mem.read::<u16>(self.address + SOUND)
    }

    pub fn reload(&self) -> u16 {
        self.mem.read::<u16>(self.address + RELOAD)
    }

    pub fn reload_time(&self) -> u16 {
        self.mem.read::<u16>(self.address + RELOAD_TIME)
    }

    pub fn spread(&self) -> u16 {
        self.mem.read::<u16>(self.address + SPREAD)
    }

    pub fn set_spread(&self, spread: u16) {
        self.mem.write::<u16>(self.address + RECOIL, &spread);
    }

    pub fn recoil(&self) -> u16 {
        self.mem.read::<u16>(self.address + RECOIL)
    }

    pub fn set_recoil(&self, recoil: u16) {
        self.mem.write::<u16>(self.address + RECOIL, &recoil);
    }
}
