use eframe::egui;
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;

struct Task {
    id: i64,
    task: String,
}

struct App {
    pool: Option<Pool<Postgres>>,
    tasks: Vec<String>,
}

impl App {
    fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        let pool = runtime.block_on(Self::init_database());

        Self {
            pool: Some(pool),
            tasks: Vec::new(),
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
            let rows = sqlx::query!("SELECT task FROM tasks")
                .fetch_all(pool)
                .await
                .expect("Failed to fetch tasks");

            self.tasks = rows.into_iter().map(|row| row.task).collect();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Load tasks").clicked() {
                let runtime = Runtime::new().unwrap();
                runtime.block_on(self.load_tasks())
            }
            
            for task in &self.tasks {
                ui.label(task);
            }
        });
        
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native("Task manager", options, Box::new(|_cc| Box::new(App::new())));
}