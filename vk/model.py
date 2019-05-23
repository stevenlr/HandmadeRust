from typing import *
import xml.etree.ElementTree as xml

CMD_TYPE_STATIC     = "static"
CMD_TYPE_ENTRY      = "entry"
CMD_TYPE_INSTANCE   = "instance"
CMD_TYPE_DEVICE     = "device"

class DependenciesSet:
    def __init__(self):
        self.deps: List[str] = []

    def add_dep(self, dep: str):
        if dep != None and dep not in self.deps:
            self.deps.append(dep)

class Entity:
    def __init__(self, name: str):
        self.name = name

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        return depset

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "prelude-type")
        node.attrib["name"] = self.name

    @staticmethod
    def from_xml(node: xml.Element):
        return Entity(node.attrib["name"])

class Type:
    def get_base_type(self):
        return None

    def to_xml(self, parent: xml.Element):
        pass

    @staticmethod
    def from_xml(node: xml.Element) -> Optional[Type]:
        if node.tag == "type":
            return TypeReference.from_xml(node)
        elif node.tag == "pointer":
            return PointerType.from_xml(node)
        elif node.tag == "array":
            return ArrayType.from_xml(node)
        else:
            return None

class TypeReference(Type):
    def __init__(self, name: str, is_const: bool):
        self.name = name
        self.is_const = is_const

    def get_base_type(self):
        return self.name

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "type")
        node.attrib["const"] = str(self.is_const)
        node.text = self.name

    @staticmethod
    def from_xml(node: xml.Element) -> Optional[Type]:
        is_const = False
        if node.attrib["const"] == "True":
            is_const = True
        return TypeReference(node.text, is_const)

class PointerType(Type):
    def __init__(self, inner: Type, is_const: bool):
        self.inner = inner
        self.is_const = is_const

    def get_base_type(self):
        return self.inner.get_base_type()

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "pointer")
        node.attrib["const"] = str(self.is_const)
        self.inner.to_xml(node)

    @staticmethod
    def from_xml(node: xml.Element) -> Optional[Type]:
        is_const = False
        if node.attrib["const"] == "True":
            is_const = True
        return PointerType(Type.from_xml(node[0]), is_const)

class ArrayType(Type):
    def __init__(self, inner: Type, length: int):
        self.inner = inner
        self.length = length

    def get_base_type(self):
        return self.inner.get_base_type()

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "array")
        node.attrib["length"] = str(self.length)
        self.inner.to_xml(node)

    @staticmethod
    def from_xml(node: xml.Element) -> Optional[Type]:
        return ArrayType(Type.from_xml(node[0]), int(node.attrib["length"]))

class TypedIdentifier:
    def __init__(self, name: str, type: Type):
        self.name = name
        self.type = type

class BaseType(Entity):
    def __init__(self, name: str, alias: str):
        Entity.__init__(self, name)
        self.alias = alias

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.alias)
        return depset

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "base")
        node.attrib["name"] = self.name
        node.attrib["type"] = self.alias

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        return BaseType(node.attrib["name"], node.attrib["type"])


class AliasType(Entity):
    def __init__(self, name: str, alias: str):
        Entity.__init__(self, name)
        self.alias = alias

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.alias)
        return depset

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "alias")
        node.attrib["name"] = self.name
        node.attrib["type"] = self.alias

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        return AliasType(node.attrib["name"], node.attrib["type"])

class BitmaskType(Entity):
    def __init__(self, name: str, alias: str):
        Entity.__init__(self, name)
        self.alias = alias

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.alias)
        return depset

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "bitmask")
        node.attrib["name"] = self.name
        node.attrib["flags"] = self.alias

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        return BitmaskType(node.attrib["name"], node.attrib["flags"])

class HandleType(Entity):
    def __init__(self, name: str, dispatchable: bool):
        Entity.__init__(self, name)
        if dispatchable:
            self.type = "uintptr_t"
        else:
            self.type = "uint64_t"

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.type)
        return depset

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "handle")
        node.attrib["name"] = self.name
        node.attrib["type"] = self.type

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        if node.attrib["type"] == "uintptr_t":
            return HandleType(node.attrib["name"], True)
        else:
            return HandleType(node.attrib["name"], False)

class EnumType(Entity):
    def __init__(self, name: str, type: str):
        Entity.__init__(self, name)
        self.values: Dict[str, int] = {}
        self.type = type

    def add_value(self, name: str, value: int):
        self.values[name] = value

    def get_value(self, name: str) -> int:
        if not name in self.values:
            raise Exception("Unknown enum value")
        return self.values[name]

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "enum")
        node.attrib["name"] = self.name
        node.attrib["type"] = self.type
        for k in sorted(self.values.items(), key=lambda kv: kv[1]):
            e = xml.SubElement(node, "entry")
            e.attrib["name"] = k[0]
            e.text = str(k[1])

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        enum = EnumType(node.attrib["name"], node.attrib["type"])
        for child in node:
            enum.values[child.attrib["name"]] = int(child.text)
        return enum

class StructureMember:
    def __init__(self, id: TypedIdentifier, optional: bool, default_value: Optional[str], length: Optional[str]):
        self.id = id
        self.optional = optional
        self.default_value = default_value
        self.length = length

class StructureType(Entity):
    def __init__(self, name: str):
        Entity.__init__(self, name)
        self.members: List[StructureMember] = []
        self.extends: List[str] = []

    def add_member(self, entity: TypedIdentifier, optional: bool, default_value: Optional[str], length: Optional[str]):
        self.members.append(StructureMember(entity, optional, default_value, length))

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        for member in self.members:
            depset.add_dep(member.id.type.get_base_type())
        return depset

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "struct")
        node.attrib["name"] = self.name
        if len(self.extends) > 0:
            node.attrib["extends"] = ",".join(self.extends)
        for m in self.members:
            e = xml.SubElement(node, "member")
            e.attrib["name"] = m.id.name
            e.attrib["optional"] = str(m.optional)
            if m.default_value:
                e.attrib["default_value"] = m.default_value
            if m.length:
                e.attrib["length"] = m.length
            m.id.type.to_xml(e)

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        struct = StructureType(node.attrib["name"])
        if "extends" in node.attrib:
            struct.extends = node.attrib["extends"].split(",")
        for m in node:
            optional = False
            if m.attrib["optional"] == "True":
                optional = True
            id = TypedIdentifier(m.attrib["name"], Type.from_xml(m[0]))
            member = StructureMember(id, optional, m.get("default_value"), m.get("length"))
            struct.members.append(member)
        return struct

class UnionType(Entity):
    def __init__(self, name: str):
        Entity.__init__(self, name)
        self.members: List[TypedIdentifier] = []

    def add_member(self, entity: TypedIdentifier):
        self.members.append(entity)

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        for member in self.members:
            depset.add_dep(member.type.get_base_type())
        return depset

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "union")
        node.attrib["name"] = self.name
        for m in self.members:
            e = xml.SubElement(node, "member")
            e.attrib["name"] = m.name
            m.type.to_xml(e)

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        union = UnionType(node.attrib["name"])
        for m in node:
            id = TypedIdentifier(m.attrib["name"], Type.from_xml(m[0]))
            union.members.append(id)
        return union

class FunctionArgument:
    def __init__(self, id: TypedIdentifier, optional: bool, length: Optional[str]):
        self.id = id
        self.optional = optional
        self.length = length

class FunctionPrototype:
    def __init__(self, return_type: Type):
        self.return_type = return_type
        self.arguments: List[FunctionArgument] = []

    def add_argument(self, arg: TypedIdentifier, optional: bool, length: Optional[str]):
        self.arguments.append(FunctionArgument(arg, optional, length))

    def make_depset(self) -> DependenciesSet:
        depset = DependenciesSet()
        depset.add_dep(self.return_type.get_base_type())
        for member in self.arguments:
            depset.add_dep(member.id.type.get_base_type())
        return depset

    def to_xml(self, parent: xml.Element):
        self.return_type.to_xml(xml.SubElement(parent, "return-type"))
        for arg in self.arguments:
            node = xml.SubElement(parent, "arg")
            node.attrib["name"] = arg.id.name
            node.attrib["optional"] = str(arg.optional)
            if arg.length != None:
                node.attrib["length"] = arg.length
            arg.id.type.to_xml(node)

    @staticmethod
    def from_xml(node: xml.Element):
        proto = FunctionPrototype(Type.from_xml(node.find("./return-type")[0]))
        for m in node.findall("./arg"):
            optional = False
            if m.attrib["optional"] == "True":
                optional = True
            length = m.get("length")
            proto.add_argument(TypedIdentifier(m.attrib["name"], Type.from_xml(m[0])), optional, length)
        return proto

class Constant(Entity):
    def __init__(self, name: str):
        Entity.__init__(self, name)

class IntegerConstant(Constant):
    def __init__(self, name: str, value: int, size: int):
        Constant.__init__(self, name)
        self.value = value
        self.size = size

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "integer-constant")
        node.attrib["name"] = self.name
        node.attrib["size"] = str(self.size)
        node.text = str(self.value)

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        return IntegerConstant(node.attrib["name"], int(node.text), int(node.attrib["size"]))

class RealConstant(Constant):
    def __init__(self, name: str, value: float, size: int):
        Constant.__init__(self, name)
        self.value = value
        self.size = size

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "real-constant")
        node.attrib["name"] = self.name
        node.attrib["size"] = str(self.size)
        node.text = str(self.value)

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        return RealConstant(node.attrib["name"], float(node.text), int(node.attrib["size"]))

class StringConstant(Constant):
    def __init__(self, name: str, value: str):
        Constant.__init__(self, name)
        self.value = value

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "string-constant")
        node.attrib["name"] = self.name
        node.text = self.value

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        return StringConstant(node.attrib["name"], node.text)

class FunctionPointerType(Entity):
    def __init__(self, name: str, prototype: FunctionPrototype):
        Entity.__init__(self, name)
        self.prototype = prototype

    def make_depset(self) -> DependenciesSet:
        return self.prototype.make_depset()

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "function-pointer")
        node.attrib["name"] = self.name
        self.prototype.to_xml(node)

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        proto = FunctionPrototype.from_xml(node)
        return FunctionPointerType(node.attrib["name"], proto)

class Command(Entity):
    def __init__(self, name: str, prototype: FunctionPrototype, successcodes: Optional[str]):
        Entity.__init__(self, name)
        self.prototype = prototype
        self.type = CMD_TYPE_INSTANCE
        self.successcodes = successcodes

    def make_depset(self) -> DependenciesSet:
        return self.prototype.make_depset()

    def to_xml(self, parent: xml.Element):
        node = xml.SubElement(parent, "command")
        node.attrib["name"] = self.name
        node.attrib["type"] = self.type
        if self.successcodes:
            node.attrib["success-codes"] = self.successcodes
        self.prototype.to_xml(node)

    @staticmethod
    def from_xml(node: xml.Element) -> Entity:
        proto = FunctionPrototype.from_xml(node)
        successcodes = node.get("success-codes")
        cmd = Command(node.attrib["name"], proto, successcodes)
        cmd.type = node.attrib["type"]
        return cmd