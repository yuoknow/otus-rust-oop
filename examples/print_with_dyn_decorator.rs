use decorator::BasePrinter;
use decorator::BracketsPrinter;
use decorator::QuotesPrinter;
use rust_oop::dyn_decorator::decorator;
use rust_oop::dyn_decorator::decorator::Printer;

fn main() {
    let with_brackets = BracketsPrinter::new(Box::new(BasePrinter {}));
    let with_quotes_and_brackets = QuotesPrinter::new(Box::new(with_brackets));

    println!("{:?}", with_quotes_and_brackets.print("test".to_string()));
}
