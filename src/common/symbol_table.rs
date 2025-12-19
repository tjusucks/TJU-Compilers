use std::collections::HashMap;

use relex::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Terminal(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NonTerminal(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SymbolTable {
    non_terminals: HashMap<String, NonTerminal>,
    terminals: HashMap<String, Terminal>,
    non_terminal_names: Vec<String>,
    terminal_names: Vec<String>,
}

impl Terminal {
    pub fn to_string(self, symbol_table: &SymbolTable) -> Option<&str> {
        symbol_table.get_terminal_name(self)
    }
}

impl NonTerminal {
    pub fn to_string(self, symbol_table: &SymbolTable) -> Option<&str> {
        symbol_table.get_non_terminal_name(self)
    }
}

impl SymbolTable {
    #[must_use]
    pub fn from_maps(
        terminals: HashMap<String, Terminal>,
        non_terminals: HashMap<String, NonTerminal>,
    ) -> Self {
        let mut terminal_names = vec![String::new(); terminals.len()];
        for (name, Terminal(id)) in &terminals {
            terminal_names[*id] = name.clone();
        }
        let mut non_terminal_names = vec![String::new(); non_terminals.len()];
        for (name, NonTerminal(id)) in &non_terminals {
            non_terminal_names[*id] = name.clone();
        }
        Self {
            non_terminals,
            terminals,
            non_terminal_names,
            terminal_names,
        }
    }

    pub fn insert_non_terminal(&mut self, non_terminal_name: String) -> NonTerminal {
        if self.non_terminals.contains_key(&non_terminal_name) {
            self.non_terminals[&non_terminal_name]
        } else {
            let non_terminal_id = self.non_terminals.len();
            let non_terminal = NonTerminal(non_terminal_id);
            self.non_terminals
                .insert(non_terminal_name.clone(), non_terminal);
            self.non_terminal_names.push(non_terminal_name);
            non_terminal
        }
    }

    pub fn insert_terminal(&mut self, terminal_name: String) -> Terminal {
        if self.terminals.contains_key(&terminal_name) {
            self.terminals[&terminal_name]
        } else {
            let terminal_id = self.terminals.len();
            let terminal = Terminal(terminal_id);
            self.terminals.insert(terminal_name.clone(), terminal);
            self.terminal_names.push(terminal_name);
            terminal
        }
    }

    #[must_use]
    pub fn get_non_terminal_id(&self, non_terminal: &str) -> Option<NonTerminal> {
        self.non_terminals.get(non_terminal).copied()
    }

    #[must_use]
    pub fn get_terminal_id(&self, terminal: &str) -> Option<Terminal> {
        self.terminals.get(terminal).copied()
    }

    pub fn get_terminal_name(&self, terminal: Terminal) -> Option<&str> {
        self.terminal_names
            .get(terminal.0)
            .map(std::string::String::as_str)
    }

    pub fn get_non_terminal_name(&self, non_terminal: NonTerminal) -> Option<&str> {
        self.non_terminal_names
            .get(non_terminal.0)
            .map(std::string::String::as_str)
    }
}

impl TokenKind for Terminal {
    fn unrecognized() -> Self {
        Self(usize::MAX)
    }
    fn eof() -> Self {
        Self(usize::MAX - 1)
    }
}
