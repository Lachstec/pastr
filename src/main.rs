use pastr::config;
use pastr::setup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::get_config().unwrap();
    let app = Application::with_config(cfg).await?;
    println!("Running Application on 127.0.0.1:{}", app.port);
    let app_task = tokio::spawn(app.run());
    let _ = app_task.await?;
    Ok(())
}
