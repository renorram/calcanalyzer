mod token;

use token::Token;

fn main() {
    let mut buffer: String = String::new();

    println!("Type an expression:");

    std::io::stdin().read_line(&mut buffer).expect("Not able to read from stdin!");

    let token = Token::process_token_stream(&buffer).unwrap_or_else(|error| {
        println!("{}", error);
        std::process::exit(1);
    });

    Token::print_token_table(token);
}
