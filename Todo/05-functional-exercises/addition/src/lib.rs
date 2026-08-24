// no borrowing,
pub trait Addition {
    fn add(n: u64, m: u64) -> u64;
}

struct Normal;
struct SetTheoretic;

impl Addition for Normal {
    fn add(n: u64, m: u64) -> u64 {
        n + m
    }
}

// of the unique binary operator
fn f(value: u64) -> u64 {
    value + 1
}

impl Addition for SetTheoretic {
    fn add(n: u64, m: u64) -> u64 {
        // f(0) = n
        // for all k until k = m+ n
        // we acculate f(l^+) = f(l)^+1 --> which is the value + 1 on the outside.

        // since
        // fn fold<B, F>(self, init: B, f: F) -> B
        // where
        // F: FnMut(B, Self::Item) -> B

        (0..m).fold(n, |value, _| f(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pairs = (0..4)
            .map(|_| {
                let n: u64 = rand::random::<u32>() as u64;
                let m: u64 = rand::random::<u16>() as u64; // small: bounds fold's iteration count
                (n, m)
            })
            .collect::<Vec<_>>();

        let result = pairs
            .iter()
            .map(|&(n, m)| Normal::add(n, m))
            .zip(pairs.iter().map(|&(n, m)| SetTheoretic::add(n, m)))
            .all(|(n, m)| n == m);

        assert!(result, "Normal::add and SetTheoretic::add diverged on some pair");
    }
}
