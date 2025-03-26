use crate::backend::{Backend, Task};
use egui::{CentralPanel, Context, Ui};
use eframe::Frame;

pub struct App {
    backend: Backend,
    tasks: Vec<Task>,
    text_input: String,
}

impl App {
    pub fn new(backend: Backend) -> Self {
        let tasks: Vec<Task> = backend.load_tasks();
        
        Self {
            backend,
            tasks,
            text_input: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            let backend: &Backend = &self.backend;
            let tasks: &mut Vec<Task> = &mut self.tasks;
            let text_input: &mut String = &mut self.text_input;
            let mut id_to_remove: Option<i32> = None;

            ui.heading("Tasks");
            for task in tasks.iter() {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label(format!("{}: {}", task.id(), task.description()));

                    if ui.button("Remove").clicked() {
                        id_to_remove = Some(task.id());
                        backend.remove_task(task.id());
                    }
                });
            }

            if let Some(id) = id_to_remove {
                tasks.retain(|task: &Task| task.id() != id);
            }

            ui.separator();
            ui.label("Add");
            ui.horizontal(|ui: &mut Ui| {
                ui.text_edit_singleline::<String>(text_input);

                if ui.button("Add").clicked() {
                    if !text_input.trim().is_empty() {
                        backend.add_task(text_input.clone());
                        *tasks = backend.load_tasks();
                        text_input.clear();
                        ctx.request_repaint();
                    } 
                }
            });
            ctx.request_repaint();
        });
    }
}