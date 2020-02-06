use viperus::Format;

static mut CONFIG_INITED: bool = false;

pub fn init() {
    unsafe {
        if !CONFIG_INITED {
            match viperus::load_file(&path!("config/app.toml"), Format::TOML) {
                Ok(_) => (),
                Err(e) => panic!(e.to_string())
            }
            CONFIG_INITED = true;
        }
    }
}

pub fn ok() -> bool { // same as dotenv
    init();
    unsafe { CONFIG_INITED }
}