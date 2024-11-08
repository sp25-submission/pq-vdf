pub static CONDUCTOR: usize = 257;
// pub static CONDUCTOR: usize = 13;
pub static MOD_Q: i128 = 4611686019232694273;
pub static BIG_MOD_Q: &str = "1324325423464534264434434342342342342345325346352367564534123546753";
pub static  LOG_Q: usize = 62;
pub static  LOG_BIG_Q: usize = 220;
pub static N_DIM: usize = 10;

pub type BASE_INT = i128;

// pub static MODULE_SIZE: usize = 4;
// pub static MODULE_SIZE: usize = 2;
pub static MODULE_SIZE: usize = 416;
// pub static MODULE_SIZE: usize = 2;
pub static COMMITMENT_MODULE_SIZE: usize = MODULE_SIZE / 2;
pub static RADIX: BASE_INT = 21;


pub static CHUNKS: usize = 57;

// pub static TIME: usize = 14592;
pub static TIME: usize = 7296;

pub static CHUNK_SIZE: usize = TIME / CHUNKS;




// MOD_1 > N LOG_Q * Conductor^2
// pub(crate) static MOD_1: i128 = 69206017;
// 2^M | MOD - 1
// either MOD_1 = MOD_Q or MOD_Q so large that no overflow happens
pub(crate) static MOD_1: i128 = MOD_Q;


