use crate::config::Config;
use crate::iostreams::IOStreams;
use std::sync::{Arc, RwLock};

pub struct Factory {
    pub config: Arc<RwLock<Config>>,
    pub io: Arc<RwLock<IOStreams>>,
}

impl Factory {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::load()?;
        let io = IOStreams::system();

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            io: Arc::new(RwLock::new(io)),
        })
    }
}
