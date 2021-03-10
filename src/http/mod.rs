use hyper::{Client, Method, Request, Body,body::to_bytes};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::iter::Map;
use mysql::{PooledConn,params};
use mysql::prelude::Queryable;

#[derive(Deserialize, Debug)]
struct ProfileApi {
    id:i32,
    #[serde(rename = "type")]
    type_name:String,
}

pub async fn get_profile_id(pat:&str) -> i32{

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::GET)
        .uri("https://api.transferwise.com/v1/profiles")
        .header("Authorization", format!("bearer {}",pat))
        .header("content-type", "application/json")
        .body(Body::empty())
        .expect("error building request");

    let resp = client.request(req).await
        .expect("error sending request");

    let body = to_bytes(resp.into_body()).await.expect("error parsing body");
    let profiles:Vec<ProfileApi> = serde_json::from_slice(&body).expect("f");

    return profiles.into_iter()
        .filter(|x|x.type_name == "business")
        .collect::<Vec<_>>().first().expect("empty vector").id;
}

pub async fn borderless_account_id(pat:&str,profile_id:i32) -> i32{

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("https://api.transferwise.com/v1/borderless-accounts?profileId={}",profile_id))
        .header("Authorization", format!("bearer {}",pat))
        .header("content-type", "application/json")
        .body(Body::empty())
        .expect("error building request");

    let resp = client.request(req).await
        .expect("error sending request");

    println!("{:?}",resp);

    let body = to_bytes(resp.into_body()).await.expect("error parsing body");
    println!("{}",String::from_utf8_lossy(&body.to_vec()));
    return 33;
    // let profiles:Vec<ProfileApi> = serde_json::from_slice(&body).expect("f");
    //
    // return profiles.into_iter()
    //     .filter(|x|x.type_name == "business")
    //     .collect::<Vec<_>>().first().expect("empty vector").id;
}


