/// a ship class describe the size or technology involved in the ship
pub enum ShipClass {
    Frigate,      // 军用船只，护卫舰
    Destroyer,    // 军用船只，驱逐舰
    Cruiser,      // 军用船只，巡洋舰
    Battleship,   // 军用船只，主力舰
    LargeCarrier, // 民用船只，运输船
    MultiPurpose, // 民用船只，多用途船
    Special,      // 特殊船只
}

/// Basic: Power, Engine, Command, Armor, Weapon, Science, Engineering, Storage.

/// Slot: Engine, Power, General, Armor[outer], Armor[inner], Weapon [], Weapon
/// [M]

/// a ship template describe the skeleton of the ship
pub struct ShipTemplate {
    base_mass: f32,
    integrity: f32,
    slot_e: i32,
    slot_p: i32,
    slot_g: i32,
    slot_a: i32,
    slot_w: i32,
}
