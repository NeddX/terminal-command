pub struct Application {
    title: String
}

impl Application {
    pub fn new() -> Self {
        return Application {
            title: "hello".to_string()
        };
    }
}
