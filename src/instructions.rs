




#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instructions {
    Add,
    Push1(u8),
}

const INSTRUCTIONS: Vec<Instruction> = vec![
    OP_STOP,
    OP_ADD,
    OP_MUL,
    OP_SUB,
    OP_DIV,
    OP_SDIV,
    OP_MOD,
    OP_SMOD,
    OP_ADDMOD,
    OP_MULMOD,
    OP_EXP,
    OP_SIGNEXTEND,
    OP_LT,
    OP_GT,
    OP_SLT,
    OP_SGT,
    OP_EQ,
    OP_ISZERO,
    OP_AND,
    OP_OR,
    OP_NOR,
    OP_NOT,
    OP_BYTE,
    OP_SHA3,
    OP_ADDRESS,
    OP_BALANCE,
    OP_ORIGIN,
    OP_CALLER,
    OP_CALLVALUE,
    OP_CALLDATALOAD,
    OP_CALLDATASIZE,
    OP_CALLDATACOPY,
    OP_CODESIZE,
    OP_CODECOPY,
    OP_GASPRICE,
    OP_EXTCODESIZE,
    OP_EXTCODECOPY,
    OP_BLOCKHASH,
    OP_COINBASE,
    OP_TIMESTAMP,
    OP_NUMBER,
    OP_DIFFICULTY,
    OP_GASLIMIT,
    OP_POP,
    OP_MLOAD,
    OP_MSTORE,
    OP_MSTORE8,
    OP_SLOAD,
    OP_SSTORE,
    OP_JUMP,
    OP_JUMP1,
    OP_PC,
    OP_MSIZE,
    OP_GAS,
    OP_JUMPDEST,
    //OP_PUSH1,
    OP_LOG0,
    OP_LOG1,
    OP_LOG2,
    OP_LOG3,
    OP_LOG4,
    OP_CREATE,
    OP_CALL,
    OP_CALLCODE,
    OP_RETURN,
    OP_DELEGATECALL,
    OP_INVALID,
    OP_SELFDESTRUCT,
];

#[derive(Debug)]
struct Instruction {
    op: u8,
    name: &'static str,
}

const OP_STOP: Instruction =  Instruction {op: 0x00, name: "STOP"};
const OP_ADD: Instruction =  Instruction {op: 0x01, name: "ADD"};
const OP_MUL: Instruction =  Instruction {op: 0x02, name: "MUL"};
const OP_SUB: Instruction =  Instruction {op: 0x03, name: "SUB"};
const OP_DIV: Instruction =  Instruction {op: 0x04, name: "DIV"};
const OP_SDIV: Instruction =  Instruction {op: 0x05, name: "SDIV"};
const OP_MOD: Instruction =  Instruction {op: 0x06, name: "MOD"};
const OP_SMOD: Instruction =  Instruction {op: 0x07, name: "SMOD"};
const OP_ADDMOD: Instruction =  Instruction {op: 0x08, name: "ADDMOD"};
const OP_MULMOD: Instruction =  Instruction {op: 0x09, name: "MULMOD"};
const OP_EXP: Instruction =  Instruction {op: 0x0a, name: "EXP"};
const OP_SIGNEXTEND: Instruction =  Instruction {op: 0x0b, name: "SIGNEXTEND"};
const OP_LT: Instruction =  Instruction {op: 0x10, name: "LT"};
const OP_GT: Instruction =  Instruction {op: 0x11, name: "GT"};
const OP_SLT: Instruction =  Instruction {op: 0x12, name: "SLT"};
const OP_SGT: Instruction =  Instruction {op: 0x13, name: "SGT"};
const OP_EQ: Instruction =  Instruction {op: 0x14, name: "EQ"};
const OP_ISZERO: Instruction =  Instruction {op: 0x15, name: "ISZERO"};
const OP_AND: Instruction =  Instruction {op: 0x16, name: "AND"};
const OP_OR: Instruction =  Instruction {op: 0x17, name: "OR"};
const OP_NOR: Instruction =  Instruction {op: 0x18, name: "NOR"};
const OP_NOT: Instruction =  Instruction {op: 0x19, name: "NOT"};
const OP_BYTE: Instruction =  Instruction {op: 0x1a, name: "BYTE"};
const OP_SHA3: Instruction =  Instruction {op: 0x20, name: "SHA3"};
const OP_ADDRESS: Instruction =  Instruction {op: 0x30, name: "ADDRESS"};
const OP_BALANCE: Instruction =  Instruction {op: 0x31, name: "BALANCE"};
const OP_ORIGIN: Instruction =  Instruction {op: 0x32, name: "ORIGIN"};
const OP_CALLER: Instruction =  Instruction {op: 0x33, name: "CALLER"};
const OP_CALLVALUE: Instruction =  Instruction {op: 0x34, name: "CALLVALUE"};
const OP_CALLDATALOAD: Instruction =  Instruction {op: 0x35, name: "CALLDATALOAD"};
const OP_CALLDATASIZE: Instruction =  Instruction {op: 0x36, name: "CALLDATASIZE"};
const OP_CALLDATACOPY: Instruction =  Instruction {op: 0x37, name: "CALLDATACOPY"};
const OP_CODESIZE: Instruction =  Instruction {op: 0x38, name: "CODESIZE"};
const OP_CODECOPY: Instruction =  Instruction {op: 0x39, name: "CODECOPY"};
const OP_GASPRICE: Instruction =  Instruction {op: 0x3a, name: "GASPRICE"};
const OP_EXTCODESIZE: Instruction =  Instruction {op: 0x3b, name: "EXTCODESIZE"};
const OP_EXTCODECOPY: Instruction =  Instruction {op: 0x3c, name: "EXTCODECOPY"};
const OP_BLOCKHASH: Instruction =  Instruction {op: 0x40, name: "BLOCKHASH"};
const OP_COINBASE: Instruction =  Instruction {op: 0x41, name: "COINBASE"};
const OP_TIMESTAMP: Instruction =  Instruction {op: 0x42, name: "TIMESTAMP"};
const OP_NUMBER: Instruction =  Instruction {op: 0x43, name: "NUMBER"};
const OP_DIFFICULTY: Instruction =  Instruction {op: 0x44, name: "DIFFICULTY"};
const OP_GASLIMIT: Instruction =  Instruction {op: 0x45, name: "GASLIMIT"};
const OP_POP: Instruction =  Instruction {op: 0x50, name: "POP"};
const OP_MLOAD: Instruction =  Instruction {op: 0x51, name: "MLOAD"};
const OP_MSTORE: Instruction =  Instruction {op: 0x52, name: "MSTORE"};
const OP_MSTORE8: Instruction =  Instruction {op: 0x53, name: "MSTORE8"};
const OP_SLOAD: Instruction =  Instruction {op: 0x54, name: "SLOAD"};
const OP_SSTORE: Instruction =  Instruction {op: 0x55, name: "SSTORE"};
const OP_JUMP: Instruction =  Instruction {op: 0x56, name: "JUMP"};
const OP_JUMP1: Instruction =  Instruction {op: 0x57, name: "JUMP1"};
const OP_PC: Instruction =  Instruction {op: 0x58, name: "PC"};
const OP_MSIZE: Instruction =  Instruction {op: 0x59, name: "MSIZE"};
const OP_GAS: Instruction =  Instruction {op: 0x5a, name: "GAS"};
const OP_JUMPDEST: Instruction =  Instruction {op: 0x5b, name: "JUMPDEST"};
const OP_PUSH1: Instruction =  Instruction {op: 0x60, name: "PUSH1"};
// 0x60 -- 0x7f PUSH
// 0x80 -- 0x8f OUP
// 0x90 --   
const OP_LOG0: Instruction =  Instruction {op: 0xa0, name: "LOG0"};
const OP_LOG1: Instruction =  Instruction {op: 0xa1, name: "LOG1"};
const OP_LOG2: Instruction =  Instruction {op: 0xa2, name: "LOG2"};
const OP_LOG3: Instruction =  Instruction {op: 0xa3, name: "LOG3"};
const OP_LOG4: Instruction =  Instruction {op: 0xa4, name: "LOG4"};
const OP_CREATE: Instruction =  Instruction {op: 0xf0, name: "CREATE"};
const OP_CALL: Instruction =  Instruction {op: 0xf1, name: "CALL"};
const OP_CALLCODE: Instruction =  Instruction {op: 0xf2, name: "CALLCODE"};
const OP_RETURN: Instruction =  Instruction {op: 0xf3, name: "RETURN"};
const OP_DELEGATECALL: Instruction =  Instruction {op: 0xf4, name: "DELEGATECALL"};
const OP_INVALID: Instruction =  Instruction {op: 0xfe, name: "INVALID"};
const OP_SELFDESTRUCT: Instruction =  Instruction {op: 0xff, name: "SELFDESTRUCT"};

