use crate::arithmetic::{call_sage_inverse_polynomial, parallel_dot_matrix_matrix, parallel_dot_matrix_vector, reshape, sample_random_mat, sample_random_vector, zero_mat, zero_vector};
use crate::helpers::println_with_timestamp;
use crate::protocol::protocol;
use crate::r#static::{CHUNKS, MOD_Q, MODULE_SIZE, RADIX, TIME};
use crate::ring_i128::Ring;
use crate::static_rings::static_generated::CONDUCTOR_COMPOSITE;

mod ring_i128;
mod poly_arithmetic_i128;
mod r#static;
mod static_rings;
mod root_of_unity;
mod arithmetic;
mod vdf;
mod ring_helpers;
mod crs;
mod subroutines;
mod protocol;
mod helpers;

fn main() {
    println_with_timestamp!("PARAMS: MODULE: {:?}, TIME: {:?}, CHUNKS: {:?}, Q: {:?}, CONDUCTOR: {:?}, RADIX: {:?}", MODULE_SIZE, TIME, CHUNKS, MOD_Q, CONDUCTOR_COMPOSITE, RADIX);
    let a = Ring::random();
    let b = call_sage_inverse_polynomial(&a).unwrap();
    assert_eq!(a * b, Ring::constant(1));
    println_with_timestamp!("OK sage");
    protocol()
}
