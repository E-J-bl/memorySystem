use crate::memory;


pub struct Node{
    pub contig: u128,
    data: Vec<u128>,
    end_car: u128,
    pub link: u32,
}

impl Node{

    pub fn new(contig: u128)-> Self{ Self{contig, data: Vec::new(), end_car: 255, link: 0}

    }

    pub fn write(&mut self,data: &Vec<u128>)-> Result<u8,String>{
        if data.len()> self.contig as usize {
            Err("to much data".to_string())
        } else {
            self.data.clear();
            data.iter().for_each(|x| self.data.push(*x));
            Ok(0)
        }
    }
    pub fn join_to(&mut self, next:u32){

        self.link=next;
    }

    pub fn insert_mem(self, mem: &mut memory::Memory, pos: u32){
        mem.write(pos, self.contig);
        let mut offset: u32=1;
        for i in 0..self.data.len(){
            mem.write(pos+offset, self.data[i]);
            offset+=1;
        }
        mem.write(pos+offset, self.end_car);

        // need to convert the address into its 4 u8 bit format so i can append each

        mem.write(pos+offset+1, self.link as u128);
    }
}


