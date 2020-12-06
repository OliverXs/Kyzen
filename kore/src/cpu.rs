#![allow(non_snake_case)]

//  Kyzen Núcleo
pub struct Kyzen{
    //Registers floting and intergers
    pub fx64_rpg: [f64;32], //For vectors and floating pointers.
    pub x64_rpg: [i64;32], //for integers
    
    pub remaider:u32,
    pcounter: usize, //point for the instruction
    pub Lcache: Vec<[u8;4]>, //Local Memory
}

//Main and Controll Unit
impl Kyzen {
    pub fn new()->Kyzen{
        Kyzen{
            fx64_rpg:[0 as f64;32],
            x64_rpg:[0;32],
            remaider:0,
            pcounter: 0,
            Lcache: vec![]
        }
    }

}



//Fetch - Decode - Execute cycle
impl Kyzen{
    pub fn CU(&mut self){
        self.Execute();
    }
    fn decode(&mut self)->u8{
        let op =self.Lcache[self.pcounter];
        match  op[0]{
            0x00=>{
                print!("Hillegal!!!");
                return 0x00
            }
            0xA0 =>{
                return 0xA0
            }
            _ => {
                return 0x00
            },
        }    
    }
    fn Execute(&mut self){
        match self.decode() {
            0xA0 => {
                self.ADD();
            },
            _ => {},
        }
    }

}

//Arithmetic Logic unit
impl Kyzen{
    fn ADD(&mut self){
        self.x64_rpg[self.Drn()] = self.x64_rpg[self.FPrn()] + self.x64_rpg[self.SPrn()];
    }
}





impl Kyzen{
    fn Drn(&self)->usize{
        let drn =self.Lcache[self.pcounter];
        match drn[1]{
            0x00 =>return 0,
            0x01 =>return 1,
            0x02 =>return 2,
            0x03 =>return 3,
            0x04 =>return 4,
            0x05 =>return 5,
            0x06 =>return 6,
            0x07 =>return 7,
            0x08 =>return 8,
            0x09 =>return 9,
            0x0A =>return 10,
            0x0B =>return 11,
            0x0C =>return 12,
            0x0D =>return 13,
            0x0E =>return 14,
            0x0F =>return 15,
            0x11 =>return 16,
            0x12 =>return 17,
            0x13 =>return 18,
            0x14 =>return 19,
            0x15 =>return 20,
            0x16 =>return 21,
            0x17 =>return 22,
            0x18 =>return 23,
            0x19 =>return 24,
            0x1A =>return 25,
            0x1B =>return 26,
            0x1C =>return 27,
            0x1D =>return 28,
            0x1E =>return 29,
            0x1F =>return 30,
            0x20 =>return 31,
            _ => 0
        }
    }
    fn FPrn(&self)->usize{
        let fprn =self.Lcache[self.pcounter];
        match fprn[2]{
            0x00 =>return 0,
            0x01 =>return 1,
            0x02 =>return 2,
            0x03 =>return 3,
            0x04 =>return 4,
            0x05 =>return 5,
            0x06 =>return 6,
            0x07 =>return 7,
            0x08 =>return 8,
            0x09 =>return 9,
            0x0A =>return 10,
            0x0B =>return 11,
            0x0C =>return 12,
            0x0D =>return 13,
            0x0E =>return 14,
            0x0F =>return 15,
            0x11 =>return 16,
            0x12 =>return 17,
            0x13 =>return 18,
            0x14 =>return 19,
            0x15 =>return 20,
            0x16 =>return 21,
            0x17 =>return 22,
            0x18 =>return 23,
            0x19 =>return 24,
            0x1A =>return 25,
            0x1B =>return 26,
            0x1C =>return 27,
            0x1D =>return 28,
            0x1E =>return 29,
            0x1F =>return 30,
            0x20 =>return 31,
            _ => 0
        }
    }
    fn SPrn(&self)->usize{        
        let sprn =  self.Lcache[self.pcounter];
        match sprn[3]{
            0x00 =>return 0,
            0x01 =>return 1,
            0x02 =>return 2,
            0x03 =>return 3,
            0x04 =>return 4,
            0x05 =>return 5,
            0x06 =>return 6,
            0x07 =>return 7,
            0x08 =>return 8,
            0x09 =>return 9,
            0x0A =>return 10,
            0x0B =>return 11,
            0x0C =>return 12,
            0x0D =>return 13,
            0x0E =>return 14,
            0x0F =>return 15,
            0x11 =>return 16,
            0x12 =>return 17,
            0x13 =>return 18,
            0x14 =>return 19,
            0x15 =>return 20,
            0x16 =>return 21,
            0x17 =>return 22,
            0x18 =>return 23,
            0x19 =>return 24,
            0x1A =>return 25,
            0x1B =>return 26,
            0x1C =>return 27,
            0x1D =>return 28,
            0x1E =>return 29,
            0x1F =>return 30,
            0x20 =>return 31,
            _ => 0
        }
    }

}



