pub mod z80core;
pub mod alu;
fn main() {
    let mut z80 = z80core::VirtZ80::new();
    /*println!(
        "af: {:04x} bc: {:04x} de: {:04x} hl: {:04x} sp: {:04x} pc: {:04x} ix: {:04x} iy: {:04x} i: {:04x} r: {:04x}",
        z80.af, z80.bc, z80.de, z80.hl, z80.sp, z80.pc, z80.ix, z80.iy, z80.i, z80.r
    );*/
    let mut a: u16 = 1;
    println!("1+1={:04x}", alu::alu16(&mut z80, &mut a, 1, 0));
}
