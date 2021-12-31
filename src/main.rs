mod lexer;
mod parser;
mod types;

fn main() {
    let mut a = "'hello \"world\"".chars();
    let lex = lexer::Lexer::new(&mut a);
    match lex.run() {
        Ok(tokens) => {
            println!("Success:");
            for t in &tokens {
                println!("{:?}", t)
            }
            let pars = parser::Parser::new(tokens).run();
            println!("{:?}", &pars);
        }
        Err(e) => println!("{}", e),
    }
}
