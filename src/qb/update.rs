// qb/update.rs
pub struct UpdateQuery {
    table: String,
    set_clauses: Vec<(String, String)>,
    filter: Vec<String>,
    returning: Vec<String>,
}

impl UpdateQuery {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            set_clauses: Vec::new(),
            filter: Vec::new(),
            returning: Vec::new(),
        }
    }

    /// SET column = value
    pub fn set(mut self, column: &str, value: &str) -> Self {
        self.set_clauses.push((column.to_string(), value.to_string()));
        self
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
        let mut query = format!("UPDATE {}", self.table);

        if !self.set_clauses.is_empty() {
            let sets: Vec<String> = self
                .set_clauses
                .iter()
                .map(|(col, val)| format!("{} = {}", col, val))
                .collect();
            query.push_str(&format!(" SET {}", sets.join(", ")));
        }

        if !self.filter.is_empty() {
            query.push_str(&format!(" WHERE {}", self.filter.join(" AND ")));
        }

        if !self.returning.is_empty() {
            query.push_str(&format!(" RETURNING {}", self.returning.join(", ")));
        }

        query
    }
}
