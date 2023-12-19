#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = link_for_later::app::new();
    Ok(app.into())
}
