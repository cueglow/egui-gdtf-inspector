use std::hash::Hash;

use eframe::egui::{self, Ui};
use gdtf_parser::{DataVersion, Gdtf};

pub fn metadata(ui: &mut egui::Ui, gdtf: &Gdtf) {
    let ft = &gdtf.fixture_type;

    ui.heading("Fixture");

    property_table(
        ui,
        &[
            ("Manufacturer", &ft.manufacturer),
            ("Name", &ft.name.0),
            ("Description", &ft.description),
            ("Short Name", &ft.short_name),
            ("Long Name", &ft.long_name),
        ],
    );

    ui.heading("GDTF");
    property_table(
        ui,
        &[
            ("GDTF Version", string(&gdtf.data_version)),
            (
                "Fixture Type ID ",
                match &ft.fixture_type_id.to_str().map_err(|e| e.to_string()) {
                    Err(e) => e,
                    Ok(s) => s,
                },
            ),
        ],
    );
}

pub fn property_table(
    ui: &mut Ui,
    input: &[(
        impl ToString + Hash,
        impl ToString + Hash,
    )],
) {
    egui::Grid::new(input)
        .striped(true)
        .max_col_width(500.)
        .show(ui, |ui| {
            for prop in input {
                ui.label(prop.0.to_string());
                ui.label(prop.1.to_string());
                ui.end_row();
            }
        });
}

pub fn string(input: &DataVersion) -> &str {
    match input {
        DataVersion::Version1_0 => "1.0",
        DataVersion::Version1_1 => "1.1",
        DataVersion::Unknown(str) => str,
    }
}
