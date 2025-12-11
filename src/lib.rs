#![no_std]

extern crate alloc;

pub mod arith;
mod fields;
mod groups;

use crate::fields::FieldElement;
use crate::groups::{GroupElement, G1Params, G2Params, GroupParams};

use alloc::vec::Vec;
use core::ops::{Add, Mul, Neg, Sub};
use rand::Rng;

#[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
mod zisk;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Fr(fields::Fr);

impl Fr {
    pub fn zero() -> Self {
        Fr(fields::Fr::zero())
    }
    pub fn one() -> Self {
        Fr(fields::Fr::one())
    }
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Fr(fields::Fr::random(rng))
    }
    pub fn pow(&self, exp: Fr) -> Self {
        Fr(self.0.pow(exp.0))
    }
    pub fn from_str(s: &str) -> Option<Self> {
        fields::Fr::from_str(s).map(|e| Fr(e))
    }
    pub fn inverse(&self) -> Option<Self> {
        self.0.inverse().map(|e| Fr(e))
    }
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    pub fn interpret(buf: &[u8; 64]) -> Fr {
        Fr(fields::Fr::interpret(buf))
    }
    pub fn from_slice(slice: &[u8]) -> Result<Self, FieldError> {
        arith::U256::from_slice(slice)
            .map_err(|_| FieldError::InvalidSliceLength) // todo: maybe more sensful error handling
            .map(|x| Fr::new_mul_factor(x))
    }
    pub fn to_big_endian(&self, slice: &mut [u8]) -> Result<(), FieldError> {
        self.0
            .raw()
            .to_big_endian(slice)
            .map_err(|_| FieldError::InvalidSliceLength)
    }
    pub fn new(val: arith::U256) -> Option<Self> {
        fields::Fr::new(val).map(|x| Fr(x))
    }
    pub fn new_mul_factor(val: arith::U256) -> Self {
        Fr(fields::Fr::new_mul_factor(val))
    }
    pub fn into_u256(self) -> arith::U256 {
        (self.0).into()
    }
    pub fn set_bit(&mut self, bit: usize, to: bool) {
        self.0.set_bit(bit, to);
    }
}

impl Add<Fr> for Fr {
    type Output = Fr;

    fn add(self, other: Fr) -> Fr {
        Fr(self.0 + other.0)
    }
}

impl Sub<Fr> for Fr {
    type Output = Fr;

    fn sub(self, other: Fr) -> Fr {
        Fr(self.0 - other.0)
    }
}

impl Neg for Fr {
    type Output = Fr;

    fn neg(self) -> Fr {
        Fr(-self.0)
    }
}

impl Mul for Fr {
    type Output = Fr;

    fn mul(self, other: Fr) -> Fr {
        Fr(self.0 * other.0)
    }
}

#[derive(Debug)]
pub enum FieldError {
    InvalidSliceLength,
    InvalidU512Encoding,
    NotMember,
}

#[derive(Debug)]
pub enum CurveError {
    InvalidEncoding,
    NotMember,
    Field(FieldError),
    ToAffineConversion,
}

impl From<FieldError> for CurveError {
    fn from(fe: FieldError) -> Self {
        CurveError::Field(fe)
    }
}

pub use crate::groups::Error as GroupError;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Fq(fields::Fq);

impl Fq {
    pub fn zero() -> Self {
        Fq(fields::Fq::zero())
    }
    pub fn one() -> Self {
        Fq(fields::Fq::one())
    }
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Fq(fields::Fq::random(rng))
    }
    pub fn pow(&self, exp: Fq) -> Self {
        Fq(self.0.pow(exp.0))
    }
    pub fn from_str(s: &str) -> Option<Self> {
        fields::Fq::from_str(s).map(|e| Fq(e))
    }
    pub fn inverse(&self) -> Option<Self> {
        self.0.inverse().map(|e| Fq(e))
    }
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    pub fn interpret(buf: &[u8; 64]) -> Fq {
        Fq(fields::Fq::interpret(buf))
    }
    pub fn from_slice(slice: &[u8]) -> Result<Self, FieldError> {
        arith::U256::from_slice(slice)
            .map_err(|_| FieldError::InvalidSliceLength) // todo: maybe more sensful error handling
            .and_then(|x| fields::Fq::new(x).ok_or(FieldError::NotMember))
            .map(|x| Fq(x))
    }
    pub fn to_big_endian(&self, slice: &mut [u8]) -> Result<(), FieldError> {
        let mut a: arith::U256 = self.0.into();
        // convert from Montgomery representation
        a.mul(
            &fields::Fq::one().raw(),
            &fields::Fq::modulus(),
            self.0.inv(),
        );
        a.to_big_endian(slice)
            .map_err(|_| FieldError::InvalidSliceLength)
    }
    pub fn from_u256(u256: arith::U256) -> Result<Self, FieldError> {
        Ok(Fq(fields::Fq::new(u256).ok_or(FieldError::NotMember)?))
    }
    pub fn into_u256(self) -> arith::U256 {
        (self.0).into()
    }
    pub fn modulus() -> arith::U256 {
        fields::Fq::modulus()
    }

    pub fn sqrt(&self) -> Option<Self> {
        self.0.sqrt().map(Fq)
    }
}

impl Add<Fq> for Fq {
    type Output = Fq;

    fn add(self, other: Fq) -> Fq {
        Fq(self.0 + other.0)
    }
}

impl Sub<Fq> for Fq {
    type Output = Fq;

    fn sub(self, other: Fq) -> Fq {
        Fq(self.0 - other.0)
    }
}

impl Neg for Fq {
    type Output = Fq;

    fn neg(self) -> Fq {
        Fq(-self.0)
    }
}

impl Mul for Fq {
    type Output = Fq;

    fn mul(self, other: Fq) -> Fq {
        Fq(self.0 * other.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Fq2(fields::Fq2);

impl Fq2 {
    pub fn one() -> Fq2 {
        Fq2(fields::Fq2::one())
    }

    pub fn i() -> Fq2 {
        Fq2(fields::Fq2::i())
    }

    pub fn zero() -> Fq2 {
        Fq2(fields::Fq2::zero())
    }

    /// Initalizes new F_q2(a + bi, a is real coeff, b is imaginary)
    pub fn new(a: Fq, b: Fq) -> Fq2 {
        Fq2(fields::Fq2::new(a.0, b.0))
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn pow(&self, exp: arith::U256) -> Self {
        Fq2(self.0.pow(exp))
    }

    pub fn real(&self) -> Fq {
        Fq(*self.0.real())
    }

    pub fn imaginary(&self) -> Fq {
        Fq(*self.0.imaginary())
    }

    pub fn sqrt(&self) -> Option<Self> {
        self.0.sqrt().map(Fq2)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, FieldError> {
        let u512 = arith::U512::from_slice(bytes).map_err(|_| FieldError::InvalidU512Encoding)?;
        let (res, c0) = u512.divrem(&Fq::modulus());
        Ok(Fq2::new(
            Fq::from_u256(c0).map_err(|_| FieldError::NotMember)?,
            Fq::from_u256(res.ok_or(FieldError::NotMember)?).map_err(|_| FieldError::NotMember)?,
        ))
    }
}


impl Add<Fq2> for Fq2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Fq2(self.0 + other.0)
    }
}

impl Sub<Fq2> for Fq2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Fq2(self.0 - other.0)
    }
}

impl Neg for Fq2 {
    type Output = Self;

    fn neg(self) -> Self {
        Fq2(-self.0)
    }
}

impl Mul for Fq2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Fq2(self.0 * other.0)
    }
}

pub trait Group
    : Send
    + Sync
    + Copy
    + Clone
    + PartialEq
    + Eq
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Neg<Output = Self>
    + Mul<Fr, Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
    fn random<R: Rng>(rng: &mut R) -> Self;
    fn is_zero(&self) -> bool;
    fn normalize(&mut self);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct G1(groups::G1);

impl G1 {
    pub fn new(x: Fq, y: Fq, z: Fq) -> Self {
        G1(groups::G1::new(x.0, y.0, z.0))
    }

    pub fn x(&self) -> Fq {
        Fq(self.0.x().clone())
    }

    pub fn set_x(&mut self, x: Fq) {
        *self.0.x_mut() = x.0
    }

    pub fn y(&self) -> Fq {
        Fq(self.0.y().clone())
    }

    pub fn set_y(&mut self, y: Fq) {
        *self.0.y_mut() = y.0
    }

    pub fn z(&self) -> Fq {
        Fq(self.0.z().clone())
    }

    pub fn set_z(&mut self, z: Fq) {
        *self.0.z_mut() = z.0
    }

    pub fn b() -> Fq {
        Fq(G1Params::coeff_b())
    }

    pub fn from_compressed(bytes: &[u8]) -> Result<Self, CurveError> {
        if bytes.len() != 33 { return Err(CurveError::InvalidEncoding); }

        let sign = bytes[0];
        let fq = Fq::from_slice(&bytes[1..])?;
        let x = fq;
        let y_squared = (fq * fq * fq) + Self::b();

        let mut y = y_squared.sqrt().ok_or(CurveError::NotMember)?;

        if sign == 2 && y.into_u256().get_bit(0).expect("bit 0 always exist; qed") { y = y.neg(); }
        else if sign == 3 && !y.into_u256().get_bit(0).expect("bit 0 always exist; qed") { y = y.neg(); }
        else if sign != 3 && sign != 2 {
            return Err(CurveError::InvalidEncoding);
        }
        AffineG1::new(x, y).map_err(|_| CurveError::NotMember).map(Into::into)
    }
}

impl Group for G1 {
    fn zero() -> Self {
        G1(groups::G1::zero())
    }
    fn one() -> Self {
        G1(groups::G1::one())
    }
    fn random<R: Rng>(rng: &mut R) -> Self {
        G1(groups::G1::random(rng))
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    fn normalize(&mut self) {
        let new = match self.0.to_affine() {
            Some(a) => a,
            None => return,
        };

        self.0 = new.to_jacobian();
    }
}

impl Add<G1> for G1 {
    type Output = G1;

    fn add(self, other: G1) -> G1 {
        #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
        {
            // Convert to appropriate format
            let (x1, y1, z1) = (
                self.x().into_u256().to_words(),
                self.y().into_u256().to_words(),
                self.z().into_u256().to_words(),
            );
            let (x2, y2, z2) = (
                other.x().into_u256().to_words(),
                other.y().into_u256().to_words(),
                other.z().into_u256().to_words(),
            );

            let p1_jac = [
                x1[0], x1[1], x1[2], x1[3], y1[0], y1[1], y1[2], y1[3], z1[0], z1[1], z1[2], z1[3],
            ];
            let p2_jac = [
                x2[0], x2[1], x2[2], x2[3], y2[0], y2[1], y2[2], y2[3], z2[0], z2[1], z2[2], z2[3],
            ];

            let mut p1_aff = [0u64; 8];
            let mut p2_aff = [0u64; 8];
            unsafe { zisk::to_affine_bn254_c(p1_jac.as_ptr(), p1_aff.as_mut_ptr()) };
            unsafe { zisk::to_affine_bn254_c(p2_jac.as_ptr(), p2_aff.as_mut_ptr()) };

            // Use the zisklib for the computation
            let mut res = [0u64; 8];
            let is_zero = unsafe { zisk::add_bn254_c(p1_aff.as_ptr(), p2_aff.as_ptr(), res.as_mut_ptr()) };

            // Check for point at infinity
            if is_zero {
                return G1::zero();
            }

            // Convert back to the original format
            G1::new(
                Fq::from_u256(arith::U256::from([res[0], res[1], res[2], res[3]])).unwrap(),
                Fq::from_u256(arith::U256::from([res[4], res[5], res[6], res[7]])).unwrap(),
                Fq::one(),
            )
        }

        #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
        {
            G1(self.0 + other.0)
        }
    }
}

impl Sub<G1> for G1 {
    type Output = G1;

    fn sub(self, other: G1) -> G1 {
        G1(self.0 - other.0)
    }
}

impl Neg for G1 {
    type Output = G1;

    fn neg(self) -> G1 {
        G1(-self.0)
    }
}

impl Mul<Fr> for G1 {
    type Output = G1;

    fn mul(self, other: Fr) -> G1 {
        #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
        {
            // Convert to appropriate format
            let (x, y, z) = (
                self.x().into_u256().to_words(),
                self.y().into_u256().to_words(),
                self.z().into_u256().to_words(),
            );

            let p_jac = [
                x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2], z[3],
            ];

            let mut p_aff = [0u64; 8];
            unsafe { zisk::to_affine_bn254_c(p_jac.as_ptr(), p_aff.as_mut_ptr()) };

            let k = other.into_u256().to_words();

            // Use the zisklib for the computation
            let mut res = [0u64; 8];
            let is_zero = unsafe { zisk::mul_bn254_c(p_aff.as_ptr(), k.as_ptr(), res.as_mut_ptr()) };

            // Check for point at infinity
            if is_zero {
                return G1::zero();
            }

            G1::new(
                Fq::from_u256(arith::U256::from([res[0], res[1], res[2], res[3]])).unwrap(),
                Fq::from_u256(arith::U256::from([res[4], res[5], res[6], res[7]])).unwrap(),
                Fq::one(),
            )
        }

        #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
        {
            G1(self.0 * other.0)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct AffineG1(groups::AffineG1);

impl AffineG1 {
    pub fn new(x: Fq, y: Fq) -> Result<Self, GroupError> {
        #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
        {
            let x_256 = x.into_u256().to_words();
            let y_256 = y.into_u256().to_words();
            let p = [
                x_256[0], x_256[1], x_256[2], x_256[3], y_256[0], y_256[1], y_256[2], y_256[3],
            ];

            if unsafe { zisk::is_on_curve_bn254_c(p.as_ptr()) } {
                Ok(AffineG1::new_unchecked(x, y))
            } else {
                Err(GroupError::NotOnCurve)
            }
        }

        #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
        {
            Ok(AffineG1(groups::AffineG1::new(x.0, y.0)?))
        }
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    pub fn new_unchecked(x: Fq, y: Fq) -> Self {
        AffineG1(groups::AffineG1::new_unchecked(x.0, y.0))
    }

    pub fn x(&self) -> Fq {
        Fq(self.0.x().clone())
    }

    pub fn set_x(&mut self, x: Fq) {
        *self.0.x_mut() = x.0
    }

    pub fn y(&self) -> Fq {
        Fq(self.0.y().clone())
    }

    pub fn set_y(&mut self, y: Fq) {
        *self.0.y_mut() = y.0
    }

    pub fn from_jacobian(g1: G1) -> Option<Self> {
        #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
        {
            // Convert to appropriate format
            let x = g1.x().into_u256().to_words();
            let y = g1.y().into_u256().to_words();
            let z = g1.z().into_u256().to_words();

            let p_jac = [
                x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2], z[3],
            ];

            // Use the zisklib for the computation
            let mut res = [0u64; 8];
            let is_zero = unsafe { zisk::to_affine_bn254_c(p_jac.as_ptr(), res.as_mut_ptr()) };

            // Check for point at infinity
            if is_zero {
                return None;
            }

            Some(AffineG1::new_unchecked(
                Fq::from_u256(arith::U256::from([res[0], res[1], res[2], res[3]])).unwrap(),
                Fq::from_u256(arith::U256::from([res[4], res[5], res[6], res[7]])).unwrap(),
            ))
        }

        #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
        {
            g1.0.to_affine().map(|x| AffineG1(x))
        }
    }
}

impl From<AffineG1> for G1 {
    fn from(affine: AffineG1) -> Self {
        G1(affine.0.to_jacobian())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct G2(groups::G2);

impl G2 {
    pub fn new(x: Fq2, y: Fq2, z: Fq2) -> Self {
        G2(groups::G2::new(x.0, y.0, z.0))
    }

    pub fn x(&self) -> Fq2 {
        Fq2(self.0.x().clone())
    }

    pub fn set_x(&mut self, x: Fq2) {
        *self.0.x_mut() = x.0
    }

    pub fn y(&self) -> Fq2 {
        Fq2(self.0.y().clone())
    }

    pub fn set_y(&mut self, y: Fq2) {
        *self.0.y_mut() = y.0
    }

    pub fn z(&self) -> Fq2 {
        Fq2(self.0.z().clone())
    }

    pub fn set_z(&mut self, z: Fq2) {
        *self.0.z_mut() = z.0
    }

    pub fn b() -> Fq2 {
        Fq2(G2Params::coeff_b())
    }

    pub fn from_compressed(bytes: &[u8]) -> Result<Self, CurveError> {

        if bytes.len() != 65 { return Err(CurveError::InvalidEncoding); }

        let sign = bytes[0];
        let x = Fq2::from_slice(&bytes[1..])?;

        let y_squared = (x * x * x) + G2::b();
        let y = y_squared.sqrt().ok_or(CurveError::NotMember)?;
        let y_neg = -y;

        let y_gt = y.0.to_u512() > y_neg.0.to_u512();

        let e_y = if sign == 10 { if y_gt { y_neg } else { y } }
        else if sign == 11 { if y_gt { y } else { y_neg } }
        else {
            return Err(CurveError::InvalidEncoding);
        };

        AffineG2::new(x, e_y).map_err(|_| CurveError::NotMember).map(Into::into)
    }
}

impl Group for G2 {
    fn zero() -> Self {
        G2(groups::G2::zero())
    }
    fn one() -> Self {
        G2(groups::G2::one())
    }
    fn random<R: Rng>(rng: &mut R) -> Self {
        G2(groups::G2::random(rng))
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    fn normalize(&mut self) {
        let new = match self.0.to_affine() {
            Some(a) => a,
            None => return,
        };

        self.0 = new.to_jacobian();
    }
}

impl Add<G2> for G2 {
    type Output = G2;

    fn add(self, other: G2) -> G2 {
        G2(self.0 + other.0)
    }
}

impl Sub<G2> for G2 {
    type Output = G2;

    fn sub(self, other: G2) -> G2 {
        G2(self.0 - other.0)
    }
}

impl Neg for G2 {
    type Output = G2;

    fn neg(self) -> G2 {
        G2(-self.0)
    }
}

impl Mul<Fr> for G2 {
    type Output = G2;

    fn mul(self, other: Fr) -> G2 {
        G2(self.0 * other.0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Gt(fields::Fq12);

impl Gt {
    pub fn one() -> Self {
        Gt(fields::Fq12::one())
    }
    pub fn pow(&self, exp: Fr) -> Self {
        Gt(self.0.pow(exp.0))
    }
    pub fn inverse(&self) -> Option<Self> {
        self.0.inverse().map(Gt)
    }
    pub fn final_exponentiation(&self) -> Option<Self> {
        self.0.final_exponentiation().map(Gt)
    }
}

impl Mul<Gt> for Gt {
    type Output = Gt;

    fn mul(self, other: Gt) -> Gt {
        Gt(self.0 * other.0)
    }
}

pub fn pairing(p: G1, q: G2) -> Gt {
    Gt(groups::pairing(&p.0, &q.0))
}

pub fn pairing_batch(pairs: &[(G1, G2)]) -> Gt {
    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Convert to appropriate format
        let mut ps: Vec<[u64; 8]> = Vec::with_capacity(pairs.len());
        let mut qs: Vec<[u64; 16]> = Vec::with_capacity(pairs.len());
        for (p, q) in pairs {
            let (p_x, p_y, p_z) = (
                p.x().into_u256().to_words(),
                p.y().into_u256().to_words(),
                p.z().into_u256().to_words(),
            );
            let (q_x, q_y, q_z) = {
                let q_x = q.x();
                let q_x_r = q_x.real();
                let q_x_i = q_x.imaginary();
                let q_y = q.y();
                let q_y_r = q_y.real();
                let q_y_i = q_y.imaginary();
                let q_z = q.z();
                let q_z_r = q_z.real();
                let q_z_i = q_z.imaginary();

                (
                    [q_x_r.into_u256().to_words(), q_x_i.into_u256().to_words()],
                    [q_y_r.into_u256().to_words(), q_y_i.into_u256().to_words()],
                    [q_z_r.into_u256().to_words(), q_z_i.into_u256().to_words()],
                )
            };

            let p_jac = [
                p_x[0], p_x[1], p_x[2], p_x[3], p_y[0], p_y[1], p_y[2], p_y[3], p_z[0], p_z[1],
                p_z[2], p_z[3],
            ];
            let q_jac = [
                q_x[0][0], q_x[0][1], q_x[0][2], q_x[0][3], q_x[1][0], q_x[1][1], q_x[1][2],
                q_x[1][3], q_y[0][0], q_y[0][1], q_y[0][2], q_y[0][3], q_y[1][0], q_y[1][1],
                q_y[1][2], q_y[1][3], q_z[0][0], q_z[0][1], q_z[0][2], q_z[0][3], q_z[1][0],
                q_z[1][1], q_z[1][2], q_z[1][3],
            ];

            let mut p_aff = [0u64; 8];
            unsafe { zisk::to_affine_bn254_c(p_jac.as_ptr(), p_aff.as_mut_ptr()) };

            let mut q_aff = [0u64; 16];
            unsafe { zisk::to_affine_twist_bn254_c(q_jac.as_ptr(), q_aff.as_mut_ptr()) };

            ps.push(p_aff);
            qs.push(q_aff);
        }

        // Use the zisklib for the computation
        let mut res = [0u64; 48];
        unsafe {
            zisk::pairing_batch_bn254_c(
                ps.as_ptr() as *const u64,
                qs.as_ptr() as *const u64,
                ps.len(),
                res.as_mut_ptr(),
            )
        };

        // A common case is checking whether e(P1, Q1)·...·e(Pn, Qn) == 1
        let mut one = [0; 48];
        one[0] = 1;
        if res == one {
            return Gt::one();
        }

        // Convert back to the original format
        let res_0 = fields::Fq::new(arith::U256::from([res[0], res[1], res[2], res[3]])).unwrap();
        let res_1 = fields::Fq::new(arith::U256::from([res[4], res[5], res[6], res[7]])).unwrap();
        let res_2 = fields::Fq::new(arith::U256::from([res[8], res[9], res[10], res[11]])).unwrap();
        let res_3 = fields::Fq::new(arith::U256::from([res[12], res[13], res[14], res[15]])).unwrap();
        let res_4 = fields::Fq::new(arith::U256::from([res[16], res[17], res[18], res[19]])).unwrap();
        let res_5 = fields::Fq::new(arith::U256::from([res[20], res[21], res[22], res[23]])).unwrap();
        let res_6 = fields::Fq::new(arith::U256::from([res[24], res[25], res[26], res[27]])).unwrap();
        let res_7 = fields::Fq::new(arith::U256::from([res[28], res[29], res[30], res[31]])).unwrap();
        let res_8 = fields::Fq::new(arith::U256::from([res[32], res[33], res[34], res[35]])).unwrap();
        let res_9 = fields::Fq::new(arith::U256::from([res[36], res[37], res[38], res[39]])).unwrap();
        let res_10 = fields::Fq::new(arith::U256::from([res[40], res[41], res[42], res[43]])).unwrap();
        let res_11 = fields::Fq::new(arith::U256::from([res[44], res[45], res[46], res[47]])).unwrap();

        let res_0 = fields::Fq2::new(res_0, res_1);
        let res_1 = fields::Fq2::new(res_2, res_3);
        let res_2 = fields::Fq2::new(res_4, res_5);
        let res_3 = fields::Fq2::new(res_6, res_7);
        let res_4 = fields::Fq2::new(res_8, res_9);
        let res_5 = fields::Fq2::new(res_10, res_11);

        let res_0 = fields::Fq6::new(res_0, res_1, res_2);
        let res_1 = fields::Fq6::new(res_3, res_4, res_5);

        let res = fields::Fq12::new(res_0, res_1);
        Gt(res)
    }

    #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
    {
        let mut ps : Vec<groups::G1> = Vec::new();
        let mut qs : Vec<groups::G2> = Vec::new();
        for (p, q) in pairs {
            ps.push(p.0);
            qs.push(q.0);
        }
        Gt(groups::pairing_batch(&ps, &qs))
    }
}

pub fn miller_loop_batch(pairs: &[(G2, G1)]) -> Result<Gt, CurveError> {
    let mut ps : Vec<groups::G2Precomp> = Vec::new();
    let mut qs : Vec<groups::AffineG<groups::G1Params>> = Vec::new();
    for (p, q) in pairs {
        ps.push(p.0.to_affine().ok_or(CurveError::ToAffineConversion)?.precompute());
        qs.push(q.0.to_affine().ok_or(CurveError::ToAffineConversion)?);
    }
    Ok(Gt(groups::miller_loop_batch(&ps, &qs)))
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct AffineG2(groups::AffineG2);

impl AffineG2 {
    pub fn new(x: Fq2, y: Fq2) -> Result<Self, GroupError> {
        #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
        {
            let x_256 = {
                let x_r = x.real().into_u256().to_words();
                let x_i = x.imaginary().into_u256().to_words();
                [
                    x_r[0], x_r[1], x_r[2], x_r[3], x_i[0], x_i[1], x_i[2], x_i[3],
                ]
            };
            let y_256 = {
                let y_r = y.real().into_u256().to_words();
                let y_i = y.imaginary().into_u256().to_words();
                [
                    y_r[0], y_r[1], y_r[2], y_r[3], y_i[0], y_i[1], y_i[2], y_i[3],
                ]
            };
            let p = [
                x_256[0], x_256[1], x_256[2], x_256[3], x_256[4], x_256[5], x_256[6], x_256[7],
                y_256[0], y_256[1], y_256[2], y_256[3], y_256[4], y_256[5], y_256[6], y_256[7],
            ];

            if unsafe { zisk::is_on_curve_twist_bn254_c(p.as_ptr()) } {
                if G2Params::check_order() {
                    if !unsafe { zisk::is_on_subgroup_twist_bn254_c(p.as_ptr()) } {
                        return Err(GroupError::NotInSubgroup);
                    }
                }

                Ok(AffineG2(groups::AffineG2::new_unchecked(x.0, y.0)))
            } else {
                Err(GroupError::NotOnCurve)
            }
        }

        #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
        {
            Ok(AffineG2(groups::AffineG2::new(x.0, y.0)?))
        }
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    pub fn new_unchecked(x: Fq2, y: Fq2) -> Self {
        AffineG2(groups::AffineG2::new_unchecked(x.0, y.0))
    }

    pub fn x(&self) -> Fq2 {
        Fq2(self.0.x().clone())
    }

    pub fn set_x(&mut self, x: Fq2) {
        *self.0.x_mut() = x.0
    }

    pub fn y(&self) -> Fq2 {
        Fq2(self.0.y().clone())
    }

    pub fn set_y(&mut self, y: Fq2) {
        *self.0.y_mut() = y.0
    }

    pub fn from_jacobian(g2: G2) -> Option<Self> {
        g2.0.to_affine().map(|x| AffineG2(x))
    }
}

impl From<AffineG2> for G2 {
    fn from(affine: AffineG2) -> Self {
        G2(affine.0.to_jacobian())
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use super::{G1, Fq, G2, Fq2};

    fn hex(s: &'static str) -> Vec<u8> {
        use rustc_hex::FromHex;
        s.from_hex().unwrap()
    }

    #[test]
    fn g1_from_compressed() {
        let g1 = G1::from_compressed(&hex("0230644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46"))
            .expect("Invalid g1 decompress result");
        assert_eq!(g1.x(), Fq::from_str("21888242871839275222246405745257275088696311157297823662689037894645226208582").unwrap());
        assert_eq!(g1.y(), Fq::from_str("3969792565221544645472939191694882283483352126195956956354061729942568608776").unwrap());
        assert_eq!(g1.z(), Fq::one());
    }


    #[test]
    fn g2_from_compressed() {
        let g2 = G2::from_compressed(
            &hex("0a023aed31b5a9e486366ea9988b05dba469c6206e58361d9c065bbea7d928204a761efc6e4fa08ed227650134b52c7f7dd0463963e8a4bf21f4899fe5da7f984a")
        ).expect("Valid g2 point hex encoding");

        assert_eq!(g2.x(),
                   Fq2::new(
                       Fq::from_str("5923585509243758863255447226263146374209884951848029582715967108651637186684").unwrap(),
                       Fq::from_str("5336385337059958111259504403491065820971993066694750945459110579338490853570").unwrap(),
                   )
        );

        assert_eq!(g2.y(),
                   Fq2::new(
                       Fq::from_str("10374495865873200088116930399159835104695426846400310764827677226300185211748").unwrap(),
                       Fq::from_str("5256529835065685814318509161957442385362539991735248614869838648137856366932").unwrap(),
                   )
        );

        // 0b prefix is point reflection on the curve
        let g2 = -G2::from_compressed(
            &hex("0b023aed31b5a9e486366ea9988b05dba469c6206e58361d9c065bbea7d928204a761efc6e4fa08ed227650134b52c7f7dd0463963e8a4bf21f4899fe5da7f984a")
        ).expect("Valid g2 point hex encoding");

        assert_eq!(g2.x(),
                   Fq2::new(
                       Fq::from_str("5923585509243758863255447226263146374209884951848029582715967108651637186684").unwrap(),
                       Fq::from_str("5336385337059958111259504403491065820971993066694750945459110579338490853570").unwrap(),
                   )
        );

        assert_eq!(g2.y(),
                   Fq2::new(
                       Fq::from_str("10374495865873200088116930399159835104695426846400310764827677226300185211748").unwrap(),
                       Fq::from_str("5256529835065685814318509161957442385362539991735248614869838648137856366932").unwrap(),
                   )
        );

        // valid point but invalid sign prefix
        assert!(
            G2::from_compressed(
                &hex("0c023aed31b5a9e486366ea9988b05dba469c6206e58361d9c065bbea7d928204a761efc6e4fa08ed227650134b52c7f7dd0463963e8a4bf21f4899fe5da7f984a")
            ).is_err()
        );
    }
}
