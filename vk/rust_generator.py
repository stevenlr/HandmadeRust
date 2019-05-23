from loader import *
from model import *
import re

model = load_model("vk.xml")

# https://stackoverflow.com/a/41510011
RE_WORDS = re.compile(r'''
    # Find words in a string. Order matters!
    [A-Z]+(?=[A-Z][a-z]) |  # All upper case before a capitalized word
    [A-Z]?[a-z]+ |  # Capitalized words / all lower case
    [A-Z]+ |  # All upper case
    \d+  # Numbers
''', re.VERBOSE)

def pascal_case_split(identifier):
    matches = re.finditer(RE_WORDS, identifier)
    return [m.group(0) for m in matches]

def remove_prefix(s: str, p: str) -> str:
    if s.startswith(p):
        return s[len(p):]
    else:
        return s

def upper_snake_to_pascal(s: str) -> str:
    return "".join([x[0].upper() + x[1:] for x in s.lower().split("_")])

def camel_to_snake(s: str) -> str:
    tokens = pascal_case_split(s)
    return "_".join(tokens).lower()

def format_enum_value(s: str, p: str) -> str:
    tokens = pascal_case_split(p)
    prefix1 = "_".join(tokens).upper() + "_"
    prefix2 = "_".join(tokens[:-1]).upper() + "_"
    attempt = remove_prefix(s, prefix1)
    if attempt == s:
        attempt = remove_prefix(s, prefix2)
    if attempt[0].isdigit():
        attempt = "K_" + attempt
    return attempt.upper()

def format_bitmask_value(s: str, p: str) -> str:
    p = p.replace("FlagBits", "")
    tokens = pascal_case_split(p)
    prefix1 = "_".join(tokens).upper() + "_"
    prefix2 = "_".join(tokens[:-1]).upper() + "_"
    attempt = remove_prefix(s, prefix1)
    if attempt == s:
        attempt = remove_prefix(s, prefix2)
    if attempt[0].isdigit():
        attempt = "K_" + attempt
    return attempt.upper()

def make_default_value_type(name: str, t: Type) -> str:
    if isinstance(t, PointerType):
        if t.inner.is_const:
            return "core::ptr::null()"
        else:
            return "core::ptr::null_mut()"
    elif isinstance(t, ArrayType):
        default_value = make_default_value_type("", t.inner)
        return "[" + (", ".join([default_value] * int(t.length))) + "]"
    elif isinstance(t, TypeReference):
        if t.name == "float":
            return "0.0"
        elif name.startswith("pfn"):
            return "unsafe { core::mem::zeroed() }"
        elif t.name.startswith("Vk"):
            return to_rust_type(t.name) + "::default()"
        elif t.name == "HINSTANCE" or t.name == "HWND":
            return "core::ptr::null_mut()"
        else:
            return "0"
    return "0"

def make_default_value(m: StructureMember) -> str:
    if m.id.name == "sType" and m.default_value:
        return "VkStructureType::" + remove_prefix(m.default_value, "VK_STRUCTURE_TYPE_")
    else:
        return make_default_value_type(m.id.name, m.id.type)

def to_rust_type(name: str) -> str:
    if name == "uint8_t":
        return "u8"
    elif name == "int8_t":
        return "i8"
    elif name == "uint16_t":
        return "u16"
    elif name == "int16_t":
        return "i16"
    elif name == "uint32_t":
        return "u32"
    elif name == "int32_t":
        return "i32"
    elif name == "uint64_t":
        return "u64"
    elif name == "int64_t":
        return "i64"
    elif name == "uintptr_t":
        return "usize"
    elif name == "float":
        return "f32"
    elif name == "char":
        return "u8"
    elif name == "void":
        return "core::ffi::c_void"
    elif name == "size_t":
        return "usize"
    elif name.startswith("PFN_vk"):
        return "PfnVk" + remove_prefix(name, "PFN_vk")
    else:
        return name

def to_rust_type_deep(t: Type) -> str:
    def to_rust_type_deep_inner(t: Type) -> str:
        if isinstance(t, TypeReference):
            if t.is_const:
                return "const " + to_rust_type(t.name)
            else:
                return "mut " + to_rust_type(t.name)
        elif isinstance(t, PointerType):
            inner = to_rust_type_deep_inner(t.inner)
            if t.is_const:
                return "const *" + inner
            else:
                return "mut *" + inner
        elif isinstance(t, ArrayType):
            inner = to_rust_type_deep(t.inner)
            return "[%s; %d]" % (inner, t.length)
        else:
            return ""
    t_str = to_rust_type_deep_inner(t)
    return remove_prefix(remove_prefix(t_str, "mut "), "const ")

def to_rust_type_deep_arg(t: FunctionArgument) -> str:
    def to_rust_type_deep_inner_arg(t: Type, is_optional: bool, has_length: bool) -> str:
        if isinstance(t, TypeReference):
            rust_type = to_rust_type(t.name)
            if rust_type == "VkBool32":
                rust_type = "bool"
            if t.is_const:
                return rust_type
            else:
                return "mut " + rust_type
        elif isinstance(t, PointerType):
            inner = to_rust_type_deep_inner_arg(t.inner, False, False)
            inner_str = ""
            if not has_length:
                if t.is_const:
                    inner_str = "&%s" % inner
                else:
                    inner_str = "mut &%s" % inner
            else:
                if inner.endswith("core::ffi::c_void"):
                    inner = inner[:-17] + "u8"
                if t.is_const:
                    inner_str = "&[%s]" % inner
                else:
                    if t.inner.is_const:
                        inner_str = "mut &[%s]" % inner
                    else:
                        inner_str = "mut &mut [%s]" % remove_prefix(inner, "mut ")
            if is_optional:
                return "Option<%s>" % remove_prefix(inner_str, "mut ")
            else:
                return inner_str
        elif isinstance(t, ArrayType):
            inner = to_rust_type_deep(t.inner)
            return "[%s; %d]" % (inner, t.length)
        else:
            return ""
    t_str = to_rust_type_deep_inner_arg(t.id.type, t.optional, t.length != None)
    return remove_prefix(t_str, "mut ")

fp = open("types.rs", "w+")

fp.write("#![allow(unused_assignments, unreachable_patterns)]\n")
fp.write("use win32::{HINSTANCE, HWND};\n")
fp.write("\n")

for t in model["integer_constants"]:
    fp.write("pub const %s: u%d = %d;\n" % (t.name, t.size, t.value))
fp.write("\n")

for t in model["real_constants"]:
    fp.write("pub const %s: f%d = %f;\n" % (t.name, t.size, t.value))
fp.write("\n")

for t in model["string_constants"]:
    fp.write("pub const %s: &str = \"%s\";\n" % (t.name, t.value))
    fp.write("pub const %s__C: &[u8] = b\"%s\\0\";\n" % (t.name, t.value))
fp.write("\n")

for t in model["base_types"]:
    fp.write("pub type %s = %s;\n" % (t.name, to_rust_type(t.alias)))
fp.write("\n")

for t in model["bitmask_types"]:
    fp.write("pub type %s = %s;\n" % (t.name, to_rust_type(t.alias)))
fp.write("\n")

for t in model["handle_types"]:
    fp.write("#[repr(transparent)]\n")
    fp.write("#[derive(Default, Copy, Clone, PartialEq, Eq)]\n")
    type_name = t.name
    rust_type = to_rust_type(t.type)
    fp.write("pub struct %s(%s);\n" % (type_name, rust_type))
    fp.write("impl %s {\n" % type_name)
    fp.write("    #[inline]\n")
    fp.write("    pub fn null() -> Self {\n")
    fp.write("        Self(0)\n")
    fp.write("    }\n\n")
    fp.write("    #[inline]\n")
    fp.write("    pub fn from_raw(r: %s) -> Self {\n" % rust_type)
    fp.write("        Self(r)\n")
    fp.write("    }\n\n")
    fp.write("    #[inline]\n")
    fp.write("    pub fn as_raw(&self) -> %s {\n" % rust_type)
    fp.write("        self.0\n")
    fp.write("    }\n")
    fp.write("}\n\n")

for t in model["alias_types"]:
    fp.write("pub type %s = %s;\n" % (t.name, to_rust_type(t.alias)))
fp.write("\n")

for t in model["enum_types"]:
    if t.type != "enum":
        continue

    enum_name = t.name
    fp.write("#[repr(transparent)]\n")
    fp.write("#[derive(Default, PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]\n")
    fp.write("pub struct %s(u32);\n" % enum_name)
    fp.write("impl %s {\n" % enum_name)
    for v in t.values:
        value = t.values[v]
        v = format_enum_value(v, enum_name)
        if value < 0:
            fp.write("    pub const %s: %s = %s(%di32 as u32);\n" % (v, enum_name, enum_name, value))
        else:
            fp.write("    pub const %s: %s = %s(%d);\n" % (v, enum_name, enum_name, value))
    fp.write("}\n\n")
    fp.write("impl core::fmt::Debug for %s {\n" % enum_name)
    fp.write("    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {\n")
    fp.write("        match *self {\n")
    for v in t.values:
        v = format_enum_value(v, enum_name)
        fp.write("            %s::%s => write!(f, \"%s(%s)\"),\n" % (enum_name, v, enum_name, v))
    fp.write("            _ => write!(f, \"%s({})\", self.0),\n" % enum_name)
    fp.write("        }\n")
    fp.write("    }\n")
    fp.write("}\n\n")

for t in model["enum_types"]:
    if t.type != "bitmask":
        continue

    enum_name = t.name
    fp.write("#[repr(transparent)]\n")
    fp.write("#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]\n")
    fp.write("pub struct %s(VkFlags);\n" % enum_name)
    fp.write("impl %s {\n" % enum_name)
    for v in t.values:
        value = t.values[v]
        v = format_bitmask_value(v, enum_name)
        if value < 0:
            fp.write("    pub const %s: %s = %s(%di32 as u32);\n" % (v, enum_name, enum_name, value))
        else:
            fp.write("    pub const %s: %s = %s(%d);\n" % (v, enum_name, enum_name, value))
    fp.write("\n")
    fp.write("    #[inline]\n")
    fp.write("    pub fn contains(&self, other: Self) -> bool { return (self.0 & other.0) == other.0; }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitOr for %s {\n" % enum_name)
    fp.write("    type Output = %s;\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitOrAssign for %s {\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitAnd for %s {\n" % enum_name)
    fp.write("    type Output = %s;\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitAndAssign for %s {\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitXor for %s {\n" % enum_name)
    fp.write("    type Output = %s;\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }\n")
    fp.write("}\n\n")
    fp.write("impl core::ops::BitXorAssign for %s {\n" % enum_name)
    fp.write("    #[inline]\n")
    fp.write("    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }\n")
    fp.write("}\n\n")
    fp.write("impl core::fmt::Debug for %s {\n" % enum_name)
    fp.write("    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {\n")
    fp.write("        f.write_str(\"%s(\")?;\n")
    fp.write("        #[allow(unused_mut, unused)]\n")
    fp.write("        let mut first = true;\n")
    for v in t.values:
        v = format_bitmask_value(v, enum_name)
        fp.write("        if self.contains(%s::%s) {\n" % (enum_name, v))
        fp.write("            if !first {\n")
        fp.write("                f.write_str(\" | \")?;\n")
        fp.write("            }\n")
        fp.write("            first = false;\n")
        fp.write("            f.write_str(\"%s\")?;\n" % v)
        fp.write("        }\n")
    fp.write("        return f.write_str(\")\");\n")
    fp.write("    }\n")
    fp.write("}\n\n")

for t in model["structure_types"]:
    struct_name = t.name
    fp.write("#[repr(C)]\n")
    fp.write("#[derive(Copy, Clone)]\n")
    fp.write("pub struct %s {\n" % struct_name)
    for m in t.members:
        member_name = m.id.name
        member_name = camel_to_snake(member_name)
        if member_name == "type":
            member_name = "kind"
        fp.write("    pub %s: %s,\n" % (member_name, to_rust_type_deep(m.id.type)))
    fp.write("}\n\n")
    fp.write("pub trait Extends%s { }\n" % remove_prefix(struct_name, "Vk"))
    for extends in t.extends:
        fp.write("impl Extends%s for %s { }\n" % (remove_prefix(extends, "Vk"), struct_name))
    fp.write("\n")
    fp.write("impl Default for %s {\n" % struct_name)
    fp.write("    fn default() -> Self {\n")
    fp.write("        Self {\n")
    for m in t.members:
        member_name = camel_to_snake(m.id.name)
        if member_name == "type":
            member_name = "kind"
        default_value = make_default_value(m)
        fp.write("            %s: %s,\n" % (member_name, default_value))
    fp.write("        }\n")
    fp.write("    }\n")
    fp.write("}\n\n")

for t in model["union_types"]:
    struct_name = t.name
    fp.write("#[repr(C)]\n")
    fp.write("#[derive(Copy, Clone)]\n")
    fp.write("pub union %s {\n" % struct_name)
    for m in t.members:
        member_name = m.name
        member_name = camel_to_snake(member_name)
        if member_name == "type":
            member_name = "kind"
        fp.write("    pub %s: %s,\n" % (member_name, to_rust_type_deep(m.type)))
    fp.write("}\n\n")
    fp.write("impl Default for %s {\n" % struct_name)
    fp.write("    fn default() -> Self {\n")
    fp.write("        unsafe { core::mem::zeroed() }\n")
    fp.write("    }\n")
    fp.write("}\n\n")

for t in model["function_pointer_types"]:
    fn_name = "PfnVk" + t.name[6:]
    fp.write("pub type %s = extern \"system\" fn(\n" % fn_name)
    for a in t.prototype.arguments:
        fp.write("    %s: %s,\n" % (camel_to_snake(a.id.name), to_rust_type_deep(a.id.type)))
    fp.write(")")
    if not isinstance(t.prototype.return_type, TypeReference) or t.prototype.return_type.name != "void":
        fp.write(" -> %s" % to_rust_type_deep(t.prototype.return_type))
    fp.write(";\n\n")

for t in model["commands"]:
    fn_name = t.name
    fp.write("pub type PfnVk%s = extern \"system\" fn(\n" % fn_name[2:])
    for a in t.prototype.arguments:
        arg_name = camel_to_snake(a.id.name)
        if arg_name == "type":
            arg_name = "kind"
        fp.write("    %s: %s,\n" % (arg_name, to_rust_type_deep(a.id.type)))
    fp.write(")")
    if not isinstance(t.prototype.return_type, TypeReference) or t.prototype.return_type.name != "void":
        fp.write(" -> %s" % to_rust_type_deep(t.prototype.return_type))
    fp.write(";\n\n")

fp.close()

fp = open("commands.rs", "w+")
fp.write("use crate::types::*;\n\n")

def write_commands_collection(fp, type: str, struct_name: str):
    fp.write("#[derive(Clone)]\n")
    fp.write("pub struct %s {\n" % struct_name)
    for t in model["commands"]:
        if t.type != type:
            continue
        fp.write("    pfn_%s: PfnVk%s,\n" % (camel_to_snake(t.name[2:]), t.name[2:]))
    fp.write("}\n\n")
    fp.write("impl %s {\n" % struct_name)
    fp.write("    pub fn load(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {\n")
    fp.write("        %s {\n" % struct_name)
    for t in model["commands"]:
        if t.type != type:
            continue
        fp.write("            pfn_%s: unsafe { core::mem::transmute(load_fn(b\"%s\\0\")) },\n" % (camel_to_snake(t.name[2:]), t.name))
    fp.write("        }\n")
    fp.write("    }\n")
    for t in model["commands"]:
        if t.type != type:
            continue
        fp.write("\n    #[inline]\n")
        fp.write("    pub unsafe fn %s(&self" % camel_to_snake(t.name[2:]))
        for a in t.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"
            fp.write(",\n        %s: %s" % (arg_name, to_rust_type_deep(a.id.type)))
        fp.write(")")
        if not isinstance(t.prototype.return_type, TypeReference) or t.prototype.return_type.name != "void":
            fp.write(" -> %s" % to_rust_type_deep(t.prototype.return_type))
        fp.write(" {\n")
        inner_fn_name = "pfn_" + camel_to_snake(t.name[2:])
        fp.write("        (self.%s)(" % inner_fn_name)
        for a in t.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"
            fp.write("\n            %s," % arg_name)
        fp.write(")\n")
        fp.write("    }\n")
    fp.write("}\n\n")

write_commands_collection(fp, "static", "StaticCommands")
write_commands_collection(fp, "entry", "EntryCommands")
write_commands_collection(fp, "instance", "InstanceCommands")
write_commands_collection(fp, "device", "DeviceCommands")

fp.close()

fp = open("builders.rs", "w+")
fp.write("use win32::{HINSTANCE, HWND};\n")
fp.write("use crate::utils::*;\n")
fp.write("use crate::types::*;\n\n")

for t in model["structure_types"]:
    struct_name = t.name
    fp.write("pub struct %sBuilder<'a> {\n" % struct_name)
    fp.write("    s: %s,\n" % struct_name)
    fp.write("    _p: core::marker::PhantomData<&'a ()>,\n")
    fp.write("}\n\n")
    fp.write("impl<'a> %sBuilder<'a> {\n" % struct_name)
    fp.write("    pub fn new() -> Self {\n")
    fp.write("        Self {\n")
    fp.write("            s: %s::default(),\n" % struct_name)
    fp.write("            _p: core::marker::PhantomData,\n")
    fp.write("        }\n")
    fp.write("    }\n\n")
    fp.write("    pub fn build(&self) -> %s {\n" % struct_name)
    fp.write("        self.s.clone()\n")
    fp.write("    }")
    for m in t.members:
        member_name = camel_to_snake(m.id.name)
        if member_name == "type":
            member_name = "kind"
        if member_name == "p_next":
            fp.write("\n\n    pub fn push_next<T: Extends%s>(mut self, next: &'a mut T) -> %sBuilder<'a> {\n" % (remove_prefix(struct_name, "Vk"), struct_name))
            fp.write("        unsafe {\n")
            fp.write("            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);\n")
            fp.write("            (*last).p_next = self.s.p_next as _;\n")
            fp.write("            self.s.p_next = core::mem::transmute(next);\n")
            fp.write("        }\n")
        elif isinstance(m.id.type, TypeReference) and m.id.type.name == "VkBool32":
            fp.write("\n\n    pub fn %s(mut self, value: bool) -> %sBuilder<'a> {\n" % (member_name, struct_name))
            fp.write("        self.s.%s = if value { VK_TRUE } else { VK_FALSE };\n" % member_name)
        elif isinstance(m.id.type, PointerType) and m.length != None:
            length_code = ""
            length_simple_match = re.match("^([a-zA-Z_][a-zA-Z0-9_]*)$", m.length)
            length_complex_match = re.match("^([a-zA-Z_][a-zA-Z0-9_]*) / ([0-9]+)+$", m.length)
            if length_complex_match:
                length_code = "self.s.%s = (values.len() * %s) as _;" % (camel_to_snake(length_complex_match.group(1)), length_complex_match.group(2))
            elif length_simple_match:
                length_code = "self.s.%s = values.len() as _;" % camel_to_snake(m.length)
            else:
                length_code = ""
            if not m.id.type.inner.is_const:
                fp.write("\n\n    pub fn %s(mut self, values: &'a mut[%s]) -> %sBuilder<'a> {\n" % (member_name, to_rust_type_deep(m.id.type.inner), struct_name))
                fp.write("        %s\n" % length_code)
                fp.write("        self.s.%s = values.as_mut_ptr();\n" % camel_to_snake(m.id.name))
            else:
                fp.write("\n\n    pub fn %s(mut self, values: &'a [%s]) -> %sBuilder<'a> {\n" % (member_name, to_rust_type_deep(m.id.type.inner), struct_name))
                fp.write("        %s\n" % length_code)
                fp.write("        self.s.%s = values.as_ptr();\n" % camel_to_snake(m.id.name))
        elif isinstance(m.id.type, PointerType) and m.optional:
            if not m.id.type.inner.is_const:
                fp.write("\n\n    pub fn %s(mut self, value: Option<&'a mut %s>) -> %sBuilder<'a> {\n" % (member_name, to_rust_type_deep(m.id.type.inner), struct_name))
                fp.write("        self.s.%s = match value {\n" % member_name)
                fp.write("            Some(r) => r,\n")
                fp.write("            None => core::ptr::null_mut(),\n")
                fp.write("        };\n")
            else:
                fp.write("\n\n    pub fn %s(mut self, value: Option<&'a %s>) -> %sBuilder<'a> {\n" % (member_name, to_rust_type_deep(m.id.type.inner), struct_name))
                fp.write("        self.s.%s = match value {\n" % member_name)
                fp.write("            Some(r) => r,\n")
                fp.write("            None => core::ptr::null(),\n")
                fp.write("        };\n")
        elif isinstance(m.id.type, PointerType) and not m.optional:
            if not m.id.type.inner.is_const:
                fp.write("\n\n    pub fn %s(mut self, value: &'a mut %s) -> %sBuilder<'a> {\n" % (member_name, to_rust_type_deep(m.id.type.inner), struct_name))
                fp.write("        self.s.%s = value;\n" % member_name)
            else:
                fp.write("\n\n    pub fn %s(mut self, value: &'a %s) -> %sBuilder<'a> {\n" % (member_name, to_rust_type_deep(m.id.type.inner), struct_name))
                fp.write("        self.s.%s = value;\n" % member_name)
        else:
            fp.write("\n\n    pub fn %s(mut self, value: %s) -> %sBuilder<'a> {\n" % (member_name, to_rust_type_deep(m.id.type), struct_name))
            fp.write("        self.s.%s = value;\n" % member_name)
        fp.write("        self\n")
        fp.write("    }")
    fp.write("\n}\n\n")
    fp.write("impl<'a> core::ops::Deref for %sBuilder<'a> {\n" % struct_name)
    fp.write("    type Target = %s;\n\n" % struct_name)
    fp.write("    fn deref(&self) -> &Self::Target {\n")
    fp.write("        &self.s\n")
    fp.write("    }\n")
    fp.write("}\n\n")
    fp.write("impl<'a> core::ops::DerefMut for %sBuilder<'a> {\n" % struct_name)
    fp.write("    fn deref_mut(&mut self) -> &mut Self::Target {\n")
    fp.write("        &mut self.s\n")
    fp.write("    }\n")
    fp.write("}\n\n")

fp.close()

class CmdGenerator:
    def __init__(self, cmd: Command):
        self.cmd = cmd

    def write_to(self, fp, handle_type):
        self.generate(fp, handle_type)

    def generate(self, fp, handle_type):
        computed_length_code = ""
        computed_length_args = []
        for arg in self.cmd.prototype.arguments:
            if isinstance(arg.id.type, PointerType) and arg.id.type.inner.is_const and arg.length and arg.length != "null-terminated":
                if arg.length in computed_length_args:
                    computed_length_code += "        assert!(%s as usize == %s.len());\n" % (camel_to_snake(arg.length), camel_to_snake(arg.id.name))
                else:
                    computed_length_code += "        let %s = %s.len() as _;\n" % (camel_to_snake(arg.length), camel_to_snake(arg.id.name))
                    computed_length_args.append(arg.length)

        fp.write("\n    pub fn %s(&self" % camel_to_snake(self.cmd.name[2:]))
        for arg in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(arg.id.name)
            if arg_name == "type":
                arg_name = "kind"

            if is_handle(arg.id.type, handle_type):
                pass
            elif not arg.id.name in computed_length_args:
                 fp.write(",\n        %s: %s" % (arg_name, to_rust_type_deep_arg(arg)))
        fp.write(")")
        inner_fn_name = camel_to_snake(self.cmd.name[2:])
        is_return_type_result = False
        if is_result(self.cmd.prototype.return_type):
            fp.write(" -> Result<VkResult, VkResult> {\n")
            is_return_type_result = True
        elif is_void(self.cmd.prototype.return_type):
            fp.write(" {\n")
        else:
            fp.write(" -> %s {\n" % to_rust_type_deep(self.cmd.prototype.return_type))
        fp.write(computed_length_code)
        fp.write("        #[allow(unused)]\n")
        fp.write("        let ret = unsafe {\n")
        fp.write("            self.%s.%s(" % (self.cmd.type[0], inner_fn_name))
        for a in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"
            elif is_handle(a.id.type, handle_type):
                fp.write("\n                self.handle,")
            elif is_bool(a.id.type):
                fp.write("\n                if %s { VK_TRUE } else { VK_FALSE }," % arg_name)
            elif is_optional_ptr(a):
                fp.write("\n                %s," % optional_ptr_call(a))
            elif isinstance(a.id.type, PointerType) and a.length:
                if a.id.type.inner.is_const:
                    fp.write("\n                core::mem::transmute(%s.as_ptr())," % arg_name)
                else:
                    fp.write("\n                core::mem::transmute(%s.as_mut_ptr())," % arg_name)
            else:
                fp.write("\n                %s," % arg_name)
        fp.write(")\n")
        fp.write("        };\n")
        if is_return_type_result:
            fp.write("        return match ret {\n")
            for return_code in self.cmd.successcodes.split(","):
                fp.write("            VkResult::%s => Ok(ret),\n" % remove_prefix(return_code, "VK_"))
            fp.write("            _ => Err(ret),\n")
            fp.write("        };\n")
        elif not is_void(self.cmd.prototype.return_type):
            fp.write("        return ret;\n")
        fp.write("    }\n")

class CmdReturnSingleGenerator(CmdGenerator):
    def __init__(self, cmd: Command, output: FunctionArgument):
        CmdGenerator.__init__(self, cmd)
        self.output = output

    def generate(self, fp, handle_type):
        fp.write("\n    pub fn %s(&self" % camel_to_snake(self.cmd.name[2:]))
        for arg in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(arg.id.name)
            if arg_name == "type":
                arg_name = "kind"

            if arg == self.output:
                pass
            elif is_handle(arg.id.type, handle_type):
                pass
            else:
                 fp.write(",\n        %s: %s" % (arg_name, to_rust_type_deep_arg(arg)))
        fp.write(")")
        inner_fn_name = camel_to_snake(self.cmd.name[2:])
        is_bool = False
        is_return_type_result = False
        if is_result(self.cmd.prototype.return_type):
            ret_type = to_rust_type_deep(self.output.id.type.inner)
            if ret_type == "VkBool32":
                ret_type = "bool"
                is_bool = True
            fp.write(" -> Result<(VkResult, %s), VkResult> {\n" % ret_type)
            is_return_type_result = True
        else:
            fp.write(" -> %s {\n" % to_rust_type_deep(self.output.id.type.inner))
        fp.write("        let mut ret_value = unsafe { core::mem::uninitialized() };\n")
        fp.write("        #[allow(unused)]\n")
        fp.write("        let ret = unsafe {\n")
        fp.write("            self.%s.%s(" % (self.cmd.type[0], inner_fn_name))
        for a in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"
            if a == self.output:
                fp.write("\n                &mut ret_value,")
            elif is_handle(a.id.type, handle_type):
                fp.write("\n                self.handle,")
            elif is_optional_ptr(a):
                fp.write("\n                %s," % optional_ptr_call(a))
            else:
                fp.write("\n                %s," % arg_name)
        fp.write(")\n")
        fp.write("        };\n")
        if is_bool:
            fp.write("        #[allow(unused)]\n")
            fp.write("        let ret_value = ret_value == VK_TRUE;\n")
        if is_return_type_result:
            fp.write("        return match ret {\n")
            for return_code in self.cmd.successcodes.split(","):
                fp.write("            VkResult::%s => Ok((ret, ret_value)),\n" % remove_prefix(return_code, "VK_"))
            fp.write("            _ => Err(ret),\n")
            fp.write("        };\n")
        else:
            fp.write("        return ret_value;\n")
        fp.write("    }\n")

class CmdReturnSliceAndCountGenerator(CmdGenerator):
    def __init__(self, cmd: Command, count: FunctionArgument, output: FunctionArgument):
        CmdGenerator.__init__(self, cmd)
        self.count = count
        self.output = output

    def generate(self, fp, handle_type):
        self.generate_count(fp, handle_type)
        self.generate_data(fp, handle_type)

    def generate_count(self, fp, handle_type):
        fp.write("\n    pub fn %s_count(&self" % camel_to_snake(self.cmd.name[2:]))
        for arg in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(arg.id.name)
            if arg_name == "type":
                arg_name = "kind"

            if arg == self.count or arg == self.output:
                pass
            elif is_handle(arg.id.type, handle_type):
                pass
            else:
                 fp.write(",\n        %s: %s" % (arg_name, to_rust_type_deep_arg(arg)))
        fp.write(")")
        inner_fn_name = camel_to_snake(self.cmd.name[2:])
        is_return_type_result = False
        if is_result(self.cmd.prototype.return_type):
            fp.write(" -> Result<(VkResult, usize), VkResult> {\n")
            is_return_type_result = True
        elif is_void(self.cmd.prototype.return_type):
            fp.write(" -> usize {\n")
        else:
            raise Exception("Not implemented")
        computed_length_args = []
        for arg in self.cmd.prototype.arguments:
            if isinstance(arg.id.type, PointerType) and arg.id.type.inner.is_const and arg.length and arg.length != "null-terminated" and arg.length != self.count.id.name:
                fp.write("        let %s = %s.len() as _;\n" % (camel_to_snake(arg.length), camel_to_snake(arg.id.name)))
                computed_length_args.append(arg.length)
        for arg in self.cmd.prototype.arguments:
            if isinstance(arg.id.type, PointerType) and not arg.id.type.inner.is_const and arg.length and arg.length != "null-terminated" and arg.length != self.count.id.name:
                if arg.length in computed_length_args or "::" in arg.length:
                    length = ".".join([camel_to_snake(x) for x in arg.length.split("::")])
                    fp.write("        assert!(%s as usize == %s.len());\n" % (length, camel_to_snake(arg.id.name)))
                else:
                    fp.write("        let mut %s = %s.len() as _;\n" % (camel_to_snake(arg.length), camel_to_snake(arg.id.name)))
                    computed_length_args.append(arg.length)
        fp.write("        let mut %s = 0;\n" % camel_to_snake(self.count.id.name))
        fp.write("        #[allow(unused)]\n")
        fp.write("        let ret = unsafe {\n")
        fp.write("            self.%s.%s(" % (self.cmd.type[0], inner_fn_name))
        for a in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"

            if is_handle(a.id.type, handle_type):
                fp.write("\n                self.handle,")
            elif a == self.count:
                fp.write("\n                &mut %s," % arg_name)
            elif a == self.output:
                fp.write("\n                core::ptr::null_mut(),")
            elif is_optional_ptr(a):
                fp.write("\n                %s," % optional_ptr_call(a))
            elif isinstance(a.id.type, PointerType) and a.length:
                if a.id.type.inner.is_const:
                    fp.write("\n                core::mem::transmute(%s.as_ptr())," % arg_name)
                else:
                    fp.write("\n                core::mem::transmute(%s.as_mut_ptr())," % arg_name)
            else:
                fp.write("\n                %s," % arg_name)
        fp.write(")\n")
        fp.write("        };\n")
        if is_return_type_result:
            fp.write("        return match ret {\n")
            for return_code in self.cmd.successcodes.split(","):
                fp.write("            VkResult::%s => Ok((ret, %s as usize)),\n" % (remove_prefix(return_code, "VK_"), camel_to_snake(self.count.id.name)))
            fp.write("            _ => Err(ret),\n")
            fp.write("        };\n")
        else:
            fp.write("        return %s as usize;\n" % camel_to_snake(self.count.id.name))
        fp.write("    }\n")

    def generate_data(self, fp, handle_type):
        fp.write("\n    pub fn %s(&self" % camel_to_snake(self.cmd.name[2:]))
        for arg in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(arg.id.name)
            if arg_name == "type":
                arg_name = "kind"

            if arg == self.count:
                pass
            elif is_handle(arg.id.type, handle_type):
                pass
            elif arg == self.output:
                old_optional = arg.optional
                arg.optional = False
                fp.write(",\n        %s: %s" % (arg_name, to_rust_type_deep_arg(arg)))
                arg.optional = old_optional
            else:
                 fp.write(",\n        %s: %s" % (arg_name, to_rust_type_deep_arg(arg)))
        fp.write(")")
        inner_fn_name = camel_to_snake(self.cmd.name[2:])
        is_return_type_result = False
        if is_result(self.cmd.prototype.return_type):
            fp.write(" -> Result<VkResult, VkResult> {\n")
            is_return_type_result = True
        elif is_void(self.cmd.prototype.return_type):
            fp.write(" {\n")
        else:
            fp.write(" -> %s {\n" % to_rust_type_deep(self.output.id.type.inner))
        computed_length_args = []
        for arg in self.cmd.prototype.arguments:
            if isinstance(arg.id.type, PointerType) and arg.id.type.inner.is_const and arg.length and arg.length != "null-terminated":
                fp.write("        let %s = %s.len() as _;\n" % (camel_to_snake(arg.length), camel_to_snake(arg.id.name)))
                computed_length_args.append(arg.length)
        for arg in self.cmd.prototype.arguments:
            if isinstance(arg.id.type, PointerType) and not arg.id.type.inner.is_const and arg.length and arg.length != "null-terminated":
                if arg.length in computed_length_args or "::" in arg.length:
                    length = ".".join([camel_to_snake(x) for x in arg.length.split("::")])
                    fp.write("        assert!(%s as usize == %s.len());\n" % (length, camel_to_snake(arg.id.name)))
                else:
                    fp.write("        let mut %s = %s.len() as _;\n" % (camel_to_snake(arg.length), camel_to_snake(arg.id.name)))
                    computed_length_args.append(arg.length)
        fp.write("        #[allow(unused)]\n")
        fp.write("        let ret = unsafe {\n")
        fp.write("            self.%s.%s(" % (self.cmd.type[0], inner_fn_name))
        for a in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"

            if is_handle(a.id.type, handle_type):
                fp.write("\n                self.handle,")
            elif a == self.count:
                fp.write("\n                &mut %s," % arg_name)
            elif a == self.output:
                fp.write("\n                core::mem::transmute(%s.as_mut_ptr())," % arg_name)
            elif is_optional_ptr(a):
                fp.write("\n                %s," % optional_ptr_call(a))
            elif isinstance(a.id.type, PointerType) and a.length:
                if a.id.type.inner.is_const:
                    fp.write("\n                core::mem::transmute(%s.as_ptr())," % arg_name)
                else:
                    fp.write("\n                core::mem::transmute(%s.as_mut_ptr())," % arg_name)
            else:
                fp.write("\n                %s," % arg_name)
        fp.write(")\n")
        fp.write("        };\n")
        if is_return_type_result:
            fp.write("        return match ret {\n")
            for return_code in self.cmd.successcodes.split(","):
                fp.write("            VkResult::%s => Ok(ret),\n" % remove_prefix(return_code, "VK_"))
            fp.write("            _ => Err(ret),\n")
            fp.write("        };\n")
        elif not is_void(self.cmd.prototype.return_type):
            fp.write("        return ret_value;\n")
        fp.write("    }\n")

class CmdReturnSliceConstantCountGenerator(CmdGenerator):
    def __init__(self, cmd: Command, output: FunctionArgument):
        CmdGenerator.__init__(self, cmd)
        self.output = output

    def generate(self, fp, handle_type):
        length_arg = find_argument(self.cmd, self.output.length)

        fp.write("\n    pub fn %s(&self" % camel_to_snake(self.cmd.name[2:]))
        for arg in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(arg.id.name)
            if arg_name == "type":
                arg_name = "kind"

            if length_arg and arg == length_arg:
                pass
            elif is_handle(arg.id.type, handle_type):
                pass
            else:
                 fp.write(",\n        %s: %s" % (arg_name, to_rust_type_deep_arg(arg)))
        fp.write(")")
        inner_fn_name = camel_to_snake(self.cmd.name[2:])
        is_return_type_result = False
        if is_result(self.cmd.prototype.return_type):
            fp.write(" -> Result<VkResult, VkResult> {\n")
            is_return_type_result = True
        elif is_void(self.cmd.prototype.return_type):
            fp.write(" {\n")
        else:
            fp.write(" -> %s {\n" % to_rust_type_deep(self.output.id.type.inner))
        computed_length_args = []
        for arg in self.cmd.prototype.arguments:
            if isinstance(arg.id.type, PointerType) and arg.id.type.inner.is_const and arg.length:
                fp.write("        let %s = %s.len() as _;\n" % (camel_to_snake(arg.length), camel_to_snake(arg.id.name)))
                computed_length_args.append(arg.length)
        for arg in self.cmd.prototype.arguments:
            if isinstance(arg.id.type, PointerType) and not arg.id.type.inner.is_const and arg.length:
                if arg.length in computed_length_args or "::" in arg.length:
                    length = ".".join([camel_to_snake(x) for x in arg.length.split("::")])
                    fp.write("        assert!(%s as usize == %s.len());\n" % (length, camel_to_snake(arg.id.name)))
                else:
                    fp.write("        let %s = %s.len() as _;\n" % (camel_to_snake(arg.length), camel_to_snake(arg.id.name)))
                    computed_length_args.append(arg.length)
        fp.write("        #[allow(unused)]\n")
        fp.write("        let ret = unsafe {\n")
        fp.write("            self.%s.%s(" % (self.cmd.type[0], inner_fn_name))
        for a in self.cmd.prototype.arguments:
            arg_name = camel_to_snake(a.id.name)
            if arg_name == "type":
                arg_name = "kind"

            if is_handle(a.id.type, handle_type):
                fp.write("\n                self.handle,")
            elif is_optional_ptr(a):
                fp.write("\n                %s," % optional_ptr_call(a))
            elif isinstance(a.id.type, PointerType) and a.length:
                if a.id.type.inner.is_const:
                    fp.write("\n                core::mem::transmute(%s.as_ptr())," % arg_name)
                else:
                    fp.write("\n                core::mem::transmute(%s.as_mut_ptr())," % arg_name)
            else:
                fp.write("\n                %s," % arg_name)
        fp.write(")\n")
        fp.write("        };\n")
        if is_return_type_result:
            fp.write("        return match ret {\n")
            for return_code in self.cmd.successcodes.split(","):
                fp.write("            VkResult::%s => Ok(ret),\n" % remove_prefix(return_code, "VK_"))
            fp.write("            _ => Err(ret),\n")
            fp.write("        };\n")
        else:
            fp.write("        return ret_value;\n")
        fp.write("    }\n")

def is_void(t: Type) -> bool:
    return isinstance(t, TypeReference) and t.name == "void"

def is_bool(t: Type) -> bool:
    return isinstance(t, TypeReference) and t.name == "VkBool32"

def is_handle(t: Type, handle: str) -> bool:
    return isinstance(t, TypeReference) and t.name == handle

def is_result(t: Type) -> bool:
    return isinstance(t, TypeReference) and t.name == "VkResult"

def is_mut_ptr(t: Type) -> bool:
    return isinstance(t, PointerType) and not t.inner.is_const

def is_optional_ptr(a: FunctionArgument):
    return isinstance(a.id.type, PointerType) and a.optional

def optional_ptr_call(a: FunctionArgument) -> str:
    arg_name = camel_to_snake(a.id.name)
    if a.length:
        if a.id.type.inner.is_const:
            return "match %s { Some(r) => r.as_ptr(), None => core::ptr::null() }" % arg_name
        else:
            return "match %s { Some(r) => r.as_mut_ptr(), None => core::ptr::null_mut() }" % arg_name
    else:
        if a.id.type.inner.is_const:
            return "match %s { Some(r) => r, None => core::ptr::null() }" % arg_name
        else:
            return "match %s { Some(r) => r, None => core::ptr::null_mut() }" % arg_name

def find_arguments_mut_ptr(args: List[FunctionArgument]) -> List[FunctionArgument]:
    result = []
    for arg in args:
        if is_mut_ptr(arg.id.type):
            result.append(arg)
    return result

def find_arguments_mut_ptr_length(args: List[FunctionArgument]) -> List[FunctionArgument]:
    result = []
    for arg in args:
        if is_mut_ptr(arg.id.type) and arg.length:
            result.append(arg)
    return result

def find_argument(cmd: Command, name: str) -> Optional[FunctionArgument]:
    for a in cmd.prototype.arguments:
        if a.id.name == name:
            return a
    return None

def categorize_command(cmd: Command) -> CmdGenerator:
    proto = cmd.prototype
    args_mut_ptr = find_arguments_mut_ptr(proto.arguments)
    args_mut_ptr_length = find_arguments_mut_ptr_length(args_mut_ptr)
    if len(args_mut_ptr_length) > 1:
        raise Exception("Not implemented")
    elif len(args_mut_ptr_length) == 1:
        arg = args_mut_ptr_length[0]
        length_arg = find_argument(cmd, arg.length)
        if length_arg and isinstance(length_arg.id.type, PointerType) and not length_arg.id.type.inner.is_const:
            return CmdReturnSliceAndCountGenerator(cmd, length_arg, arg)
        else:
            return CmdReturnSliceConstantCountGenerator(cmd, arg)
    elif len(args_mut_ptr) == 1:
        return CmdReturnSingleGenerator(cmd, args_mut_ptr[0])
    elif len(args_mut_ptr) > 0:
        raise Exception("Not implemented")
    return CmdGenerator(cmd)

def write_commands(types: List[str], fp, handle_type: str):
    for t in model["commands"]:
        if t.type in types:
            cmd_gen = categorize_command(t)
            cmd_gen.write_to(fp, handle_type)

fp = open("entry_point.rs", "w+")
fp.write("use crate::types::*;\n")
fp.write("use crate::commands::{StaticCommands, EntryCommands};\n\n")
fp.write("#[derive(Clone)]\n")
fp.write("pub struct EntryPoint {\n")
fp.write("    pub(crate) s: StaticCommands,\n")
fp.write("    pub(crate) e: EntryCommands,\n")
fp.write("}\n\n")
fp.write("impl EntryPoint {\n")
fp.write("    pub fn new(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {\n")
fp.write("        let static_commands = StaticCommands::load(load_fn);\n")
fp.write("        let entry_commands = EntryCommands::load(|fn_name| {\n")
fp.write("            unsafe { static_commands.get_instance_proc_addr(VkInstance::null(), fn_name.as_ptr()) }\n")
fp.write("        });\n")
fp.write("        Self {\n")
fp.write("            s: static_commands,\n")
fp.write("            e: entry_commands,\n")
fp.write("        }\n")
fp.write("    }\n")
write_commands(["static", "entry"], fp, "")
fp.write("}\n")
fp.close()

fp = open("instance.rs", "w+")
fp.write("use crate::types::*;\n")
fp.write("use crate::entry_point::EntryPoint;\n")
fp.write("use crate::commands::InstanceCommands;\n\n")
fp.write("#[derive(Clone)]\n")
fp.write("pub struct Instance {\n")
fp.write("    pub(crate) handle: VkInstance,\n")
fp.write("    pub(crate) i: InstanceCommands,\n")
fp.write("}\n\n")
fp.write("impl Instance {\n")
fp.write("    pub fn new(instance: VkInstance, entry: &EntryPoint) -> Self {\n")
fp.write("        let commands = InstanceCommands::load(|fn_name| {\n")
fp.write("            unsafe { entry.s.get_instance_proc_addr(instance, fn_name.as_ptr()) }\n")
fp.write("        });\n")
fp.write("        Self {\n")
fp.write("            handle: instance,\n")
fp.write("            i: commands,\n")
fp.write("        }\n")
fp.write("    }\n")
write_commands(["instance"], fp, "VkInstance")
fp.write("}\n")
fp.close()

fp = open("device.rs", "w+")
fp.write("use crate::types::*;\n")
fp.write("use crate::instance::Instance;\n")
fp.write("use crate::commands::DeviceCommands;\n\n")
fp.write("#[derive(Clone)]\n")
fp.write("pub struct Device {\n")
fp.write("    handle: VkDevice,\n")
fp.write("    d: DeviceCommands,\n")
fp.write("}\n\n")
fp.write("impl Device {\n")
fp.write("    pub fn new(device: VkDevice, instance: &Instance) -> Self {\n")
fp.write("        let commands = DeviceCommands::load(|fn_name| {\n")
fp.write("            unsafe { instance.i.get_device_proc_addr(device, fn_name.as_ptr()) }\n")
fp.write("        });\n")
fp.write("        Self {\n")
fp.write("            handle: device,\n")
fp.write("            d: commands,\n")
fp.write("        }\n")
fp.write("    }\n")
write_commands(["device"], fp, "VkDevice")
fp.write("}\n")
fp.close()