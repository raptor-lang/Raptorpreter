
enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    NOP = 0x00,
    HALT = 0x01,
    ICONST = 0x02,
    POP = 0x03,
    ADD = 0x1A,
    SUB = 0x1B,
    MULTIPLY = 0x1C,
    DIVIDE = 0x1D,
    MODULUS = 0x1E,
    AND = 0x11,
    OR = 0x12,
    NOT = 0x13,
    RSHIFT = 0x14,
    LSHIFT = 0x15,
    COMP = 0x20,
    COMP_LT = 0x21,
    COMP_EQ = 0x22,
    COMP_GT = 0x23,
    RELJUMP = 0x2A,
    RELJUMP_GT = 0x2B,
    RELJUMP_LT = 0x2C,
    RELJUMP_EQ = 0x2D,
    STORE = 0x80,
    LOAD = 0x81,
    PRINT = 0xA0,
    DUMP_STACK = 0xFA,
    DUMP_GLOBALS = 0xFB,
}
}