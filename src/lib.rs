extern crate num_traits;

use num_traits::cast::AsPrimitive;

pub trait UInt where
    Self: AsPrimitive<u8> +
        Eq +
        std::ops::SubAssign<Self> +
        std::ops::AddAssign +
        std::ops::DivAssign +
        std::ops::Shl<u8, Output = Self> +
        std::ops::ShrAssign<u8>
{
    const _0: Self;
    const _1: Self;
    fn from_u8(v: u8) -> Self;
}

impl UInt for u8 {
    const _0: u8 = 0;
    const _1: u8 = 1;
    fn from_u8(v: u8) -> Self { v }
}
impl UInt for u16 {
    const _0: u16 = 0;
    const _1: u16 = 1;
    fn from_u8(v: u8) -> Self { v as Self }
}
impl UInt for u32 {
    const _0: u32 = 0;
    const _1: u32 = 1;
    fn from_u8(v: u8) -> Self { v as Self }
}
impl UInt for u64 {
    const _0: u64 = 0;
    const _1: u64 = 1;
    fn from_u8(v: u8) -> Self { v as Self }
}
impl UInt for u128 {
    const _0: u128 = 0;
    const _1: u128 = 1;
    fn from_u8(v: u8) -> Self { v as Self }
}
impl UInt for usize {
    const _0: usize = 0;
    const _1: usize = 1;
    fn from_u8(v: u8) -> Self { v as Self }
}
