use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::near_bindgen;
use serde::{Deserialize, Serialize};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct A {
    a: u32,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct GasFeeTester {}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn global_noop() {}

#[allow(unused_variables)]
#[near_bindgen]
impl GasFeeTester {
    pub fn structure_noop() {}

    // Integers

    pub fn input_json_u32_a(a: u32) {}

    pub fn input_json_u32_aa(aa: u32) {}

    pub fn output_json_u32_a(a: u32) -> u32 {
        a
    }

    pub fn input_borsh_u32_a(#[serializer(borsh)] a: u32) {}

    #[result_serializer(borsh)]
    pub fn output_borsh_u32_a(#[serializer(borsh)] a: u32) -> u32 {
        a
    }

    pub fn input_json_u32_ab(a: u32, b: u32) {}

    pub fn input_borsh_u32_ab(#[serializer(borsh)] a: u32, #[serializer(borsh)] b: u32) {}

    // Strings

    pub fn input_json_string_s(s: String) {}

    pub fn input_borsh_string_s(#[serializer(borsh)] s: String) {}

    pub fn output_json_string_s(s: String) -> String {
        s
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_string_s(#[serializer(borsh)] s: String) -> String {
        s
    }

    // Vec<u8>

    pub fn input_json_vec_u8_v(v: Vec<u8>) {}

    pub fn input_borsh_vec_u8_v(#[serializer(borsh)] v: Vec<u8>) {}

    pub fn output_json_vec_u8_v(v: Vec<u8>) -> Vec<u8> {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_vec_u8_v(#[serializer(borsh)] v: Vec<u8>) -> Vec<u8> {
        v
    }

    // Vec<u32>

    pub fn input_json_vec_u32_v(v: Vec<u32>) {}

    pub fn input_borsh_vec_u32_v(#[serializer(borsh)] v: Vec<u32>) {}

    pub fn output_json_vec_u32_v(v: Vec<u32>) -> Vec<u32> {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_vec_u32_v(#[serializer(borsh)] v: Vec<u32>) -> Vec<u32> {
        v
    }

    // Simple Struct

    pub fn input_json_struct_a(a: A) {}

    pub fn input_borsh_struct_a(#[serializer(borsh)] a: A) {}

    pub fn output_json_struct_a(a: A) -> A {
        a
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_struct_a(#[serializer(borsh)] a: A) -> A {
        a
    }

    // Vec of vecs

    pub fn input_json_vec_vec_u8_v(v: Vec<Vec<u8>>) {}

    pub fn input_borsh_vec_vec_u8_v(#[serializer(borsh)] v: Vec<Vec<u8>>) {}

    pub fn output_json_vec_vec_u8_v(v: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_vec_vec_u8_v(#[serializer(borsh)] v: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        v
    }

    // Vec of strings

    pub fn input_json_vec_string_v(v: Vec<String>) {}

    pub fn input_borsh_vec_string_v(#[serializer(borsh)] v: Vec<String>) {}

    pub fn output_json_vec_string_v(v: Vec<String>) -> Vec<String> {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_vec_string_v(#[serializer(borsh)] v: Vec<String>) -> Vec<String> {
        v
    }

    // Fixed size arrays.

    pub fn input_json_array_0(v: [u8; 0]) {}

    pub fn input_borsh_array_0(#[serializer(borsh)] v: [u8; 0]) {}

    pub fn output_json_array_0(v: [u8; 0]) -> [u8; 0] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_0(#[serializer(borsh)] v: [u8; 0]) -> [u8; 0] {
        v
    }

    pub fn input_json_array_1(v: [u8; 1]) {}

    pub fn input_borsh_array_1(#[serializer(borsh)] v: [u8; 1]) {}

    pub fn output_json_array_1(v: [u8; 1]) -> [u8; 1] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_1(#[serializer(borsh)] v: [u8; 1]) -> [u8; 1] {
        v
    }

    pub fn input_json_array_2(v: [u8; 2]) {}

    pub fn input_borsh_array_2(#[serializer(borsh)] v: [u8; 2]) {}

    pub fn output_json_array_2(v: [u8; 2]) -> [u8; 2] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_2(#[serializer(borsh)] v: [u8; 2]) -> [u8; 2] {
        v
    }

    pub fn input_json_array_3(v: [u8; 3]) {}

    pub fn input_borsh_array_3(#[serializer(borsh)] v: [u8; 3]) {}

    pub fn output_json_array_3(v: [u8; 3]) -> [u8; 3] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_3(#[serializer(borsh)] v: [u8; 3]) -> [u8; 3] {
        v
    }

    pub fn input_json_array_4(v: [u8; 4]) {}

    pub fn input_borsh_array_4(#[serializer(borsh)] v: [u8; 4]) {}

    pub fn output_json_array_4(v: [u8; 4]) -> [u8; 4] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_4(#[serializer(borsh)] v: [u8; 4]) -> [u8; 4] {
        v
    }

    pub fn input_json_array_5(v: [u8; 5]) {}

    pub fn input_borsh_array_5(#[serializer(borsh)] v: [u8; 5]) {}

    pub fn output_json_array_5(v: [u8; 5]) -> [u8; 5] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_5(#[serializer(borsh)] v: [u8; 5]) -> [u8; 5] {
        v
    }

    pub fn input_json_array_6(v: [u8; 6]) {}

    pub fn input_borsh_array_6(#[serializer(borsh)] v: [u8; 6]) {}

    pub fn output_json_array_6(v: [u8; 6]) -> [u8; 6] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_6(#[serializer(borsh)] v: [u8; 6]) -> [u8; 6] {
        v
    }

    pub fn input_json_array_7(v: [u8; 7]) {}

    pub fn input_borsh_array_7(#[serializer(borsh)] v: [u8; 7]) {}

    pub fn output_json_array_7(v: [u8; 7]) -> [u8; 7] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_7(#[serializer(borsh)] v: [u8; 7]) -> [u8; 7] {
        v
    }

    pub fn input_json_array_8(v: [u8; 8]) {}

    pub fn input_borsh_array_8(#[serializer(borsh)] v: [u8; 8]) {}

    pub fn output_json_array_8(v: [u8; 8]) -> [u8; 8] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_8(#[serializer(borsh)] v: [u8; 8]) -> [u8; 8] {
        v
    }

    pub fn input_json_array_9(v: [u8; 9]) {}

    pub fn input_borsh_array_9(#[serializer(borsh)] v: [u8; 9]) {}

    pub fn output_json_array_9(v: [u8; 9]) -> [u8; 9] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_9(#[serializer(borsh)] v: [u8; 9]) -> [u8; 9] {
        v
    }

    pub fn input_json_array_10(v: [u8; 10]) {}

    pub fn input_borsh_array_10(#[serializer(borsh)] v: [u8; 10]) {}

    pub fn output_json_array_10(v: [u8; 10]) -> [u8; 10] {
        v
    }

    #[result_serializer(borsh)]
    pub fn output_borsh_array_10(#[serializer(borsh)] v: [u8; 10]) -> [u8; 10] {
        v
    }
}
