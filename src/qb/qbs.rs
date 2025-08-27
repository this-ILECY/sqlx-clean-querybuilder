pub struct Query {
    select: String,
    table: String,
    joins: Vec<String>,
    filter: String,
}

#[derive(Clone, Copy)]
pub enum Order {
    Asc,
    Desc,
}

impl Query {
    // Constructor for new empty Query
    pub fn new() -> Self {
        Self {
            select: "".to_string(),
            table: "".to_string(),
            joins: Vec::new(),
            filter: "".to_string(),
        }
    }

    // Setter for select with chaining
    pub fn select(mut self, column_name: &str) -> Self {
        self.select = format!("SELECT {}", column_name);
        self
    }

    // Setter for filter with chaining
    pub fn filter(mut self, statement: &str) -> Self {
        if self.filter.is_empty() {
            self.filter = " WHERE ".to_string();
        } else {
            self.filter.push_str(" AND ");
        }
        self.filter.push_str(statement);
        self
    }

    // Setter for table with chaining
    pub fn table(mut self, table_name: &str) -> Self {
        self.table = format!(" FROM {}", table_name);
        self
    }

    fn join(mut self, join_type: &str, table: &str, left_col: &str, right_col: &str) -> Self {
        let join_stmt = format!(
            " {} JOIN {} ON {} = {}",
            join_type, table, left_col, right_col
        );
        self.joins.push(join_stmt);
        self
    }

    pub fn join_inner(self, table: &str, left_col: &str, right_col: &str) -> Self {
        self.join("INNER", table, left_col, right_col)
    }

    pub fn join_left_outer(self, table: &str, left_col: &str, right_col: &str) -> Self {
        self.join("LEFT OUTER", table, left_col, right_col)
    }

    pub fn join_right_outer(self, table: &str, left_col: &str, right_col: &str) -> Self {
        self.join("RIGHT OUTER", table, left_col, right_col)
    }

    pub fn join_full_outer(self, table: &str, left_col: &str, right_col: &str) -> Self {
        self.join("FULL OUTER", table, left_col, right_col)
    }

    // Method to build final query string
    pub fn build(self) -> String {
        let joins_str = self.joins.join("");
        format!("{}{}{}{}", self.select, self.table,joins_str, self.filter)
    }
}