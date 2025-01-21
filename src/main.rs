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
fn print_bin_memory_free_table(mem: &memory::Memory)-> String{
    sorted_keys_to_vec(&mem.free_table)
        .iter()
        .map(|val| format!("{}:{:#b},  ", val.0,val.1))
        .collect::<Vec<_>>()
        .join("")
}


fn main() {
    let mut mem = memory::Memory::new(100);

    println!(
        "{}",
        print_bin_memory_free_table(&mem)
    );




    let y=mem.malloc(127);
    println!(
        "{}",
        print_bin_memory_free_table(&mem)
    );
    print!("{:#b}",mem.free_table.get(&0).unwrap()+1);

    let z=mem.malloc(2);

    println!("{:?},{:?}",y,z);
    println!(
        "{}",
        print_bin_memory_free_table(&mem)
    );


    //let mut node1 = node::Node::new(3);
    //node1.write(&Vec::from([1,2,3]));

}
