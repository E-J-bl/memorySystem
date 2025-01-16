
use std::collections::HashMap;
use std::fmt;
use itertools::Itertools;

pub struct Memory {
    //memory all starts at addr 1
    pub free_table:HashMap<u32,u128>,
    look_up:HashMap<u32,u128>,
    flash: HashMap<u32, u128>,
}


impl Memory{
    pub fn new(size:u32)->Self{
        let mut free_table = HashMap::new();
        for i in 0..size {
            free_table.insert(i, 0);
        }

        Self {
            free_table,
            look_up: HashMap::new(),
            flash: HashMap::new(),
        }

    }
    pub fn access(&self, addr: u32) -> u128{
        if self.flash.contains_key(&addr){
            return *self.flash.get(&addr).unwrap();
        }
        else {
            return 0
        } }

    pub fn write(&mut self, addr: u32, value: u128)-> (){
        // need to add a check to see if the malloc value given to it is being written to so it does not write to an area where it will collide
        self.flash.insert(addr, value);

    }
    fn clear(&mut self){
        self.flash.clear();
    }

    fn left_shift_until_msb(mut value: u128) -> u128 {
        let bits = u128::BITS; // Total number of bits in the type (8 for u8)
        let msb_mask = 1 << (bits - 1); // Mask for the MSB (0b1000_0000 for u8)

        while value & msb_mask == 0 {
            value <<= 1; // Left shift by 1
        }

        value
    }
    fn mall_under_128(&mut self, num_addresses:u8) -> u32 {
        let mut test:u128;
        if num_addresses<128 {
            test = Self::left_shift_until_msb((2 * *(&num_addresses)-1) as u128);
        }else {
            test=u128::MAX;
        }
        let mut offset:u32=0;
        let mut cur_addr:u32=0;

        let mut found_space=false;

        while !found_space{
            //two possibilities it fits inside, or it fits over two blocks
            if self.free_table.get(&cur_addr)!=Option::None {
                while self.free_table.get(&cur_addr).unwrap().count_zeros() < test.count_ones() {
                    cur_addr += 1;
                    if self.free_table.get(&cur_addr)==Option::None{
                        self.free_table.insert(cur_addr, 0);
                        break

                    }
                }
            }
            while test.count_ones()==num_addresses as u32{
                if test & self.free_table.get(&cur_addr).unwrap()==0{
                    self.free_table.insert(cur_addr,self.free_table.get(&cur_addr).unwrap()| test);
                    return cur_addr*128+offset;
                }
                else{
                    offset+=1;
                    test>>=1;

                }
            }
           if self.free_table.get(&cur_addr).unwrap().trailing_zeros()+self.free_table.get(&(cur_addr+1)).unwrap().leading_zeros()>num_addresses as u32{
               return cur_addr*128 +(128-self.free_table.get(&cur_addr).unwrap().trailing_zeros())
           }


            cur_addr+=1;
            offset=0;
            if num_addresses<128 {
                test = Self::left_shift_until_msb((2 * *(&num_addresses)) as u128);
            }else {
                test=u128::MAX;
            }

        }
        return 0
    }

    fn mall_over_128(&mut self,num_addresses:u16){

    }


    pub fn malloc(&mut self, num_addresses:u16) -> (u32,u32) {

        if num_addresses<129{
            let st=self.mall_under_128(num_addresses as u8);
            return (st,st+num_addresses as u32)
        } else{
            return (0,0)
        }

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
