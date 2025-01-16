use std::collections::HashMap;
mod memory;
mod node;

mod dir_file;



fn sorted_keys_to_vec(hash_map: &HashMap<u32, u128>) ->  Vec<(u32, u128)>{
    let mut keys: Vec<_> = hash_map.keys().collect(); // Collect keys into a Vec
    keys.sort(); // Sort the keys
    keys.iter().map(|&&x| (x,hash_map[&x])).collect()
         // Collect formatted strings into a Vec
}

fn main() {
    let mut mem = memory::Memory::new(100);

    let x: u128 = mem.access(0);

    println!("{}", mem);
    println!("\n{}", x);

    let y=mem.malloc(128);
    //let z=mem.malloc(2);

    //println!("{},{}",y,z);
    println!("{:?}",sorted_keys_to_vec(&mem.free_table));
    println!(
        "{}",
        sorted_keys_to_vec(&mem.free_table)
            .iter()
            .map(|val| format!("{}:{:#b},  ", val.0,val.1))
            .collect::<Vec<_>>()
            .join("")
    );


    //let mut node1 = node::Node::new(3);
    //node1.write(&Vec::from([1,2,3]));

}
