use crate::frame::element_wise::*;
use crate::frame::mmm::*;

extern_kernel!(fn armv7neon_mmm_i8_8x4(op: *const MatMatMulKerSpec<i32>) -> isize);
extern_kernel!(fn armv7neon_mmm_f32_8x4(op: *const MatMatMulKerSpec<f32>) -> isize);
extern_kernel!(fn armv7neon_mmm_f32_32x1(op: *const MatMatMulKerSpec<f32>) -> isize);
extern_kernel!(fn armv7neon_sigmoid_f32_4n(ptr: *mut f32, count: usize) -> ());
extern_kernel!(fn armv7neon_tanh_f32_4n(ptr: *mut f32, count: usize) -> ());
extern_kernel!(fn armv7neon_prefetch(start: *const u8, end: *const u8) -> ());

pub fn prefetch(start: *const u8, len: usize) {
    unsafe { armv7neon_prefetch(start, start.offset(len as isize)) }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulI8x8x4;

impl MatMatMulKer<i32> for MatMatMulI8x8x4 {
    #[inline(always)]
    fn name() -> &'static str {
        "neon"
    }
    #[inline(always)]
    fn mr() -> usize {
        8
    }
    #[inline(always)]
    fn nr() -> usize {
        4
    }
    fn alignment_bytes_packed_a() -> usize {
        32
    }
    fn alignment_bytes_packed_b() -> usize {
        4
    }
    fn end_padding_packed_a() -> usize {
        0
    }
    fn end_padding_packed_b() -> usize {
        0
    }
    #[inline(never)]
    fn kernel(spec: &MatMatMulKerSpec<i32>) -> isize {
        unsafe { armv7neon_mmm_i8_8x4(spec) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulI8xI32x8x4;

impl MatMatMulKer<i32> for MatMatMulI8xI32x8x4 {
    #[inline(always)]
    fn name() -> &'static str {
        "neon"
    }
    #[inline(always)]
    fn mr() -> usize {
        8
    }
    #[inline(always)]
    fn nr() -> usize {
        4
    }
    fn alignment_bytes_packed_a() -> usize {
        4
    }
    fn alignment_bytes_packed_b() -> usize {
        4
    }
    fn end_padding_packed_a() -> usize {
        0
    }
    fn end_padding_packed_b() -> usize {
        0
    }
    #[inline(never)]
    fn kernel(spec: &MatMatMulKerSpec<i32>) -> isize {
        unsafe { armv7neon_mmm_i8_8x4(spec as *const _ as _) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulF32x8x4;

impl MatMatMulKer<f32> for MatMatMulF32x8x4 {
    #[inline(always)]
    fn name() -> &'static str {
        "neon"
    }
    #[inline(always)]
    fn mr() -> usize {
        8
    }
    #[inline(always)]
    fn nr() -> usize {
        4
    }
    fn alignment_bytes_packed_a() -> usize {
        4
    }
    fn alignment_bytes_packed_b() -> usize {
        4
    }
    fn end_padding_packed_a() -> usize {
        0
    }
    fn end_padding_packed_b() -> usize {
        0
    }
    #[inline(never)]
    fn kernel(spec: &MatMatMulKerSpec<f32>) -> isize {
        unsafe { armv7neon_mmm_f32_8x4(spec) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulF32x32x1;

impl MatMatMulKer<f32> for MatMatMulF32x32x1 {
    #[inline(always)]
    fn name() -> &'static str {
        "neon"
    }
    #[inline(always)]
    fn mr() -> usize {
        32
    }
    #[inline(always)]
    fn nr() -> usize {
        1
    }
    fn alignment_bytes_packed_a() -> usize {
        4
    }
    fn alignment_bytes_packed_b() -> usize {
        4
    }
    fn end_padding_packed_a() -> usize {
        0
    }
    fn end_padding_packed_b() -> usize {
        0
    }
    #[inline(never)]
    fn kernel(spec: &MatMatMulKerSpec<f32>) -> isize {
        unsafe { armv7neon_mmm_f32_32x1(spec) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SigmoidF32x4n;

impl ElementWiseKer<f32> for SigmoidF32x4n {
    #[inline(always)]
    fn name() -> &'static str {
        "neon"
    }
    #[inline(always)]
    fn nr() -> usize {
        4
    }
    #[inline(always)]
    fn alignment_items() -> usize {
        4
    }
    #[inline(always)]
    fn alignment_bytes() -> usize {
        16
    }
    #[inline(never)]
    fn run(buf: &mut [f32]) {
        unsafe { armv7neon_sigmoid_f32_4n(buf.as_mut_ptr(), buf.len()) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TanhF32x4n;

impl ElementWiseKer<f32> for TanhF32x4n {
    #[inline(always)]
    fn name() -> &'static str {
        "neon"
    }
    #[inline(always)]
    fn nr() -> usize {
        4
    }
    #[inline(always)]
    fn alignment_items() -> usize {
        4
    }
    #[inline(always)]
    fn alignment_bytes() -> usize {
        16
    }
    #[inline(never)]
    fn run(buf: &mut [f32]) {
        unsafe { armv7neon_tanh_f32_4n(buf.as_mut_ptr(), buf.len()) }
    }
}

test_mmm_kernel_f32!(
    crate::arm32::armv7neon::MatMatMulF32x8x4,
    test_MatMatMulF32x8x4,
    crate::arm32::has_neon()
);

test_mmm_kernel_f32!(
    crate::arm32::armv7neon::MatMatMulF32x32x1,
    test_MatMatMulF32x32x1,
    crate::arm32::has_neon()
);

test_mmm_kernel_i8!(
    crate::arm32::armv7neon::MatMatMulI8x8x4,
    test_MatMatMulI8x8x4,
    crate::arm32::has_neon()
);
test_mmm_kernel_i8_i32!(
    crate::arm32::armv7neon::MatMatMulI8xI32x8x4,
    test_MatMatMulI8xI32x8x4,
    crate::arm32::has_neon()
);

#[cfg(test)]
mod test_neon_fn {
    sigmoid_frame_tests!(crate::arm32::has_neon(), crate::arm32::armv7neon::SigmoidF32x4n);
    tanh_frame_tests!(crate::arm32::has_neon(), crate::arm32::armv7neon::TanhF32x4n);
}
