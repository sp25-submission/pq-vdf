use ndarray::s;
use crate::arithmetic::{add_matrices, add_vectors, columns, first_n_columns, join_matrices_horizontally, last_n_columns, multiply_matrix_constant_in_place, parallel_dot_matrix_matrix, parallel_dot_matrix_vector, reshape, row_wise_tensor, sample_random_bin_mat, sample_random_bin_vec, sample_random_mat, sample_random_vector, split_into_submatrices_by_columns, vector_element_product};
use crate::crs::{CRS, gen_crs};
use crate::r#static::{BASE_INT, CHUNK_SIZE, CHUNKS, LOG_Q, MODULE_SIZE, RADIX, TIME};
use crate::ring_helpers::{chunk_witness, tensor_identity_matrix_with_vector, transpose};
use crate::ring_i128::{get_g, get_g_custom, Ring, ring_inner_product, RingElement};
use crate::subroutines::bdecomp::{b_decomp, DecompOutput};
use crate::subroutines::norm_first_round::{Norm1Output, norm_1};
use crate::subroutines::norm_second_round::{Norm2Output, norm_2};
use crate::subroutines::split::{get_power_series_multiplier, split, SplitOutput};
use crate::vdf::{execute_vdf, flat_vdf};
use std::time::{Duration, Instant};
use rayon::iter::IntoParallelIterator;
use crate::helpers::println_with_timestamp;
use crate::subroutines::fold::fold;
use crate::subroutines::verifier::{challenge_for_fold, norm_challenge, verifier_fold, verifier_squeeze, verify_bdecomp, verify_norm_2, verify_split};

pub(crate) struct VerifierState {
    pub(crate) wit_cols: usize,
    pub(crate) wit_rows: usize,
    pub(crate) commitment: Vec<Vec<RingElement>>
}


pub fn vdf_flatten_challenge() -> RingElement {
     Ring::random()
}




pub fn protocol() {
    let crs = gen_crs();
    let y_a = sample_random_vector(MODULE_SIZE);

    let now = Instant::now();
    let output = execute_vdf(&y_a, &crs.a, CHUNKS, TIME);
    let output_witness = output.witness.clone(); // TODO!!
    // let output_witness = sample_random_bin_vec(MODULE_SIZE * LOG_Q * TIME);
    let vdf_elapsed = now.elapsed();
    let mut verifier_runtime = Instant::now().elapsed();
    let mut prover_runtime = Instant::now().elapsed();
    println_with_timestamp!("Time for execute_vdf: {:.2?}", vdf_elapsed);

    let now = Instant::now();
    let mut witness = transpose(&reshape(&output_witness, CHUNK_SIZE * MODULE_SIZE * LOG_Q));
    let elapsed = now.elapsed();
    println_with_timestamp!("Time for transpose & reshape: {:.2?}", elapsed);
    prover_runtime = prover_runtime + elapsed;
    let now = Instant::now();
    let (result, r, power) = verifier_squeeze(&crs, &output, y_a, CHUNK_SIZE);
    let elapsed = now.elapsed();
    println_with_timestamp!("Time for VDF squeeze challenge: {:.2?}", elapsed);
    verifier_runtime = verifier_runtime + elapsed;

    let now = Instant::now();
    let mut commitment = parallel_dot_matrix_matrix(&crs.ck, &witness);
    let elapsed = now.elapsed();

    println_with_timestamp!("Time for parallel_dot_matrix_matrix (commitment): {:.2?}", elapsed);
    prover_runtime = prover_runtime + elapsed;

    let mut verifier_state = VerifierState {
        wit_cols: CHUNKS,
        wit_rows: CHUNK_SIZE * MODULE_SIZE * LOG_Q,
        commitment: vec![vec![r], commitment].concat(),
    };

    let mut statement = vec![vec![result], crs.ck].concat();


   for i in 0..6 {
    // for i in 0..5 {
        if i == 3 || i == 5 {
        // if true {
            let now = Instant::now();
            let radix = RADIX;
            let (new_witness, bdecomp_output) = b_decomp(&statement, &witness, radix);
            witness = new_witness;
            let elapsed = now.elapsed();
            println_with_timestamp!("Time for b_decomp: {:.2?}", elapsed);
            prover_runtime = prover_runtime + elapsed;


            let now = Instant::now();
            let new_verifier_state = verify_bdecomp(&bdecomp_output, &statement, &verifier_state, radix);
            verifier_state = new_verifier_state;
            let elapsed = now.elapsed();
            println_with_timestamp!("Time for verify_bdecomp: {:.2?}", elapsed);
            verifier_runtime = verifier_runtime + elapsed;

            assert_eq!(parallel_dot_matrix_matrix(&statement, &witness), verifier_state.commitment);
        }


        let now = Instant::now();
        let (new_witness, norm_1_output) = norm_1(&statement, &witness);
        witness = new_witness;
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for norm_1: {:.2?}", elapsed);
        prover_runtime = prover_runtime + elapsed;

        let now = Instant::now();
        let (new_verifier_state, challenge, inverse_challenge) = norm_challenge(&norm_1_output, &verifier_state);
        verifier_state = new_verifier_state;
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for norm_challenge: {:.2?}", elapsed);
        verifier_runtime = verifier_runtime + elapsed;

        let now = Instant::now();
        let (new_power_series, norm_2_output) = norm_2(&statement, &witness, &challenge, &inverse_challenge, i == 0);
        statement = new_power_series;
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for norm_2: {:.2?}", elapsed);
        prover_runtime = prover_runtime + elapsed;

        let now = Instant::now();
        verifier_state = verify_norm_2(&norm_1_output, &norm_2_output, &verifier_state, i == 0);
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for verify_norm_2: {:.2?}", elapsed);
        verifier_runtime = verifier_runtime + elapsed;

        // assert_eq!(parallel_dot_matrix_matrix(&statement, &witness), verifier_state.commitment);
        let now = Instant::now();
        let (new_witness, new_power_series, split_output) = split(&statement, &witness);
        witness = new_witness;
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for split: {:.2?}", elapsed);
        prover_runtime = prover_runtime + elapsed;

        let now = Instant::now();
        verifier_state = verify_split(&statement, &split_output, &verifier_state, &power);
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for verify_split: {:.2?}", elapsed);
        verifier_runtime = verifier_runtime + elapsed;

        // we need to keep the old statement for the verifier.
        statement = new_power_series;


        let now = Instant::now();
        let challenge = challenge_for_fold(&verifier_state);
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for challenge fold: {:.2?}", elapsed);
        verifier_runtime = verifier_runtime + elapsed;

        let now = Instant::now();
        let new_witness = fold(&witness, &challenge);
        witness = new_witness;
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for fold: {:.2?}", elapsed);
        prover_runtime = prover_runtime + elapsed;

        let now = Instant::now();
        verifier_state = verifier_fold(&verifier_state, &challenge);
        let elapsed = now.elapsed();
        println_with_timestamp!("Time for fold verifier: {:.2?}", elapsed);
        verifier_runtime = verifier_runtime + elapsed;
    }



    let now = Instant::now();
    assert_eq!(parallel_dot_matrix_matrix(&statement, &witness), verifier_state.commitment);
    let elapsed = now.elapsed();
    println_with_timestamp!("Time for final assert_eq: {:.2?}", elapsed);
    verifier_runtime = verifier_runtime + elapsed;


    println_with_timestamp!("VDF: {:.2?}", vdf_elapsed);
    println_with_timestamp!("PRV: {:.2?}", prover_runtime);
    println_with_timestamp!("VER: {:.2?}", verifier_runtime);

}
