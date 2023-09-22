pub mod tcmd;
pub mod termgl;
pub mod netrs;

fn main() {
    let mut app = tcmd::Application::new("Yes".to_string(), 100, 100, 10);
    app.run();
    println!("Hello, world!");
}
