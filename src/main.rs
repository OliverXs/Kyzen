#![allow(non_snake_case)]
use kore::cpu;

fn main() {
    //create a computers and manipule them
    let mut kayn = cpu::Kyzen::new();
    kayn.x64_rpg[0] = 2;
    kayn.x64_rpg[1] = 2;
    let scr = vec![0xA0,0x02,0x20,0x00];
    kayn.Lcache.push(scr);


    kayn.CU();
    println!("{}",kayn.x64_rpg[2])


}


