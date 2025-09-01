use crate::geyser_plugin::GeyserPluginHook;
use agave_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the GeyserPluginHook pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = GeyserPluginHook::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
