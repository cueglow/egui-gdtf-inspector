use std::path::PathBuf;

use eframe::{
    egui::{self, FontDefinitions, TextStyle},
    epi,
};
use gdtf_parser::Gdtf;
use rfd::FileDialog;

pub struct TemplateApp {
    gdtf_filename: PathBuf,
    gdtf: Option<Gdtf>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            gdtf_filename: PathBuf::new(),
            gdtf: None,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui GDTF Inspector"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Increase font sizes
        let mut fonts = FontDefinitions::default();
        for (key, (_family, size)) in fonts.family_and_size.iter_mut() {
            match key {
                TextStyle::Small => *size = 16.,
                TextStyle::Body => *size = 20.,
                TextStyle::Button => *size = 20.,
                TextStyle::Heading => *size = 30.,
                TextStyle::Monospace => *size = 20.,
            }
        }
        ctx.set_fonts(fonts);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self {
            gdtf_filename,
            gdtf,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                if ui.button("Open").clicked() {
                    println!("{}", "Open clicked");
                    let files = FileDialog::new().add_filter("GDTF", &["gdtf"]).pick_file();
                    println!("File Picker Output: {:#?}", files);
                    if let Some(filepath) = files {
                        *gdtf = Gdtf::try_from(filepath.as_path())
                            .or_else(|e| {
                                println!("{:#?}", e);
                                Err(e)
                            })
                            .ok();
                        *gdtf_filename = filepath;
                    };
                };

                ui.label("Current File: ");

                ui.label(match gdtf_filename.to_str() {
                    Some("") | None => "None",
                    Some(f) => f,
                });

                if ui.button("🔄").clicked() {
                    println!("{}", "Refresh clicked");
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| ui.label(format!("{:#?}", gdtf)))
            // The central panel the region left after adding TopPanel's and SidePanel's

            // ui.heading("eframe template");
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));
            // egui::warn_if_debug_build(ui);

            // ui.add(DragValue::new(value));

            // ui.label(format!("{:#?}", ctx.fonts().definitions().family_and_size));
            // // println!("{:#?}", ctx.fonts().definitions().family_and_size);
        });
    }
}
