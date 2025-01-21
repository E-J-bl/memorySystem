
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

        let msb_mask:u128 = (1 << (u128::BITS - 1)); // Mask for the MSB (0b1000_0000 for u8)

        while value & msb_mask == 0 {
            value <<= 1; // Left shift by 1
        }

        value
    }
    fn mall_under_128(&mut self, num_addresses:u32) -> u32 {
        let mut return_address:u32=u32::MAX;
        let mut offset:u32=0;
        let mut cur_addr:u32=0;
        let mut found_space=false;
        let mut reg_0: u128 = *self.free_table.get(&cur_addr).unwrap();
        let mut reg_1: u128= *self.free_table.get(&(cur_addr + 1)).unwrap();
        let mut buff_one:u128= 2_u128.pow(128-num_addresses)-1;
        let mut buff_two:u128= u128::MAX;

        while !found_space{
            if cur_addr>=100{
                found_space=true;
                continue
            }

            if !(reg_0 & buff_one== 0) | !(reg_1& buff_two ==0){
                offset+=1;
                if offset>128{
                    cur_addr+=1;
                    offset=0;
                    reg_0=reg_1;
                    reg_1=*self.free_table.get(&(cur_addr+1)).unwrap();
                }

                buff_one=Self::left_shift_until_msb((2_u128.pow(offset) - 1) )+(2_u128.pow(128-num_addresses-offset)-1);

                if 128-(num_addresses  +offset)<=128{
                    buff_two=u128::MAX

                } else {
                    buff_two = 2_u32.pow(256 - offset - num_addresses) as u128
                }
            }
            else if (reg_0 & buff_one== 0) & (reg_1& buff_two ==0){
                found_space=true;
                return_address=128*cur_addr+offset;


            }
        }

        if offset+num_addresses<=128{

            let r1=reg_0 + (2_u128.pow(num_addresses) - 1)<<(offset);
            self.free_table.insert(cur_addr, r1);
        }
        return return_address
    }




    fn mall_over_128(&mut self, num_addresses:u16) -> (u32, u32) {
        let mut count:u128=0;
        let mut found:bool=false;
        let mut cur_addr:u32=0;
        let contig_min:u32= ((num_addresses / 128) - 1) as u32;
        let mut found_min_cont=false;
        while !found{
            if self.free_table.get(&cur_addr)!=Option::None{
                found_min_cont=true;
                for i  in (0..contig_min){
                    if self.free_table.get(&(cur_addr+(i as u32))).unwrap()!= &u128::MIN{
                        found_min_cont=false;
                    } else {

                    }
                if found_min_cont{

                    let before= self.free_table.get(&(cur_addr - 1)).unwrap().trailing_zeros();
                    let after= self.free_table.get(&(cur_addr+contig_min+1)).unwrap().leading_zeros();

                    if (before+after+128*contig_min>=num_addresses as u32) {
                        self.free_table.insert((cur_addr-1), match self.free_table.get(&(cur_addr - 1)) {
                            Some(x) => x,
                            None => panic!(),
                        } & (2 ** &before - 1)as u128);
                        for i in (0..contig_min){
                            self.free_table.insert(cur_addr+i,u128::MAX);
                        };
                        self.free_table.insert((cur_addr+contig_min+1),self.free_table.get(&(cur_addr+contig_min+1)).unwrap()& Self::left_shift_until_msb((2 * *&after - 1) as u128));
                        return ((cur_addr)*128-before,(cur_addr+contig_min)*128+after);

                        //need to change so it does not just fill the last address it only fills as much as it needs;

                    }



                    }
                }
            }else{
                cur_addr+=1
            }
            if cur_addr> *(match self.look_up.keys().max() {
                None => panic!(),
                Some(x) => x
            }) {
                found=true
            } else {}
        }
        (0,0)

    }







    pub fn malloc(&mut self, num_addresses:u16) -> (u32,u32) {

        if num_addresses<129{
            let st=self.mall_under_128(num_addresses as u32);
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
