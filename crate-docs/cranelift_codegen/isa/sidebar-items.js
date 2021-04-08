initSidebarItems({"enum":[["BackendVariant","The “variant” for a given target. On one platform (x86-64), we have two backends, the “old” and “new” one; the new one is the default if included in the build configuration and not otherwise specified."],["CallConv","Calling convention identifiers."],["ConstraintKind","The different kinds of operand constraints."],["LookupError","Describes reason for target lookup failure"],["StackBase","Generic base register for referencing stack slots."]],"fn":[["base_size","Returns the base size of the Recipe, assuming it’s fixed. This is the default for most encodings; others can be variable and longer than this base size, depending on the registers they’re using and use a different function, specific per platform."],["lookup","Look for an ISA for the given `triple`. Return a builder that can create a corresponding `TargetIsa`."],["lookup_by_name","Look for a supported ISA with the given `name`. Return a builder that can create a corresponding `TargetIsa`."],["lookup_variant","Look for an ISA for the given `triple`, selecting the backend variant given by `variant` if available."]],"mod":[["registers","Data structures describing the registers in an ISA."],["unwind","Represents information relating to function unwinding."],["x64","X86_64-bit Instruction Set Architecture."]],"struct":[["BranchRange","Constraints on the range of a branch instruction."],["Builder","Builder for a `TargetIsa`. Modify the ISA-specific settings before creating the `TargetIsa` trait object with `finish`."],["EncInfo","Information about all the encodings in this ISA."],["Encoding","Bits needed to encode an instruction as binary machine code."],["Encodings","An iterator over legal encodings for the instruction."],["OperandConstraint","Register constraint for a single value operand or instruction result."],["RecipeConstraints","Value operand constraints for an encoding recipe."],["StackBaseMask","Bit mask of supported stack bases."],["StackRef","A method for referencing a stack slot in the current stack frame."],["TargetFrontendConfig","This struct provides information that a frontend may need to know about a target to produce Cranelift IR for the target."]],"trait":[["TargetIsa","Methods that are specialized to a target ISA. Implies a Display trait that shows the shared flags, as well as any isa-specific flags."]],"type":[["Legalize","After determining that an instruction doesn’t have an encoding, how should we proceed to legalize it?"]]});