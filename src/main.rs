mod token;

use token::Scanner;

fn main() {
    let source = r#"
        let x = 10;
        print x + 20;
    "#;

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    println!("{:#?}", tokens);
}
