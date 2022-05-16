use sqlx::{error::Error, pool::Pool, Connection, Database};

#[derive(Clone)]
pub struct SqlEventStore<DB>
where
    DB: Database,
{
    pool: Pool<DB>,
}

impl<DB> SqlEventStore<DB>
where
    DB: Database,
{
    pub async fn connect(
        connect_options: <<DB as Database>::Connection as Connection>::Options,
    ) -> Result<Self, Error> {
        let x: Result<Pool<DB>, Error> = Pool::connect_with(connect_options).await;
        match x {
            Ok(pool) => Ok(Self { pool }),
            Err(e) => Err(e),
        }
    }
}
