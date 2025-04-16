pub struct AppState {
    pub artifact_id: &'static str,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            artifact_id: "storefront",
        }
    }
}
