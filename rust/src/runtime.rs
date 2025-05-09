use crate::whitenoise::Whitenoise;
use once_cell::sync::OnceCell;
use std::{path::PathBuf, sync::Arc};
use tokio::runtime::Runtime;

static WN: OnceCell<Arc<Whitenoise>> = OnceCell::new();

pub fn init(data_dir: PathBuf) {
    let rt = Runtime::new().expect("tokio-rt");
    let wn = rt.block_on(Whitenoise::new(data_dir));

    if WN.set(Arc::new(wn)).is_err() {
        panic!("runtime::init called more than once");
    }
}

pub fn wn() -> Arc<Whitenoise> {
    WN.get().expect("runtime::init not called").clone()
}
