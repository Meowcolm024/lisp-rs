mod lexer;
mod parser;
mod types;

fn main() {
    let mut a = "(map (lambda (x) (* x 2)) (list 1 3 4))".chars();
    let lex = lexer::Lexer::new(&mut a);
    match lex.run() {
        Ok(tokens) => {
            println!("Success:");
            for t in tokens {
                println!("{:?}", t)
            }
        }
        Err(e) => println!("Lex error: {}", e),
    }
}
