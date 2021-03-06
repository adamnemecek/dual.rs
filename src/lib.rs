
//#![feature(zero_one)]
//#![feature(core)]
//#![feature(std_misc)]

//extern crate onezero;
extern crate num;

use num::*;
use std::num::FpCategory;
use std::ops::*;
use std::cmp::PartialEq;
use std::cmp::Ordering;

//use onezero::{One, Zero};

pub struct Dual<T> {
  pub val:T,
  pub der:T
}

impl<T:Add<T>> Add<Dual<T>> for Dual<T> {
 type Output = Dual<<T as Add<T>>::Output> ;
 fn add(self, rhs: Dual<T>) -> Dual<<T as Add<T>>::Output> {
  Dual {
   val:self.val+rhs.val,
   der:self.der+rhs.der
  }
 }
}

impl<T:Sub<T>> Sub<Dual<T>> for Dual<T> {
 type Output = Dual<<T as Sub<T>>::Output> ;
 fn sub(self, rhs:Dual<T>) -> Dual<<T as Sub<T>>::Output> {
  Dual {
   val:self.val-rhs.val,
   der:self.der-rhs.der
  }
 }
}

impl<T:Zero> Zero for Dual<T> {
 fn zero() -> Dual<T> {
  Dual {
   val: Zero::zero(),
   der: Zero::zero()
  }
 }
 fn is_zero(&self) -> bool {
  self.val.is_zero()
 }
}

impl<T:Num+Float> Num for Dual<T> {
  type FromStrRadixErr = T::FromStrRadixErr;
  fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
    let s:Vec<_> = str.split("?").collect();
    let v = try! { Num::from_str_radix(s[0], radix) };
    let d = try! { Num::from_str_radix(s[1], radix) };
    Ok(Dual { val: v, der: d })
  }
}

impl<T:Zero+One+Float> One for Dual<T> {
 fn one() -> Dual<T> {
  Dual {
   val: One::one(),
   der: Zero::zero()
  }
 }
}

impl<T:Float /*Add<T>+Mul<T>+Clone*/> Mul<Dual<T>> for Dual<T> {
 type Output = Dual<<T as Mul<T>>::Output> ;
 fn mul(self, rhs:Dual<T>) -> Dual<<T as Mul<T>>::Output> {
  Dual {
   val:self.val*rhs.val,
   der:self.val*rhs.der+rhs.val*self.der
  }
 }
}

impl<T:Float /*Mul<T>+Sub<T>+Div<T>+Clone*/> Div<Dual<T>> for Dual<T> {
 type Output = Dual<<T as Div<T>>::Output> ;
 fn div(self, rhs:Dual<T>) -> Dual<<T as Div<T>>::Output> {
   Dual {
    val:self.val/rhs.val,
    der:(self.der-self.val*rhs.der/rhs.val)/rhs.val
   }
 }
}

impl<T:Float> Rem<Dual<T>> for Dual<T> {
 type Output = Dual<<T as Rem<T>>::Output> ;
 fn rem(self, rhs:Dual<T>) -> Dual<<T as Rem<T>>::Output> {
   Dual {
    val:self.val % rhs.val,
    der:Zero::zero()
   }
 }
}

impl<T:Neg> Neg for Dual<T> {
 type Output = Dual<<T as Neg>::Output> ;
 fn neg(self) -> Dual<<T as Neg>::Output> {
   Dual {
    val: -self.val,
    der: -self.der
   }
 }
}

impl<T:PartialEq> PartialEq for Dual<T> {
 fn eq(&self, other:&Dual<T>) -> bool {
  (self.val==other.val) && (self.der==other.der)
 }
 fn ne(&self, other:&Dual<T>) -> bool {
  (self.val!=other.val) || (self.der!=other.der)
 }
}

impl<T:PartialOrd> PartialOrd for Dual<T> {
 fn partial_cmp(&self, other: &Dual<T>) -> Option<Ordering> {
  match self.val.partial_cmp(&other.val) {
   Some(Ordering::Equal) => self.der.partial_cmp(&other.der), // XXX It is righr?
   res => res
  }
 }
}

impl<T:Copy> Copy for Dual<T> { }

/*
impl<T:Bounded> Bounded for Dual<T> {
 fn min_value() -> Dual<T> {
  Dual {
   val: Bounded::min_value(),
   der: Zero::zero()
  }
 }
 fn max_value() -> Dual<T> {
  Dual {
   val: Bounded::max_value(),
   der: Zero::zero()
  }
 }
}
impl<T:Num+Clone> Num for Dual<T> {}
*/

impl<T:ToPrimitive> ToPrimitive for Dual<T> {
 fn to_i64(&self) -> Option<i64> { self.val.to_i64() }
 fn to_u64(&self) -> Option<u64> { self.val.to_u64() }

// fn to_int(&self) -> Option<isize> { self.val.to_int() }
 fn to_i8(&self) -> Option<i8> { self.val.to_i8() }
 fn to_i16(&self) -> Option<i16> { self.val.to_i16() }
 fn to_i32(&self) -> Option<i32> { self.val.to_i32() }
// fn to_uint(&self) -> Option<usize> { self.val.to_uint() }
 fn to_u8(&self) -> Option<u8> { self.val.to_u8() }
 fn to_u16(&self) -> Option<u16> { self.val.to_u16() }
 fn to_u32(&self) -> Option<u32> { self.val.to_u32() }
 fn to_f32(&self) -> Option<f32> { self.val.to_f32() }
 fn to_f64(&self) -> Option<f64> { self.val.to_f64() }
}

impl<T:NumCast+Zero> NumCast for Dual<T> {
 fn from<N: ToPrimitive>(n: N) -> Option<Dual<T>> {
   match NumCast::from(n) {
    Some(v) => Some( Dual { val:v, der: Zero::zero() } ), 
    None => None
  }
 }
}

impl<T:Clone> Clone for Dual<T> {
 fn clone(&self) -> Dual<T> {
  Dual {
   val:self.val.clone(),
   der:self.der.clone()
  }
 }
}

impl<T:Float> Float for Dual<T> {
 fn nan() -> Dual<T> {
  Dual {
   val: Float::nan(),
   der: Float::nan()
  }
 }
 fn infinity() -> Dual<T> {
  Dual {
   val: Float::infinity(),
   der: Zero::zero()
  }
 }
 fn neg_infinity() -> Dual<T> {
  Dual {
   val: Float::neg_infinity(),
   der: Zero::zero()
  }
 }
 fn neg_zero() -> Dual<T> {
  Dual {
   val: Float::neg_zero(),
   der: Zero::zero()
  }
 }
 fn min_value() -> Dual<T> {
  Dual {
   val: Float::min_value(),
   der: Zero::zero()
  }
 }
 fn min_positive_value() -> Dual<T> {
  Dual {
   val: Float::min_positive_value(),
   der: Zero::zero()
  }
 }
 fn max_value() -> Dual<T> {
  Dual {
   val: Float::max_value(),
   der: Zero::zero()
  }
 }
 fn is_nan(self) -> bool {
  self.val.is_nan()
 }
 fn is_infinite(self) -> bool {
  self.val.is_infinite()
 }
 fn is_finite(self) -> bool {
  self.val.is_finite()
 }
 fn is_normal(self) -> bool {
  self.val.is_normal()
 }
 fn classify(self) -> FpCategory {
  self.val.classify()
 }
/*#[allow(unused_variables)]
 fn mantissa_digits(_unused_self: Option<Dual<T>>) -> usize {
  let n: Option<T> = None;
  Float::mantissa_digits(n)
 }
#[allow(unused_variables)]
 fn digits(_unused_self: Option<Dual<T>>) -> usize {
  let n: Option<T> = None;
  Float::digits(n)
 }
 fn epsilon() -> Dual<T> {
  Dual {
   val:Float::epsilon(),
   der:Float::zero()
  }
 }
#[allow(unused_variables)]
 fn min_exp(_unused_self: Option<Dual<T>>) -> isize {
  let n: Option<T> = None;
  Float::min_exp(n)
 }
#[allow(unused_variables)]
 fn max_exp(_unused_self: Option<Dual<T>>) -> isize {
  let n: Option<T> = None;
  Float::max_exp(n)
 }
#[allow(unused_variables)]
 fn min_10_exp(_unused_self: Option<Dual<T>>) -> isize {
  let n: Option<T> = None;
  Float::min_10_exp(n)
 }
#[allow(unused_variables)]
 fn max_10_exp(_unused_self: Option<Dual<T>>) -> isize {
  let n: Option<T> = None;
  Float::max_10_exp(n)
 }
#[allow(unused_variables)]
 fn min_pos_value(_unused_self: Option<Dual<T>>) -> Dual<T> {
  let n: Option<T> = None;
  Dual {
   val: Float::min_pos_value(n),
   der: Zero::zero()
  }
 }
*/
 fn integer_decode(self) -> (u64, i16, i8) {
  self.val.integer_decode()
 }
 fn floor(self) -> Dual<T> {
  let i = self.val.floor();
  Dual {
   val: i,
   der: if i == self.val {
    Float::infinity()
   } else {
    Zero::zero()
   }
  }
 }
 fn ceil(self) -> Dual<T> {
  let i = self.val.ceil();
  Dual {
   val: i,
   der: if i == self.val {
    Float::infinity()
   } else {
    Zero::zero()
   }
  }
 }
 fn round(self) -> Dual<T> {
  let i = self.val.round();
  Dual {
   val: i,
   der: if i == self.val {
    Float::infinity()
   } else {
    Zero::zero()
   }
  }
 }
 fn trunc(self) -> Dual<T> {
  let i = self.val.trunc();
  Dual {
   val: i,
   der: if i == self.val {
    Float::infinity()
   } else {
    Zero::zero()
   }
  }
 }
 fn fract(self) -> Dual<T> {
  let i = self.val.fract();
  Dual {
   val: i,
   der: if i.is_zero()  {
    Float::neg_infinity()
   } else {
    Zero::zero()
   }
  }
 }
 fn mul_add(self, a: Dual<T>, b: Dual<T>) -> Dual<T> {
  Dual {
   val: self.val.mul_add(a.val, b.val),
   der: self.val.mul_add(a.der, a.val.mul_add(self.der, b.der))
  }
 }
 fn recip(self) -> Dual<T> {
  let r = self.val.recip();
  Dual {
   val: r,
   der: -self.der * r * r
  }
 }
 fn powi(self, n:i32) -> Dual<T> {
  let r = self.val.powi(n-1);
  Dual {
   val: r*self.val,
   der: self.der * r * (match NumCast::from(n) {
                            Some(v) => v,
                            None => Float::nan()
                        })
  }
 }
 fn powf(self, n: Dual<T>) -> Dual<T> {
  let v = self.val;
  let r = v.powf(n.val-One::one());
  Dual {
   val: r*v,
   der: r*(n.der.mul_add(v*v.ln(),self.der*n.val))
  }
 }
/*
 fn sqrt2() -> Dual<T> {
  Dual {
   val:Float::sqrt2(),
   der:Zero::zero()
  }
 }
 fn frac_1_sqrt2() -> Dual<T> {
  Dual {
   val:Float::frac_1_sqrt2(),
   der:Zero::zero()
  }
 }
*/
 fn sqrt(self) -> Dual<T> {
  let s = self.val.sqrt();
  Dual {
   val:s,
   der:self.der/(s+s)  // 2.0*s
  }
 }
/* fn rsqrt(self) -> Dual<T> {
  let v = self.val;
  let s = v.rsqrt();
  Dual {
   val:s,
   der: -self.der*s/(v+v)
  }
 }
*/
/*
 fn pi() -> Dual<T> {
  Dual {
   val:Float::pi(),
   der:Zero::zero()
  }
 }
 fn two_pi() -> Dual<T> {
  Dual {
   val:Float::two_pi(),
   der:Zero::zero()
  }
 }
 fn frac_pi_2() -> Dual<T> {
  Dual {
   val:Float::frac_pi_2(),
   der:Zero::zero()
  }
 }
 fn frac_pi_3() -> Dual<T> {
  Dual {
   val:Float::frac_pi_3(),
   der:Zero::zero()
  }
 }
 fn frac_pi_4() -> Dual<T> {
  Dual {
   val:Float::frac_pi_4(),
   der:Zero::zero()
  }
 }
 fn frac_pi_6() -> Dual<T> {
  Dual {
   val:Float::frac_pi_6(),
   der:Zero::zero()
  }
 }
 fn frac_pi_8() -> Dual<T> {
  Dual {
   val:Float::frac_pi_8(),
   der:Zero::zero()
  }
 }
 fn frac_1_pi() -> Dual<T> {
  Dual {
   val:Float::frac_1_pi(),
   der:Zero::zero()
  }
 }
 fn frac_2_pi() -> Dual<T> {
  Dual {
   val:Float::frac_2_pi(),
   der:Zero::zero()
  }
 }
 fn frac_2_sqrtpi() -> Dual<T> {
  Dual {
   val:Float::frac_2_sqrtpi(),
   der:Zero::zero()
  }
 }
 fn e() -> Dual<T> {
  Dual {
   val:Float::e(),
   der:Zero::zero()
  }
 }
 fn log2_e() -> Dual<T> {
  Dual {
   val:Float::log2_e(),
   der:Zero::zero()
  }
 }
 fn log10_e() -> Dual<T> {
  Dual {
   val:Float::log10_e(),
   der:Zero::zero()
  }
 }
 fn ln_2() -> Dual<T> {
  Dual {
   val:Float::ln_2(),
   der:Zero::zero()
  }
 }
 fn ln_10() -> Dual<T> {
  Dual {
   val: Float::ln_10(),
   der: Zero::zero()
  }
 }
*/
 fn exp(self) -> Dual<T> {
  let v = self.val.exp();
  Dual {
   val:v,
   der:self.der*v
  }
 }
 fn exp2(self) -> Dual<T> {
  let v = self.val.exp2();
  let o:T = One::one();
  Dual {
   val:v,
   der:self.der*v*((o + o).ln()) //Float::ln_2()
  }
 }
 fn ln(self) -> Dual<T> {
  Dual {
   val:self.val.ln(),
   der:self.der/self.val
  }
 }
 fn log(self, base:Dual<T>) -> Dual<T> {
  let v = self.val;
  let b = base.val;
  let l = b.ln();
  let r = v.log(b);
  Dual {
   val: r,
   der: (self.der/v - base.der*r/b)/l
  }
 }
 fn log2(self) -> Dual<T> {
  let o:T = One::one();
  Dual {
   val:self.val.log2(),
   der:self.der*((o + o).ln())/self.val
  }
 }
 fn log10(self) -> Dual<T> {
  let o:T = One::one();
  let five:T = o+o+o+o+o;
  let ten = five+five;
  Dual {
   val:self.val.log10(),
   der:self.der*(ten.ln())/self.val
  }
 }
/*
 fn to_degrees(self) -> Dual<T> {
  Dual {
   val:self.val.to_degrees(),
   der:self.der.to_degrees()
  }
 }
 fn to_radians(self) -> Dual<T> {
  Dual {
   val:self.val.to_radians(),
   der:self.der.to_radians()
  }
 }
*/
 fn abs(self) -> Dual<T> {
   if self.is_sign_positive() || self.is_zero() {
    self
   } else if self.is_sign_negative() {
    -self
   } else {
     Dual {
        val: Zero::zero(),
        der: Float::nan()
     }
   }
 }
 fn signum(self) -> Dual<T> {
  if self.is_sign_positive() {
   One::one()
  } else if self.is_sign_negative() {
   Dual { val:self.val.signum(), der:Zero::zero() }
  } else if self.der == Zero::zero() || self.der == Float::neg_zero() {
   Zero::zero()
  } else {
    Float::nan()
  }
 }
 fn is_sign_positive(self) -> bool {
  self.val.is_sign_positive()
 }
 fn is_sign_negative(self) -> bool {
  self.val.is_sign_negative()
 }
/*
 fn zero() -> Dual<T> {
   Dual {val: Zero::zero(), der: Zero::zero() }
 }
 fn one() -> Dual<T> {
   Dual {
    val: One::one(),
    der: Zero::zero()
   }
 }
*/
/*
 fn ldexp(self, exp: isize) -> Dual<T> {
   Dual {
    val:self.val.ldexp(exp),
    der:self.der.ldexp(exp)
   }
 }
 fn frexp(self) -> (Dual<T>, isize) {
  match self.val.frexp() {
   (v,p) => (Dual {
              val: v,
              der: Float::ldexp(self.der, -p)
             }, p)
  }
 }
 fn next_after(self, other: Dual<T>) -> Dual<T> {
  Dual {
   val: self.val.next_after(other.val),
   der: self.der
  }
 }
*/
 fn max(self, other: Dual<T>) -> Dual<T> {
  if self < other {
   other
  } else {
   self
  }
 }
 fn min(self, other: Dual<T>) -> Dual<T> {
  if self > other {
   other
  } else {
   self
  }
 }
 fn abs_sub(self, other: Dual<T>) -> Dual<T> {
   (self-other).abs()
 }
 fn cbrt(self) -> Dual<T> {
  let v = self.val.cbrt();
  let frac_m1_3:T = match NumCast::from(-1f64/3f64) { Some(v) => v, None => panic!("Internal numberic errer") };
  Dual {
   val: v,
   der: self.der*frac_m1_3/(v*v)
  }
 }
 fn hypot(self, other: Dual<T>) -> Dual<T> {
  let h = self.val.hypot(other.val);
  Dual {
   val: h,
   der: self.val.mul_add(other.der, self.der*other.val)/h
  }
 }
 fn sin(self) -> Dual<T> {
  let (s,c) = self.val.sin_cos();
  Dual {
   val: s,
   der: self.der*c
  }
 }
 fn cos(self) -> Dual<T> {
  let (s,c) = self.val.sin_cos();
  Dual {
   val: c,
   der: -self.der*s
  }
 }
 fn tan(self) -> Dual<T> {
  let s = self.val.sin();
  Dual {
   val: self.val.tan(),
   der: self.der/(s*s)
  }
 }
 fn asin(self) -> Dual<T> {
  Dual {
   val: self.val.asin(),
   der: self.der/self.val.mul_add(-self.val, One::one()).sqrt()
  }
 }
 fn acos(self) -> Dual<T> {
  Dual {
   val: self.val.acos(),
   der: -self.der/self.val.mul_add(-self.val, One::one()).sqrt()
  }
 }
 fn atan(self) -> Dual<T> {
  Dual {
   val: self.val.atan(),
   der: self.der/self.val.mul_add(self.val,One::one())
  }
 }
 fn atan2(self, other: Dual<T>) -> Dual<T> {
  (self/other).atan()
 }
 fn sin_cos(self) -> (Dual<T>, Dual<T>) {
  let (s,c) = self.val.sin_cos();
  (Dual {
   val: s,
   der: self.der*c
  }, Dual {
   val: c,
   der: -self.der*s
  })
 }
 fn exp_m1(self) -> Dual<T> {
  let d = self.val.exp_m1();
  Dual {
   val: d,
   der: self.der.mul_add(d,self.der)
  }
 }
 fn ln_1p(self) -> Dual<T> {
  let o:T = One::one();
  Dual {
   val: self.val.ln_1p(),
   der: self.der/(o+self.val)
  }
 }
 fn sinh(self) -> Dual<T> {
  Dual {
   val: self.val.sinh(),
   der: self.der*self.val.cosh()
  }
 }
 fn cosh(self) -> Dual<T> {
  Dual {
   val: self.val.cosh(),
   der: self.der*self.val.sinh()
  }
 }
 fn tanh(self) -> Dual<T> {
  let s = self.val.sinh();
  Dual {
   val: self.val.tanh(),
   der: self.der/(s*s)
  }
 }
 fn asinh(self) -> Dual<T> {
  Dual {
   val: self.val.asinh(),
   der: self.der/self.val.mul_add(self.val,One::one()).sqrt()
  }
 }
 fn acosh(self) -> Dual<T> {
  let o:T = One::one();
  let mo:T = -o;
  Dual {
   val: self.val.acosh(),
   der: self.der/self.val.mul_add(self.val,mo).sqrt()
  }
 }
 fn atanh(self) -> Dual<T> {
  Dual {
   val: self.val.atanh(),
   der: self.der/self.val.mul_add(-self.val,One::one())
  }
 }
}
