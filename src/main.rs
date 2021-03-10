use std::convert::Infallible;
// use std::net::SocketAddr;
extern crate openssl;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use open_pay::db::{truncate_table, Model, Profile, Pat, Bank, Person};
use mysql::{Pool};
use std::env;
use open_pay::http::{get_profile_id, borderless_account_id};
use openssl::rsa::{Rsa, Padding};

// async fn main_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     match (req.method(), req.uri().path()) {
//         (&Method::GET, "/") => {
//             *response.body_mut() = Body::from("Try POSTing data to /echo");
//         },
//         (&Method::POST, "/echo") => {
//             // we'll be back
//         },
//         _ => {
//             Ok(StatusCode::NOT_FOUND)
//         },
//     };
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

    // setup - pat, profile, bank, person
    Model::Pat(vec![
        Pat{
            id: pat.to_string(),
        }
    ]).write(&mut conn);

    let profile_id = get_profile_id(pat).await;
    Model::Profile(vec![
        Profile{
            id: profile_id
        }
    ]).write(&mut conn);

    Model::Bank(vec![
        Bank{
            id: None,
            pat_id:pat.to_string(),
            profile_id,
        }
    ]).write(&mut conn);

    Model::Person(vec![
        Person{
            id: None,
            first_name: "Jack".to_string(),
            last_name: "Nielson".to_string(),
            buyer: 1,
            seller: 0,
            sort_code: "101010".to_string(),
            account_number: "12345678".to_string()
        },
        Person{
            id: None,
            first_name: "Gary".to_string(),
            last_name: "Barlow".to_string(),
            buyer: 0,
            seller: 1,
            sort_code: "101010".to_string(),
            account_number: "10101010".to_string()
        },
    ]).write(&mut conn);

    borderless_account_id(pat,profile_id).await;

    // setup - pat, profile_id, bank
    // write_pat(&mut conn, pat);

    // write_profile_id(&mut conn,profile_id);
    // write_bank(&mut conn,pat,profile_id);



    // We'll bind to 127.0.0.1:3000
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
