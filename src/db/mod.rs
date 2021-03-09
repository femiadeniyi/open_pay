use mysql::{PooledConn, params};
use mysql::prelude::Queryable;

enum DbTables {
    ProfileId
}

pub struct ProfileId {
    id: String
}

pub struct Db {}

trait Write<T> {
    fn write(conn: &mut PooledConn, data: Vec<T>);
}

impl Write<Vec<ProfileId>> for Db {
    fn write(conn: &mut PooledConn, data: Vec<ProfileId>) {
        let query = format!("INSERT INTO profile (id) VALUES (:id)");

        conn.exec_batch(query,
            data.iter().map(|p| params! {
                "id" => &p.id,
            }),
        ).expect("error inserting profile_id");
        conn.exec_first::<i32, _, _>(query, params! {"id" => data.id}).expect("error inserting pat");
    }
}

pub fn truncate_table(conn: &mut PooledConn, table: &str) {
    let query = format!("TRUNCATE TABLE {}", table);
    conn.query_drop(r"SET FOREIGN_KEY_CHECKS = 0").expect("error setting foreign key checks");
    conn.query_drop(query).expect("error truncating profile table");
}

pub fn write_bank(conn: &mut PooledConn, pat_id: &str, profile_id: i32) {
    let query = format!("INSERT INTO bank (pat_id,profile_id) VALUES (:pat_id,:profile_id)");
    conn.exec_first::<i32, _, _>(query, params! {pat_id,profile_id}).expect("error inserting pat");
}

pub fn write_pat(conn: &mut PooledConn, pat: &str) {
    let query = format!("INSERT INTO pat (id) VALUES (:id)");
    conn.exec_first::<u32, _, _>(query, params! {"id" => pat}).expect("error inserting pat");
}

pub fn write_profile_id(conn: &mut PooledConn, id: i32) {
    let query = format!("INSERT INTO profile (id) VALUES (:id)");
    conn.exec_first::<i32, _, _>(query, params! {id}).expect("error inserting pat");
}

pub fn write_person<T>(conn: &mut PooledConn, k: T) {}
