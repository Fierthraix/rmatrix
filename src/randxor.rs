static mut STATE: [u64; 4] = [4848, 99809, 2052667, 81119];

// Used to set range for printable characters
const RANDNUM: u8 = 93;
const RANDMIN: u8 = 33;

//TODO: See if we can make this panic! when not in the parent thread
pub fn gen() -> usize {
    let mut t;
    unsafe {
        t = STATE[3];
        t ^= t << 11;
        t ^= t << 8;
        STATE[3] = STATE[2];
        STATE[2] = STATE[1];
        STATE[1] = STATE[0];
        t ^= STATE[0];
        t ^= STATE[0] >> 19;
        STATE[0] = t;
    }
    t as usize
}

pub fn gen_char() -> char {
    (gen() as u8 % RANDNUM + RANDMIN) as char
}
