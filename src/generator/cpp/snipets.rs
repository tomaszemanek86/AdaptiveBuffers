use super::*;

pub fn def_context(member: &dyn CppDetails, writer: &mut Writer) {
    writer.def_member_var(
        &member.member_context_variable(), 
        &format!("{}&", &member.as_context_typename()), 
        None);
}

pub fn def_offset(writer: &mut Writer) {
    writer.def_member_var("offset", "uint16_t", Some("0"));
}

pub fn def_context_ctor(context: &dyn CppDetails, init_members: bool, addtional_members: &[Argument], in_cpp: bool, writer: &mut Writer) {
    writer.def_ctor(
        &context.as_context_typename(), &[
            Argument {
                name: "previous".to_string(),
                typename: format!("uint32_t*"),
            },
            Argument {
                name: "next_back_reference".to_string(),
                typename: format!("uint32_t**"),
            }
        ]
            .iter()
            .chain(addtional_members.iter())
            .map(|a| a.clone())
            .collect::<Vec<Argument>>(), 
        in_cpp
    );
    if in_cpp && init_members {
        writer.write(&format!(" : {}(previous, next_back_reference) ", context.member_context_variable()));
    }
}

pub fn def_isset_member<TMember: CppDetails>(member: &TMember, writer: &mut Writer) {
    writer.def_member_var(&member.member_isset_variable(), "bool", Some("false"));
}

pub fn def_index_member<TMember: CppDetails>(member: &TMember, writer: &mut Writer) {
    writer.def_member_var(&member.member_index_variable(), "uint16_t", Some("false"));
}

pub fn def_member_data_var<TMember: CppDetails>(member: &TMember, writer: &mut Writer) {
    writer.def_member_var(&member.member_data_variable(), &member.as_typename(), None);
}

pub fn def_member_context_var<TMember: CppDetails>(member: &TMember, writer: &mut Writer) {
    writer.def_member_var(&member.member_context_variable(), &format!("{}&", member.as_context_typename()), None);
}

pub fn def_member_buffer_var<TMember: CppDetails>(member: &TMember, reference: bool, writer: &mut Writer) {
    writer.def_member_var(&member.member_buffer_variable(), 
        &format!("{}{}", 
        member.as_buffer_typename(), if reference { "&" } else { "" }), 
        None);
}

pub fn def_reader(writer: &mut Writer) {
    writer.def_member_var("_reader", "core:IReader", None);
}

pub fn if_isset_is_throw<TMember: CppDetails>(member: &TMember,value: bool, writer: &mut Writer) {
    writer.write_with_offset(&format!("if ({}.{} == {})", 
        member.member_buffer_variable(), 
        member.member_isset_variable(), 
        if value { "true" } else { "false" } ));
    writer.scope_in();
    writer.write_line(&format!(
        "throw \"{} {} set;\"",
        member.member_data_variable(),
        if value { "already" } else { "not" }
    ));
    writer.scope_out(false);
}

pub fn def_serialize_method<TMember: CppDetails>(memory: &TMember, in_cpp: bool, writer: &mut Writer) {
    let classname = memory.as_serializer_typename();
    writer.def_member_function("serialize", 
    &[Argument {
        name: "buffer".to_string(),
        typename: "void*".to_string()
    }], Some(&NativeType::U16.as_typename()), 
    if in_cpp { Some(&classname) } else { None });
    if !in_cpp {
        writer.semicolon();
    }
}

pub fn def_serializer_ctor<TClass: CppDetails>(
    class: &TClass,
    in_cpp: bool,
    writer: &mut Writer,
) {
    writer.def_ctor(&class.as_serializer_typename(), &[
        Argument {
            name: "end".to_string(),
            typename: format!("{}**", class.as_buffer_typename())
        }
    ], in_cpp);
    if in_cpp == false {
        writer.semicolon();
    } else {
        writer.write(&format!(" : {}(buffer)", class.member_previous_end_ptr_variable()));
        writer.scope_in();
        writer.scope_out(false);
    }
}

pub fn def_with_method(
    class: &dyn CppDetails,
    member: &dyn CppDetails,
    cpp: bool,
    writer: &mut Writer,
) {
    let serializer_classname = class.as_serializer_typename();
    let has_only_native_members = member.has_only_native_member();
    if has_only_native_members {
        writer.def_member_function(
            &member.as_variable(Some("with_"), None),
            &[Argument {
                name: "value".to_string(),
                typename: member.as_typename()
            }],
            Some(&format!("{}&", class.as_serializer_typename())),
            if cpp { Some(&serializer_classname) } else { None },
        );
    } else {
        writer.def_member_function(
            &member.as_variable(Some("with_"), None),
            &[],
            Some(&format!("{}&", member.as_serializer_typename())),
            if cpp { Some(&serializer_classname) } else { None },
        );
    }
}

pub fn def_with_method_for_enum(
    enum_class: &dyn CppDetails,
    cpp: bool,
    writer: &mut Writer,
) {
    let serializer_classname = enum_class.as_serializer_typename();
    writer.def_member_function(
        &enum_class.as_variable(Some("with_"), None),
        &[Argument {
            name: "value".to_string(),
            typename: enum_class.as_typename()
        }],
        Some(&format!("{}&", serializer_classname)),
        if cpp { Some(&serializer_classname) } else { None },
    );
    if cpp == false {
        writer.semicolon();
    } else {
        writer.scope_in();
        writer.write_line(&format!("{}.{} = true;", 
            enum_class.member_buffer_variable(), 
            enum_class.member_isset_variable()));
        writer.write_line(&format!("{}.{} = value;", 
            enum_class.member_buffer_variable(), 
            enum_class.member_data_variable()
        ));
        writer.write_line("return *this");
        writer.scope_out(false);
    }
}

pub fn def_with_method_for_view(
    view: &dyn CppDetails,
    view_posibility: &dyn CppDetails,
    index: usize, 
    cpp: bool,
    writer: &mut Writer,
) {
    def_with_method(view, view_posibility, cpp, writer);
    if cpp == false {
        writer.semicolon();
    } else {
        writer.scope_in();
        let has_only_native_members = view_posibility.has_only_native_member();
        if has_only_native_members {
            writer.write_line(&format!("{}.{} = true;", 
                view.member_buffer_variable(), 
                view.member_isset_variable()));
            writer.write_line(&format!("{}.{} = {};", 
                view.member_buffer_variable(), 
                view.member_index_variable(), 
                index));
        }

        if has_only_native_members {
            writer.write_line("return *this");
        } else {
            writer.write_line(&format!("return {}(&{}.{});", 
                view_posibility.as_serializer_typename(), 
                view_posibility.member_data_variable(),
                view_posibility.as_variable(None, None)
            ));
        }
        writer.scope_out(false);
    }
}

pub fn def_with_method_for_struct(struct_class: &dyn CppDetails, member: &dyn CppDetails, cpp: bool, writer: &mut Writer) {
    def_with_method(struct_class, member, cpp, writer);
    if cpp == false {
        writer.semicolon();
    } else {
        writer.scope_in();
        let has_only_native_members = member.has_only_native_member();
        if has_only_native_members {
            // isset = true
            writer.write_line(&format!("{}.{}.{} = true;", 
                struct_class.member_buffer_variable(), 
                member.member_buffer_variable(), 
                member.member_isset_variable()));
        }

        if has_only_native_members {
            // data = value
            writer.write_line(&format!("{}.{}.{} = value;", 
                struct_class.member_buffer_variable(), 
                member.member_buffer_variable(), 
                member.member_data_variable()
            ));
        }

        if has_only_native_members {
            writer.write_line("return *this");
        } else {
            writer.write_line(&format!("return {}(&{}.{});", 
                member.as_serializer_typename(), 
                struct_class.member_buffer_variable(),
                member.member_data_variable()
            ));
        }
        writer.scope_out(false);
    }
}

pub fn serialize_copy<TMember: CppDetails>
    (
        member: &TMember, 
        member_size: Option<&dyn CppDetails>, 
        writer: &mut Writer) 
    {
    if let Some(ms) = member_size {
        if ms.has_only_native_member() {
            writer.write_line(&format!("std::memcpy(buffer + offset, &{}.{}, {});", 
                member.member_buffer_variable(), 
                member.member_data_variable(), 
                ms.exact_size().unwrap()));
            writer.write_line(&format!("offset += {};", 
                ms.exact_size().unwrap()));
        } else {
            if member.has_only_native_member() {
                writer.write_line(&format!("std::memcpy(buffer + offset, &{}.{}, {});", 
                    member.member_buffer_variable(), 
                    member.member_data_variable(), 
                    member.exact_size().unwrap()));
                writer.write_line(&format!("offset += {};", 
                    member.exact_size().unwrap()));
            } else {
                writer.write_line(&format!("offset += {}({}).serialize(buffer + offset);", 
                    member.member_serializer_variable(), 
                    member.member_buffer_variable()));
            }
        }
    } else {
        writer.write_line(&format!("offset += {}({}).serialize(buffer + offset);", 
            member.member_serializer_variable(), 
            member.member_buffer_variable()));
    }
}

pub fn serialize_return(writer: &mut Writer) {
    writer.write_line("return offset;");
}

pub fn def_set_offset(class: &dyn CppDetails, cpp: bool, writer: &mut Writer) {
    if cpp {
        writer.def_member_function("set_data_ptr", 
        &[Argument {
            name: "ptr".to_string(),
            typename: "void*".to_string()
        }], None, None);
    } else {
        writer.def_member_function("set_data_ptr", 
        &[Argument {
            name: "ptr".to_string(),
            typename: "void*".to_string()
        }], None, Some(&class.as_deserializer_typename()));
    }
    if !cpp {
        writer.semicolon();
    } else {
        writer.scope_in();
        writer.write_line(&format!("{} = ptr;", class.member_buffer_ptr()));
        writer.scope_out(false);
    }
    
}

pub fn def_deserializer_ctor(
    class: &dyn CppDetails,
    in_cpp: bool,
    writer: &mut Writer,
) {
    writer.def_ctor(&class.as_deserializer_typename(), &[
        Argument {
            name: "reader".to_string(),
            typename: "core::IReader&".to_string()
        },
        Argument {
            name: "context".to_string(),
            typename: format!("{}&", class.as_context_typename())
        }
    ], in_cpp);
    if in_cpp {
        writer.write(&format!(" : _reader(reader), {}(context)", class.member_context_variable()));
    }
}

pub fn def_deserializer_get(class: &dyn CppDetails, member: &dyn CppDetails, cpp: bool, writer: &mut Writer) {
    if cpp {
        if member.has_only_native_member() {
            writer.def_member_function(&member.fn_get(), &[], Some(&member.as_typename()), Some(&class.as_deserializer_typename()));
        } else {
            writer.def_member_function(&member.fn_get(), &[], Some(&member.as_deserializer_typename()), Some(&class.as_deserializer_typename()));
        }
    } else {
        if member.has_only_native_member() {
            writer.def_member_function(&member.fn_get(), &[], Some(&member.as_typename()), None);
        } else {
            writer.def_member_function(&member.fn_get(), &[], Some(&member.as_deserializer_typename()), None);
        }
    }
    if !cpp {
        writer.semicolon();
    }
}

pub fn if_previous_not_set_throw(context: &dyn CppDetails, writer: &mut Writer) {
    writer.write_with_offset(&format!("if ({}._previous_end == nullptr) ", context.member_context_variable()));
    writer.scope_in();
    writer.write_line(&format!("throw std::exception(\"previous data not deserialized\");"));
    writer.scope_out(false);
}

pub fn read_and_return_value(context: &dyn CppDetails, subcontext: Option<&dyn CppDetails>, typename: &dyn CppDetails, writer: &mut Writer) {
    writer.write_line(&format!("{} value;", typename.as_typename()));
    writer.write_line(&format!("_reader.read(*{}.{}_previous_end, &value, {});", 
        context.member_context_variable(), 
        if let Some(sctx) = subcontext {
            format!("{}.", sctx.member_context_variable())
        } else {
            "".into()
        },
        typename.exact_size().unwrap()));
    writer.write_line("return value");
}
