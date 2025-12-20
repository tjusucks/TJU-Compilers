use std::collections::HashMap;
use std::sync::Arc;

use relex::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Terminal(pub Arc<str>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NonTerminal(pub Arc<str>);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SymbolTable {
    non_terminals: HashMap<String, NonTerminal>,
    terminals: HashMap<String, Terminal>,
    non_terminal_names: HashMap<NonTerminal, String>,
    terminal_names: HashMap<Terminal, String>,
}

impl Terminal {
    pub fn to_string(self, symbol_table: &SymbolTable) -> Option<&str> {
        symbol_table.get_terminal_name(&self)
    }
}

impl NonTerminal {
    pub fn to_string(self, symbol_table: &SymbolTable) -> Option<&str> {
        symbol_table.get_non_terminal_name(&self)
    }
}

impl SymbolTable {
    #[must_use]
    pub fn from_maps(
        terminals: HashMap<String, Terminal>,
        non_terminals: HashMap<String, NonTerminal>,
    ) -> Self {
        let mut terminal_names = HashMap::new();
        for (name, terminal) in &terminals {
            terminal_names.insert(terminal.clone(), name.clone());
        }
        let mut non_terminal_names = HashMap::new();
        for (name, non_terminal) in &non_terminals {
            non_terminal_names.insert(non_terminal.clone(), name.clone());
        }
        Self {
            non_terminals,
            terminals,
            non_terminal_names,
            terminal_names,
        }
    }

    pub fn insert_non_terminal(&mut self, non_terminal_name: String) -> NonTerminal {
        if let Some(non_terminal) = self.non_terminals.get(&non_terminal_name) {
            non_terminal.clone()
        } else {
            let arc_name = Arc::from(non_terminal_name.clone().into_boxed_str());
            let non_terminal = NonTerminal(arc_name);
            self.non_terminals
                .insert(non_terminal_name.clone(), non_terminal.clone());
            self.non_terminal_names
                .insert(non_terminal.clone(), non_terminal_name);
            non_terminal
        }
    }

    pub fn insert_terminal(&mut self, terminal_name: String) -> Terminal {
        if let Some(terminal) = self.terminals.get(&terminal_name) {
            terminal.clone()
        } else {
            let arc_name = Arc::from(terminal_name.clone().into_boxed_str());
            let terminal = Terminal(arc_name);
            self.terminals
                .insert(terminal_name.clone(), terminal.clone());
            self.terminal_names.insert(terminal.clone(), terminal_name);
            terminal
        }
    }

    #[must_use]
    pub fn get_non_terminal_id(&self, non_terminal: &str) -> Option<NonTerminal> {
        self.non_terminals.get(non_terminal).cloned()
    }

    #[must_use]
    pub fn get_terminal_id(&self, terminal: &str) -> Option<Terminal> {
        self.terminals.get(terminal).cloned()
    }

    pub fn get_non_terminal_name(&self, non_terminal: &NonTerminal) -> Option<&str> {
        self.non_terminal_names
            .get(non_terminal)
            .map(std::string::String::as_str)
    }

    pub fn get_terminal_name(&self, terminal: &Terminal) -> Option<&str> {
        self.terminal_names
            .get(terminal)
            .map(std::string::String::as_str)
    }
}

impl TokenKind for Terminal {
    fn unrecognized() -> Self {
        Self(Arc::from("<UNRECOGNIZED>"))
    }
    fn eof() -> Self {
        Self(Arc::from("<EOF>"))
    }
}
