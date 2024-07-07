use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FullAddrOutput {
    sum: bool,
    carry: bool,
}

#[wasm_bindgen]
impl FullAddrOutput {
   #[wasm_bindgen(constructor)]
   pub fn new(sum: bool, carry: bool) -> FullAddrOutput {
      FullAddrOutput { sum, carry }
   }

   pub fn get_sum(&self) -> bool {
    self.sum
   }

   pub fn get_carry(&self) -> bool {
    self.carry
   }
}


// Function to toggle a boolean value (used in Level 1)
#[wasm_bindgen]
pub fn toggle_transistor(on: bool) -> bool {
    !on
}

// Function to perform an AND operation on two boolean values (used in Level 2)
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
pub fn full_adder(a: bool, b: bool, carry_in: bool) -> FullAddrOutput {
    let sum1 = xor_gate(a, b);
    let carry1 = and_gate(a, b);
    let sum2 = xor_gate(sum1, carry_in);
    let carry2 = and_gate(sum1, carry_in);
    let carry_out = or_gate(carry1, carry2);
    FullAddrOutput { sum: sum2, carry: carry_out }
}