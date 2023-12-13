// https://github.com/rust-lang/log/issues/421#issuecomment-885764004

use tracing_log::log::Metadata;
#[repr(C)]
pub struct SharedLogger {
    formatter: for<'a> extern "C" fn(&'a tracing_log::log::Record<'_>),
}
impl tracing_log::log::Log for SharedLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &tracing_log::log::Record) {
        (self.formatter)(record)
    }
    fn flush(&self) {}
}

pub fn build_shared_logger() -> SharedLogger {
    extern "C" fn formatter(r: &tracing_log::log::Record<'_>) {
        tracing_log::format_trace(r).unwrap()
    }
    SharedLogger { formatter }
}
#[no_mangle]
pub extern "C" fn setup_shared_logger(logger: SharedLogger) {
    if let Err(err) = tracing_log::log::set_boxed_logger(Box::new(logger)) {
        tracing_log::log::warn!("{}", err)
    }
}
