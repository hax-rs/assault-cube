use hax::memlib::MemoryReadExt;
use hax::ExternalMemory;
use memlib::MemoryWriteExt;

// __int64 newplayerent(void)

pub mod gun;

// https://github.com/assaultcube/AC/blob/da5cb69c009b4c8fafbb2498787bd4b05d0274e7/source/src/entity.h#L59
const NUM_GUNS: u32 = 9;

#[derive(Debug)]
pub enum Gun {
    Knife = 0,
    Pistol,
    Carbine,
    Shotgun,
    Subgun,
    Sniper,
    Assault,
    Grenade,
    Akimbo,
}

const PLAYER: u64 = 0x5F0E10;

// void __fastcall pickupeffects(int a1, struct_player *player)

// https://github.com/assaultcube/AC/blob/da5cb69c009b4c8fafbb2498787bd4b05d0274e7/source/src/entity.h#L254-L259
const HEALTH: u64 = 0x100;
const ARMOR: u64 = 0x104;
const PRIMARY: u64 = 0x108;
const NEXT_PRIMARY: u64 = 0x10C;
const AKIMBO: u64 = 0x114;

const AMMO_ARRAY: u64 = 0x13C;
const MAG_ARRAY: u64 = 0x118;
const GRENADE: u64 = 0x158;
const GUN_INFO: u64 = 0x15C;

pub struct Player {
    mem: ExternalMemory,
    address: u32,
}

impl Player {
    pub fn new(mem: ExternalMemory) -> Self {
        let address = mem.try_read::<u32>(PLAYER).unwrap();
        Self { mem, address }
    }

    pub fn health(&self) -> u16 {
        self.mem.read::<u16>(self.address as u64 + HEALTH)
    }

    pub fn armor(&self) -> u16 {
        self.mem.write::<u16>(self.address as u64 + ARMOR, &1337);
        self.mem.read::<u16>(self.address as u64 + ARMOR)
    }

    pub fn akimbo(&self) -> u8 {
        self.mem.write::<u8>(self.address as u64 + AKIMBO, &1);
        self.mem.read::<u8>(self.address as u64 + AKIMBO)
    }

    pub fn set_primary_gun(&self, gun: Gun) {
        let gun = gun as u32;
        self.mem.write::<u32>(self.address as u64 + PRIMARY, &gun);
    }

    pub fn primary_gun(&self) -> Gun {
        let value = self.mem.read::<u32>(self.address as u64 + PRIMARY);
        assert!(value < NUM_GUNS);

        match value {
            0 => Gun::Knife,
            1 => Gun::Pistol,
            2 => Gun::Carbine,
            3 => Gun::Shotgun,
            4 => Gun::Subgun,
            5 => Gun::Sniper,
            6 => Gun::Assault,
            7 => Gun::Grenade,
            8 => Gun::Akimbo,
            _ => unreachable!(),
        }
    }

    /// Sets the magazine size of the specified gun.
    pub fn set_ammo(&self, gun: Gun, ammo: u32) {
        let offset = (gun as u64) * 0x4;
        self.mem
            .write::<u32>(self.address as u64 + AMMO_ARRAY + offset, &ammo);
    }

    pub fn ammo(&self, gun: Gun) -> u32 {
        let offset = (gun as u64) * 0x4;
        self.mem
            .read::<u32>(self.address as u64 + AMMO_ARRAY + offset)
    }

    pub fn mag(&self, gun: Gun) -> u32 {
        let offset = (gun as u64) * 0x4;
        self.mem
            .read::<u32>(self.address as u64 + AMMO_ARRAY + offset)
    }

    pub fn grenade(&self) -> u32 {
        self.mem.read::<u32>(self.address as u64 + GRENADE)
    }

    pub fn test(&self) {
        for i in 0..20 {
            let offset = i * 0x4;

            // self.mem
            //     .write::<u32>(self.address as u64 + MAG_ARRAY + offset, &42);

            let value = self
                .mem
                .read::<u32>(self.address as u64 + MAG_ARRAY + offset);
            println!("{:x}: {}", MAG_ARRAY + offset, value);
        }
    }
}
