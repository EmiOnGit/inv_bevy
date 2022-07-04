mod lib;
use bevy::prelude::*;

use lib::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InventoryPlugin)
        .add_startup_system(setup)
        .add_system(print_inv)
        .add_system(add_on_press)
        .run();
}
#[derive(PartialEq, Debug, Clone)]
enum Items {
    Apple(u32),
    Sword,
}
#[derive(Component)]
struct Player;
fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Inventory::<Items>::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
}
fn print_inv(inv: Query<&Inventory<Items>>) {
    for inventory in inv.iter() {
        inventory.print();
    }
}
fn add_on_press(mut inv: Query<&mut Inventory<Items>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
        for mut inventory in inv.iter_mut() {
            inventory.add_item(Items::Sword);
            inventory.add_item(Items::Apple(10));
        }
    }
    if keys.just_pressed(KeyCode::A) {
        for mut inventory in inv.iter_mut() {
            println!("{:?}", inventory.use_item());
        }
    }
}
