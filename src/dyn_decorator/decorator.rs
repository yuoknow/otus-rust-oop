pub trait Printer {
    fn print(&self, str: String) -> String;
}

pub struct BasePrinter {}

impl Printer for BasePrinter {
    fn print(&self, str: String) -> String {
        str
    }
}

pub struct BracketsPrinter {
    decorated: Box<dyn Printer>,
}

impl BracketsPrinter {
    pub fn new(decorated: Box<dyn Printer>) -> Self {
        Self { decorated }
    }
}

impl Printer for BracketsPrinter {
    fn print(&self, str: String) -> String {
        let printed = self.decorated.print(str);
        format!("({})", printed)
    }
}

pub struct QuotesPrinter {
    decorated: Box<dyn Printer>,
}

impl QuotesPrinter {
    pub fn new(decorated: Box<dyn Printer>) -> Self {
        Self { decorated }
    }
}

impl Printer for QuotesPrinter {
    fn print(&self, str: String) -> String {
        let printed = self.decorated.print(str);
        format!("\"{}\"", printed)
    }
}
