// Rectangular selection practice

pub fn configure_logging() {
    // [COMMENT] block start — comment these 5 lines out with Ctrl+v
    // let log_file = "/var/log/app.log";
    // let log_rotate = true;
    // let log_max_size = 10 * 1024 * 1024;
    // let log_compress = false;
    // let log_retention = 7;
    // [COMMENT] block end

    let log_file = "/var/log/app.log";
    println!("log_file={}", log_file);
}

pub fn emit_events(events: &[&str]) {
    // [SEMI] remove trailing ; from these 4 lines using Ctrl+v
    let level = "debug"
    let sink = "stdout"
    let format = "json"
    let buffer = 4096

    for event in events {
        println!("[{}] {} -> {}: {}", level, format, sink, event);
    }
    let _ = buffer;
}

pub fn setup_tracing() {
    // [LEVEL] change "debug" to "info" on these 3 lines
    let default_level = "info";
    let fallback_level = "info";
    let override_level = "info";

    println!("tracing: {}/{}/{}", default_level, fallback_level, override_level);
}
