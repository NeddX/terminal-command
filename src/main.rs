pub mod tcmd;
pub mod termgl;

fn main() {
    let app = tcmd::Application::new("Yes".to_string(), 100, 100, 10);
    println!("Hello, world!");
}
