//! Lane-wise arithmetic operations.

macro_rules! impl_arithmetic_ops {
    ($id:ident) => {
        impl ops::Add for $id {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: Self) -> Self {
                unsafe { simd_add(self, other) }
            }
        }

        impl ops::Sub for $id {
            type Output = Self;
            #[inline(always)]
            fn sub(self, other: Self) -> Self {
                unsafe { simd_sub(self, other) }
            }
        }

        impl ops::Mul for $id {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: Self) -> Self {
                unsafe { simd_mul(self, other) }
            }
        }

        impl ops::Div for $id {
            type Output = Self;
            #[inline(always)]
            fn div(self, other: Self) -> Self {
                unsafe { simd_div(self, other) }
            }
        }

        impl ops::Rem for $id {
            type Output = Self;
            #[inline(always)]
            fn rem(self, other: Self) -> Self {
                unsafe { simd_rem(self, other) }
            }
        }

        impl ops::AddAssign for $id {
            #[inline(always)]
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl ops::SubAssign for $id {
            #[inline(always)]
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }

        impl ops::MulAssign for $id {
            #[inline(always)]
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other;
            }
        }

        impl ops::DivAssign for $id {
            #[inline(always)]
            fn div_assign(&mut self, other: Self) {
                *self = *self / other;
            }
        }

        impl ops::RemAssign for $id {
            #[inline(always)]
            fn rem_assign(&mut self, other: Self) {
                *self = *self % other;
            }
        }
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! test_arithmetic_ops {
    ($id:ident, $elem_ty:ident) => {
        #[test]
        fn arithmetic() {
            use ::coresimd::simd::$id;
            let z = $id::splat(0 as $elem_ty);
            let o = $id::splat(1 as $elem_ty);
            let t = $id::splat(2 as $elem_ty);
            let f = $id::splat(4 as $elem_ty);

            // add
            assert_eq!(z + z, z);
            assert_eq!(o + z, o);
            assert_eq!(t + z, t);
            assert_eq!(t + t, f);
            // sub
            assert_eq!(z - z, z);
            assert_eq!(o - z, o);
            assert_eq!(t - z, t);
            assert_eq!(f - t, t);
            assert_eq!(f - o - o, t);
            // mul
            assert_eq!(z * z, z);
            assert_eq!(z * o, z);
            assert_eq!(z * t, z);
            assert_eq!(o * t, t);
            assert_eq!(t * t, f);
            // div
            assert_eq!(z / o, z);
            assert_eq!(t / o, t);
            assert_eq!(f / o, f);
            assert_eq!(t / t, o);
            assert_eq!(f / t, t);
            // rem
            assert_eq!(o % o, z);
            assert_eq!(f % t, z);

            {
                let mut v = z;
                assert_eq!(v, z);
                v += o;  // add_assign
                assert_eq!(v, o);
                v -= o; // sub_assign
                assert_eq!(v, z);
                v = t;
                v *= o; // mul_assign
                assert_eq!(v, t);
                v *= t;
                assert_eq!(v, f);
                v /= o; // div_assign
                assert_eq!(v, f);
                v /= t;
                assert_eq!(v, t);
                v %= t; // rem_assign
                assert_eq!(v, z);
            }
        }
    };
}
