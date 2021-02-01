use std::io::{self, Write};
use users::{get_user_by_uid, get_current_uid};
use crate::lexer::{Lexer};

const PROMPT: &[u8] =  b">> ";

pub fn start() {
  let stdout = io::stdout();
  let user = get_user_by_uid(get_current_uid()).unwrap();
  let username = user.name().to_string_lossy();
  let welcome = format!("Hello {}! This is the Monkey programming language! \
  Feel free to type in commands.\n", username);

  let mut handle = stdout.lock();
  handle.write(String::from(welcome).as_bytes()).unwrap();

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
        let tokens = l.read_tokens();
        for tok in tokens {
          println!("{}", tok);
        }
      },
      Err(error) => {
        println!("Fatal: {}", error);
        break;
      }
    }
  }
}
