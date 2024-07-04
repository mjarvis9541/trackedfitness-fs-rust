use sqlx::{Postgres, QueryBuilder};

pub trait Filter<'a> {
    fn filter(&mut self, field: &str, expr: &str, value: &'a str) -> &mut Self;
    fn order(&mut self, default: &str, order: &str) -> &mut Self;
    fn paginate(&mut self, size: i64, page: i64) -> &mut Self;
}

impl<'a> Filter<'a> for QueryBuilder<'a, Postgres> {
    fn filter(&mut self, field: &str, expr: &str, value: &'a str) -> &mut Self {
        if !value.is_empty() {
            self.push(" AND ");
            self.push(field);
            match expr {
                "ilike" => {
                    self.push(" ILIKE ");
                    self.push_bind(format!("%{}%", value));
                }
                _ => {
                    self.push(" = ");
                    self.push_bind(value);
                }
            }
        }
        self
    }

    fn order(&mut self, default: &str, order: &str) -> &mut Self {
        if !order.is_empty() {
            self.push(" ORDER BY ");
            if !order.contains('-') {
                self.push(format!("{} ASC NULLS LAST", order));
            } else {
                let order = &order[1..];
                self.push(format!("{} DESC NULLS LAST", order));
            }
        } else {
            self.push(" ORDER BY ");
            self.push(default);
        }
        self
    }

    fn paginate(&mut self, size: i64, page: i64) -> &mut Self {
        let size = size.min(100);

        self.push(" LIMIT ");
        self.push_bind(size);

        if page > 1 {
            let offset = (page - 1) * size;
            self.push(" OFFSET ");
            self.push_bind(offset);
        }

        self
    }
}
