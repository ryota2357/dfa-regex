mod automaton;
mod lexer;
mod parser;

use automaton::*;
use lexer::*;
use parser::*;

pub struct Regex {
    dfa: DeterministicFiniteAutomaton,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, String> {
        let parser = &mut Parser::new(Lexer::new(pattern));
        let node = parser.parse()?;
        let nfa = NondeterministicFiniteAutomaton::from_node(node);
        let dfa = DeterministicFiniteAutomaton::from_nfa(nfa);
        Ok(Regex { dfa })
    }

    pub fn matches(&self, text: &str) -> bool {
        let mut current_state = self.dfa.start;
        for char in text.chars() {
            if let Some(state) = self.dfa.next_state(current_state, char) {
                current_state = state;
            } else {
                return false;
            }
        }
        self.dfa.accepts.contains(&current_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_case1() {
        let regex = Regex::new(r"(p(erl|ython|hp)|ruby)").unwrap();
        assert!(regex.matches("python"));
        assert!(regex.matches("ruby"));
        assert!(!regex.matches("VB"));
    }

    #[test]
    fn matches_case2() {
        let regex = Regex::new(r"山田(太|一|次|三)郎").unwrap();
        assert!(regex.matches("山田太郎"));
        assert!(regex.matches("山田三郎"));
        assert!(!regex.matches("山田郎"));
    }

    #[test]
    fn matches_case3() {
        let regex = Regex::new(r"ｗｗ*|\(笑\)").unwrap();
        assert!(regex.matches("(笑)"));
        assert!(regex.matches("ｗｗｗ"));
        assert!(!regex.matches("笑"));
    }

    #[test]
    fn matches_case4() {
        let regex = Regex::new(r"a\c").unwrap();
        assert!(regex.matches(r"ac"));
        assert!(!regex.matches(r"a\c"));
    }

    #[test]
    fn matches_case5() {
        let regex = Regex::new(r"a\\c").unwrap();
        assert!(regex.matches(r"a\c"));
        assert!(!regex.matches(r"ac"));
    }

    #[test]
    fn matches_case6() {
        let regex = Regex::new(r"a(b|)").unwrap();
        assert!(regex.matches(r"ab"));
        assert!(regex.matches(r"a"));
        assert!(!regex.matches(r"abb"));
    }

    #[test]
    fn syntax_error() {
        for test in [r"ab(cd", r"e(*)f", r")h", r"i|*", r"*"] {
            let regex = Regex::new(test);
            assert!(regex.is_err());
        }
    }
}
