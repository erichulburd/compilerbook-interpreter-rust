use interpreter::ast::program::Program;
use interpreter::ast::token_node::TokenNode;
use interpreter::lexer::Lexer;
use interpreter::parser::parser::Parser;
use std::io::{self, Write};
use users::{get_current_uid, get_user_by_uid};

const PROMPT: &[u8] = b">> ";

const MONKEY_FACE: &'static str = r#"
            __,__
   .--.  .-"     "-.  .--.\
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

pub fn start() {
    let stdout = io::stdout();
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = user.name().to_string_lossy();
    let welcome = format!(
        "Hello {}! This is the Monkey programming language! \
  Feel free to type in commands.\n",
        username
    );

    let mut handle = stdout.lock();
    handle.write_all(String::from(welcome).as_bytes()).unwrap();

    loop {
        handle.write_all(PROMPT).unwrap();
        handle.flush().unwrap();
        // println!("BYE");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if input.trim().eq("quit") {
                    return;
                }
                let mut l = Lexer::new(input.as_str());
                let mut p = Parser::new(&mut l);
                let program: Program = p.parse_program();
                if p.errors.len() != 0 {
                    println!("{}", MONKEY_FACE);
                    for err in p.errors.iter() {
                        println!("\t{}\n", err);
                    }
                    return;
                }
                println!("{}\n", program.string());
            }
            Err(error) => {
                println!("Fatal: {}", error);
                break;
            }
        }
    }
}

fn main() {
    start();
}
