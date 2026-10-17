use std::{env, fs, path::PathBuf};
use tokio_postgres::{Client, NoTls, Row};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Postgres{
    pub connection: Client,
    migration_directory: PathBuf,
}

struct Migration {
    name: String,
    sql: String,
}

impl Postgres{

   pub async fn new() -> Result<Self> {
       let password = env::var("PASSWORD")?;
       let host = env::var("HOST")?;
       let config = format!("host={host} user=postgres password={password} dbname=news");

       let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;

       tokio::spawn(async move {
           if let Err(e) = connection.await {
               eprintln!("connection error: {e}");
           }
       });

       let db = Self {
           connection: client,
           migration_directory: PathBuf::from("./src/migrations"),
       };
       db.init().await?;

       db.migrate().await?;

       Ok(db)
   }

    async fn migrate(&self) -> Result<()> {
        // read_dir has no guaranteed order, so sort to run migrations in sequence
        let mut migration_files = fs::read_dir(&self.migration_directory)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<std::io::Result<Vec<_>>>()?;
        migration_files.sort();

        for migration_file in migration_files {


            let migration = Migration{
                name: migration_file.clone().file_name().unwrap().to_str().unwrap().to_string(),
                sql: fs::read_to_string(migration_file)?
            };

            if self.find_migration(&migration.name).await.is_none()
            {
               // batch_execute allows multiple statements per file, unlike a prepared query
               self.connection.batch_execute(&migration.sql).await?;
               self.connection
                   .execute("INSERT INTO migrations (name) VALUES ($1)", &[&migration.name])
                   .await?;
            }
        }

        Ok(())
    }

    async fn find_migration(&self, name: &String) -> Option<Row> {
        self.connection.query_one("SELECT name FROM migrations WHERE name = $1", &[&name]).await.ok()
    }

    pub async fn init(&self) -> Result<()> {
        self.connection
            .batch_execute("CREATE TABLE IF NOT EXISTS migrations (name TEXT PRIMARY KEY)")
            .await?;

        Ok(())
    }
}