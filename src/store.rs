use wasm_bindgen::prelude::*;

pub struct Store {
    local_storage: web_sys::Storage,
    name: String,
}

impl Store {
    pub fn new(name: &str) -> Option<Store> {
        let window = web_sys::window()?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            let mut store = Store {
                local_storage,
                name: String::from(name),
            };
            Some(store)
        } else {
            None
        }
    }
}
