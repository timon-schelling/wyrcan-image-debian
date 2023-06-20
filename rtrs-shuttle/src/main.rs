#[shuttle_runtime::main]
async fn shuttle_main() -> shuttle_axum::ShuttleAxum {
    Ok(rtrs_lib::router().into())
}
