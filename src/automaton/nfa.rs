use crate::parser::Node;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct NFAState(pub u32);

struct Context {
    state_count: u32,
}

impl Context {
    fn new() -> Self {
        Context { state_count: 0 }
    }

    fn new_state(&mut self) -> NFAState {
        let id = self.state_count;
        self.state_count += 1;
        NFAState(id)
    }
}

impl Node {
    fn assemble(&self, context: &mut Context) -> NondeterministicFiniteAutomaton {
        match self {
            Node::Character(char) => {
                let start = context.new_state();
                let accept = context.new_state();
                NondeterministicFiniteAutomaton::new(start, [accept].into())
                    .add_transition(start, *char, accept)
            }
            Node::Empty => {
                let start = context.new_state();
                let accept = context.new_state();
                NondeterministicFiniteAutomaton::new(start, [accept].into())
                    .add_empty_transition(start, accept)
            }
            Node::Star(node) => {
                let frag = node.assemble(context);
                let start = context.new_state();
                let accepts = frag.accepts.union(&[start].into()).cloned().collect();
                let mut nfa = NondeterministicFiniteAutomaton::new(start, accepts)
                    .merge_transition(&frag)
                    .add_empty_transition(start, frag.start);
                for accept in &frag.accepts {
                    nfa = nfa.add_empty_transition(*accept, frag.start);
                }
                nfa
            }
            Node::Union(node1, node2) => {
                let frag1 = node1.assemble(context);
                let frag2 = node2.assemble(context);
                let start = context.new_state();
                let accepts = frag1.accepts.union(&frag2.accepts).cloned().collect();
                NondeterministicFiniteAutomaton::new(start, accepts)
                    .merge_transition(&frag1)
                    .merge_transition(&frag2)
                    .add_empty_transition(start, frag1.start)
                    .add_empty_transition(start, frag2.start)
            }
            Node::Concat(node1, node2) => {
                let frag1 = node1.assemble(context);
                let frag2 = node2.assemble(context);
                let mut fragment =
                    NondeterministicFiniteAutomaton::new(frag1.start, frag2.accepts.clone())
                        .merge_transition(&frag1)
                        .merge_transition(&frag2);
                for accept1 in &frag1.accepts {
                    fragment = fragment.add_empty_transition(*accept1, frag2.start);
                }
                fragment
            }
        }
    }
}

pub struct NondeterministicFiniteAutomaton {
    pub start: NFAState,
    pub accepts: HashSet<NFAState>,
    transition: HashMap<NFAState, HashMap<Option<char>, HashSet<NFAState>>>,
}

impl NondeterministicFiniteAutomaton {
    pub fn new(start: NFAState, accepts: HashSet<NFAState>) -> Self {
        NondeterministicFiniteAutomaton {
            start,
            accepts,
            transition: HashMap::new(),
        }
    }

    pub fn from_node(node: Node) -> Self {
        node.assemble(&mut Context::new())
    }

    pub fn next_chars(&self, state: NFAState) -> HashSet<Option<char>> {
        self.transition
            .get(&state)
            .map(|table| table.keys().cloned().collect())
            .unwrap_or(HashSet::new())
    }

    pub fn next_states(&self, state: NFAState, char: Option<char>) -> HashSet<NFAState> {
        self.transition
            .get(&state)
            .and_then(|table| table.get(&char))
            .cloned()
            .unwrap_or(HashSet::new())
    }

    pub fn add_transition(mut self, from: NFAState, char: char, to: NFAState) -> Self {
        self._insert_transition(from, to, Some(char));
        self
    }

    pub fn add_empty_transition(mut self, from: NFAState, to: NFAState) -> Self {
        self._insert_transition(from, to, None);
        self
    }

    fn merge_transition(mut self, other: &Self) -> Self {
        for (from_state, trans) in &other.transition {
            for (char, to_states) in trans {
                self.transition
                    .entry(*from_state)
                    .or_insert(HashMap::new())
                    .entry(*char)
                    .or_insert(HashSet::new())
                    .extend(to_states);
            }
        }
        self
    }

    fn _insert_transition(&mut self, from: NFAState, to: NFAState, char: Option<char>) {
        let to_states = self
            .transition
            .entry(from)
            .or_insert(HashMap::new())
            .entry(char)
            .or_insert(HashSet::new());
        to_states.insert(to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context() {
        let mut context = Context::new();
        assert_eq!(context.new_state(), NFAState(0));
        assert_eq!(context.new_state(), NFAState(1));
        assert_eq!(context.new_state(), NFAState(2));
    }

    #[test]
    fn from_character_node() {
        let nfa = NondeterministicFiniteAutomaton::from_node(Node::Character('a'));

        // -> 0 --a--> 1
        // accept: 1
        assert_eq!(nfa.start, NFAState(0));
        assert_eq!(nfa.accepts, [NFAState(1)].into());
        assert_eq!(
            nfa.transition,
            [(NFAState(0), [(Some('a'), [NFAState(1)].into())].into())].into()
        );
    }

    #[test]
    fn from_empty_node() {
        let nfa = NondeterministicFiniteAutomaton::from_node(Node::Empty);

        // -> 0 --ε--> 1
        // accept: 1
        assert_eq!(nfa.start, NFAState(0));
        assert_eq!(nfa.accepts, [NFAState(1)].into());
        assert_eq!(
            nfa.transition,
            [(NFAState(0), [(None, [NFAState(1)].into())].into())].into()
        );
    }

    #[test]
    fn from_star_node() {
        let nfa =
            NondeterministicFiniteAutomaton::from_node(Node::Star(Box::new(Node::Character('a'))));

        //              /<--ε--\
        // -> 2 --ε--> 0 --a--> 1
        // accept: 2, 1
        assert_eq!(nfa.start, NFAState(2));
        assert_eq!(nfa.accepts, [NFAState(2), NFAState(1)].into());
        assert_eq!(
            nfa.transition,
            [
                (NFAState(2), [(None, [NFAState(0)].into())].into()),
                (NFAState(0), [(Some('a'), [NFAState(1)].into())].into()),
                (NFAState(1), [(None, [NFAState(0)].into())].into())
            ]
            .into()
        );
    }

    #[test]
    fn from_union_node() {
        let nfa = NondeterministicFiniteAutomaton::from_node(Node::Union(
            Box::new(Node::Character('a')),
            Box::new(Node::Character('b')),
        ));

        //     /--ε--> 0 --a--> 1
        // -> 4
        //     \--ε--> 2 --b--> 3
        // accept: 1, 3
        assert_eq!(nfa.start, NFAState(4));
        assert_eq!(nfa.accepts, [NFAState(1), NFAState(3)].into());
        assert_eq!(
            nfa.transition,
            [
                (
                    NFAState(4),
                    [(None, [NFAState(0), NFAState(2)].into())].into()
                ),
                (NFAState(0), [(Some('a'), [NFAState(1)].into())].into()),
                (NFAState(2), [(Some('b'), [NFAState(3)].into())].into())
            ]
            .into()
        );
    }

    #[test]
    fn from_concat_node() {
        let nfa = NondeterministicFiniteAutomaton::from_node(Node::Concat(
            Box::new(Node::Character('a')),
            Box::new(Node::Character('b')),
        ));

        // -> 0 --a--> 1 --ε--> 2 --b--> 3
        // accept: 3
        assert_eq!(nfa.start, NFAState(0));
        assert_eq!(nfa.accepts, [NFAState(3)].into());
        assert_eq!(
            nfa.transition,
            [
                (NFAState(0), [(Some('a'), [NFAState(1)].into())].into()),
                (NFAState(1), [(None, [NFAState(2)].into())].into()),
                (NFAState(2), [(Some('b'), [NFAState(3)].into())].into())
            ]
            .into()
        );
    }
}
