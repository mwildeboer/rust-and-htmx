use htmx::app;
use htmx::environment::Environment;

#[tokio::main]
async fn main() {
    let env = envy::from_env::<Environment>().unwrap();
    app::serve(&env).await;
}
