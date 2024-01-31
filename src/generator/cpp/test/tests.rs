use super::*;

#[test]
fn view_with_natives() {
    generate_test("view_with_natives.abf", "view_with_natives.cpp", true);
}

#[test]
fn struct_with_natives() {
    generate_test("struct_with_natives.abf", "struct_with_natives.cpp", true);
}

#[test]
fn struct_with_views() {
    generate_test("struct_with_views.abf", "struct_with_views.cpp", true);
}

#[test]
fn struct_with_array_of_natives() {
    generate_test("struct_with_array_of_natives.abf", "struct_with_array_of_natives.cpp", true);
}

#[test]
fn struct_with_arrays_of_views() {
    generate_test("struct_with_arrays_of_views.abf", "struct_with_arrays_of_views.cpp", true);
}

#[test]
fn struct_with_sized_array_of_natives() {
    generate_test("struct_with_sized_array_of_natives.abf", "struct_with_sized_array_of_natives.cpp", true);
}

#[test]
fn struct_with_reference_view() {
    generate_test("struct_with_reference_view.abf", "struct_with_reference_view.cpp", true);
}

#[test]
fn struct_with_constant() {
    generate_test("struct_with_constant.abf", "struct_with_constant.cpp", true);
}

#[test]
fn enum1() {
    generate_test("enum1.abf", "enum1.cpp", true);
}

#[test]
fn d3sb() {
    generate_test("d3sb.abf", "d3sb.cpp", true);
}

#[test]
fn bswap() {
    generate_test("no_type.abf", "bswap.cpp", true);
}

