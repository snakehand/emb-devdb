#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqliteConnection;
    use sqlx::Connection;
    use sqlx::Executor;

    async fn coneection_test() -> Result<(), sqlx::Error> {
        let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

        let crstm = r"CREATE TABLE device (
            device_id INTEGER PRIMARY KEY AUTOINCREMENT,
            mac VARCHAR(32)
        );";

        let _ = conn.execute(crstm).await?;

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn it_works() {
        let ctest = coneection_test().await;
        println!("{:?}", ctest);
        assert!(ctest.is_ok());
    }
}
