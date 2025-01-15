use std::collections::HashMap;
mod memory;
mod node;


fn main() {
    let mut mem = memory::Memory {
        flash: HashMap::new(),
    };
    mem.write(0, 1);
    mem.write(1, 2);
    mem.write(17,8);
    let x: u128 = mem.access(0);

    println!("{}", mem);
    println!("\n{}", x);
    mem.clear();

    let mut node1 = node::Node::new(3);
    node1.write(&Vec::from([1,2,3]));
    node1.insert_mem(&mut mem, 0);
    println!("{}", mem);
}
