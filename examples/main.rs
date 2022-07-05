use bevy::prelude::*;
use inv_bevy::inventory_builder::InventoryBuilder;
use inv_bevy::plugin::InventoryPlugin;
use inv_bevy::*;
use inv_bevy::item_map::ItemMap;
use serde::Deserialize;
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InventoryPlugin::<Item>::from_file("resources/items.ron"))
        .add_startup_system(setup)
        .add_system(print_inv)
        .add_system(add_on_press)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    
    let inv = InventoryBuilder::<Item>::new("player_inv")
        .with_inventory_file("resources/inv_player.ron")
        .build();

    commands
        .spawn()
        .insert(Player)
        .insert(inv);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
fn print_inv(
    inv: Query<&Inventory<Item>>, 
    item_map: Res<ItemMap<Item>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::P) {

        for inventory in inv.iter() {
            let item_keys = inventory.get_items();
            for key in item_keys {
                println!("item: {:?}", item_map.get(key.to_string()));
            }
            println!("---")
        }
    }
}

fn add_on_press(
    mut inv: Query<&mut Inventory<Item>>, 
    keys: Res<Input<KeyCode>>,
    item_map: Res<ItemMap<Item>>
) {
        // Space was pressed
    for mut inventory in inv.iter_mut() {
        if keys.just_pressed(KeyCode::Space) {
            inventory.add_item(&item_map,"apple");
        }
        if keys.just_pressed(KeyCode::Key1) {
            inventory.set_index(0);
        }
        if keys.just_pressed(KeyCode::Key2) {
            inventory.set_index(1);
        }
        if keys.just_pressed(KeyCode::Key3) {
            inventory.set_index(2);
        }
        if keys.just_pressed(KeyCode::Key4) {
            inventory.set_index(3);
        }
        if keys.just_pressed(KeyCode::D) {
            println!("{:?}",inventory.pop_active());
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Item {
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