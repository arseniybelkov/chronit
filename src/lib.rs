use std::time::Instant;

pub struct Output<T> {
    pub value: T,
    pub time: f32,
}

#[inline(always)]
pub fn chronit<T>(fun: impl FnOnce() -> T) -> Output<T> {
    let timer = Instant::now();
    let value = fun();
    let delta = timer.elapsed();

    Output {value: value, time: delta.as_secs_f32()}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    fn one_sec() {
        let closure = || {
            thread::sleep(Duration::new(1, 0));
            2f32 * 3f32
        };
        let six = chronit(closure);

        assert_eq!(six.value, 6f32);
        assert!(f32::abs(six.time - 1f32) < 0.1);
    }

    #[test]
    fn one_and_half_sec() {
        let closure = || {
            thread::sleep(Duration::new(1, 500_000_000));
            2f32 * 3f32
        };
        let six = chronit(closure);

        assert_eq!(six.value, 6f32);
        assert!(f32::abs(six.time - 1.5f32) < 0.1);
    }

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];
        let get_len = || {
            thread::sleep(Duration::new(0, 500_000_000));
            v.len()
        };
        let out = chronit(get_len);

        assert_eq!(out.value, 3);
        assert!(f32::abs(out.time - 0.5f32) < 0.1);
    }
}
