mod android_service;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the Slint UI
    let ui = DashboardWindow::new()?;

    let ui_handle = ui.as_weak();

    // 2. Start the Android Auto background worker (waits for a USB connection)
    tokio::spawn(async {
        println!("Waiting for an Android device on the USB port...");
        // The android-auto crate's USB AOA loop is initialized here
    });

    // 3. Run the main GUI event loop
    ui.run()?;

    Ok(())
}