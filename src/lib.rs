pub fn _prime_factors_1(mut num: i32) -> Vec<i32> {
    let mut answer = vec![];

    while num != 1 {
        for candidate in 2..=num {
            if num % candidate == 0 {
                answer.push(candidate);
                num = num / candidate;
                break;
            }
        }
    }
    answer
}

pub fn _prime_factors(num: i32) -> Vec<i32> {
    for candidate in 2..num {
        if num % candidate == 0 {
            let mut answer = vec![candidate];
            answer.extend(&prime_factors(num / candidate));
            return answer;
        }
    }
    vec![num]
}

pub struct PrimeFactorsIterator {
    num: i32,
    candidate: i32,
}

impl PrimeFactorsIterator {
    pub fn new(num: i32) -> PrimeFactorsIterator {
        PrimeFactorsIterator { num, candidate: 2 }
    }
}

impl Iterator for PrimeFactorsIterator {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        while self.num > 1 {
            while self.num % self.candidate == 0 {
                self.num /= self.candidate;
                return Some(self.candidate);
            }
            self.candidate += 1;
        }
        None
    }
}

pub fn prime_factors(num: i32) -> Vec<i32> {
    PrimeFactorsIterator::new(num).collect()
}

#[cfg(test)]
mod tests {
    use crate::prime_factors;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn prime_factors_of_two() {
        assert_eq!(prime_factors(2), [2]);
    }

    #[test]
    fn prime_factors_of_three() {
        assert_eq!(prime_factors(3), [3]);
    }

    #[test]
    fn prime_factors_of_four() {
        assert_eq!(prime_factors(4), [2, 2]);
    }

    #[test]
    fn prime_factors_of_nine() {
        assert_eq!(prime_factors(9), [3, 3]);
    }
}
