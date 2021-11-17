use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use clipboard_win::{get_clipboard_string, set_clipboard_string};
use eframe::{egui, epi};

type PastaList = Arc<Mutex<Vec<String>>>;
type LastPasta = Arc<Mutex<String>>;
type AutoAdd = Arc<Mutex<bool>>;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct PastaBox {
    new_pasta: String,
    last_pasta: LastPasta,
    pastabox: PastaList,
    auto_add: AutoAdd,
}

impl Default for PastaBox {
    fn default() -> Self {
        Self {
            last_pasta: Arc::new(Mutex::new(String::new())),
            new_pasta: String::new(),
            pastabox: Arc::new(Mutex::new(Vec::new())),
            auto_add: Arc::new(Mutex::new(true)),
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
        // let size = Vec2::new(900.0, 600.0);

        // frame.set_window_size(size);

        #[cfg(feature = "persistence")]
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }

        seek_pastas(
            self.pastabox.clone(),
            self.last_pasta.clone(),
            self.auto_add.clone(),
        )
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let last_pasta = &mut self.last_pasta;
        let new_pasta = &mut self.new_pasta;
        let pastabox = &mut self.pastabox;
        let auto_add = &mut self.auto_add;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                let add_button = ui.button("Add");
                ui.text_edit_singleline(new_pasta);

                let auto_status = {
                    let guard = auto_add.lock();

                    if let Ok(status) = guard {
                        match *status {
                            true => "On",
                            false => "Off",
                        }
                        .to_string()
                    } else {
                        "Off".to_string()
                    }
                };
                let auto_status = format!("Auto: {}", auto_status);

                let auto = ui.button(auto_status);
                let clean = ui.button("Clean");

                // ui.add_space(ui.available_width());

                if add_button.clicked() && new_pasta.len() > 0 {
                    let guard = pastabox.lock();

                    if let Ok(mut list) = guard {
                        list.push(new_pasta.clone());
                        *new_pasta = String::new();
                    }
                } else if auto.clicked() {
                    let guard = auto_add.lock();

                    if let Ok(mut auto_add) = guard {
                        *auto_add = !*auto_add;
                    }
                } else if clean.clicked() {
                    let pastabox = pastabox.lock();
                    let last_pasta = last_pasta.lock();

                    match (pastabox, last_pasta) {
                        (Ok(mut pastabox), Ok(mut last_pasta)) => {
                            pastabox.clear();
                            *last_pasta = String::new();
                        }
                        _ => (),
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

// to auto update pastabox
fn seek_pastas(pastabox: PastaList, last_clipboard: LastPasta, auto_add: AutoAdd) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));

        let pasta = get_clipboard_string();
        let last_clipboard = last_clipboard.lock();
        let auto_add = auto_add.lock();

        match (last_clipboard, pasta, auto_add) {
            (Ok(mut last), Ok(pasta), Ok(auto_add))
                if pasta != last.to_string() && *auto_add == true =>
            {
                let guard = pastabox.lock();
                if let Ok(mut list) = guard {
                    list.push(pasta.clone());
                    *last = pasta;
                }
            }
            _ => (),
        }
    });
}
