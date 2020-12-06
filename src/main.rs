#![allow(non_snake_case)]
use std::time::Instant;
use fore::cpu::*;
use std::io;
fn main() {
    let now = Instant::now();
    let mut Kayn = Kyzen::new();
    let test:[u8;4]=[0xA0,0x02,0x00,0x01];
    let mut wait=String::new();

    Kayn.x64_rpg[0]=2;
    Kayn.x64_rpg[1]=3;

    Kayn.Lcache.push(test);


    Kayn.CU();
    println!("testando:{}\n",Kayn.x64_rpg[2]);
    println!("{:?}",now.elapsed());

    io::stdin().read_line(&mut wait)
        .expect("Failed to read line");
}


