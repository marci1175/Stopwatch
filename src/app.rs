use egui::RichText;
use stopwatch::{Stopwatch};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    #[serde(skip)]
    sw : Stopwatch,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    lap: Vec<String>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            sw: Stopwatch::default(),
            value: 2.7,
            lap: Vec::new(),
        }
    }
}
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
impl eframe::App for TemplateApp {
    
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //_frame.close();
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui|{
            if ui.button("Start").clicked(){
                if Stopwatch::is_running(&mut self.sw) {
                    //fosd le
                }
                else {
                    Stopwatch::start(&mut self.sw);
                }
            }
            if ui.button("Stop").clicked(){
                Stopwatch::stop(&mut self.sw);
                
            }
            if ui.button("Reset").clicked(){
                self.sw = Stopwatch::new();
                self.lap.clear();
            }
            if ui.button("Lap").clicked(){
                let swelap = Stopwatch::elapsed(&mut self.sw);
                let swetext = format!("{:?}", swelap);
                self.lap.push(swetext);
            }});
            
        });
        if Stopwatch::is_running(&mut self.sw) {
            ctx.request_repaint();
        }
        let swelapsed = Stopwatch::elapsed(&mut self.sw);
        let swetext = format!("{:?}", swelapsed);
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages     
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new(swetext).size(60.0));
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                for i in 0..self.lap.len() {
                    ui.group(|ui| {
                        ui.label(self.lap[i].clone())
                    });
                ui.add_space(5.0);
            }
            })
        })
    });
        
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
