pub trait Printer {
    fn print(&self, str: String) -> String;
}

pub struct BasePrinter {}

impl Printer for BasePrinter {
    fn print(&self, str: String) -> String {
        str
    }
}

pub struct BracketsPrinter<R> {
    pub decorated: R,
}

impl<T: Printer> BracketsPrinter<T> {
    pub fn new(decorated: T) -> Self {
        Self { decorated }
    }
}

impl<T: Printer> Printer for BracketsPrinter<T> {
    fn print(&self, str: String) -> String {
        let printed = self.decorated.print(str);
        format!("({})", printed)
    }
}

pub struct QuotesPrinter<R: Printer> {
    decorated: R,
}

impl<T: Printer> QuotesPrinter<T> {
    pub fn new(decorated: T) -> Self {
        Self { decorated }
    }
}

impl<T: Printer> Printer for QuotesPrinter<T> {
    fn print(&self, str: String) -> String {
        let printed = self.decorated.print(str);
        format!("\"{}\"", printed)
    }
}
