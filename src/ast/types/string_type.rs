// String type for SHARP Pocket Computer PC-1500

#[derive(Debug, Clone, PartialEq)]
struct String {
    bytes: [u8; 8],
    content: Vec<u8>,
}

impl String {
    fn new(value: &str) -> String {
        if value.len() > 156 {
            // 156 is the maximum length of a string for the PLC
            panic!("String too long");
        }

        let mut bytes = [0; 8];
        mem_dir = content.as_ptr() as usize as u16;
        bytes[4] = 0xD0;
        bytes[6] = mem_dir as u8; // The memory address of the string is saved in bytes[5] and bytes[6], 16 bits of memory address
        bytes[5] = (mem_dir >> 8) as u8;
        bytes[7] = value.len() as u8;

        fill_content(&value, &mut content);

        String {
            bytes,
            content: value.to_string(),
        }
    }

    fn fill_content(&value: &str, &mut content: Vec<u8>) {
        for c in value.chars() {
            content.push(c as u8);
        }
    }

    fn get_content(&self) -> &str {
        &self.content
    }
}
