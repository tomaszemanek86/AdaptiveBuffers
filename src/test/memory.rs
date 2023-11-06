use super::*;

#[test]
fn memory_of_struct() {
    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
    struct Test {
        packet_size: u8 = $size,
        member_u8: u8,
        member_u16: u16
    }",
    );

    let memory = generate_memory(source);

    assert_eq!(memory.len(), 1);
    assert!(memory.iter().any(|it| it.name == "Test"));
    assert!(memory
        .iter()
        .find(|it| it.name == "Test")
        .unwrap()
        .memory
        .memory
        .is_struct());
    let structure = memory
        .iter()
        .find(|it| it.name == "Test")
        .unwrap()
        .memory
        .memory
        .as_struct()
        .unwrap();
    assert_eq!(structure.name, "Test");
    assert!(structure.fields.iter().any(|it| it.name == "member_u8"));
    assert!(structure.fields.iter().any(|it| it.name == "member_u16"));
}

#[test]
fn memory_of_2_structs() {
    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
    struct Struct1 {
        packte_size: u8 = $size,
        s1_member_u8: u8,
        s2: Struct2
    }
    
    struct Struct2 {
        s2_member_u8: u8
    }",
    );

    let memory = generate_memory(source);

    assert_eq!(memory.len(), 2);
    assert_eq!(memory.iter().filter(|it| it.memory.memory.is_struct()).count(), 2);
    assert_eq!(
        memory
            .iter()
            .find(|it| it.name == "Struct1")
            .unwrap()
            .memory
            .memory
            .as_struct()
            .unwrap()
            .name,
        "Struct1"
    );
    assert_eq!(
        memory
            .iter()
            .find(|it| it.name == "Struct2")
            .unwrap()
            .memory
            .memory
            .as_struct()
            .unwrap()
            .name,
        "Struct2"
    );
}

#[test]
fn memory_of_enum() {
    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
    enum AnEnum : u8 {
        EvnumValue1 = 1,
        EvnumValue2 = 2,
        EvnumValue3 = 3
    }",
    );

    let memory = generate_memory(source);

    assert_eq!(memory.len(), 1);
    assert!(memory.iter().any(|it| it.name == "AnEnum"));
    assert!(memory
        .iter()
        .find(|it| it.name == "AnEnum")
        .unwrap()
        .memory
        .memory
        .is_enum());
    let enum_memory = memory
        .iter()
        .find(|it| it.name == "AnEnum")
        .unwrap()
        .memory
        .memory
        .as_enum()
        .unwrap();
    assert_eq!(enum_memory.name, "AnEnum");
    assert!(enum_memory.underlaying_type.is_u8());
    assert_eq!(enum_memory.constants.len(), 3);
    assert!(enum_memory
        .constants
        .iter()
        .any(|it| it.name == "EvnumValue1"));
    assert!(enum_memory
        .constants
        .iter()
        .any(|it| it.name == "EvnumValue2"));
    assert!(enum_memory
        .constants
        .iter()
        .any(|it| it.name == "EvnumValue3"));
}

#[test]
fn memory_of_view() {
    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
    view AnView {
        u8, i16
    }",
    );

    let memory = generate_memory(source);

    assert_eq!(memory.len(), 1);
    assert!(memory.iter().any(|it| it.name == "AnView"));
    assert!(memory
        .iter()
        .find(|it| it.name == "AnView")
        .unwrap()
        .memory
        .memory
        .is_view());
    let view_memory = memory
        .iter()
        .find(|it| it.name == "AnView")
        .unwrap()
        .memory
        .memory
        .as_view()
        .unwrap();
    assert_eq!(view_memory.name, "AnView");
    assert_eq!(view_memory.types.len(), 2);
    assert!(view_memory.types[0].memory.is_native());
    assert!(view_memory.types[1].memory.is_native());
    let type0 = view_memory.types[0].memory.as_native().unwrap();
    assert!(type0.is_u8());
    let type1 = view_memory.types[1].memory.as_native().unwrap();
    assert!(type1.is_i16());
}
