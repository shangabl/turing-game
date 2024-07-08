use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn toggle_transistor(on: bool) -> bool {
    !on
}

#[wasm_bindgen]
pub fn and_gate(input1: bool, input2: bool) -> bool {
    input1 && input2
}

#[wasm_bindgen]
pub fn or_gate(input1: bool, input2: bool) -> bool {
    input1 || input2
}

#[wasm_bindgen]
pub fn not_gate(input: bool) -> bool {
    !input
}

#[wasm_bindgen]
pub fn nand_gate(input1: bool, input2: bool) -> bool {
    if input1 && input2 {
        return false
    }
    true
}

#[wasm_bindgen]
pub fn nor_gate(input1: bool, input2: bool) -> bool {
    if !input1 && !input2 {
        return true
    }
    false
}

#[wasm_bindgen]
pub fn xor_gate(input1: bool, input2: bool) -> bool {
    if input1 == input2 {
        return false
    }
    true
}

#[wasm_bindgen]
pub fn xnor_gate(input1: bool, input2: bool) -> bool {
    input1 == input2
}

#[wasm_bindgen]
pub fn set_bit(position: u8) -> u8 {
    1 << position
}

#[wasm_bindgen]
pub struct FullAdderResult {
    pub sum: bool,
    pub carry: bool,
}

#[wasm_bindgen]
pub fn full_adder(a: bool, b: bool, carry_in: bool) -> FullAdderResult {
    let sum = a ^ b ^ carry_in; // Sum bit
    let carry = (a & b) | (carry_in & (a ^ b));
    FullAdderResult { sum, carry }
}