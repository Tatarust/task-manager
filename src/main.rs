use eframe::egui;
use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;

struct Task {
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
}

// struct App {
//     pool: Option<Pool<Postgres>>,
//     tasks: Arc<Mutex<Vec<Task>>>,
//     text_input: String,
//     runtime: Runtime,
// }

// impl App {
//     fn new() -> Self {
//         let runtime = Runtime::new().unwrap();
//         let pool = runtime.block_on(async {
//             Self::init_database().await
//         });

//         Self {
//             pool: Some(pool),
//             tasks: Arc::new(Mutex::new(Vec::new())),
//             text_input: String::new(),
//             runtime,
//         }
//     }

//     async fn init_database() -> Pool<Postgres> {
//         // let database_url = "postgres://user:password@localhost/task_manager_database";
//         let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL DOESNT EXPORTED");
//         println!("{}", database_url);
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .connect(&database_url)
//             .await
//             .expect("Failed to connect to Database");
//         match sqlx::query_scalar!("SELECT 1").fetch_one(&pool).await {
//             Ok(_) => println!("Testing SQL-query works"),
//             Err(err) => println!("Error: {}", err),
//         }
//         // sqlx::query!("CREATE TABLE IF NOT EXISTS tasks (id SERIAL PRIMARY KEY, task TEXT NOT NULL)")
//         //     .execute(&pool)
//         //     .await
//         //     .expect("Failed to create table");
//         println!("Database init works");
//         pool
//     }

//     async fn load_tasks(&mut self, ctx: egui::Context) {
//             let pool = self.pool.clone();
//             let tasks = Arc::clone(&self.tasks);
//             let ctx = ctx.clone();

//             self.runtime.spawn(async move {
//                 if let Some(pool) = pool {
//                     println!("loading works");
//                     let rows = sqlx::query!("SELECT task, id FROM tasks")
//                     .fetch_all(&pool)
//                     .await
//                     .expect("Failed to fetch tasks");
//                     let mut tasks_lock = tasks.lock().unwrap();
//                     *tasks_lock = rows.into_iter().map(|row| Task::new(row.id, row.task)).collect();
//                     println!("loading works");
//                 } else {
//                     println!("load_tasks():no pool");
//                 }
//                 ctx.request_repaint();
//             });
//     }

//     async fn remove_task(&mut self, ctx: egui::Context, task_id: i32) {
//         let tasks = Arc::clone(&self.tasks);
//         let pool = self.pool.clone();
//         let ctx = ctx.clone();
 
//         self.runtime.spawn( async move {
//             let ctx = ctx.clone();
//             if let Some(pool) = pool { 
//                 sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
//                     .execute(&pool)
//                     .await
//                     .expect("Failed to remove task");
//                 let mut tasks_lock = tasks.lock().unwrap();
//                 tasks_lock.retain(|task| task.id != task_id);
//                 println!("removing works");
//         } else {
//             println!("remove_task(): no pool")
//         }
//             ctx.request_repaint();
//         });
//     }

//     async fn add_task(&mut self, ctx: egui::Context, input: String) {
//         if input.trim().is_empty() {
//             println!("add_task(): empty task");
//             return;
//         }
//         println!("Adding input: {}", input);

//         let pool = self.pool.clone();
//         let tasks = Arc::clone(&self.tasks);
//         let ctx = ctx.clone();

//         self.runtime.spawn(async move {
//             if let Some(pool) = pool {
//                 let row = sqlx::query!("INSERT INTO tasks (task) VALUES ($1) RETURNING id", input)
//                     .fetch_one(&pool)
//                     .await
//                     .expect("Failed to add task");
//                 let mut tasks_lock = tasks.lock().unwrap();
//                 tasks_lock.push(Task::new(row.id, input));
//                 println!("adding works");
//             } else {
//                 println!("add_task(): no pool")
//             }
//             ctx.request_repaint();
//         });
//     }
// }

// impl eframe::App for App {
//     fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//         if self.pool.is_none() {
//             println!("pool is empty");
//         }
//         egui::CentralPanel::default().show(ctx, |ui| {
//             if ui.button("Load tasks").clicked() {
//                 println!("load tasks pressed");
//                 self.load_tasks(ctx.clone());
//             }

//             ui.horizontal(|ui| {
//                 ui.text_edit_singleline(&mut self.text_input);

//                 if ui.button("Add").clicked() {
//                     let input = self.text_input.trim().to_string();
//                     if !input.is_empty() {
//                         println!("add task works");
//                         self.add_task(ctx.clone(), input);
//                         self.text_input.clear();
//                     }
//                 }
//             });

//             let mut remove_id: Option<i32> = None;
//             let tasks = self.tasks.lock().unwrap();
           
//             for task in tasks.iter() {
//                 ui.horizontal(|ui| {
//                     ui.label(format!("{}: {}", task.id, task.task));

//                     if ui.button("Remove").clicked() {
//                         remove_id = Some(task.id);
//                     }
//                 });
//             }

//             drop(tasks);

//             if let Some(id) = remove_id {
//                 self.remove_task(ctx.clone(), id);
//                 println!("remove task works");
//             }
//         });
        
//     }
// }

struct Backend {
    runtime: tokio::runtime::Handle,
    pool: sqlx::PgPool,
}

impl Backend {
    fn new(runtime: tokio::runtime::Handle, database_url: &str) -> Self {
        let pool = runtime.block_on(async {
            sqlx::PgPool::connect(database_url).await.expect("Cannot connect")
        });
        Self { runtime, pool }
    }
    
    fn load_tasks(&self) -> Vec<Task> {
        let pool = self.pool.clone();
        self.runtime.block_on(async move {
            let rows = sqlx::query!("SELECT task, id FROM tasks")
                .fetch_all(&self.pool)
                .await
                .expect("Failed to load tasks");
            rows.into_iter().map(|row| Task::new(row.id, row.task)).collect()
        })
    }
    fn add_task(&self, description: String) {
        let pool = self.pool.clone();
        self.runtime.spawn(async move {
            sqlx::query!("INSERT INTO tasks (task) VALUES ($1)", description)
                .execute(&pool)
                .await
                .ok();
        });
    }
    fn remove_task(&self, id: i32) {
        let pool = self.pool.clone();
        self.runtime.spawn(async move {
            if let Err(e) = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
                .execute(&pool)
                .await
            {
                eprintln!("Error removing: {:?}", e);
            }
        });
    }
}

struct App {
    backend: Backend,
    tasks: Vec<Task>,
    text_input: String,
}

impl App {
    fn new(backend: Backend) -> Self {
        let tasks = backend.load_tasks();
        Self {
            backend,
            tasks,
            text_input: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasks");

            let mut remove_id = None;

            for task in &self.tasks {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", task.id, task.description));
                    if ui.button("Remove").clicked() {
                        remove_id = Some(task.id);
                        self.backend.remove_task(task.id);
                    }
                });
            }

            if let Some(id) = remove_id {
                self.tasks.retain(|task| task.id != id);
            }

            ui.separator();
            ui.label("Add");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.text_input);
                if ui.button("Add").clicked() {
                    if !self.text_input.trim().is_empty() {
                        self.backend.add_task(self.text_input.clone());
                        self.tasks = self.backend.load_tasks();
                        self.text_input.clear();
                    } //
                }
            });
            ctx.request_repaint();
        });
    }

}

fn main() {
    //     Some(pool) => {
    //         println!("Connect to database");
    //         pool
    //     }
    //     None => {
    //         eprintln!("Error: cannot connect to database");
    //         return;
    //     }
    // };

    let runtime = Runtime::new().expect("Cannot create tokio::runtime::Runtime");

    let backend = Backend::new(runtime.handle().clone(), std::env::var("DATABASE_URL").unwrap().as_str());

    let app = App::new(backend);

    let options = eframe::NativeOptions::default();

    eframe::run_native("Task manager", options, Box::new(|_cc| Box::new(app)));
}