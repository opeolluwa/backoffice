use sqlx::PgPool;

pub trait Repository {
    fn init(pool: &PgPool) -> Self;
}
