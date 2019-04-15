use crate::types::*;
use crate::commands::{StaticCommands, EntryCommands};

#[derive(Clone)]
pub struct EntryPoint {
    pub(crate) s: StaticCommands,
    pub(crate) e: EntryCommands,
}

impl EntryPoint {
    pub fn new(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {
        let static_commands = StaticCommands::load(load_fn);
        let entry_commands = EntryCommands::load(|fn_name| {
            unsafe { static_commands.get_instance_proc_addr(VkInstance::null(), fn_name.as_ptr()) }
        });
        Self {
            s: static_commands,
            e: entry_commands,
        }
    }

    pub fn enumerate_instance_layer_properties_count(&self) -> Result<(VkResult, usize), VkResult> {
        let mut p_property_count = 0;
        let ret = unsafe {
            self.e.enumerate_instance_layer_properties(
                &mut p_property_count,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_property_count as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_property_count as usize)),
            _ => Err(ret),
        };
    }

    pub fn enumerate_instance_layer_properties(&self,
        p_properties: &mut [VkLayerProperties]) -> Result<VkResult, VkResult> {
        let mut p_property_count = p_properties.len() as _;
        let ret = unsafe {
            self.e.enumerate_instance_layer_properties(
                &mut p_property_count,
                core::mem::transmute(p_properties.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn enumerate_instance_extension_properties_count(&self,
        p_layer_name: Option<&[u8]>) -> Result<(VkResult, usize), VkResult> {
        let mut p_property_count = 0;
        let ret = unsafe {
            self.e.enumerate_instance_extension_properties(
                match p_layer_name { Some(r) => r.as_ptr(), None => core::ptr::null() },
                &mut p_property_count,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_property_count as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_property_count as usize)),
            _ => Err(ret),
        };
    }

    pub fn enumerate_instance_extension_properties(&self,
        p_layer_name: Option<&[u8]>,
        p_properties: &mut [VkExtensionProperties]) -> Result<VkResult, VkResult> {
        let mut p_property_count = p_properties.len() as _;
        let ret = unsafe {
            self.e.enumerate_instance_extension_properties(
                match p_layer_name { Some(r) => r.as_ptr(), None => core::ptr::null() },
                &mut p_property_count,
                core::mem::transmute(p_properties.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn get_instance_proc_addr(&self,
        instance: VkInstance,
        p_name: &[u8]) -> PfnVkVoidFunction {
        let ret = unsafe {
            self.s.get_instance_proc_addr(
                instance,
                core::mem::transmute(p_name.as_ptr()),)
        };
        return ret;
    }

    pub fn create_instance(&self,
        p_create_info: &VkInstanceCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkInstance), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.e.create_instance(
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }
}
