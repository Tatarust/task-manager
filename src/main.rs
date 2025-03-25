use eframe::egui;
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;

struct Task {
    id: i32,
    task: String,
}

impl Task {
    fn new(id: i32, task: String) -> Self {
        Self {
            id,
            task,
        }
    }
}

struct App {
    pool: Option<Pool<Postgres>>,
    tasks: Vec<Task>,
    runtime: Runtime,
}

impl App {
    fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        let pool = runtime.block_on(Self::init_database());

        Self {
            pool: Some(pool),
            tasks: Vec::new(),
            runtime,
        }
    }

    async fn init_database() -> Pool<Postgres> {
        let database_url = "postgres://user:password@localhost/task_manager_database";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(database_url)
            .await
            .expect("Failed to connect to Database");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS tasks (id SERIAL PRIMARY KEY, task TEXT NOT NULL)"
        )
        .execute(&pool)
        .await
        .expect("Failed to create table");

        pool
    }

    async fn load_tasks(&mut self) {
        if let Some(pool) = &self.pool {
            let rows = sqlx::query!("SELECT task, id FROM tasks")
                .fetch_all(pool)
                .await
                .expect("Failed to fetch tasks");

            self.tasks = rows.into_iter().map(|row| Task::new(row.id, row.task)).collect();
        }
    }

    async fn remove_task(&mut self, task_id: i32) {
        if let Some(pool) = &self.pool {
            sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
                .execute(pool)
                .await
                .expect("Failed to delete task");

            self.load_tasks().await;
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Load tasks").clicked() {
                let pool = self.pool.clone();
                let ctx = ctx.clone();
                self.runtime.spawn(async move {
                    self.load_tasks();
                    ctx.request_repaint();
                });
            }

            let mut remove_id: Option<i32> = None;
            
            for task in &self.tasks {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", task.id, task.task));

                    if ui.button("Remove").clicked() {
                        remove_id = Some(task.id);
                    }
                });
            }

            if let Some(id) = remove_id {
                let pool = self.pool.clone();
                let ctx = ctx.clone();
                self.runtime.spawn(async move {
                    if 
                });
            }
        });
        
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native("Task manager", options, Box::new(|_cc| Box::new(App::new())));
}