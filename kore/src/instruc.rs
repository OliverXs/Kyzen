pub enum Opcode{
    //Arithmetics Integers
    ADD, //0xA1
    SUB, //0xA2
    MULT, //0xA3
    DIV, //0xA4
    REM, //0xA5
    //Arithmetics Floating point
    FADD,
    FSUB,
    FMULT,
    FDIV,
    //Logic operands
    MOV,

    //CONTROL FLOW
    LOOP,
    CPM,

    //Breaks
    CALL,
    JUMP, //0xF1
    HLG,
}

pub enum Registers{
    R1, //0x01 .. 0x1F
    R2,
    R3,

    XR1, //0x20 .. 0x3F
}