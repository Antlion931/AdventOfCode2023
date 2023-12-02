// Structure to match a pattern in a text
#[derive(Debug, Clone)]
pub struct PatternMatcher {
    pattern: String,
    value: u32,
    current_matched_chars: usize,
    pi: Vec<usize>,
}

impl PatternMatcher {
    pub fn new(pattern: &str, value: u32) -> Self {
        Self {
            pattern: pattern.to_string(),
            value,
            current_matched_chars: 0,
            pi: pi(pattern.chars().collect::<Vec<_>>()),
        }
    }

    // Based on KMP algorithm, becouse after reading 'nin' and when next char is 'i' we can't just
    // start from the beginning, we need to go back to 'ni', returns bool if match is found
    pub fn matches(&mut self, c: char) -> bool {
        while self.current_matched_chars > 0
            && self.pattern.chars().nth(self.current_matched_chars) != Some(c)
        {
            self.current_matched_chars = self.pi[self.current_matched_chars - 1];
        }

        if self.pattern.chars().nth(self.current_matched_chars) == Some(c) {
            self.current_matched_chars += 1;
        }

        if self.current_matched_chars == self.pattern.len() {
            self.current_matched_chars = self.pi[self.current_matched_chars - 1];
            true
        } else {
            false
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    // Reverse pattern and pi table, becouse we need to match from the end of the line
    pub fn reverse(&self) -> Self {
        Self {
            pattern: self.pattern.chars().rev().collect(),
            value: self.value,
            current_matched_chars: 0,
            pi: self.pi.iter().rev().cloned().collect(),
        }
    }
}

// Calculate the prefix function of a pattern, used in KMP algorithm
fn pi(pattern: Vec<char>) -> Vec<usize> {
    let mut result = vec![0];
    let m = pattern.len();
    let mut k = 0;

    for q in 1..m {
        while k > 0 && pattern[k] != pattern[q] {
            k = result[k - 1];
        }

        if pattern[k] == pattern[q] {
            k += 1;
        }

        result.push(k);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi() {
        let text: Vec<_> = "ababababca".chars().collect();
        assert_eq!(pi(text), vec![0, 0, 1, 2, 3, 4, 5, 6, 0, 1]);
    }

    #[test]
    fn test_pattern_matcher() {
        let mut matcher = PatternMatcher::new("ababc", 1);
        assert_eq!(matcher.matches('a'), false);
        assert_eq!(matcher.matches('b'), false);
        assert_eq!(matcher.matches('a'), false);
        assert_eq!(matcher.matches('b'), false);
        assert_eq!(matcher.matches('c'), true);
    }

    #[test]
    fn test_pattern_matcher_correctly_go_back_to_longest_preffix() {
        let mut matcher = PatternMatcher::new("ababc", 1);
        assert_eq!(matcher.matches('a'), false);
        assert_eq!(matcher.matches('b'), false);
        assert_eq!(matcher.matches('a'), false);
        assert_eq!(matcher.matches('b'), false);

        assert_eq!(matcher.matches('a'), false); // go back to 'aba'

        assert_eq!(matcher.matches('b'), false);
        assert_eq!(matcher.matches('c'), true);
    }
}
