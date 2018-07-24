use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

const BINDINGS_INPUT: &'static str = "qt/bindings.json";
const BINDINGS_OUTPUT: &'static str = "qt/src/Bindings.h";
const MOC_OUTPUT: &'static str = "qt/src/moc_Bindings.cpp";

const GENERATED_INTERFACE: &'static str = "qt/rust/src/interface.rs";
const GENERATED_IMPL: &'static str = "qt/rust/src/implementation.rs";

const TARGET_INTERFACE: &'static str = "src/interface.rs";
const TARGET_IMPL: &'static str = "src/implementation.rs";

fn main() {
    println!("Running build.rs");
    if bindings_changed().unwrap() {
        println!("Regenerating bindings based on {}", &BINDINGS_INPUT);
        generate_bindings().unwrap();
    }
    println!("build.rs complete");
}

fn bindings_changed() -> Result<bool, io::Error> {
    let input =  Path::new(&BINDINGS_INPUT);
    let output = Path::new(&MOC_OUTPUT);

    if !output.exists() {
        println!("Output {} does not exist", &MOC_OUTPUT);
        return Ok(true);
    }

    let input_modified = input.metadata()?.modified()?;
    let output_modified = output.metadata()?.modified()?;

    Ok(input_modified > output_modified)
}

fn generate_bindings() -> Result<(), io::Error> {
    fs::create_dir_all("qt/rust/src")?; // required or generator will error
    let generator = Command::new(get_binding_generator())
        .args(&[BINDINGS_INPUT])
        .output()?;
    println!("generator stdout: {}", String::from_utf8_lossy(&generator.stdout));
    println!("generator stderr: {}", String::from_utf8_lossy(&generator.stderr));

    // generator creates two files
    //   - interface.rs: the generated trait. We always want the updated version copied to /src
    //   - implementation.rs: a sample implementation of the trait, we only want this if the user
    //     doesn't yet have one.
    println!("copying generated output");
    fs::rename(&GENERATED_INTERFACE, &TARGET_INTERFACE)?;
    if !Path::new(&TARGET_IMPL).exists() {
        fs::rename(&GENERATED_IMPL, &TARGET_IMPL)?;
    } else {
        fs::remove_file(&GENERATED_IMPL)?;
    }
    fs::remove_dir("qt/rust/src")?;
    fs::remove_dir("qt/rust")?;

    // qt's moc command for required codegen, assumed to be on the path
    println!("invoking moc");
    let moc = Command::new("moc")
        .args(&[BINDINGS_OUTPUT, "-o", MOC_OUTPUT])
        .output()?;
    println!("moc stdout: {}", String::from_utf8_lossy(&moc.stdout));
    println!("moc stderr: {}", String::from_utf8_lossy(&moc.stderr));

    Ok(())
}

#[cfg(target_os="windows")]
fn get_binding_generator() -> &'static str {
    println!("using windows version of rust_qt_binding_generator");
    "tools/rust_qt_binding_generator/win/rust_qt_binding_generator.exe"
}

#[cfg(target_os="macos")]
fn get_binding_generator() -> &'static str {
    println!("using mac os version of rust_qt_binding_generator");
    "tools/rust_qt_binding_generator/mac/rust_qt_binding_generator"
}
