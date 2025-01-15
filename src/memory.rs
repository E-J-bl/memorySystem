
use std::collections::HashMap;
use std::fmt;
use itertools::Itertools;

pub struct Memory {
    pub flash: HashMap<u32, u128>,
}


impl Memory{
    pub fn access(&self, addr: u32) -> u128{
        if self.flash.contains_key(&addr){
            return *self.flash.get(&addr).unwrap();
        }
        else {
            return 0
        } }

    pub fn write(&mut self, addr: u32, value: u128)-> (){
        self.flash.insert(addr, value);

    }
    pub fn clear(&mut self){
        self.flash.clear();
    }
}

impl fmt::Display for Memory{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
        let mut memfla: Vec<(&u32,&u128)>=self.flash.iter().collect::<Vec<(&u32,&u128)>>();
        memfla.sort_by(|a,b| a.0.cmp(b.0));

        write!(f,"{}"
               ,memfla.iter().map(|(k,v)| format!("{}={}", k, v)).join(", "))
    }
}
