
extern "C" {
    // Curve operations
    pub fn to_affine_bn254_c(p_ptr: *const u64, out_ptr: *mut u64) -> bool;
    pub fn is_on_curve_bn254_c(p_ptr: *const u64) -> bool;
    pub fn add_bn254_c(p1_ptr: *const u64, p2_ptr: *const u64, out_ptr: *mut u64) -> bool;
    pub fn mul_bn254_c(p_ptr: *const u64, k_ptr: *const u64, out_ptr: *mut u64) -> bool;

    // Twist operations
    pub fn to_affine_twist_bn254_c(p_ptr: *const u64, out_ptr: *mut u64);
    pub fn is_on_curve_twist_bn254_c(p_ptr: *const u64) -> bool;
    pub fn is_on_subgroup_twist_bn254_c(p_ptr: *const u64) -> bool;

    // Pairing
    pub fn pairing_batch_bn254_c(g1_ptr: *const u64, g2_ptr: *const u64, num_points: usize, out_ptr: *mut u64);
}