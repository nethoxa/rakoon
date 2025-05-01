use crate::App;

impl App {
    pub fn handle_command(&mut self, command: String) -> Result<(), ()> {
        match command.trim() {
            "start" => {
                self.status.clone_from(&"running".to_string());
                *self.output.lock().unwrap() = "fuzzers spawned correctly".to_string();
                Ok(())
            },
            "stop" => {
                self.status.clone_from(&"stopped".to_string());
                *self.output.lock().unwrap() = "fuzzers stopped correctly".to_string();
                Ok(())
            },
            "exit" => {
                *self.output.lock().unwrap() = "fuzzers stopped correctly".to_string();
                Err(())
            }
             _ => {
                *self.output.lock().unwrap() = "invalid command".to_string();
                Ok(())
            }
        }
    }
}