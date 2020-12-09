#![allow(non_snake_case)]
//  Kyzen Núcleo
pub struct Kyzen{
    //Registers floting and intergers
    pub fx64_rpg: [f64;32], //For vectors and floating pointers.
    pub x64_rpg: [i64;32], //for integers
    pub remaider:u32, //for DIV integers
    xr_ptr:i32,
    pub pcounter: usize, //point for the instruction
    pub Lcache: Vec<Vec<u8>>, //Local Memory
}





//Main and Controll Unit
impl Kyzen{
    pub fn new()->Kyzen{
        Kyzen{
            fx64_rpg:[0 as f64;32],
            x64_rpg:[0;32],
            remaider:0,
            pcounter: 0,
            xr_ptr: 0,
            Lcache: vec![],

        }
    }

}



//Fetch - Decode - Execute cycle
impl Kyzen{
    pub fn CU(&mut self){
            self.Execute();   
    }

    fn decode(&mut self)->u8{
        let op = &self.Lcache[self.pcounter];
        match  op[0]{
            0x00 =>  0x00,
            0xA0 =>{
                0xA0
            }
            0xA1 =>{
                0xA1
            }
            0xA2=>{
                0xA2
            }
            0xA3=>{
                0xA3
            }
            0xF1=>{
                0xF1
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
            0xA1=>{
                self.SUB();
            }
            0xA2 =>{
                self.MULT();
            }
            0xA3=>{
                self.DIV();
            }
            0xF1 =>{
                self.JUMP();
            }
            _ => {},
        }
    }

    fn set_pcounter(&mut self){
        if self.pcounter <= self.Lcache.len(){
            self.pcounter +=1;
        }
    }
}


//Arithmetic Logic unit
impl Kyzen{
    fn ADD(&mut self){
        let result = &self.next_3byte();
        self.x64_rpg[result[0]] = self.x64_rpg[result[1]] + self.x64_rpg[result[2]];
        self.set_pcounter();
    }
    fn SUB(&mut self){
        let result = &self.next_3byte();
        self.x64_rpg[result[0]] = self.x64_rpg[result[1]] - self.x64_rpg[result[2]];
        self.set_pcounter();
    }
    fn MULT(&mut self){
        let result = &self.next_3byte();
        self.x64_rpg[result[0]] = self.x64_rpg[result[1]] * self.x64_rpg[result[2]];
        self.set_pcounter();
    }
    fn DIV(&mut self){
        let result = &self.next_3byte();
        self.x64_rpg[result[0]] = self.x64_rpg[result[1]] / self.x64_rpg[result[2]];
        self.remaider = (self.x64_rpg[1] % self.x64_rpg[1]) as u32;
        self.set_pcounter();
    }
    fn JUMP(&mut self){
        //self.xr_ptr =  function that return the value in a stack in the memory. 
        self.pcounter = self.xr_ptr as usize;
    }
}





impl Kyzen{
    fn next_3byte(&self)->[usize;3]{
        let mut result:[usize;3] = [0,0,0];
        let opr = &self.Lcache[self.pcounter];
        for i in 1..4{
            match opr[i]{
                0x00 =>result[i-1] = 0,
                0x01 =>result[i-1] = 1,
                0x02 =>result[i-1] = 2,
                0x03 =>result[i-1] = 3,
                0x04 =>result[i-1] = 4,
                0x05 =>result[i-1] = 5,
                0x06 =>result[i-1] = 6,
                0x07 =>result[i-1] = 7,
                0x08 =>result[i-1] = 8,
                0x09 =>result[i-1] = 9,
                0x0A =>result[i-1] = 10,
                0x0B =>result[i-1] = 11,
                0x0C =>result[i-1] = 12,
                0x0D =>result[i-1] = 13,
                0x0E =>result[i-1] = 14,
                0x0F =>result[i-1] = 15,
                0x11 =>result[i-1] = 16,
                0x12 =>result[i-1] = 17,
                0x13 =>result[i-1] = 18,
                0x14 =>result[i-1] = 19,
                0x15 =>result[i-1] = 20,
                0x16 =>result[i-1] = 21,
                0x17 =>result[i-1] = 22,
                0x18 =>result[i-1] = 23,
                0x19 =>result[i-1] = 24,
                0x1A =>result[i-1] = 25,
                0x1B =>result[i-1] = 26,
                0x1C =>result[i-1] = 27,
                0x1D =>result[i-1] = 28,
                0x1E =>result[i-1] = 29,
                0x1F =>result[i-1] = 30,
                0x20 =>result[i-1] = 31,
                _ => result[i-1] = 32}
        }
      

        return result
    }

}


