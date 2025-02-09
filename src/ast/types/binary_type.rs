// Binary type for SHARP Pocket Computer PC-1500

#[derive(Debug, Clone, PartialEq)]
struct Binary {
    bytes: [u8; 8],
    value: i64,
}

impl Binary {
    fn new(value: i64) -> Binary {
        // The unused bits are unused but still reserved as part of the binary number memory space
        let mut bytes = [0; 8];
        let mut val = value;
        bytes[4] = 0xB2;
        bytes[5] = val as u8;
        bytes[6] = (val >> 8) as u8;

        Binary { bytes, value }
    }

    fn get_value(&self) -> i64 {
        self.value
    }
}
