use viperus::Format;
use std::sync::Once;

static INIT: Once = Once::new();

//static mut CONFIG_INITED: bool = false;

pub fn init() {
    INIT.call_once(|| {
//        println!("Config once ...");
        viperus::load_file(&path!("config/app.toml"), Format::TOML).expect("Can't load config file");
    })

//    unsafe {
//        if !CONFIG_INITED {
//            match viperus::load_file(&path!("config/app.toml"), Format::TOML) {
//                Ok(_) => (),
//                Err(e) => panic!(e.to_string())
//            }
//            CONFIG_INITED = true;
//        }
//    }
}

pub fn ok() -> bool { // same as dotenv
    init();
//    unsafe { CONFIG_INITED }
    true
}