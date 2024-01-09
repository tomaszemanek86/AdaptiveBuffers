mod tests;

use super::*;

static TEST_DIR: &str = "src/generator/cpp/test/cpp_tests";
static THIS_DIR: &str = "src/generator/cpp/test";

fn compile_cpp(cpp_file: &str, object_file: &str) {
    let pwd = std::env::current_dir().unwrap().to_string_lossy().to_string();
    let basename = std::path::Path::new(cpp_file)
        .file_stem()
        .expect("could not extract stem")
        .to_str()
        .unwrap();
    let term_out = std::process::Command::new("g++")
        .args(&[
            &format!("-I{}", TEST_DIR),
            &format!("-I{}/{}", TEST_DIR, basename),
            &format!("-I{}/{}", pwd, THIS_DIR),
            "-std=c++20",
            "-c",
            &format!("{}/{}", TEST_DIR, cpp_file),
            "-o",
            &object_file,
        ])
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("compilation failed");
        
    println!("{}", String::from_utf8(term_out.stderr).unwrap());
}

fn generate_test(buffer_file: &str, test_file: &str, generate: bool) {
    let pwd = std::env::current_dir().unwrap().to_string_lossy().to_string();
    let test_file_noext = std::path::Path::new(test_file)
        .file_stem()
        .expect("could not extract stem")
        .to_str()
        .unwrap();

    let test_out_dir = format!("{}/{}/{}", pwd, TEST_DIR, test_file_noext);

    let _ = std::fs::remove_dir_all(&test_out_dir); // try remove folder

    let buffer_file_path = format!("{}/{}/{}", pwd, TEST_DIR, buffer_file);
    let source = std::fs::read_to_string(buffer_file_path)
        .expect("could not read file");

    if generate {
        generate_cpp(
            interpet_memory(source),
            &Args {
                protofile: buffer_file.into(),
                language: Language::Cpp,
                endian: "big".into(),
                output_dir: test_out_dir.clone(),
            },
        );
    }

    let object_file = format!("{}/{}.o", test_out_dir, test_file_noext);

    compile_cpp(
        &format!("{}.cpp", test_file_noext),
        &object_file
    );

    let out = format!("{}/{}", test_out_dir, test_file_noext);

    std::process::Command::new("g++")
        .args(&[&object_file,
                "-o",
                &out])
        .output()
        .expect("linking failed");

    let result = std::process::Command::new(&out)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .expect("test failed");

    let stdout = String::from_utf8(result.stdout).unwrap();
    let stderr = String::from_utf8(result.stderr).unwrap();

    println!("{}", stdout);
    println!("{}", stderr);

    assert!(result.status.success());
}
