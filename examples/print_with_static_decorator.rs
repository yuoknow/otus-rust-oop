use rust_oop::static_decorator::decorator::{BasePrinter, BracketsPrinter, Printer, QuotesPrinter};

fn main() {
    let with_brackets = BracketsPrinter::new(BasePrinter {});
    let with_quotes_and_brackets = QuotesPrinter::new(with_brackets);

    println!("{:?}", with_quotes_and_brackets.print("test".to_string()));
}
