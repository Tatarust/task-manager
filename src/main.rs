pub mod backend;
pub mod frontend;

use backend::Backend;
use frontend::App;
use tokio::runtime::Runtime;

fn main() {

    let runtime: Runtime = Runtime::new().expect("Cannot create tokio::runtime::Runtime");

    let backend: Backend = Backend::new(runtime.handle().clone(), "sqlite://task_manager.db");

    let app: App = App::new(backend);

    let options: eframe::NativeOptions = eframe::NativeOptions::default();

    eframe::run_native("Task manager", options, Box::new(|_cc| Box::new(app))).expect("Error: run_native");
}