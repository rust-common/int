extern crate num_traits;

use num_traits::cast::AsPrimitive;

pub trait UInt where
    Self: AsPrimitive<u8> +
        Eq +
        std::ops::SubAssign<Self> +
        std::ops::AddAssign +
        std::ops::Shl<u8, Output = Self> +
        std::ops::ShrAssign<u8>,
    u8: AsPrimitive<Self>
{
    const _0: Self;
    const _1: Self;
}

impl UInt for u8 {
    const _0: u8 = 0;
    const _1: u8 = 1;
}
impl UInt for u16 {
    const _0: u16 = 0;
    const _1: u16 = 1;
}
impl UInt for u32 {
    const _0: u32 = 0;
    const _1: u32 = 1;
}
impl UInt for u64 {
    const _0: u64 = 0;
    const _1: u64 = 1;
}
impl UInt for u128 {
    const _0: u128 = 0;
    const _1: u128 = 1;
}
