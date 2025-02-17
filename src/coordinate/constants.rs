const EPISOLON: f64 = 0.00001;

pub fn fuzzy_equal(first: f64, second: f64) -> bool {
    if (first - second).abs() < EPISOLON {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn are_fuzzy_equal() {
        assert!(fuzzy_equal(1.00000, 1.000001));
    }
}