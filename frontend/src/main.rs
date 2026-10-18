// SPDX-License-Identifier: GPL-3.0-or-later

//! Entry point. All the actual wiring lives in `lib.rs` so the tests
//! can reuse it — see that file's doc comment.

use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use slint::ComponentHandle;

use frontend::backend::{BackendCommand, SocketBackend};
use frontend::data::DataFiles;
use frontend::live_session::{Hyprland, LiveKeyboard, NoLiveKeyboard};
use frontend::mock_backend::{MockBackend, MockWifi};
use frontend::wifi::NetworkManager;
use frontend::{Services, build_ui, home_dir, load_config};

/// dawn: the LuminOS installer.
#[derive(Parser)]
#[command(name = "dawn")]
struct Cli {
    /// Use the real backend, but only report what an install would do:
    /// nothing runs as root and no disk is touched.
    #[arg(long, conflicts_with = "mock_backend")]
    dry_run: bool,

    /// Use canned answers instead of dawn-backend (SPEC.md's
    /// mock-backend mode), for working on the screens.
    #[arg(long)]
    mock_backend: bool,
}

/// The Hyprland session Dawn runs in, if it runs in one; otherwise
/// nothing to switch.
fn live_keyboard() -> Arc<dyn LiveKeyboard> {
    match Hyprland::from_env() {
        Some(hyprland) => Arc::new(hyprland),
        None => Arc::new(NoLiveKeyboard),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    // Slint uses the Cargo package name as the gettext domain: frontend.
    slint::init_translations!("/usr/share/locale");

    // VMs without a render node can still show the installer through
    // winit's software renderer. Respect an explicit SLINT_BACKEND.
    let has_gpu = std::fs::read_dir("/dev/dri").ok().is_some_and(|entries| {
        entries
            .flatten()
            .any(|entry| entry.file_name().to_string_lossy().starts_with("renderD"))
    });
    if !has_gpu
        && std::env::var_os("SLINT_BACKEND").is_none()
        && let Err(err) = slint::BackendSelector::new()
            .backend_name("winit".into())
            .renderer_name("software".into())
            .select()
    {
        eprintln!("error: {err}");
        return ExitCode::FAILURE;
    }

    let (config, config_override) = match load_config() {
        Ok(loaded) => loaded,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };

    let services = if cli.mock_backend {
        Services {
            backend: Arc::new(MockBackend::new().offline()),
            wifi: Arc::new(MockWifi),
            save_log_dir: home_dir(),
            data_files: DataFiles::system(),
            live_keyboard: live_keyboard(),
        }
    } else {
        Services {
            backend: Arc::new(SocketBackend::new(BackendCommand::locate(
                cli.dry_run,
                config_override,
            ))),
            wifi: Arc::new(NetworkManager::new()),
            save_log_dir: home_dir(),
            data_files: DataFiles::system(),
            live_keyboard: live_keyboard(),
        }
    };

    match build_ui(&config, services).and_then(|app| app.run()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
