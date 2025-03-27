use tokio::runtime::Handle;
use sqlx::{Row, Sqlite, SqlitePool, Pool};


pub struct Task {
    id: i32,
    description: String,
}

impl Task {
    fn new(id: i32, description: String) -> Self {
        Self {
            id,
            description,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

}



pub struct Backend {
    runtime: Handle,
    pool: SqlitePool,
}

impl Backend {
    pub fn new(runtime: Handle, database_url: &str) -> Self {
        let pool: Pool<Sqlite> = runtime.block_on(async {
            SqlitePool::connect(database_url).await.expect("Error: error connection to pool")
        });

        let pool: &Pool<Sqlite> = &pool;

        runtime.block_on(async move {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    task TEXT NOT NULL,
                    deleted INTEGER DEFAULT 0
                )
                "#
            )
            .execute(pool)
            .await
            .expect("Error: create table");            
        });

        Self { runtime, pool: pool.clone() }
    }
    
    pub fn load_tasks(&self) -> Vec<Task> {
        let pool: Pool<Sqlite> = self.pool.clone();

        self.runtime.block_on(async move {
            let rows = sqlx::query("SELECT id, task FROM tasks WHERE deleted = 0 ORDER BY id ASC")
                .fetch_all(&pool)
                .await
                .expect("Failed to load tasks");
            
            rows.into_iter().map(|row| Task::new(row.get(0), row.get(1))).collect()
        })
    }

    pub fn add_task(&self, description: String) {
        let pool: Pool<Sqlite> = self.pool.clone();

        self.runtime.block_on(async move {
            sqlx::query("INSERT INTO tasks (task) VALUES ($1)")
                .bind(description)
                .execute(&pool)
                .await
                .expect("Error add task");
        });
    }

    pub fn remove_task(&self, id: i32) {
        let pool: Pool<Sqlite> = self.pool.clone();

        self.runtime.block_on(async move {
            sqlx::query("UPDATE tasks SET deleted = 1 WHERE id = $1")
                .bind(id)
                .execute(&pool)
                .await
                .expect("Error remove task");
        });
    }

    pub fn update_task(&self, id: i32, description: String) {
        let pool: Pool<Sqlite> = self.pool.clone();

        self.runtime.block_on(async move {
            sqlx::query("UPDATE tasks SET task = $1 WHERE id = $2")
                .bind(description)
                .bind(id)
                .execute(&pool)
                .await
                .expect("Error update task");
        });
    }
}