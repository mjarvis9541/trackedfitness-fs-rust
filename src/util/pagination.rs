use sqlx::query::QueryAs;
use sqlx::{PgPool, QueryBuilder};

pub struct Paginator {
    pub size: i64,
    pub page: i64,
    pub order: String,
}

impl Paginator {
    pub fn new(size: i64, page: i64, order: &str) -> Self {
        Self {
            size,
            page,
            order: order.to_string(),
        }
    }

    pub fn apply_pagination<T>(&self, qb: &mut QueryBuilder<T>)
    where
        T: sqlx::database::HasArguments<'static> + sqlx::database::Database,
    {
        qb.push(" ORDER BY ").push(&self.order);
        qb.push(" LIMIT ").push_bind(self.size);
        qb.push(" OFFSET ").push_bind(self.page * self.size);
    }
}
