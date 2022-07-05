use std::{marker::PhantomData, fs::File};

use bevy::prelude::Plugin;
use serde::de::DeserializeOwned;
use ron::de::from_reader;

use crate::item_map::ItemMap;

pub struct InventoryPlugin <'a,ItemLayout: 'a >{
    item_layout: PhantomData<&'a ItemLayout>,
    item_path: &'static str,
}
impl <Item> InventoryPlugin <'_, Item> {
    pub fn from_file(path: &'static str) -> Self{
        InventoryPlugin::<Item> {
            item_layout: PhantomData,
            item_path: path,
        }
    }
}
impl <ItemLayout> Plugin for InventoryPlugin<'static, ItemLayout> 
where ItemLayout: Send + Sync + DeserializeOwned + 'static {
    fn build(&self, app: &mut bevy::prelude::App) {
        let f = File::open(self.item_path).expect("Failed opening file");

        let item_map: ItemMap<ItemLayout> = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };
        app.insert_resource(item_map);

    }
}

