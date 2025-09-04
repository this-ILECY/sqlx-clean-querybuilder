pub struct SelectQuery {
    distinct: bool,
    columns: Vec<String>,
    table: Option<(String, Option<String>)>, // (table, alias)
    joins: Vec<String>,
    filter: Vec<String>,
    group_by: Vec<String>,
    having: Vec<String>,
    order_by: Vec<(String, Order)>,
    limit: Option<u32>,
    offset: Option<u32>,
}

#[derive(Clone, Copy)]
pub enum Order {
    Asc,
    Desc,
}

impl SelectQuery {
    pub fn new() -> Self {
        Self {
            distinct: false,
            columns: Vec::new(),
            table: None,
            joins: Vec::new(),
            filter: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// SELECT columns
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// SELECT DISTINCT
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// FROM table [AS alias]
    pub fn table(mut self, table: &str, alias: Option<&str>) -> Self {
        self.table = Some((table.to_string(), alias.map(|s| s.to_string())));
        self
    }

    /// WHERE condition
    pub fn filter(mut self, condition: &str) -> Self {
        self.filter.push(condition.to_string());
        self
    }

    fn join(mut self, join_type: &str, table: &str, alias: Option<&str>, left: &str, right: &str) -> Self {
        let alias_str = alias.map(|a| format!(" AS {}", a)).unwrap_or_default();
        let stmt = format!("{} JOIN {}{} ON {} = {}", join_type, table, alias_str, left, right);
        self.joins.push(stmt);
        self
    }

    pub fn join_inner(self, table: &str, alias: Option<&str>, left: &str, right: &str) -> Self {
        self.join("INNER", table, alias, left, right)
    }

    pub fn join_left_outer(self, table: &str, alias: Option<&str>, left: &str, right: &str) -> Self {
        self.join("LEFT OUTER", table, alias, left, right)
    }

    pub fn join_right_outer(self, table: &str, alias: Option<&str>, left: &str, right: &str) -> Self {
        self.join("RIGHT OUTER", table, alias, left, right)
    }

    pub fn join_full_outer(self, table: &str, alias: Option<&str>, left: &str, right: &str) -> Self {
        self.join("FULL OUTER", table, alias, left, right)
    }

    pub fn group_by(mut self, column: &str) -> Self {
        self.group_by.push(column.to_string());
        self
    }

    pub fn having(mut self, condition: &str) -> Self {
        self.having.push(condition.to_string());
        self
    }

    pub fn order_by(mut self, column: &str, order: Order) -> Self {
        self.order_by.push((column.to_string(), order));
        self
    }

    pub fn limit(mut self, count: u32) -> Self {
        self.limit = Some(count);
        self
    }

    pub fn offset(mut self, count: u32) -> Self {
        self.offset = Some(count);
        self
    }

    pub fn build(self) -> String {
        let mut parts = Vec::new();

        // SELECT
        let select_clause = if self.columns.is_empty() {
            "SELECT *".to_string()
        } else {
            let mut clause = "SELECT".to_string();
            if self.distinct {
                clause.push_str(" DISTINCT");
            }
            clause.push(' ');
            clause.push_str(&self.columns.join(", "));
            clause
        };
        parts.push(select_clause);

        // FROM
        if let Some((table, alias)) = self.table {
            let mut clause = format!("FROM \"{}\"", table);
            if let Some(a) = alias {
                clause.push_str(&format!(" AS {}", a));
            }
            parts.push(clause);
        }

        // JOIN
        if !self.joins.is_empty() {
            parts.push(self.joins.join(" "));
        }

        // WHERE
        if !self.filter.is_empty() {
            parts.push(format!("WHERE {}", self.filter.join(" AND ")));
        }

        // GROUP BY
        if !self.group_by.is_empty() {
            parts.push(format!("GROUP BY {}", self.group_by.join(", ")));
        }

        // HAVING
        if !self.having.is_empty() {
            parts.push(format!("HAVING {}", self.having.join(" AND ")));
        }

        // ORDER BY
        if !self.order_by.is_empty() {
            let order_parts: Vec<String> = self
                .order_by
                .iter()
                .map(|(col, ord)| format!("{} {}", col, match ord {
                    Order::Asc => "ASC",
                    Order::Desc => "DESC",
                }))
                .collect();
            parts.push(format!("ORDER BY {}", order_parts.join(", ")));
        }

        // LIMIT
        if let Some(limit) = self.limit {
            parts.push(format!("LIMIT {}", limit));
        }

        // OFFSET
        if let Some(offset) = self.offset {
            parts.push(format!("OFFSET {}", offset));
        }

        parts.join(" ")
    }
}
