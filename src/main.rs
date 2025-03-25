use eframe::egui;
use std::sync::{Arc, Mutex};
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
    tasks: Arc<Mutex<Vec<Task>>>,
    text_input: String,
    runtime: Runtime,
}

impl App {
    fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        let pool = runtime.block_on(Self::init_database());

        Self {
            pool: Some(pool),
            tasks: Arc::new(Mutex::new(Vec::new())),
            text_input: String::new(),
            runtime,
        }
    }

    async fn init_database() -> Pool<Postgres> {
        let database_url = "postgres://user:password@localhost/task_manager_database";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(database_url)
            .await
            .expect("Failed to connect to Database");

        sqlx::query("CREATE TABLE IF NOT EXISTS tasks (id SERIAL PRIMARY KEY, task TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("Failed to create table");
        println!("Database init works");
        pool
    }

    async fn load_tasks(&mut self, ctx: egui::Context) {
            let pool = self.pool.clone();
            let tasks = Arc::clone(&self.tasks);
            let ctx = ctx.clone();

            self.runtime.spawn(async move {
                if let Some(pool) = pool {
                    let rows = sqlx::query!("SELECT task, id FROM tasks")
                    .fetch_all(&pool)
                    .await
                    .expect("Failed to fetch tasks");
                    let mut tasks_lock = tasks.lock().unwrap();
                    *tasks_lock = rows.into_iter().map(|row| Task::new(row.id, row.task)).collect();
                    println!("loading works");
                }
                ctx.request_repaint();
            });
    }

    async fn remove_task(&mut self, ctx: egui::Context, task_id: i32) {
        let tasks = Arc::clone(&self.tasks);
        let pool = self.pool.clone();
        let ctx = ctx.clone();
 
        self.runtime.spawn( async move {
            let ctx = ctx.clone();
            if let Some(pool) = pool { 
                sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
                    .execute(&pool)
                    .await
                    .expect("Failed to remove task");
                let mut tasks_lock = tasks.lock().unwrap();
                tasks_lock.retain(|task| task.id != task_id);
                println!("removing works");
        }
            ctx.request_repaint();
        });
    }

    async fn add_task(&mut self, ctx: egui::Context, input: String) {
        if input.trim().is_empty() {
            return;
        }

        let pool = self.pool.clone();
        let tasks = Arc::clone(&self.tasks);
        let ctx = ctx.clone();

        self.runtime.spawn(async move {
            if let Some(pool) = pool {
                let row = sqlx::query!("INSERT INTO tasks (task) VALUES ($1) RETURNING id", input)
                    .fetch_one(&pool)
                    .await
                    .expect("Failed to add task");
                let mut tasks_lock = tasks.lock().unwrap();
                tasks_lock.push(Task::new(row.id, input));
                println!("adding works");
            }
            ctx.request_repaint();
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Load tasks").clicked() {
                println!("load tasks pressed");
                self.load_tasks(ctx.clone());
            }

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.text_input);

                if ui.button("Add").clicked() {
                    let input = self.text_input.trim().to_string();
                    if !input.is_empty() {
                        println!("add task works");
                        self.add_task(ctx.clone(), input);
                        self.text_input.clear();
                    }
                }
            });

            let mut remove_id: Option<i32> = None;
            let tasks = self.tasks.lock().unwrap();
           
            for task in tasks.iter() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", task.id, task.task));

                    if ui.button("Remove").clicked() {
                        remove_id = Some(task.id);
                    }
                });
            }

            drop(tasks);

            if let Some(id) = remove_id {
                self.remove_task(ctx.clone(), id);
                println!("remove task works");
            }
        });
        
    }
}

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native("Task manager", options, Box::new(|_cc| Box::new(App::new())));
}