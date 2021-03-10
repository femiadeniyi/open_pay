use mysql::{PooledConn, params};
use mysql::prelude::Queryable;
use mysql::chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

// bank id
#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct Bank {
    pub id: Option<i64>,
    pub pat_id:String,
    pub profile_id:i32
}

#[derive(Serialize, Deserialize)]
pub struct Pat {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id:Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub buyer: u8,
    pub seller: u8,
    pub sort_code: String,
    pub account_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct Reference {
    pub id: Option<i64>,
    pub name:String
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<i64>,
    pub reference_id: i64,
    pub buyer_id: i8,
    pub seller_id: u8,
    pub transaction_status_id:i64,
    pub timestamp:NaiveDateTime
}

#[derive(Serialize, Deserialize)]
pub struct TransactionStatus {
    pub id: Option<i64>,
    pub status:String
}

pub enum Model {
    Bank(Vec<Bank>),
    Pat(Vec<Pat>),
    Person(Vec<Person>),
    Profile(Vec<Profile>),
    Reference(Vec<Reference>),
    Transaction(Vec<Transaction>),
    TransactionStatus(Vec<TransactionStatus>),
}

impl Model {
    pub fn database_name(&self) -> &str{
        match self {
            Model::Bank(_) => "bank",
            Model::Pat(_) => "pat",
            Model::Person(_) => "person",
            Model::Profile(_) => "profile",
            Model::Reference(_) => "reference",
            Model::Transaction(_) => "transaction",
            Model::TransactionStatus(_) => "transaction_status"
        }
    }
    pub fn fields(&self) -> Vec<&str>{
        match self {
            Model::Bank(_) => vec![
                "pat_id",
                "profile_id"
            ],
            Model::Pat(_) => vec![
                "id"
            ],
            Model::Person(_) =>  vec![
                "first_name",
                "last_name",
                "buyer",
                "seller",
                "sort_code",
                "account_number",
            ],
            Model::Profile(_) => vec![
                "id",
            ],
            Model::Reference(_) => vec![
                "id",
                "name"
            ],
            Model::Transaction(_) => vec![
                "reference_id",
                "buyer_id",
                "seller_id",
                "transaction_status_id",
                "timestamp"
            ],
            Model::TransactionStatus(_) => vec![
                "id",
                "status"
            ],
        }
    }
    pub fn write(&self, conn: &mut PooledConn){
        match self {
            Model::Bank(data) => {
                let query = query_string(self.database_name(),self.fields());
                let params = data.iter().map(|p| params! {
                    "pat_id" => &p.pat_id,
                    "profile_id" => &p.profile_id,
                });
                conn.exec_batch(query, params)
                    .expect(format!("error inserting {}",self.database_name()).as_str());
            }
            Model::Pat(data) => {
                let query = query_string(self.database_name(),self.fields());
                let params = data.iter().map(|p| params! {
                    "id" => &p.id,
                });
                conn.exec_batch(query, params)
                    .expect(format!("error inserting {}",self.database_name()).as_str());
            }
            Model::Profile(data)=> {
                let query = query_string(self.database_name(),self.fields());
                let params = data.iter().map(|p| params! {
                    "id" => &p.id,
                });
                conn.exec_batch(query, params)
                    .expect(format!("error inserting {}",self.database_name()).as_str());
            }
            Model::Person(data) => {
                let query = query_string(self.database_name(),self.fields());
                let params = data.iter().map(|p| params! {
                    "first_name" => &p.first_name,
                    "last_name" => &p.last_name,
                    "buyer" => &p.buyer,
                    "seller" => &p.seller,
                    "sort_code" => &p.sort_code,
                    "account_number" => &p.account_number,
                });
                conn.exec_batch(query, params)
                    .expect(format!("error inserting {}",self.database_name()).as_str());
            }
            Model::Reference(data) => {
                let query = query_string(self.database_name(),self.fields());
                let params = data.iter().map(|p| params! {
                    "name" => &p.name,
                });
                conn.exec_batch(query, params)
                    .expect(format!("error inserting {}",self.database_name()).as_str());
            }
            Model::Transaction(data) => {
                let query = query_string(self.database_name(),self.fields());
                let params = data.iter().map(|p| params! {
                    "reference_id" => &p.reference_id,
                    "buyer_id" => &p.buyer_id,
                    "seller_id" => &p.seller_id,
                    "transaction_status_id" => &p.transaction_status_id,
                    "timestamp" => &p.timestamp,
                });
                conn.exec_batch(query, params)
                    .expect(format!("error inserting {}",self.database_name()).as_str());
            }
            Model::TransactionStatus(data) => {
                let query = query_string(self.database_name(),self.fields());
                let params = data.iter().map(|p| params! {
                    "status" => &p.status,
                });
                conn.exec_batch(query, params)
                    .expect(format!("error inserting {}",self.database_name()).as_str());
            }
        }
    }
}

fn query_string(database_name:&str,fields:Vec<&str>) -> String {
    let insert = fields.iter().fold("".to_string(),|acc, &x| match x {
        x if x.to_string() == fields.last().expect("couldn't access last item").to_string() => format!("{}{}",acc,x),
        _ => format!("{}{},",acc,x)
    });
    let values = fields.iter().fold("".to_string(),|acc, &x| match x {
        x if x.to_string() == fields.last().expect("couldn't access last item").to_string() => format!("{}:{}",acc,x),
        _ => format!("{}:{},",acc,x)
    });
    let query = format!("INSERT INTO {} ({}) VALUES ({})",database_name,insert,values);
    return query
}


pub fn truncate_table(conn: &mut PooledConn, table: &str) {
    let query = format!("TRUNCATE TABLE {}", table);
    conn.query_drop(r"SET FOREIGN_KEY_CHECKS = 0").expect("error setting foreign key checks");
    conn.query_drop(query).expect("error truncating profile table");
}
