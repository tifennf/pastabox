use std::{
    sync::{mpsc, Mutex},
    thread,
    time::Duration,
};

use clipboard_win::{get_clipboard_string, set_clipboard_string};
use eframe::{egui, epi};
use windows::{ApplicationModel::DataTransfer::Clipboard, Foundation::EventHandler};

type PastaList = Mutex<Vec<String>>;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct PastaBox {
    last_pasta: Mutex<String>,
    new_pasta: String,
    pastabox: PastaList,
}

impl Default for PastaBox {
    fn default() -> Self {
        Self {
            last_pasta: Mutex::new(String::new()),
            new_pasta: String::new(),
            pastabox: Mutex::new(Vec::new()),
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
        let new_pasta = &mut self.new_pasta;
        let pastabox = &mut self.pastabox;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                let add_button = ui.button("Add");
                ui.text_edit_singleline(new_pasta);

                if add_button.clicked() {
                    let guard = pastabox.lock();

                    if let Ok(mut list) = guard {
                        list.push(new_pasta.clone());
                        *new_pasta = String::new();
                    }
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let guard = pastabox.lock();

            if let Ok(mut list) = guard {
                list.clone().iter().enumerate().for_each(|(index, pasta)| {
                    ui.horizontal(|ui| {
                        let copy_button = ui.button("COPY");
                        let del_button = ui.small_button("DEL");

                        ui.label(pasta);

                        if copy_button.clicked() {
                            set_clipboard_string(pasta).unwrap_or_default();
                        }

                        if del_button.clicked() {
                            list.remove(index);
                        }
                    });
                });
            }

            egui::warn_if_debug_build(ui);
        });
    }
}

fn seek_pastas(pastabox: &'static PastaList, last_clipboard: Mutex<String>) {
    thread::spawn(move || loop {
        let pasta = get_clipboard_string();

        // match (pasta,)

        // if let Some(last) = &last_clipboard {
        //     if let Ok(pasta) = pasta {
        //         if &pasta != last {
        //             let guard = pastabox.lock();
        //             if let Ok(mut list) = guard {
        //                 list.push(pasta);
        //             }
        //         }
        //     }
        // }

        thread::sleep(Duration::from_secs(5));
    });
}
