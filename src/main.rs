extern crate futures;

#[macro_use]
extern crate dotenv_codegen;
extern crate mysql_async as my;
extern crate tokio_core as tokio;

use futures::Future;
use my::prelude::*;
use tokio::reactor::Core;

#[derive(Debug, PartialEq, Eq, Clone)]


fn main() {

    let mut builder = OptsBuilder::new();
    builder.ip_or_hostname(dotenv!("DB_HOST"))
        .db_name(dotevn!("DB_NAME"))
        .user(dotenv!("DB_USER"))
        .pass(dotenv!("DB_PASS"));
    println!("{}", builder);
}
