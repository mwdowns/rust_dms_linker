// extern crate futures;

#[macro_use]
extern crate dotenv_codegen;
extern crate mysql_async as my;
// extern crate tokio_core as tokio;

// use futures::Future;
// use my::prelude::*;
// use tokio::reactor::Core;

// #[derive(Debug, PartialEq, Eq, Clone)]


fn main() {

    let mut builder = my::OptsBuilder::new();
    let host = dotenv!("DB_HOST");
    let db = dotenv!("DB_NAME");
    let user = dotenv!("DB_USER");
    let pass = dotenv!("DB_PASS");
    builder.ip_or_hostname(host);
    builder.db_name(Some(db));
    builder.user(Some(user));
    builder.pass(Some(pass));
    println!("{:?}", builder);
}
