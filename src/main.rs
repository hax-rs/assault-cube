#![feature(cstr_from_bytes_until_nul)]

use hax::{memlib::MemoryReadExt, memlib::MemoryWriteExt};
use memlib::MemoryRead;

use crate::sdk::{gun::GunInfo, Gun};

pub mod features;
pub mod sdk;

fn setup_hooks() {
    println!("setup_hooks");
}

#[hax::main(process = "ac_client")]
#[hax::init(setup_hooks)]
fn main() {
    env_logger::init();

    let memory = hax::ExternalMemory::new(594513);

    let base = memory.base_address("ac_client").unwrap() as u64;
    log::info!("Base address: {:x}", base);

    let player = sdk::Player::new(memory.clone());
    log::info!("health: {}", player.health());
    log::info!("armor: {}", player.armor());
    log::info!("primary_gun: {:?}", player.primary_gun());
    log::info!("akimbo: {}", player.akimbo());
    log::info!("grenade: {}", player.grenade());
    log::info!("ammo: {}", player.ammo(Gun::Assault));
    log::info!("mag: {}", player.mag(Gun::Assault));

    player.set_primary_gun(Gun::Sniper);
    player.set_ammo(Gun::Akimbo, 1000);
    player.set_ammo(Gun::Assault, 1000);

    let gun_info = GunInfo::new(memory, Gun::Knife);
    log::info!("name: {:?}", gun_info.name());
    log::info!("title: {:?}", gun_info.title());
    log::info!("sound: {:x?}", gun_info.sound());
    log::info!("reload: {:x?}", gun_info.reload());
    log::info!("reload_time: {:x?}", gun_info.reload_time());
    log::info!("spread: {:x?}", gun_info.spread());
    log::info!("recoil: {:x?}", gun_info.recoil());

    gun_info.set_recoil(0);
    gun_info.set_spread(0);

    // player.test();

    //
    //
    let mut features = hax::features();

    // 0xC8 = team
    // TODO: How to pass overlay?
    // TODO: How to handle keyboard input?

    features.iter_mut().for_each(|f| f.setup());
    for f in features.iter_mut() {
        f.tick();
    }
    features.iter_mut().for_each(|f| f.cleanup());

    features.iter_mut().for_each(|f| f.save());
}
