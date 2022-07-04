use bevy::prelude::*;
use ron::de::from_reader;
use std::fs::File;
use inv_bevy::*;
use inv_bevy::item_map::ItemMap;
use serde::Deserialize;
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(print_inv)
        // .add_system(add_on_press)
        .run();
}
#[derive(Debug, Clone, Deserialize)]
struct Item {
    name: String,
    max_stack: usize,
    description: String,
    item_type: ItemType,
    tag: Vec<Tags>
}
#[derive(Debug, Clone, Deserialize)]

pub enum ItemType {
    Consumable(Heal),
}
#[derive(Debug, Clone, Deserialize)]

pub struct Heal(u32);
#[derive(Debug, Clone, Deserialize)]
pub enum Tags {
    Food, Weapon
}
#[derive(Component)]
struct Player;
fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let input_path = "resources/items.ron";
    let f = File::open(input_path).expect("Failed opening file");
    let item_map: ItemMap<Item> = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };
    commands.insert_resource(item_map);
    // .init_resource(ItemMap<Item>)

    let inv = InventoryBuilder::new("player_inv")
        // .with_item_file("resources/items.ron")
        .with_inventory_file("resources/inv_player.ron")
        .build();

    commands
        .spawn()
        .insert(Player)
        .insert(inv);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
fn print_inv(inv: Query<&Inventory>, item_map: Res<ItemMap<Item>>) {
    for inventory in inv.iter() {
        let keys = inventory.get_items();
        for key in keys {
            println!("item: {:?}", item_map.get(key.to_string()));

        }
        println!("---")
        // inventory.print();
    }
}
// fn add_on_press(mut inv: Query<&mut Inventory>, keys: Res<Input<KeyCode>>) {
//     if keys.just_pressed(KeyCode::Space) {
//         // Space was pressed
//         for mut inventory in inv.iter_mut() {
//             inventory.add_item(Items::Sword);
//             inventory.add_item(Items::Apple(10));
//         }
//     }
//     if keys.just_pressed(KeyCode::A) {
//         for mut inventory in inv.iter_mut() {
//             println!("{:?}", inventory.use_item());
//         }
//     }
// }
