use std::ops::Range;

use super::Ranged;

macro_rules! make_numeric_coord {
    ($type:ty, $name:ident, $key_points:ident) => {
        pub struct $name($type, $type);
        impl From<Range<$type>> for $name {
            fn from(range: Range<$type>) -> Self {
                return Self(range.start, range.end);
            }
        }
        impl Ranged for $name {
            type ValueType = $type;
            fn map(&self, v: &$type, limit: (i32, i32)) -> i32 {
                let logic_length = (*v - self.0) as f64 / (self.1 - self.0) as f64;
                let actual_length = limit.1 - limit.0;

                if actual_length == 0 {
                    return limit.1;
                }

                return limit.0 + (actual_length as f64 * logic_length) as i32;
            }
            fn key_points(&self, max_points: usize) -> Vec<$type> {
                $key_points((self.0, self.1), max_points)
            }
            fn range(&self) -> Range<$type> {
                return self.0..self.1;
            }
        }
    };
}

macro_rules! gen_key_points_comp {
    (float, $name:ident, $type:ty) => {
        fn $name(range: ($type, $type), max_points: usize) -> Vec<$type> {
            let mut scale = (10 as $type).powf((range.1 - range.0).log(10.0).floor());
            fn rem_euclid(a: $type, b: $type) -> $type {
                if b > 0.0 {
                    a - (a / b).floor() * b
                } else {
                    a - (a / b).ceil() * b
                }
            }
            let mut left = range.0 + rem_euclid(range.0, -scale);
            let mut right = range.1 - rem_euclid(range.1, scale);
            'outer: loop {
                let old_scale = scale;
                for nxt in [2.0, 5.0, 10.0].iter() {
                    let new_left = range.0 + rem_euclid(range.0, -scale / nxt);
                    let new_right = range.1 - rem_euclid(range.1, scale / nxt);

                    let npoints = 1 + ((new_right - new_left) / old_scale * nxt) as usize;

                    if npoints > max_points {
                        break 'outer;
                    }

                    scale = old_scale / nxt;
                    left = new_left;
                    right = new_right;
                }
            }

            let mut ret = vec![];
            while left <= right {
                ret.push(left as $type);
                left += scale;
            }

            return ret;
        }
    };
    (integer, $name:ident, $type:ty) => {
        fn $name(range: ($type, $type), max_points: usize) -> Vec<$type> {
            let mut scale: $type = 1;
            'outter: while (range.1 - range.0 + scale - 1) as usize / (scale as usize) > max_points
            {
                let next_scale = scale * 10;
                for new_scale in [scale * 2, scale * 5, scale * 10].iter() {
                    scale = *new_scale;
                    if (range.1 - range.0 + *new_scale - 1) as usize / (*new_scale as usize)
                        < max_points
                    {
                        break 'outter;
                    }
                }
                scale = next_scale;
            }

            let (mut left, right) = (
                range.0 + (scale - range.0 % scale) % scale,
                range.1 - range.1 % scale,
            );

            let mut ret = vec![];
            while left <= right {
                ret.push(left as $type);
                left += scale;
            }

            return ret;
        }
    };
}

gen_key_points_comp!(float, compute_f32_key_points, f32);
gen_key_points_comp!(float, compute_f64_key_points, f64);
gen_key_points_comp!(integer, compute_i32_key_points, i32);
gen_key_points_comp!(integer, compute_u32_key_points, u32);
gen_key_points_comp!(integer, compute_i64_key_points, i64);
gen_key_points_comp!(integer, compute_u64_key_points, u64);

make_numeric_coord!(f32, RangedCoordf32, compute_f32_key_points);
make_numeric_coord!(f64, RangedCoordf64, compute_f64_key_points);
make_numeric_coord!(u32, RangedCoordu32, compute_u32_key_points);
make_numeric_coord!(i32, RangedCoordi32, compute_i32_key_points);
make_numeric_coord!(u64, RangedCoordu64, compute_u64_key_points);
make_numeric_coord!(i64, RangedCoordi64, compute_i64_key_points);

/*
pub fn keypoints_i64(range:(i64,i64), n:usize) -> Vec<i64> {
    compute_i64_key_points(range,n)
}*/

#[cfg(test)]
mod test {
    use super::*;
    use crate::coord::*;
    #[test]
    fn test_key_points() {
        let kp = compute_i32_key_points((0, 999), 28);

        assert!(kp.len() > 0);
        assert!(kp.len() <= 28);
    }

    #[test]
    fn test_linear_coord_map() {
        let coord: RangedCoordu32 = (0..20).into();
        assert_eq!(coord.key_points(11).len(), 11);
        assert_eq!(coord.key_points(11)[0], 0);
        assert_eq!(coord.key_points(11)[10], 20);
        assert_eq!(coord.map(&5, (0, 100)), 25);

        let coord: RangedCoordf32 = (0f32..20f32).into();
        assert_eq!(coord.map(&5.0, (0, 100)), 25);
    }

    #[test]
    fn test_linear_coord_system() {
        let _coord =
            RangedCoord::<RangedCoordu32, RangedCoordu32>::new(0..10, 0..10, (0..1024, 0..768));
    }
}