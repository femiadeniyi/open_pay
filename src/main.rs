// use std::convert::Infallible;
// use std::net::SocketAddr;
// use hyper::{Body, Request, Response, Server};
// use hyper::service::{make_service_fn, service_fn};
use open_pay::db::{write_pat, write_profile_id, truncate_table, write_bank};
use mysql::{Pool};
use std::env;
use open_pay::http::{get_profile_id,};


// async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new("Hello, World".into()))
// }

fn env_setup(){
    // mysql database & url
    let mysql_url = "mysql://root:abc123@localhost:3306/";
    let mysql_db = "open_pay";
    env::set_var("mysql_url", mysql_url);
    env::set_var("mysql_db", mysql_db);
}

#[tokio::main]
async fn main() {

    // env setup
    env_setup();

    // vars pat
    let pat = "05712a70-b067-4886-8197-a7db4baf32f2";

    // vars mysql
    let mysql_url = env::var("mysql_url").expect("error getting mysql url");
    let mysql_db = env::var("mysql_db").expect("error getting mysql db");
    let url = format!("{}{}",mysql_url,mysql_db);
    let pool = Pool::new(url).expect("error opening pool");
    let mut conn = pool.get_conn().expect("error getting connection from pool");

    // setup tables - truncate
    let tables = vec![
        "pat",
        "bank",
        "person",
        "profile",
        "reference",
        "transaction",
        "transaction_status"
    ];
    tables.iter().for_each(|&x|{
        truncate_table(&mut conn,x);
    });

    // setup - pat, profile_id, bank
    write_pat(&mut conn, pat);
    let profile_id = get_profile_id(pat).await;
    write_profile_id(&mut conn,profile_id);
    write_bank(&mut conn,pat,profile_id);



    // // We'll bind to 127.0.0.1:3000
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    //
    // // A `Service` is needed for every connection, so this
    // // creates one from our `hello_world` function.
    // let make_svc = make_service_fn(|_conn| async {
    //     // service_fn converts our function into a `Service`
    //     Ok::<_, Infallible>(service_fn(hello_world))
    // });
    //
    // let server = Server::bind(&addr).serve(make_svc);
    //
    // // Run this server for... forever!
    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }
}
