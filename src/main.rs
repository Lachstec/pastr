use pastr::config;

fn main() {
    let cfg = config::get_config().unwrap();
    let db_opts = cfg.database.as_connect_options();
    println!("Config: {:?}", db_opts)
}
