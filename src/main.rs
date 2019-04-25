use exitfailure::ExitFailure;
use i3ipc::{
    event::{inner::WindowChange, Event},
    I3Connection, I3EventListener, Subscription,
};
use signal_hook::{iterator::Signals, SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use std::{process::exit, thread};

fn update_opacity(ipc: &mut I3Connection) {
    ipc.run_command("[tiling] opacity 0.78; opacity 1").unwrap();
}

fn reset_opacity(ipc: &mut I3Connection) {
    ipc.run_command("[tiling] opacity 1").unwrap();
}

fn handle_signals() -> Result<(), ExitFailure> {
    let mut conn = I3Connection::connect().unwrap();
    let signals = Signals::new(&[SIGHUP, SIGINT, SIGQUIT, SIGTERM]).unwrap();
    signals.forever().next();
    reset_opacity(&mut conn);
    exit(0);
}

fn main() -> Result<(), ExitFailure> {
    thread::spawn(handle_signals);
    let mut conn = I3Connection::connect()?;
    let mut listener = I3EventListener::connect()?;
    listener.subscribe(&[Subscription::Window])?;

    reset_opacity(&mut conn);
    for event in listener.listen() {
        match event? {
            Event::WindowEvent(info) => {
                if let WindowChange::Focus = info.change {
                    update_opacity(&mut conn);
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
