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

let data2 = "
###########################################
#
#  Sprite scrolling demo:
#
#  Draw a computer monitor with a scrolling
#  image by using two copies of its sprites
#  and adjusting an offset into that data
#  before each draw.
#
###########################################

: main
	# draw the background:

	v0 := 16
	v1 :=  4
	i := comp-LT
	sprite v0 v1 11
	v0 += 8
	i := comp-T
	sprite v0 v1 3
	v0 += 8
	sprite v0 v1 3
	v0 += 8
	i := comp-RT
	sprite v0 v1 11

	v0 := 16
	v1 += 11
	i := comp-LB
	sprite v0 v1 15
	v0 += 8
	v1 += 7
	i := comp-B
	sprite v0 v1 8
	v0 += 8
	sprite v0 v1 8
	v0 += 8
	v1 += -7
	i := comp-RB
	sprite v0 v1 15

	# main animation loop:

	va :=     24 # left x
	vb :=     32 # right x
	vc :=      7 # common y
	v9 :=      0 # scroll offset
	v8 := 0b1111 # constant
	draw-texture
	loop
		draw-texture
		v9 += 1
		v9 &= v8
		draw-texture

		vF := 4
		delay := vF
		loop
			vF := delay
			if vF != 0 then
		again
	again

: draw-texture
	i := grenade-L
	i += v9
	sprite va vc 15
	i := grenade-R
	i += v9
	sprite vb vc 15
;

: grenade-L  0x0F 0x30 0x7C 0x7C 0xF8 0xF4 0xE0 0xE8 0xF0 0xE8 0xE0 0x68 0x70 0x34 0x08 0x00
             0x0F 0x30 0x7C 0x7C 0xF8 0xF4 0xE0 0xE8 0xF0 0xE8 0xE0 0x68 0x70 0x34 0x08 0x00
: grenade-R  0xF0 0x0C 0x46 0x66 0x33 0x13 0x0B 0x0B 0x1F 0x0F 0x0F 0x1E 0x1E 0x1C 0x30 0x00
             0xF0 0x0C 0x46 0x66 0x33 0x13 0x0B 0x0B 0x1F 0x0F 0x0F 0x1E 0x1E 0x1C 0x30 0x00
: comp-LT    0x3F 0x3F 0x3F 0x3C 0x3C 0x3C 0x3C 0x3C 0x3C 0x3C 0x3C
: comp-RT    0xFC 0xFC 0xFC 0x3C 0x3C 0x3C 0x3C 0x3C 0x3C 0x3C 0x3C
: comp-T     0xFF 0xFF 0xFF
: comp-LB    0x3C 0x3C 0x3C 0x3C 0x3C 0x3C 0x3C 0x3F 0x3F 0x3F 0x00 0x07 0x1C 0x73 0x7F
: comp-RB    0x3C 0x3C 0x3C 0x3C 0x3C 0x3C 0x3C 0xFC 0xFC 0xFC 0x00 0xE0 0xD8 0x26 0xFE
: comp-B     0xFF 0xFF 0xFF 0xFF 0x33 0xCC 0x33 0xFF
";
    let fragments = parse(data2);
    let lines = data2.split("\n");

    println!("\n\n");


    for (n, l) in lines.enumerate() {
        println!("{}: {}", n, l);
    }

    for fragment in  fragments {
        println!("{:?}", fragment);
    }




}
