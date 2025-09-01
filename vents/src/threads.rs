// use std::{sync::OnceLock, time::Duration};
//
// #[cfg(not_wasm)]
// pub(crate) async fn sleep(milis: u32) {
//     tokio::time::sleep(Duration::from_millis(milis.into())).await
// }
//
// #[cfg(wasm)]
// pub(crate) async fn sleep(milis: u32) {
//     gloo_timers::future::TimeoutFuture::new(milis).await
// }
//
// // Better use it only in tests
// pub(crate) fn busy_sleep(milis: u32) {
//     let start = instant::Instant::now();
//     let duration = Duration::from_millis(milis.into());
//     while start.elapsed() < duration {}
// }
//
// #[cfg(not_wasm)]
// pub fn spawn<F>(future: F)
// where F: std::future::Future<Output = ()> + Send + 'static {
//     static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
//
//     RUNTIME.get_or_init(||
// tokio::runtime::Runtime::new().unwrap()).spawn(future); }
//
// #[cfg(wasm)]
// pub fn spawn<F>(future: F)
// where F: std::future::Future<Output = ()> + 'static {
//     wasm_bindgen_futures::spawn_local(future);
// }
