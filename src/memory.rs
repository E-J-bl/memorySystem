
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
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
            *self.flash.get(&addr).unwrap()
        }
        else {
            0
        } }

    pub fn write(&mut self, addr: u32, value: u128)-> (){
        // need to add a check to see if the malloc value given to it is being written to so it does not write to an area where it will collide
        self.flash.insert(addr, value);

    }
    fn clear(&mut self){
        self.flash.clear();
    }

    fn left_shift_until_msb(mut value: u128) -> u128 {

        let msb_mask:u128 = 1 << (u128::BITS - 1); // Mask for the MSB (0b1000_0000 for u8)
        while value & msb_mask == 0 {
            value <<= 1; // Left shift by 1
        }

        value
    }
    fn loop_over_two_buffers(num_addresses:u32, reg_0:u128, reg_1:u128) -> u32 {
        let mut found_space=false;
        let mut buff_one:u128= 2_u128.pow(128-num_addresses)-1;
        let mut buff_two:u128= u128::MAX;
        let mut offset:u32=0;
        while !found_space{
            if (reg_0 & !buff_one== 0) & (reg_1& !buff_two ==0){
                found_space=true;
            }else {
                offset+=1;
                if offset>127{
                    return 129;
                }
                else if (num_addresses  +offset)<=128{
                    buff_one=buff_one/2+Self::left_shift_until_msb(2_u128.pow(offset));
                } else {
                    buff_two = !(u128::MAX-(2_u128.pow(256 - offset - num_addresses)-1)) ;
                    buff_one=Self::left_shift_until_msb(2_u128.pow(offset) - 1);

                }
            }
        }
        offset
    }


    fn mall_under_128(&mut self, num_addresses:u32) -> u32 {

        let mut offset:u32=0;
        let mut cur_addr:u32=0;
        let mut found_space=false;
        let mut reg_0: u128 = *self.free_table.get(&cur_addr).unwrap();
        let mut reg_1: u128= *self.free_table.get(&(cur_addr + 1)).unwrap();
        let mut check_regs:u32=0;


        while !found_space{
            //println!("{},{},{:0128b},{:0128b},{:0128b},{:0128b}",offset,256 - offset - num_addresses,reg_0,!buff_one,reg_1,!buff_two);
            if &cur_addr>= (self.free_table.iter().max().unwrap().0) {
                found_space=true;
                continue
            }
            check_regs= Self::loop_over_two_buffers(num_addresses,reg_0,reg_1);

            if check_regs== 129{
                cur_addr+=1;
                reg_0=reg_1;
                reg_1=*self.free_table.get(&(cur_addr + 1)).unwrap();
            }
            else {
                offset=check_regs;
                found_space=true;
            }

        }


        if offset+num_addresses<=128{

            let r1:u128;
            if num_addresses==128{
                r1 = u128::MAX;
            }else {
                r1 = reg_0 + (Self::left_shift_until_msb(2_u128.pow(num_addresses) - 1) >> (offset));
            }

            self.free_table.insert(cur_addr, r1);
        }else{

            let r1=reg_0 +(2_u128.pow(num_addresses) - 1)/2_u128.pow(offset-1);
            let r2= reg_1+Self::left_shift_until_msb(2_u128.pow(offset+num_addresses-128));
            self.free_table.insert(cur_addr,r1);

            self.free_table.insert(cur_addr+1,r2);
        }
        offset +128*cur_addr
    }




    fn mall_over_128(&mut self, num_addresses:u32) -> u32 {
        let mut offset:u32=0;
        let mut found:bool=false;
        let mut cur_addr:u32=0;
        let contig_min:u32= ((num_addresses / 128) - 1);
        let mut found_min_cont=true;
        let mut check_regs:u32=0;

        while !found{
            if &cur_addr>= (self.free_table.iter().max().unwrap().0) {
                found=true;
                continue
            }
            for i in 0..contig_min{
                if self.free_table.get(&(cur_addr + i)).unwrap()!=&0{
                    cur_addr+=1;
                    found_min_cont=false;
                    break
                }
            }
            if !found_min_cont{
                continue
            }
            if cur_addr!=0{
                check_regs= Self::loop_over_two_buffers(num_addresses-128*contig_min,
                                                        *self.free_table.get(&cur_addr).unwrap(),
                                                        *self.free_table.get(&(cur_addr+contig_min+1)).unwrap())
                ;
                if check_regs== 129{
                    cur_addr+=1;
                }
                else {
                    offset=check_regs;
                    found=true;
                }
            }
            else {
                if !*self.free_table.get(&(cur_addr+contig_min+1)).unwrap() & Self::left_shift_until_msb((num_addresses - 128 * contig_min) as u128){
                    found=true;
                    offset=0;
                }
                else {
                    cur_addr+=1;
                }
            }
        }

        //to add
        // ticks the areas that are being allocated over



        cur_addr*128 +offset

    }







    pub fn malloc(&mut self, num_addresses:u16) -> (u128,u32,u32) {
        //returns the key for the region, the start and the end of the  region which it has permissions for

        if num_addresses<128{
            let st=self.mall_under_128((num_addresses+1) as u32);
            println!("{} {}",num_addresses,st);
            let key:u128= SystemTime::now().duration_since(UNIX_EPOCH).expect("error with the time").as_nanos() ;
            self.flash.insert(st,key);
            (key,st,st+num_addresses as u32)
        } else{
            (0,0,0)
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
