#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KBlock {
    Keys = 1<<0,
    Multimedia = 1<<1,
    Gkeys = 1<<2,
    Logo = 1<<4,
    Modes = 1<<6,
}
use std::convert::TryFrom;
impl TryFrom<i64> for KBlock {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
	    v if v ==(KBlock::Keys as i64) => Ok(KBlock::Keys),
	    v if v ==(KBlock::Multimedia as i64) => Ok(KBlock::Multimedia),
	    v if v == (KBlock::Gkeys as i64) => Ok(KBlock::Gkeys),
	    v if v == (KBlock::Logo as i64) => Ok(KBlock::Logo),
	    v if v == (KBlock::Modes as i64) => Ok(KBlock::Modes),
	    _ => Err(format!("Not a valid block:{}", value)),
	}
    }
}

const STK_SIZE: usize = 256;
const SCANCODE_TO_KEYCODE: [u8; STK_SIZE] = [
      0,  0,  0,  0, 30, 48, 46, 32, 18, 33, 34, 35, 23, 36, 37, 38,    /* 0x00 */
     50, 49, 24, 25, 16, 19, 31, 20, 22, 47, 17, 45, 21, 44,  2,  3,    /* 0x10 */
      4,  5,  6,  7,  8,  9, 10, 11, 28,  1, 14, 15, 57, 12, 13, 26,    /* 0x20 */
     27, 43, 43, 39, 40, 41, 51, 52, 53, 58, 59, 60, 61, 62, 63, 64,    /* 0x30 */
     65, 66, 67, 68, 87, 88, 99, 70,119,110,102,104,111,107,109,106,    /* 0x40 */
    105,108,103, 69, 98, 55, 74, 78, 96, 79, 80, 81, 75, 76, 77, 71,    /* 0x50 */
     72, 73, 82, 83, 86,127,116,117,183,184,185,186,187,188,189,190,    /* 0x60 */
    191,192,193,194,134,138,130,132,128,129,131,137,133,135,136,113,    /* 0x70 */
    115,114,  0,  0,  0,121,  0, 89, 93,124, 92, 94, 95,  0,  0,  0,    /* 0x80 */
    122,123, 90, 91, 85,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,    /* 0x90 */
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,    /* 0xa0 */
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,    /* 0xb0 */
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,    /* 0xc0 */
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,    /* 0xd0 */
     29, 42, 56,125, 97, 54,100,126,164,166,165,163,161,115,114,113,    /* 0xe0 */
    150,158,159,128,136,177,178,176,142,152,173,140,  0,  0,  0,  0     /* 0xf0 */
];

pub fn scancode_to_keycode(sc: u8) -> Option<u8> {
    let sc = sc as usize;
    let sc = if sc < STK_SIZE { Some(sc)} else {None};
    sc.map(|s| SCANCODE_TO_KEYCODE[s])
        .filter(|s| s != &0u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scantokeycode() {
	let sc = 0x4; let key_a = 30;
	assert_eq!(scancode_to_keycode(sc), Some(key_a));
    }

    #[test]
    fn test_block() {
	assert_eq!(Ok(KBlock::Gkeys), TryFrom::try_from(4i64));
    }
}

