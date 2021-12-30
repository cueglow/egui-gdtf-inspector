mod metadata;
use metadata::metadata;

use std::path::PathBuf;

use eframe::{
    egui::{self, Color32, FontDefinitions, TextStyle},
    epi,
};
use gdtf_parser::{utils::errors::GdtfError, Gdtf};
use rfd::FileDialog;

#[derive(PartialEq)]
pub enum Section {
    METADATA,
    DEBUG,
}
pub struct TemplateApp {
    gdtf_filename: PathBuf,
    gdtf: Option<Result<Gdtf, GdtfError>>,
    selected_section: Section,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            gdtf_filename: PathBuf::new(),
            gdtf: None,
            selected_section: Section::METADATA,
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
        _frame: &epi::Frame,
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

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        let Self {
            gdtf_filename,
            gdtf,
            selected_section,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                if ui.button("Open").clicked() {
                    println!("{}", "Open clicked");
                    let files = FileDialog::new().add_filter("GDTF", &["gdtf"]).pick_file();
                    println!("File Picker Output: {:#?}", files);
                    if let Some(filepath) = files {
                        *gdtf = Some(Gdtf::try_from(filepath.as_path()).or_else(|e| {
                            println!("{:#?}", e);
                            Err(e)
                        }));
                        *gdtf_filename = filepath;
                    };
                };

                match gdtf_filename.to_str() {
                    Some("") | None => ui.label(""),
                    Some(f) => ui.label(format!("Current File: {}", f)),
                };
            })
        });

        egui::SidePanel::left("left_main_side_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    // only show if GDTF is opened and is not an error
                    if gdtf.as_ref().filter(|r| r.is_ok()).is_some() {
                        ui.selectable_value(selected_section, Section::METADATA, "Metadata");
                        ui.selectable_value(selected_section, Section::DEBUG, "Debug");
                    };

                    egui::warn_if_debug_build(ui);
                })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match gdtf {
                None => {
                    ui.label("Please open a GDTF File");
                }
                Some(Err(e)) => {
                    ui.colored_label(Color32::RED, "Error during Parsing");
                    ui.label(format!("{:#?}", e));
                }
                Some(Ok(gdtf)) => {
                    match selected_section {
                        Section::DEBUG => {
                            egui::ScrollArea::vertical()
                            .auto_shrink([false; 2])
                            .show(ui, |ui| {
                                ui.label(format!("{:#?}", gdtf));
                            })
                        } // TODO Performance bad for large files
                        Section::METADATA => metadata(ui, gdtf)
                    }
                }
            }
        });
    }
}
