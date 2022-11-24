use mysql::*;
use mysql::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:root@localhost:8889/test_db";
    let pool = Pool::new(url)?;
    let mut connection = pool.get_conn()?;
    // connection.query_drop("INSERT INTO `user` (`username`, `password`, `created`) VALUES ('rust@user', '12345', CURRENT_TIMESTAMP)")?;
    let users: Vec<(String, String, String, String)> = connection.query("SELECT * FROM user")?;
    for user in users {
        println!("{}\t{}\t{}\t{}", user.0, user.1, user.2, user.3);
    }
    Ok(())
}
