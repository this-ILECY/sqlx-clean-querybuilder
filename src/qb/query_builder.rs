use crate::qb::{delete::DeleteQuery, insert::InsertQuery, select::SelectQuery, update::UpdateQuery};

pub struct PostgreSqlQueryBuilder;

impl PostgreSqlQueryBuilder {
    pub fn select() -> SelectQuery { SelectQuery::new() }
    pub fn insert(table: &str) -> InsertQuery { InsertQuery::new(table) }
    pub fn update(table: &str) -> UpdateQuery { UpdateQuery::new(table) }
    pub fn delete(table: &str) -> DeleteQuery { DeleteQuery::new(table) }
}
