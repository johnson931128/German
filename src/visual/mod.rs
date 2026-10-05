mod app;

pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([800.0, 560.0]),
        ..Default::default()
    };

    eframe::run_native(
        "German — Publish / Subscribe Model",
        options,
        Box::new(|_creation_context| Ok(Box::new(app::PubSubApp::new()))),
    )
}
