use tauri::Window;
use wormhole_common::{
    finder::find_install_dir,
    installer::spacewarp::SpaceWarpInstallManager,
    instances::KSPGame,
    models::download::{DownloadComplete, DownloadProgress},
};

pub fn on_download_progress(total: u64, received: usize, window: Window) {
    window
        .emit("download_progress", DownloadProgress { total, received })
        .unwrap();
}

pub fn on_download_finish(size: u64, window: Window) {
    window
        .emit("download_complete", DownloadComplete { size })
        .unwrap();
}

pub async fn install_spacewarp(window: Window) {
    let ksp2_dir = find_install_dir(KSPGame::KSP2);

    if let Some(dir) = ksp2_dir {
        let mut install_manager = SpaceWarpInstallManager::new(dir);

        install_manager.resolve().await.unwrap();

        install_manager
            .download(on_download_progress, on_download_finish, window)
            .await
            .unwrap();
    }
}

pub fn uninstall_spacewarp() {
    let ksp2_dir = find_install_dir(KSPGame::KSP2);

    if let Some(dir) = ksp2_dir {
        let mut install_manager = SpaceWarpInstallManager::new(dir);

        install_manager.uninstall();
    }
}
