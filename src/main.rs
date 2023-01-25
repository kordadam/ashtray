use std::{
    error::Error,
    result::Result,
    ffi::{CString}
};

mod util;

struct VulkanApp {
    _entry: ash::Entry,
    instance: ash::Instance
}

impl VulkanApp {
    fn new() -> Result<Self, Box<dyn Error>> {
        log::debug!("Creating application.");

        let entry = unsafe {ash::Entry::load()?};
        let instance = Self::create_instance(&entry)?;

        Ok(Self{_entry: entry, instance})
    }

    fn run(&mut self) {
        log::debug!("Running application.");
    }

    fn create_instance(entry: &ash::Entry) -> Result<ash::Instance, Box<dyn Error>> {
        let app_info = ash::vk::ApplicationInfo::builder()
            .application_name(CString::new("Vulkan Application")?.as_c_str())
            .application_version(ash::vk::make_api_version(0, 1, 0, 0))
            .engine_name(CString::new("No Engine")?.as_c_str())
            .engine_version(ash::vk::make_api_version(0, 1, 0, 0))
            .api_version(ash::vk::make_api_version(0, 1, 0, 0))
            .build();

        let extension_names = util::required_extension_names();

        let instance_create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        unsafe {Ok(entry.create_instance(&instance_create_info, None)?)}
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        log::debug!("Dropping application.");
        unsafe {self.instance.destroy_instance(None);}
    }
}

fn main() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::max())
        .init();

    match VulkanApp::new() {
        Ok(mut app) => app.run(),
        Err(error) => log::error!("Failed to create application. Cause: {}", error)
    }
}
