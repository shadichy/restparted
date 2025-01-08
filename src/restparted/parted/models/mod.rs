pub mod commands;
pub mod device;
pub mod request;
pub mod response;

pub fn initialize() {
  device::initialize();
}
