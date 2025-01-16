enum privileges{
    creator,
    local,
    global
}
struct dir_node{
    pub name:String,
    place_mem:u32,
    pub next_chain:u32,
}

impl dir_node{
    pub fn new(name:String,){

    }
}
pub struct dir{
    pub address_out: u32,
    ownership: (u8, u8, u8),// copying unix design
    encryption:u8,
    special_characteristics:u64,
    time_of_last_access: u128,
    last_in_chain:u32,
    first_in_chain:u32,
    dir_chain:Vec<dir_node>

}

impl dir {
    pub fn new(&mut self,address_out:u32,level:privileges,encryp:Option<u8>,spectial_char:Option<u64>){
        let spectial_char=spectial_char.unwrap_or(0);
        let encryp= encryp.unwrap_or(0);
        self.address_out=address_out;
        match level {
            privileges::creator=> self.ownership.0=u8::MAX,
            privileges::local=> self.ownership.1=u8::MAX,
            privileges::global=> self.ownership.2=u8::MAX,
        }
        self.encryption=encryp;
        self.special_characteristics=spectial_char;
        //need to find a get time in form i want
    }


}