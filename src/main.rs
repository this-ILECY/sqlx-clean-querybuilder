use crate::qb::qbs::Query;
mod qb;

#[tokio::main]
async fn main() {
    let inner_select = Query::new()
        .select("id, name")
        .table("users")
        .filter("id = 12")
        .filter("name LIKE '%mo%'")
        .build();

    let query = Query::new()
        .select("users.id, products.name")
        .table("users")
        .join_inner("products", "users.id", "products.userId")
        .filter("users.id = 12")
        .build();

    println!("{}", query);
}