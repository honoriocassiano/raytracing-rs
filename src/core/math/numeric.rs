pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::core::math::numeric::clamp;

    #[test]
    #[allow(clippy::float_cmp)]
    fn must_clamp_to_max() {
        let min = 0.45;
        let max = 1.50874;

        let val = 1.5087400001;

        let expected = max;

        assert_eq!(expected, clamp(val, min, max));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn must_clamp_to_min() {
        let min = -14.3756;
        let max = 67.903746;

        let val = -15.497;

        let expected = min;

        assert_eq!(expected, clamp(val, min, max));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn must_not_clamp() {
        let min = -8.5763;
        let max = 7.4957;

        let val = 5.4957;

        let expected = val;

        assert_eq!(expected, clamp(val, min, max));
    }
}
