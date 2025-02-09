// Program inner structure for SHARP PC-1500 BASIC

#[derive(Debug, Clone, PartialEq)]
struct Program {
    bytes: [u8; 4],
    program: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
enum Basic_Command {
    ABS = 0xF170,
    ACS = 0xF174,
    AND = 0xF150,
    AREAD = 0xF180,
    ARUN = 0xF181,
    ASC = 0xF160,
    ASN = 0xF173,
    ATN = 0xF175,
    BEEP = 0xF182,
    BREAK = 0xF0B3,
    CALL = 0xF18A,
    CHAIN = 0xF0B2,
    CHR = 0xF163,
    CLEAR = 0xF187,
    CLOAD = 0xF089,
    CLS = 0xF088,
    COM = 0xF858,
    CONSOLE = 0xF0B1,
    CONT = 0xF183,
    COLOR = 0xF0B5,
    COS = 0xF17E,
    CSAVE = 0xF095,
    CSIZE = 0xE680,
    CURSOR = 0xF084,
    DATA = 0xF18D,
    DEF = 0xF165,
    DEGREE = 0xF18C,
    DEV = 0xE857,
    DIM = 0xF179,
    DMS = 0xF166,
    DTE = 0xE884,
    END = 0xF19E,
    ERL = 0xF053,
    ERN = 0xF052,
    ERROR = 0xF1B4,
    EXP = 0xF176,
    FEED = 0xF0B0,
    FOR = 0xF1A5,
    GOSUB = 0xF194,
    GOTO = 0xF192,
    IF = 0xF196,
    INPUT = 0xF091,
    INT = 0xF171,
    LET = 0xF198,
    LIST = 0xF090,
    LOG = 0xF177,
    MEM = 0xF158,
    MID = 0xF17B,
    NEW = 0xF19B,
    NEXT = 0xF19A,
    NOT = 0xF16D,
    OR = 0xF151,
    PRINT = 0xF097,
    RANDOM = 0xF1A8,
    RETURN = 0xF199,
    RUN = 0xF1A4,
    SGN = 0xF179,
    SIN = 0xF17D,
    SQR = 0xF16B,
    STOP = 0xF1AC,
    STR = 0xF161,
    TAN = 0xF17F,
    TIME = 0xF15B,
    VAL = 0xF162,
    WAIT = 0xF1B3,
}

impl Program {
    fn new(value: &str) -> Program {
        if value.len() > 1024 {
            // 1024 is the maximum length of a program for the PLC
            panic!("Program too long");
        }

        let mut bytes = [0; 4];
        mem_dir = program.as_ptr() as usize as u16;
        line_number = value[1].parse::<u16>().unwrap();
        bytes[1] = line_number as u8;
        bytes[0] = (line_number >> 8) as u8;
        bytes[2] = (value.len() - 1) as u8; // The length of the program - the line number
        bytes[3] = 0x0D; // 0x0D is the end of a line

        fill_program(&value, &mut program);

        Program {
            bytes,
            program: value.to_string(),
        }
    }

    fn fill_program(&value: &str, &mut program: Vec<u8>) {
        for word in value.split_whitespace() {
            match (word) {
                "ABS" | "ACS" | "AND" | "AREAD" | "ARUN" | "ASC" | "ASN" | "ATN" | "BEEP"
                | "BREAK" | "CALL" | "CHAIN" | "CHR" | "CLEAR" | "CLOAD" | "CLS" | "COM"
                | "CONSOLE" | "CONT" | "COLOR" | "COS" | "CSAVE" | "CSIZE" | "CURSOR" | "DATA"
                | "DEF" | "DEGREE" | "DEV" | "DIM" | "DMS" | "DTE" | "END" | "ERL" | "ERN"
                | "ERROR" | "EXP" | "FEED" | "FOR" | "GOSUB" | "GOTO" | "IF" | "INPUT" | "INT"
                | "LET" | "LIST" | "LOG" | "MEM" | "MID" | "NEW" | "NEXT" | "NOT" | "OR"
                | "PRINT" | "RANDOM" | "RETURN" | "RUN" | "SGN" | "SIN" | "SQR" | "STOP"
                | "STR" | "TAN" | "TIME" | "VAL" | "WAIT" => {
                    program.push((Basic_Command::ABS >> 8) as u8);
                    program.puch(Basic_Command::ABS as u8);
                }
                _ => {
                    // TODO Implement number and string handling
                }
            }
        }
    }

    fn get_program(&self) -> &str {
        &self.program
    }
}
