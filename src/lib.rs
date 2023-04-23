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
    fn syntax_error() {
        for test in [r"ab(cd", r"e(*)f", r")h", r"i|*", r"*", r"+", r"a*+"] {
            let regex = Regex::new(test);
            assert!(regex.is_err());
        }
    }
}
