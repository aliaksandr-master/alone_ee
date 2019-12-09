
pub struct Event<TEvent> {
    message: TEvent,
    was_stopped: bool,
}

impl<TEvent> Event<TEvent> {
    pub fn new(message: TEvent) -> Self {
        Self { message, was_stopped: false }
    }

    pub fn data(&self) -> &TEvent {
        &self.message
    }

    pub fn stop(&mut self) {
        self.was_stopped = true
    }

    pub fn stopped(&self) -> bool {
        self.was_stopped
    }
}
