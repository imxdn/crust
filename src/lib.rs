mod scanner;

use scanner::Scanner;

pub fn greet() {
    let exp = String::from("11+1.13455");
    let mut scanner = Scanner::new(&exp);
    dbg!(scanner.scan());
}