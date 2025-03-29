use crate::backend::{Backend, Task};
use egui::{CentralPanel, Context, FontFamily::Proportional, FontId, ScrollArea, TextStyle, Ui};
use eframe::Frame;

pub struct App {
    backend: Backend,
    tasks: Vec<Task>,
    pin_input: String,
    change_input: String,
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
            let mut remove_id: Option<i32> = None;

            configure_fonts(ctx);

            ui.vertical_centered(|ui: &mut Ui| {
                ui.heading("Pin task");
                ui.text_edit_singleline(&mut self.pin_input);

                if ui.button("📌").clicked() {
                    if !self.pin_input.trim().is_empty() {
                        self.backend.add_task(self.pin_input.clone());
                        self.pin_input.clear();
                    } 
                }
            });

            if let Some(id) = remove_id {
                self.tasks.retain(|task: &Task| task.id() != id);
            }

            ui.separator();

            ui.heading("Tasks");
            
            ScrollArea::vertical().show(ui, |ui| {
                for (index, task) in self.tasks.iter().enumerate() {
                    ui.collapsing(format!("{}: {}", index + 1, task.description()), |ui: &mut Ui |{
                        ui.menu_button("✏", |ui: &mut Ui| {
                            ui.text_edit_singleline(&mut self.change_input);
                            
                            if ui.button("Change").clicked() {
                                self.backend.update_task(task.id().clone(), self.change_input.clone());
                                self.change_input.clear();
                                ui.close_menu();
                            }
                        });
    
                        if ui.button("🗙").clicked() {
                            remove_id = Some(task.id());
                            self.backend.remove_task(task.id());
                        }
                    });
                }
            });
            
            self.tasks = self.backend.load_tasks();
            ctx.request_repaint();
        });
    }
}

fn configure_fonts(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles.insert(TextStyle::Heading, FontId::new(32.0, Proportional));
    style.text_styles.insert(TextStyle::Body, FontId::new(20.0, Proportional));
    style.text_styles.insert(TextStyle::Button, FontId::new(24.0, Proportional));

    ctx.set_style(style);
}