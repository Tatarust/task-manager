use sqlx::{Postgres, PgPool, Pool};
use tokio::runtime::Handle;

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
    pool: PgPool,
}

impl Backend {
    pub fn new(runtime: Handle, database_url: &str) -> Self {
        let pool: Pool<Postgres> = runtime.block_on(async {
            PgPool::connect(database_url).await.expect("Error: error connection to pool")
        });

        let pool: &Pool<Postgres> = &pool;

        runtime.block_on(async move {
            sqlx::query!(
                r#"
                CREATE TABLE IF NOT EXISTS tasks (
                    id SERIAL PRIMARY KEY,
                    task TEXT NOT NULL
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
        let pool: Pool<Postgres> = self.pool.clone();

        self.runtime.block_on(async move {
            let rows = sqlx::query!("SELECT task, id FROM tasks")
                .fetch_all(&pool)
                .await
                .expect("Failed to load tasks");
            
            rows.into_iter().map(|row| Task::new(row.id, row.task)).collect()
        })
    }

    pub fn add_task(&self, description: String) {
        let pool: Pool<Postgres> = self.pool.clone();

        self.runtime.block_on(async move {
            sqlx::query!("INSERT INTO tasks (task) VALUES ($1)", description)
                .execute(&pool)
                .await
                .expect("Error add task");
        });
    }

    pub fn remove_task(&self, id: i32) {
        let pool: Pool<Postgres> = self.pool.clone();

        self.runtime.block_on(async move {
            sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
                .execute(&pool)
                .await
                .expect("Error delete task")
        });
    }

    pub fn update_task(&self, id: i32, description: String) {
        let pool: Pool<Postgres> = self.pool.clone();

        self.runtime.block_on(async move {
            sqlx::query!("UPDATE tasks SET task = $1 WHERE id = $2", description, id)
                .execute(&pool)
                .await
                .expect("Error update task");
        });
    }
}