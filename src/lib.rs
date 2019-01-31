extern crate num_traits;

/// ## Examples
///
/// ```
/// use int::UInt;
/// assert_eq!(u8::BIT_COUNT, 8);
/// assert_eq!(u16::BIT_COUNT, 16);
/// assert_eq!(u32::BIT_COUNT, 32);
/// assert_eq!(u64::BIT_COUNT, 64);
/// assert_eq!(u128::BIT_COUNT, 128);
/// ```
pub trait UInt:
    num_traits::cast::AsPrimitive<u8> +
    num_traits::PrimInt +
    num_traits::sign::Unsigned +
    std::ops::AddAssign +
    std::ops::DivAssign +
    std::ops::Shl<u8, Output = Self> +
    std::ops::ShlAssign<u8> +
    std::ops::Shr<u8, Output = Self> +
    std::ops::ShrAssign<u8> +
    std::ops::SubAssign<Self>
{
    const _0: Self;
    const _1: Self;
    const BIT_COUNT: u8 = (std::mem::size_of::<Self>() * 8) as u8;
    const MAX_VALUE: Self;
    fn from_u8(v: u8) -> Self;
}

impl UInt for u8 {
    const _0: u8 = 0;
    const _1: u8 = 1;
    const MAX_VALUE: Self = Self::max_value();
    fn from_u8(v: u8) -> Self { v }
}
impl UInt for u16 {
    const _0: u16 = 0;
    const _1: u16 = 1;
    const MAX_VALUE: Self = Self::max_value();
    fn from_u8(v: u8) -> Self { v as Self }
}
impl UInt for u32 {
    const _0: u32 = 0;
    const _1: u32 = 1;
    const MAX_VALUE: Self = Self::max_value();
    fn from_u8(v: u8) -> Self { v as Self }
}
impl UInt for u64 {
    const _0: u64 = 0;
    const _1: u64 = 1;
    const MAX_VALUE: Self = Self::max_value();
    fn from_u8(v: u8) -> Self { v as Self }
}
impl UInt for u128 {
    const _0: u128 = 0;
    const _1: u128 = 1;
    const MAX_VALUE: Self = Self::max_value();
    fn from_u8(v: u8) -> Self { v as Self }
}
impl UInt for usize {
    const _0: usize = 0;
    const _1: usize = 1;
    const MAX_VALUE: Self = Self::max_value();
    fn from_u8(v: u8) -> Self { v as Self }
}
