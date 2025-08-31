use crate::qb::{query_builder::PostgreSqlQueryBuilder};
mod qb;

#[tokio::main]
async fn main() {
    let query = PostgreSqlQueryBuilder::select()
        .columns(&["id", "name", "email"])
        .table("users", None)
        .distinct()
        .build();

    let insert_query = PostgreSqlQueryBuilder::insert("users")
        .columns(&["id", "name", "email"])
        .values(&["1", "'Alice'", "'alice@example.com'"])
        .returning(&["id"])
        .build();

    let update = PostgreSqlQueryBuilder::update("users")
        .set("name", "'Alice Updated'")
        .set("email", "'alice@newmail.com'")
        .filter("id = 1")
        .returning(&["id", "name"])
        .build();

    let delete = PostgreSqlQueryBuilder::delete("users")
        .filter("id = 1")
        .returning(&["id"])
        .build();

    println!("{} {} {} {}", query, insert_query, update, delete);
}
