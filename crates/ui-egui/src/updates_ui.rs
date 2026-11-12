//! Software updates: Help ▸ Check for Updates, the check at start, and their windows.
//!
//! The flow and the windows are `nodalia-update`'s, shared with the other Nodalia apps; this file
//! is what is this app's own: when an update may close it.

use crate::EffectcraftApp;

#[cfg(not(target_arch = "wasm32"))]
use nodalia_update::egui::{Action, Updater};

pub(crate) struct Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
    /// The web build has no updater; its window only says so.
    #[cfg(target_arch = "wasm32")]
    pub(crate) manual_window_open: bool,
    #[cfg(target_arch = "wasm32")]
    pub(crate) status_message: Option<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(crate::UPDATE_APP).with_accent([0xe0, 0x36, 0x8f]),
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
            #[cfg(target_arch = "wasm32")]
            manual_window_open: false,
            #[cfg(target_arch = "wasm32")]
            status_message: None,
        }
    }
}

impl EffectcraftApp {
    /// Whether the app looks for a newer release when it starts (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }
}

pub(crate) fn check(app: &mut EffectcraftApp) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.updates_rt.check_requested = true;
    }
    #[cfg(target_arch = "wasm32")]
    {
        app.updates_rt.manual_window_open = true;
        app.updates_rt.status_message = Some(crate::i18n::tr("Update check is not supported in the web browser.").to_string());
    }
}

pub(crate) fn tick(app: &mut EffectcraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let rt = &mut app.updates_rt;
        if std::mem::take(&mut rt.check_requested) {
            rt.updater.check_now(ctx);
        } else if rt.startup_check {
            rt.updater.check_at_start(ctx);
        }
        let action = rt.updater.poll();
        act(ctx, action);
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

/// Carries out what the update windows asked for.
#[cfg(not(target_arch = "wasm32"))]
fn act(ctx: &egui::Context, action: Action) {
    match action {
        Action::OpenUrl(url) => ctx.open_url(egui::OpenUrl::new_tab(url)),
        Action::Exit => {
            // The new version is starting, or a helper waits for this process to end. Close
            // normally so the preferences are saved; the install only ran once nothing was
            // unsaved. Exit anyway if the app has not ended soon.
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(0);
            });
        }
        Action::None => {}
    }
}

pub(crate) fn show(app: &mut EffectcraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // An installation waits until nothing is unsaved.
        let clean = !app.session.is_dirty();
        let language = nodalia_update::Language::from_code(crate::i18n::language(app));
        let action = app.updates_rt.updater.show(ctx, language, clean);
        act(ctx, action);
    }

    #[cfg(target_arch = "wasm32")]
    if app.updates_rt.manual_window_open {
        let mut open = true;
        let mut close = false;
        egui::Window::new(crate::i18n::tr("Check for Updates"))
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                if let Some(msg) = &app.updates_rt.status_message {
                    ui.label(msg);
                }
                ui.add_space(8.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(crate::i18n::tr("Close")).clicked() {
                        close = true;
                    }
                });
            });
        if !open || close {
            app.updates_rt.manual_window_open = false;
            app.updates_rt.status_message = None;
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use nodalia_update::egui::Status;

    #[test]
    fn the_releases_are_this_apps_own() {
        assert_eq!(crate::UPDATE_APP.repo, "getnodalia/pulso");
        assert_eq!(crate::UPDATE_APP.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn default_runtime_is_idle() {
        let rt = Runtime::default();
        assert!(!rt.startup_check);
        assert!(!rt.updater.is_open());
        assert_eq!(rt.updater.status(), &Status::Idle);
    }
}
