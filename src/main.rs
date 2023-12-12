use edge_executor::LocalExecutor;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::peripherals::Peripherals};

mod wifi;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    // Connect to wifi
    let executor: LocalExecutor = Default::default();
    edge_executor::block_on(executor.run(async {
        let mut peripherals = Peripherals::take().unwrap();
        let sysloop = EspSystemEventLoop::take().unwrap();

        let wifi = wifi::init_wifi(
            "EXAMPLE",
            "EXAMPLE",
            &mut peripherals.modem,
            sysloop.clone(),
        )
        .await
        .unwrap();

        log::info!("Wifi initialized");

        let addr: std::net::SocketAddr = "0.0.0.0:8000".parse().unwrap();

        let normal_listener = std::net::TcpListener::bind(addr).unwrap();
        log::info!("Created regular tcp listener {normal_listener:?}");

        // Crashes here
        let async_listener = async_io::Async::new(normal_listener);
        log::info!("Created async listener {async_listener:?}");
    }))
}
