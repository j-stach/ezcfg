
pub trait Config {

    /// Static path where the config file resides.
    const PATH: &str;

    /// Read the configuration file and parse.
    fn read() -> Result<Self, super::Error> where Self: Sized;

    /// Overwrite the configuration file with new values.
    fn write(&self) -> Result<(), super::Error>;

    // TBD: Provided methods
}

