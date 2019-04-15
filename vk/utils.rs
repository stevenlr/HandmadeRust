use crate::types::*;

pub(crate) unsafe fn get_last_base_out_struct_chain(
    mut s: *mut VkBaseOutStructure,
) -> *mut VkBaseOutStructure {
    while !(*s).p_next.is_null() {
        s = (*s).p_next as *mut VkBaseOutStructure;
    }
    return s;
}
