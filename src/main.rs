use log::trace;

fn main() {
    web_logger::init();
    trace!("Starting forrs-frontend...");
    yew::start_app::<forrs_frontend::Model>();
}
