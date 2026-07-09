pub const SCREEN_WIDTH: usize = 64; // screen pixel width
pub const SCREEN_HEIGHT: usize = 32; // screen pixel height

const RAM_SIZE: usize = 4096; // total 4 KB of RAM
const NUM_VREG: usize = 16; // number of general putpose registers -> V0 .. VF
const STACK_SIZE: usize = 16; // total stack height
const NUM_KEYS: usize = 16; // Number of keys on keyboard -> 4x4

const START_ADDR: u16 = 200; // 0x200 (512) -> Start of most Chip-8 programs in RAM

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_VREG],
    i_reg: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

#[allow(dead_code)]
impl Emu {
    pub fn new() -> Self {
        Emu {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_VREG],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }

    // stack operation functions
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
