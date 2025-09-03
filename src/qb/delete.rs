// qb/delete.rs
pub struct DeleteQuery {
    table: String,
    filter: Vec<String>,
    returning: Vec<String>,
}

impl DeleteQuery {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            filter: Vec::new(),
            returning: Vec::new(),
        }
    }

    /// WHERE condition
    pub fn filter(mut self, condition: &str) -> Self {
        self.filter.push(condition.to_string());
        self
    }

    /// RETURNING col1, col2 (Postgres-specific)
    pub fn returning(mut self, cols: &[&str]) -> Self {
        self.returning = cols.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(self) -> String {
        let mut query = format!("DELETE FROM {}", self.table);

        if !self.filter.is_empty() {
            query.push_str(&format!(" WHERE {}", self.filter.join(" AND ")));
        }

        if !self.returning.is_empty() {
            query.push_str(&format!(" RETURNING {}", self.returning.join(", ")));
        }

        query
    }
}
