use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();
    ctx.register_csv("test", "reading/test.csv", CsvReadOptions::new())
        .await?;
    let df = ctx.sql("SELECT \"Variable_code\", \"Variable_name\", \"Variable_category\", \"Value\" FROM test WHERE \"Industry_aggregation_NZSIOC\" = 'Level 1' LIMIT 10").await?;
    df.show().await?;
    Ok(())
}
