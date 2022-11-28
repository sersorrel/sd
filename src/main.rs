//! # sd: screenshot daemon

use std::{
    path::PathBuf,
    sync::{atomic::AtomicBool, mpsc, Arc},
    thread,
};

use eyre::WrapErr;

#[derive(Debug)]
enum Event {
    /// A new screenshot was discovered at a path.
    NewScreenshot(PathBuf),
    /// The application should exit.
    Exit,
}

/// Set up signal handlers to send an `Event::Exit` when a termination signal is received.
fn configure_signals(tx: mpsc::Sender<Event>) -> eyre::Result<()> {
    let interrupted = Arc::new(AtomicBool::new(false));
    for signal in signal_hook::consts::TERM_SIGNALS {
        // if we can't shut down cleanly, just exit...
        signal_hook::flag::register_conditional_shutdown(*signal, 1, Arc::clone(&interrupted))
            .wrap_err("couldn't register fallback shutdown hook")?;
        // ...but try to shut down cleanly first.
        signal_hook::flag::register(*signal, Arc::clone(&interrupted))
            .wrap_err("couldn't register shutdown hook")?;
    }
    let mut signals = signal_hook::iterator::Signals::new(signal_hook::consts::TERM_SIGNALS)
        .wrap_err("couldn't register interest in shutdown signals")?;
    thread::Builder::new()
        .name("sd signal handler".into())
        .spawn(move || {
            signals.forever().next();
            tx.send(Event::Exit).unwrap();
        })
        .wrap_err("couldn't spawn signal handler thread")?;
    Ok(())
}

fn spawn_screenshot_watcher(tx: mpsc::Sender<Event>) -> eyre::Result<()> {
    tx.send(Event::NewScreenshot(todo!()))?;
}

fn main() -> eyre::Result<()> {
    let (tx, rx) = mpsc::channel();

    configure_signals(tx.clone())?;

    spawn_screenshot_watcher(tx.clone())?;

    for event in rx.into_iter() {
        dbg!(&event);
        match event {
            Event::Exit => break,
            Event::NewScreenshot(path) => todo!("new screenshot at {:?}", path),
        }
    }
    Ok(())
}
