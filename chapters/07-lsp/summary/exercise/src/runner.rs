use crate::config::Config;
use crate::error::Error;

pub trait Lifecycle {
    fn start(&self) -> Result<(), Error>;
    fn stop(&self) -> Result<(), Error>;
    // Add `validate` here using LSP code action "Add missing impl items"
}

pub struct Runner {
    config: Config,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        Runner { config }
    }

    pub fn execute(&self) -> Result<(), Error> {
        if self.config.name.is_empty() {
            return Err(Error::Config("missing name".to_string()));
        }
        println!("running with config {:?}", self.config);
        Ok(())
    }
}

impl Lifecycle for Runner {
    fn start(&self) -> Result<(), Error> {
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        Ok(())
    }
}
