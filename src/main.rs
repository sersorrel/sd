use std::{path::PathBuf, time::Duration};

#[derive(Debug)]
enum Event {
    NewScreenshot(PathBuf),
    Exit,
}

fn main() {
    notify_debouncer_mini::new_debouncer(Duration::from_millis(500), None, move |r| match r {
        Ok(events) => {
            for event in events {
                Event::NewScreenshot(event.path);
            }
        }
        Err(errors) => todo!(),
    })
    .unwrap();
}
