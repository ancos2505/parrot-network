//! These opcodes are used in Parrot VM to define conditions and
//! operations in scripts.
//!

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Opcode {
    /// Alias for OP_PUSHBYTES_0
    OP_0,
    /// Alias for OP_PUSHBYTES_0
    /// Empty stack is also FALSE.
    OP_FALSE,
    /// Alias for OP_PUSHNUM_1
    /// Number 1 is also TRUE.
    OP_TRUE,
    ///  0x00, "Push an empty array onto the stack."
    OP_PUSHBYTES_0,
    ///  0x01, "Push the next byte as an array onto the stack."
    OP_PUSHBYTES_1,
    ///  0x02, "Push the next 2 bytes as an array onto the stack."
    OP_PUSHBYTES_2,
    ///  0x03, "Push the next 3 bytes as an array onto the stack."
    OP_PUSHBYTES_3,
    ///  0x04, "Push the next 4 bytes as an array onto the stack."
    OP_PUSHBYTES_4,
    ///  0x4c, "Read the next byte as N; push the next N bytes as an array onto the stack."
    OP_PUSHDATA1,
    ///  0x4d, "Read the next 2 bytes as N; push the next N bytes as an array onto the stack."
    OP_PUSHDATA2,
    ///  0x4e, "Read the next 4 bytes as N; push the next N bytes as an array onto the stack."
    OP_PUSHDATA4,
    ///  0x4f, "Push the array `0x81` onto the stack."
    OP_PUSHNUM_NEG1,
    ///  0x50, "Synonym for OP_RETURN."
    OP_RESERVED,
    ///  0x51, "Push the array `0x01` onto the stack."
    OP_PUSHNUM_1,
    ///  0x52, "Push the array `0x02` onto the stack."
    OP_PUSHNUM_2,
    ///  0x53, "Push the array `0x03` onto the stack."
    OP_PUSHNUM_3,
    ///  0x54, "Push the array `0x04` onto the stack."
    OP_PUSHNUM_4,
    ///  0x61, "Does nothing."
    OP_NOP,
    ///  0x62, "Synonym for OP_RETURN."
    OP_VER,
    ///  0x63, "Pop and execute the next statements if a nonzero element was popped."
    OP_IF,
    ///  0x64, "Pop and execute the next statements if a zero element was popped."
    OP_NOTIF,
    ///  0x65, "Fail the script unconditionally, does not even need to be executed."
    OP_VERIF,
    ///  0x66, "Fail the script unconditionally, does not even need to be executed."
    OP_VERNOTIF,
    /// 0x67, "Execute statements if those after the previous OP_IF were not,
    ///       and vice-versa. If there is no previous OP_IF, this acts as a
    ///       RETURN."
    OP_ELSE,
    ///  0x68, "Pop and execute the next statements if a zero element was popped."
    OP_ENDIF,
    ///  0x69, "If the top value is zero or the stack is empty, fail; otherwise, pop the stack."
    OP_VERIFY,
    ///  0x6a, "Fail the script immediately. (Must be executed.)."
    OP_RETURN,
    ///  0x6b, "Pop one element from the main stack onto the alt stack."
    OP_TOALTSTACK,
    ///  0x6c, "Pop one element from the alt stack onto the main stack."
    OP_FROMALTSTACK,
    ///  0x6d, "Drops the top two stack items."
    OP_2DROP,
    ///  0x6e, "Duplicates the top two stack items as AB -> ABAB."
    OP_2DUP,
    ///  0x6f, "Duplicates the two three stack items as ABC -> ABCABC."
    OP_3DUP,
    ///  0x70, "Copies the two stack items of items two spaces back to the front, as xxAB -> ABxxAB."
    OP_2OVER,
    ///  0x71, "Moves the two stack items four spaces back to the front, as xxxxAB -> ABxxxx."
    OP_2ROT,
    ///  0x72, "Swaps the top two pairs, as ABCD -> CDAB."
    OP_2SWAP,
    ///  0x73, "Duplicate the top stack element unless it is zero."
    OP_IFDUP,
    ///  0x74, "Push the current number of stack items onto the stack."
    OP_DEPTH,
    ///  0x75, "Drops the top stack item."
    OP_DROP,
    ///  0x76, "Duplicates the top stack item."
    OP_DUP,
    ///  0x77, "Drops the second-to-top stack item."
    OP_NIP,
    ///  0x78, "Copies the second-to-top stack item, as xA -> AxA."
    OP_OVER,
    ///  0x79, "Pop the top stack element as N. Copy the Nth stack element to the top."
    OP_PICK,
    ///  0x7a, "Pop the top stack element as N. Move the Nth stack element to the top."
    OP_ROLL,
    ///  0x7b, "Rotate the top three stack items, as [top next1 next2] -> [next2 top next1]."
    OP_ROT,
    ///  0x7c, "Swap the top two stack items."
    OP_SWAP,
    ///  0x7d, "Copy the top stack item to before the second item, as [top next] -> [top next top]."
    OP_TUCK,
    ///  0x7e, "Fail the script unconditionally, does not even need to be executed."
    OP_CAT,
    ///  0x7f, "Fail the script unconditionally, does not even need to be executed."
    OP_SUBSTR,
    ///  0x80, "Fail the script unconditionally, does not even need to be executed."
    OP_LEFT,
    ///  0x81, "Fail the script unconditionally, does not even need to be executed."
    OP_RIGHT,
    ///  0x82, "Pushes the length of the top stack item onto the stack."
    OP_SIZE,
    ///  0x83, "Fail the script unconditionally, does not even need to be executed."
    OP_INVERT,
    ///  0x84, "Fail the script unconditionally, does not even need to be executed."
    OP_AND,
    ///  0x85, "Fail the script unconditionally, does not even need to be executed."
    OP_OR,
    ///  0x86, "Fail the script unconditionally, does not even need to be executed."
    OP_XOR,
    ///  0x87, "Pushes 1 if the inputs are exactly equal, 0 otherwise."
    OP_EQUAL,
    ///  0x88, "Returns success if the inputs are exactly equal, failure otherwise."
    OP_EQUALVERIFY,
    ///  0x89, "Synonym for OP_RETURN."
    OP_RESERVED1,
    ///  0x8a, "Synonym for OP_RETURN."
    OP_RESERVED2,
    ///  0x8b, "Increment the top stack element in place."
    OP_1ADD,
    ///  0x8c, "Decrement the top stack element in place."
    OP_1SUB,
    ///  0x90, "Absolute value the top stack item in place."
    OP_ABS,
    ///  0x91, "Map 0 to 1 and everything else to 0, in place."
    OP_NOT,
    ///  0x92, "Map 0 to 0 and everything else to 1, in place."
    OP_0NOTEQUAL,
    ///  0x93, "Pop two stack items and push their sum."
    OP_ADD,
    ///  0x94, "Pop two stack items and push the second minus the top."
    OP_SUB,
    ///  0x9a, "Pop the top two stack items and push 1 if both are nonzero, else push 0."
    OP_BOOLAND,
    ///  0x9b, "Pop the top two stack items and push 1 if either is nonzero, else push 0."
    OP_BOOLOR,
    ///  0x9c, "Pop the top two stack items and push 1 if both are numerically equal, else push 0."
    OP_NUMEQUAL,
    ///  0x9d, "Pop the top two stack items and return success if both are numerically equal, else return failure."
    OP_NUMEQUALVERIFY,
    ///  0x9e, "Pop the top two stack items and push 0 if both are numerically equal, else push 1."
    OP_NUMNOTEQUAL,
    ///  0x9f, "Pop the top two items; push 1 if the second is less than the top, 0 otherwise."
    OP_LESSTHAN,
    ///  0xa0, "Pop the top two items; push 1 if the second is greater than the top, 0 otherwise."
    OP_GREATERTHAN,
    ///  0xa1, "Pop the top two items; push 1 if the second is <= the top, 0 otherwise."
    OP_LESSTHANOREQUAL,
    ///  0xa2, "Pop the top two items; push 1 if the second is >= the top, 0 otherwise."
    OP_GREATERTHANOREQUAL,
    ///  0xa3, "Pop the top two items; push the smaller."
    OP_MIN,
    ///  0xa4, "Pop the top two items; push the larger."
    OP_MAX,
    ///  0xa5, "Pop the top three items; if the top is >= the second and < the third, push 1, otherwise push 0."
    OP_WITHIN,
    ///  0xa8, "Pop the top stack item and push its SHA256 hash."
    OP_SHA256,
}
