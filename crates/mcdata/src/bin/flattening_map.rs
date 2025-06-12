use std::collections::HashMap;

use java_string::JavaStr;
use world_transmuter_mcdata::helpers::block_flattening_v1450::*;
use world_transmuter_mcdata::helpers::block_name_to_id_v1451::block_name_to_id;

fn main() {
    let block_to_block_name: HashMap<_, _> = HashMap::from_iter(
        block_name_to_id()
            .iter_mc_to_value()
            .map(|(&name, &block)| (block, String::from("minecraft:") + name.as_str().unwrap())),
    );

    for old_id in 0..4096 {
        let Some(old_name) = block_to_block_name.get(&((old_id >> 4) as u8)) else {
            continue;
        };
        let state = get_new_state_for_id_raw(old_id).unwrap();
        let new_name = get_new_block_name(&old_name);
        println!(
            "{}:{} {old_id} {old_name:?} -> {new_name} {state:?}",
            old_id >> 4,
            old_id & 15
        );
    }
}
