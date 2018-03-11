#[macro_use]
extern crate dotenv_codegen;

fn main() {
    let user = dotenv!("DB_USER");
    println!("{}", user);
}
