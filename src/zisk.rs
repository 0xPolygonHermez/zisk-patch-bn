
extern "C" {
    // Curve operations
    pub fn jacobian_to_affine_bn254_c(p_ptr: *const u64, result_ptr: *mut u64) -> u8;
    pub fn is_on_curve_bn254_c(p_ptr: *const u64) -> u8;
    pub fn add_bn254_c(p1_ptr: *const u64, p2_ptr: *const u64, result_ptr: *mut u64) -> u8;
    pub fn scalar_mul_bn254_c(p_ptr: *const u64, k_ptr: *const u64, result_ptr: *mut u64) -> u8;

    // Twist operations
    pub fn jacobian_to_affine_twist_bn254_c(p_ptr: *const u64, result_ptr: *mut u64) -> u8;
    pub fn is_on_curve_twist_bn254_c(p_ptr: *const u64) -> u8;
    pub fn is_on_subgroup_twist_bn254_c(p_ptr: *const u64) -> u8;

    // Pairing
    pub fn pairing_batch_bn254_c(g1_ptr: *const u64, g2_ptr: *const u64, num_points: usize, result_ptr: *mut u64);
}