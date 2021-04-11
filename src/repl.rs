use interpreter::evaluator::evaluate::evaluate;
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

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if input.trim().eq("quit") {
                    return;
                }
                let result = evaluate(input.as_str());
                match result {
                    Ok(object) => {
                        println!("{}", object.string())
                    }
                    Err(e) => {
                        println!("{}\n{}", MONKEY_FACE, e);
                    }
                }
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
