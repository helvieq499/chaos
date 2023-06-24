pub fn get() -> Option<web_sys::Storage> {
    web_sys::window()
        .map(|window| {
            window
                .local_storage()
                .ok()
                .flatten()
                .map(|local_storage| local_storage)
        })
        .flatten()
}
