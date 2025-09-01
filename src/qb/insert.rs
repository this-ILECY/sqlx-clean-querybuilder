pub(crate) struct InsertQuery {
    table: String,
    columns: Vec<String>,
    values: Vec<String>,
    returning: Vec<String>,
}

impl InsertQuery {
    pub(crate) fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            columns: Vec::new(),
            values: Vec::new(),
            returning: Vec::new(),
        }
    }

    /// INSERT INTO table (col1, col2, ...)
    pub fn columns(mut self, cols: &[&str]) -> Self {
        self.columns = cols.iter().map(|s| s.to_string()).collect();
        self
    }

    /// VALUES (val1, val2, ...)
    pub fn values(mut self, vals: &[&str]) -> Self {
        self.values = vals.iter().map(|s| s.to_string()).collect();
        self
    }

    /// RETURNING col1, col2 (Postgres-specific)
    pub fn returning(mut self, cols: &[&str]) -> Self {
        self.returning = cols.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Build SQL string
    pub fn build(self) -> String {
        let mut query = format!("INSERT INTO {}", self.table);

        if !self.columns.is_empty() {
            query.push_str(&format!(" ({})", self.columns.join(", ")));
        }

        if !self.values.is_empty() {
            query.push_str(&format!(" VALUES ({})", self.values.join(", ")));
        }

        if !self.returning.is_empty() {
            query.push_str(&format!(" RETURNING {}", self.returning.join(", ")));
        }

        query
    }
}