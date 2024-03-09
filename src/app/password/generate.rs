use core::{iter, ops::RangeInclusive};

use rand::distributions::{Distribution, Slice, Uniform};
use rand::prelude::Rng;

use super::PasswordType;

const NUMBERS_RANGE: RangeInclusive<char> = '1'..='9';
const ALPHABET_LOWER_RANGE: RangeInclusive<char> = 'a'..='z';
const ALPHABET_UPPER_RANGE: RangeInclusive<char> = 'A'..='Z';
// I excluded whitespace characters and the backtick comma, and period, and quote, and backslash. [original list](https://owasp.org/www-community/password-special-characters)
const SYMBOLS_ARR: [char; 25] = [
    '!', '#', '$', '%', '&', '(', ')', '*', '+', '-', '/', ':', ';', '<', '=', '>', '?', '@', '[',
    ']', '^', '_', '{', '|', '}',
];

impl PasswordType {
    pub fn generate(&self, length: usize) -> String {
        match self {
            PasswordType::Pin => get_random_string(length, true, false, false),
            PasswordType::Random { numbers, symbols } => {
                get_random_string(length, *numbers, *symbols, true)
            }
        }
    }
}

enum DistributionType<'a> {
    Uniform(Uniform<char>),
    Slice(Slice<'a, char>),
}

impl DistributionType<'_> {
    fn sample(&self, rng: &mut impl Rng) -> char {
        match self {
            DistributionType::Uniform(dist) => dist.sample(rng),
            DistributionType::Slice(dist) => *dist.sample(rng),
        }
    }
}

fn get_distributions<'a>(numbers: bool, symbols: bool, letters: bool) -> Vec<DistributionType<'a>> {
    use DistributionType as D;

    let mut ranges = vec![];

    if numbers {
        ranges.push(D::Uniform(Uniform::from(NUMBERS_RANGE)));
    }
    if letters {
        ranges.push(D::Uniform(Uniform::from(ALPHABET_LOWER_RANGE)));
        ranges.push(D::Uniform(Uniform::from(ALPHABET_UPPER_RANGE)));
    }
    if symbols {
        let dist = Slice::new(&SYMBOLS_ARR).expect("slice shouldn't be empty");
        ranges.push(D::Slice(dist));
    }

    ranges
}

/// generates a random string of length `length` with at least one character
/// from each of the options marked true
fn get_random_string(length: usize, numbers: bool, symbols: bool, letters: bool) -> String {
    let distributions = get_distributions(numbers, symbols, letters);

    (0..distributions.len())
        .chain(iter::from_fn(|| {
            Some(rand::thread_rng().gen_range(0..distributions.len()))
        }))
        .map(|i| distributions[i].sample(&mut rand::thread_rng()))
        .take(length)
        .collect::<String>()
}
