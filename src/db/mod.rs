use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::db::models::{User, NewUser};
use crate::config;
use diesel::result::Error;
use diesel::r2d2::ConnectionManager;
use r2d2::{PooledConnection, Pool};

pub mod schema;
pub mod models;


lazy_static! {
    pub static ref DB_POOL :Pool<ConnectionManager<MysqlConnection>> = {
    config::ok();
    let url = viperus::get::<String>("db.url").unwrap();
    let manager = ConnectionManager::<MysqlConnection>::new(&url);
//    println!("Call pool...");
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
    };
}

pub fn establish_connection() -> Result<MysqlConnection, diesel::ConnectionError> {
    config::ok();
    let url = viperus::get::<String>("db.url").unwrap();
    MysqlConnection::establish(&url)
}

pub fn get_connection() -> PooledConnection<ConnectionManager<MysqlConnection>> {
    let pool = DB_POOL.clone();
    pool.get().unwrap()

}

pub fn create_user(name: &str, email: &str, enabled: bool) -> Option<User> {
    use schema::users::dsl::{id, users};
//    let conn = establish_connection().unwrap();
    let conn = get_connection();
    let new_user = NewUser { name, email, enabled };
    if let Ok(_) = diesel::insert_into(users).values(&new_user)
        .execute(&conn) {
        users.order(id.desc()).first(&conn).ok()
    } else { None }
}

// or like this, for more methods see
//  https://github.com/diesel-rs/diesel/blob/master/examples/postgres/all_about_inserts/src/lib.rs

//pub fn create_user_2(conn: &MysqlConnection, the_name: &str, the_email: &str, the_enabled: bool) -> Option<User> {
//    use schema::users::dsl::{users, id, name, email, enabled};
//    if let Ok(_) = diesel::insert_into(users)
//        .values((name.eq(the_name), email.eq(the_email), enabled.eq(the_enabled)))
//        .execute(conn) {
//        users.order(id.desc()).first(conn).ok()
//    } else { None }
//}

pub fn get_user(the_id: i32) -> Result<User, Error> {
    use schema::users::dsl::{id, users};
//    let conn = establish_connection().unwrap();
    let conn = get_connection();
    users.filter(id.eq(the_id)).first(&conn)
}

pub fn del_user(the_id: i32) -> Result<usize, Error> {
    use schema::users::dsl::{id, users};
//    let conn = establish_connection().unwrap();
    let conn = get_connection();
    diesel::delete(users.filter(id.eq(the_id)))
        .execute(&conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        assert_eq!(establish_connection().is_ok(), true)
    }

    #[test]
    fn test_create_and_delete_user() {
        let u = create_user("Daniel", "qiuzhanghua@icloud.com", true);
        assert_eq!(u.is_some(), true);
        let id = u.unwrap().id;
//            println!("id = {:?}", id);
        let r = del_user(id);
        assert_eq!(r.is_ok(), true);
        assert_eq!(r.unwrap(), 1);
    }

    #[test]
    fn test_get_user() {
        let u = get_user(5);
        if u.is_ok() {
            assert_eq!(u.unwrap().id, 5);
        } else {
            // panic!("Not found!")
        }
    }
}