pub mod backend;
pub mod frontend;

use backend::Backend;
use frontend::App;
use tokio::runtime::Runtime;

fn main() {

    let runtime = Runtime::new().expect("Cannot create tokio::runtime::Runtime");

    let backend = Backend::new(runtime.handle().clone(), std::env::var("DATABASE_URL").unwrap().as_str());

    let app = App::new(backend);

    let options = eframe::NativeOptions::default();

    eframe::run_native("Task manager", options, Box::new(|_cc| Box::new(app)));
}