// #![allow(dead_code, unused_imports, unused_mut, unused_variables)]// TODO: remove me
mod app_styles;
mod builder;
mod errors;
mod utils;


use std::thread;

use fltk::{
    app::{
        channel,
        Sender,
        Receiver,
        // Scheme
    },
    // enums::{
    //     LabelType
    // },
    text::TextBuffer,
    prelude::{
        WidgetExt,
        GroupExt,
    },
    window::{
        // Window,
        DoubleWindow
    }
};

use errors::InstallerError;


// Include the icon
#[cfg(feature="compile_icon")]
static APP_ICON_DATA: &'static [u8] = include_bytes!("static/icon.png");
#[cfg(not(feature="compile_icon"))]
static APP_ICON_DATA: &'static [u8] = b"";
// Include license
#[cfg(feature="compile_license")]
static APP_LICENSE: &'static str = include_str!("static/license.md");
#[cfg(not(feature="compile_license"))]
static APP_LICENSE: &'static str = "Hello, World!";
// GH link parts to accept the API
const ORG_NAME: &str = "Monika-After-Story";
const REPO_NAME: &str = "MonikaModDev";
// IDs of assets in github release
const DEF_VERSION_ASSET_ID: usize = 1;
const DLX_VERSION_ASSET_ID: usize = 0;


#[derive(Clone, Copy)]
pub enum Message {
    UpdateProgressBar(f64),
    Close,
    NextPage,
    PrevPage,
    SelectDir,
    DlxVersionCheck,
    Install,
    Preparing,
    Downloading,
    Extracting,
    CleaningUp,
    Error,
    Done
}

type InstallResult = Result<(), InstallerError>;


/// The entry point
fn main() {
    utils::disable_global_hotkeys();

    let (sender, receiver): (Sender<Message>, Receiver<Message>) = channel();
    let mut is_deluxe_version: bool = true;
    let mut installer_th_handle: Option<thread::JoinHandle<InstallResult>> = None;
    let mut extraction_dir = utils::get_cwd();
    let mut path_txt_buf = TextBuffer::default();
    path_txt_buf.set_text(extraction_dir.to_str().unwrap_or_default());
    let mut progress_bar = builder::build_progress_bar();

    let app = builder::build_app();

    let mut main_win = builder::build_outer_win();
    utils::load_icon(&mut main_win);
    main_win.begin();


    let welcome_win = builder::build_welcome_win(sender);
    let license_win = builder::build_license_win(sender);
    let dir_sel_win = builder::build_select_dir_win(sender, path_txt_buf.clone());
    let options_win = builder::build_options_win(sender, is_deluxe_version);
    let mut progress_win = builder::build_propgress_win(sender, &progress_bar);


    main_win.end();

    let mut current_win_id: usize = 0;
    let mut windows: Vec<DoubleWindow> = vec![
        welcome_win,
        license_win,
        dir_sel_win,
        options_win
    ];

    main_win.show();

    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::UpdateProgressBar(val) => {
                    progress_bar.set_value(val);
                },
                Message::Close => {
                    break;
                },
                Message::NextPage => {
                    let new_id = current_win_id+1;
                    utils::switch_win(&mut windows, &mut current_win_id, new_id);
                },
                Message::PrevPage => {
                    let new_id = current_win_id-1;
                    utils::switch_win(&mut windows, &mut current_win_id, new_id);
                },
                Message::SelectDir => {
                    let selected_dir = utils::run_select_dir_dlg(app_styles::SEL_DIR_DLG_PROMPT);
                    if selected_dir.is_dir() || selected_dir.parent().is_some() {
                        extraction_dir = selected_dir;
                        path_txt_buf.set_text(extraction_dir.to_str().unwrap_or_default());
                    }
                },
                Message::DlxVersionCheck => {
                    is_deluxe_version = !is_deluxe_version;
                },
                Message::Install => {
                    utils::hide_current_win(&mut windows, current_win_id);
                    progress_win.show();
                    installer_th_handle = Some(
                        utils::install_game_in_thread(&extraction_dir, sender, is_deluxe_version)
                    );
                },
                Message::Preparing => {
                    println!("Preparing");
                    progress_bar.set_label("Preparing...");
                },
                Message::Downloading => {
                    println!("Downloading");
                    progress_bar.set_label("Downloading...");
                },
                Message::Extracting => {
                    println!("Extracting");
                    progress_bar.set_label("Extracting...");
                },
                Message::CleaningUp => {
                    println!("Cleaning up");
                    progress_bar.set_label("Cleaning up...");
                },
                Message::Error => {
                    let _rv = cleanup_th_handle(installer_th_handle);
                    // We've moved the handle, set it to None
                    installer_th_handle = None;
                    // Request exit
                    sender.send(Message::Close);
                },
                Message::Done => {
                    println!("Done");
                    sender.send(Message::Close);
                }
            }
        }
    }
    cleanup_th_handle(installer_th_handle);
    app.quit();
}

/// Joins the thread handle
fn cleanup_th_handle(th_handle: Option<thread::JoinHandle<InstallResult>>) -> Option<InstallerError> {
    if let Some(th_handle) = th_handle {
        match th_handle.join() {
            Err(rv) => {
                eprintln!("Failed to join installer thread {:?}", rv);
            },
            Ok(rv) => {
                if let Err(e) = rv {
                    eprintln!("Installer thread failed: {}", e);
                    return Some(e);
                }
            }
        }
    }
    return None;
}
