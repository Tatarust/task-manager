use crate::backend::{Backend, Task};
use egui::{CentralPanel, Context, FontFamily::Proportional, FontId, ScrollArea, TextStyle, Ui};
use eframe::Frame;

pub struct App {
    backend: Backend,
    tasks: Vec<Task>,
    pin_input: String,
    change_input: String,
}

fn configure_fonts(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles.insert(TextStyle::Heading, FontId::new(32.0, Proportional));
    style.text_styles.insert(TextStyle::Body, FontId::new(20.0, Proportional));
    style.text_styles.insert(TextStyle::Button, FontId::new(24.0, Proportional));

    ctx.set_style(style);
}

impl App {
    pub fn new(backend: Backend) -> Self {
        let tasks: Vec<Task> = backend.load_tasks();

        Self {
            backend,
            tasks,
            pin_input: String::new(),
            change_input: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            configure_fonts(ctx);
            let backend: &Backend = &self.backend;
            let tasks: &mut Vec<Task> = &mut self.tasks;
            let pin_input: &mut String = &mut self.pin_input;
            let change_input: &mut String = &mut self.change_input;
            let mut remove_id: Option<i32> = None;

            ui.vertical_centered(|ui: &mut Ui| {
                ui.heading("Pin task");
                ui.text_edit_singleline(pin_input);

                if ui.button("📌").clicked() {
                    if !pin_input.trim().is_empty() {
                        backend.add_task(pin_input.clone());
                        pin_input.clear();
                    } 
                }
            });

            if let Some(id) = remove_id {
                tasks.retain(|task: &Task| task.id() != id);
            }

            ui.separator();

            ui.heading("Tasks");
            ScrollArea::vertical().show(ui, |ui| {
                for task in tasks.iter() {
                    ui.collapsing(format!("{}: {}", task.id(), task.description()), |ui: &mut Ui |{
                        ui.menu_button("✏", |ui: &mut Ui| {
                            ui.text_edit_singleline(change_input);
                            
                            if ui.button("Change").clicked() {
                                backend.update_task(task.id().clone(), change_input.clone());
                                change_input.clear();
                                ui.close_menu();
                            }
                        });
    
                        if ui.button("🗙").clicked() {
                            remove_id = Some(task.id());
                            backend.remove_task(task.id());
                        }
                    });
                }
            });
            

            *tasks = backend.load_tasks();
            ctx.request_repaint();
        });
    }
}