pub mod filesystem;
pub mod device;

pub fn initialize() {
  filesystem::initialize();
}
