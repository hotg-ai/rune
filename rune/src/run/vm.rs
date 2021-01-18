use log;
use runic_types::*;


/// Rune Executor 
///  Executes the Rune and provides the appropriate interfaces
pub struct VM {

}

///
impl VM {
    pub fn init(fileloc: &str) -> VM {
        log::info!("Initializing VM {}", fileloc);

        return VM{};
    }
}