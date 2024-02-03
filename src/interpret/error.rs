use super::*;

impl ToString for InterpretError {
    fn to_string(&self) -> String {
        match self {
            InterpretError::TypeContainsItself(t) => {
                format!("Type '{}' contains itself", t.data.name.data)
            }
            InterpretError::UnknownType(t) => format!("Unknown type '{}'", t.code_view().view()),
            InterpretError::CyclicalReference(t) => {
                format!("Cyclical reference: {}", t.join(" -> "))
            }
            InterpretError::StructMemberNotUnique => format!("Struct member not unique"),
            InterpretError::MemberReferenceDoesntPointToView(c) => {
                format!("Member reference '{}' doesn't point to view", c.view())
            },
            InterpretError::MemberReferenceDoesntPointToArray(c) => {
                format!("Member reference '{}' doesn't point to array", c.view())
            },
            InterpretError::UnknownStructMemberReference(c) => {
                format!("Unknown struct member reference '{}'", c.view())
            }
            InterpretError::StructMemberConstantCanBeApliedOnlyForInt(c) => {
                format!("Struct member constant '{}' can be aplied only for integral type", c.view())
            }
            InterpretError::ViewReferenceTypeTooSmall(c) => {
                format!("View reference integral type '{}' is too small", c.view())
            }
            InterpretError::UnknownIntSize(t) => format!("Unknown int size '{}'", t),
            InterpretError::EnumAlreadyExists(t) => {
                format!("Enum '{}' already exists", t.data.name)
            }
            InterpretError::EnumConstantNotUnique(t) => {
                format!("Enum constant '{}' not unique", t.view())
            }
            InterpretError::EnumConstantValueNotUnique(t) => {
                format!("Enum constant value '{}' not unique", t.view())
            }
            InterpretError::EnumConstantValueNotFitting(t) => {
                format!("Enum constant value '{}' not fitting", t.view())
            }
            InterpretError::UnknownEnumMember(t) => {
                format!("Unknown enum member '{}' in '{}'", t.view(), t.pos())
            }
            InterpretError::UnknownEnum(t) => {
                format!("Unknown enum '{}' in '{}'", t.view(), t.pos())
            }
            InterpretError::ViewAlreadyExists(t) => format!("View '{}' already exists", t.view()),
            InterpretError::ViewItemNotUniqueWithinView(t) => {
                format!("View item '{}' not unique within view", t.view())
            }
            InterpretError::ViewReferenceKeyIsTooBig(c) => format!("View reference key '{}' is too big (max is 4 bytes)", c.view()),
            InterpretError::ViewEmpty(t) => format!("View '{}' is empty", t),
            InterpretError::VievConstantsMustBeAllEnumsOrAllIntsOrAllUndefined => {
                format!("View constants must be all enums or all ints or all undefined")
            },
            InterpretError::EndianNotSet => "Endian not set".into(),
            InterpretError::EndianOverrided(origin, overrided) => {
                format!("Endian cannot be override, originally defined here {} overided here {}", origin.pos(), overrided.pos())
            }
            InterpretError::GenericError(text) => text.clone(),
            InterpretError::GenericWithPosError(cv, text) => format!("{} in {}", text, cv.pos()),
            InterpretError::CannotAsignUsizeCstToNonUnsignedMemory(value) => format!("Cannot asign {} to non unsigned memory", value),
            InterpretError::ExpectedOperator(cv) => format!("expected operator + or - in {}", cv.pos()),
            InterpretError::ExpectedMemberSize(cv) => format!("expected member size reference (e.g. any_valid_member.size) at {}", cv.pos()),
            InterpretError::MemberValueNoUnsigned(cv) => format!("not unsigned member nor unsigned constant at {}", cv.pos()),
            InterpretError::BitMaskAlreadyExists(name, cv) => format!("mask name '{}' not unique at {}", name, cv.pos()),
            InterpretError::InvalidBitExpression(CodeView)
        }
    }
}
