
pub mod tcmd;
pub mod termgl;
pub mod netrs;

fn main() {
    let mut app = tcmd::Application::new(10, 10, 10);
    app.run();
}
