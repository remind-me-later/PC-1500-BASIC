// Decimal type for SHARP Pocket Computer PC-1500

#[derive(Debug, Clone, PartialEq)]
struct Decimal {
    bytes: [u8; 8],
    value: i64,
}

impl Decimal {
    const max_exponent: u8 = 0x99; // The maximum exponent for a decimal number in this machine is 10^99

    fn new(value: i64) -> Decimal {
        let mut bytes = [0; 8];
        bytes[0] = find_expontent(&value);
        bytes[1] = if value < 0 { 0x80 } else { 0x00 }; // 0x00 for positive mantissa, 0x80 for negative mantissa
        fill_mantissa(&value, &mut bytes);
        bytes[7] = 0x00; // 0x00

        Decimal { bytes, value }
    }

    fn find_expontent(&value: i64) -> u8 {
        // Find the exponent of the value

        let mut exp = 0x00;
        let mut val = value;
        while val >= 10 {
            // Leaving a number before the decimal point
            val /= 10;
            if exp >= 0x09 {
                exp += 0x10;
            } else {
                exp += 0x01;
            }
        }

        if exp > max_exponent {
            panic!("Number too big!");
        }

        exp
    }

    fn fill_mantissa(&value: i64, &mut bytes: [u8; 8]) {
        // Fill the mantissa with the value, bytes[2] to bytes[6]

        let mut val = value;
        let mut i = 6;
        while val > 0 && i > 1 {
            bytes[i] = (val % 10) as u8;
            val /= 10;
            i -= 1;
        }
    }

    fn get_value(&self) -> i64 {
        self.value
    }
}
