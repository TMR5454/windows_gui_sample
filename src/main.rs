extern crate native_windows_gui as nwg;
use std::rc::Rc;

fn main() {
    nwg::init().expect("Initialize Native Windows GUI failed.");

    let mut window = Default::default();
    nwg::Window::builder()
        .size((500, 300))
        .title("Window title")
        .build(&mut window)
        .expect("create window failed");

    let rc_window = Rc::new(window);
    let events_window = rc_window.clone();

    let handler = nwg::full_bind_event_handler(&rc_window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => {
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                }
            }
            _ => {}
        }
    });
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler)
}
