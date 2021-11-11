use clipboard_win::set_clipboard_string;
use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct PastaBox {
    new_pasta: String,
    pastabox: Vec<String>,
}

impl Default for PastaBox {
    fn default() -> Self {
        Self {
            new_pasta: String::new(),
            pastabox: Vec::new(),
        }
    }
}

impl epi::App for PastaBox {
    fn name(&self) -> &str {
        "PastaBox"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self {
            new_pasta,
            pastabox,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                let add_button = ui.button("Add");
                ui.text_edit_singleline(new_pasta);

                if add_button.clicked() {
                    pastabox.push(new_pasta.clone());
                    *new_pasta = String::new();
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            pastabox
                .clone()
                .iter()
                .enumerate()
                .for_each(|(index, pasta)| {
                    ui.horizontal(|ui| {
                        let copy_button = ui.button("COPY");
                        let del_button = ui.button("DEL");

                        ui.label(pasta);

                        if copy_button.clicked() {
                            set_clipboard_string(pasta).unwrap_or_default();
                        }

                        if del_button.clicked() {
                            pastabox.remove(index);
                        }
                    });
                });

            egui::warn_if_debug_build(ui);
        });
    }
}
