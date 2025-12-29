fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hola");
    log::info!("Soy tu esp32");
    log::info!("envio estos mensajes por serial");
}
