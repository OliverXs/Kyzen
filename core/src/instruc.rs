pub enum Opcode{
    //Arithmetics Integers
    ADD,
    SUB,
    MULT,
    DIV,
    REM,
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
    JUMP,
    HLG,
}