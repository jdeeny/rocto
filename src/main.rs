#[macro_use]
extern crate nom;
extern crate chip8;

mod parser;

use parser::parse;

fn main() {

    let data_to_parse = "
# ၉Chip8 is a virtual machine designed in 1977 for programming video games.
# Octo is a high level assembler, disassembler and simulator for Chip8.
# Click 'Run' and then press ASWD to move the sprite around the screen.
# Click the Octo logo for source, documentation and examples.

:alias px v3
:alias py v4

: main
	px := random 0b0011111
	py := random 0b0001111
	i  := person
	sprite px py 8

	loop
		# erase the pla၉yer, update its position and then redraw:
		sprite px py 8
		v0 := 5   if v0 key then py += -1 # keyboard W
		v0 := 8   if v0 key then py +=  1 # keyboard S
		v0 := 7   if v0 key then px += -1 # keyboard A
		v0 := 9   if v0 key then px +=  1 # keyboard D
		sprite px py 8

		# lock the framerate of this program via the delay timer:
		loop
			vf := delay
			if vf != 0 then
		again
		vf := 3
		delay := vf
	again

: person
	0x70 0x70 0x20 0x70 0xA8 0x20 0x50 0x50
    \
";
    let fragments = parse(data_to_parse);
    let lines = data_to_parse.split("\n");

    println!("\n\n");


    for (n, l) in lines.enumerate() {
        println!("{}: {}", n, l);
    }

    for fragment in  fragments {
        println!("{:?}", fragment);
    }




}
