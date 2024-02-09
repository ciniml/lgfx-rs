use std::{
    env,
    fs::File,
    io::Write,
    path::PathBuf,
};

const LGFX_FONTS: [&str; 186] = [
    "Font0",
    "Font2",
    "Font4",
    "Font6",
    "Font7",
    "Font8",
    "Font8x8C64",
    "AsciiFont8x16",
    "AsciiFont24x48",
    "TomThumb",
    "FreeMono9pt7b",
    "FreeMono12pt7b",
    "FreeMono18pt7b",
    "FreeMono24pt7b",
    "FreeMonoBold9pt7b",
    "FreeMonoBold12pt7b",
    "FreeMonoBold18pt7b",
    "FreeMonoBold24pt7b",
    "FreeMonoOblique9pt7b",
    "FreeMonoOblique12pt7b",
    "FreeMonoOblique18pt7b",
    "FreeMonoOblique24pt7b",
    "FreeMonoBoldOblique9pt7b",
    "FreeMonoBoldOblique12pt7b",
    "FreeMonoBoldOblique18pt7b",
    "FreeMonoBoldOblique24pt7b",
    "FreeSans9pt7b",
    "FreeSans12pt7b",
    "FreeSans18pt7b",
    "FreeSans24pt7b",
    "FreeSansBold9pt7b",
    "FreeSansBold12pt7b",
    "FreeSansBold18pt7b",
    "FreeSansBold24pt7b",
    "FreeSansOblique9pt7b",
    "FreeSansOblique12pt7b",
    "FreeSansOblique18pt7b",
    "FreeSansOblique24pt7b",
    "FreeSansBoldOblique9pt7b",
    "FreeSansBoldOblique12pt7b",
    "FreeSansBoldOblique18pt7b",
    "FreeSansBoldOblique24pt7b",
    "FreeSerif9pt7b",
    "FreeSerif12pt7b",
    "FreeSerif18pt7b",
    "FreeSerif24pt7b",
    "FreeSerifItalic9pt7b",
    "FreeSerifItalic12pt7b",
    "FreeSerifItalic18pt7b",
    "FreeSerifItalic24pt7b",
    "FreeSerifBold9pt7b",
    "FreeSerifBold12pt7b",
    "FreeSerifBold18pt7b",
    "FreeSerifBold24pt7b",
    "FreeSerifBoldItalic9pt7b",
    "FreeSerifBoldItalic12pt7b",
    "FreeSerifBoldItalic18pt7b",
    "FreeSerifBoldItalic24pt7b",
    "Orbitron_Light_24",
    "Orbitron_Light_32",
    "Roboto_Thin_24",
    "Satisfy_24",
    "Yellowtail_32",
    "DejaVu9",
    "DejaVu12",
    "DejaVu18",
    "DejaVu24",
    "DejaVu40",
    "DejaVu56",
    "DejaVu72",
    "lgfxJapanMincho_8",
    "lgfxJapanMincho_12",
    "lgfxJapanMincho_16",
    "lgfxJapanMincho_20",
    "lgfxJapanMincho_24",
    "lgfxJapanMincho_28",
    "lgfxJapanMincho_32",
    "lgfxJapanMincho_36",
    "lgfxJapanMincho_40",
    "lgfxJapanMinchoP_8",
    "lgfxJapanMinchoP_12",
    "lgfxJapanMinchoP_16",
    "lgfxJapanMinchoP_20",
    "lgfxJapanMinchoP_24",
    "lgfxJapanMinchoP_28",
    "lgfxJapanMinchoP_32",
    "lgfxJapanMinchoP_36",
    "lgfxJapanMinchoP_40",
    "lgfxJapanGothic_8",
    "lgfxJapanGothic_12",
    "lgfxJapanGothic_16",
    "lgfxJapanGothic_20",
    "lgfxJapanGothic_24",
    "lgfxJapanGothic_28",
    "lgfxJapanGothic_32",
    "lgfxJapanGothic_36",
    "lgfxJapanGothic_40",
    "lgfxJapanGothicP_8",
    "lgfxJapanGothicP_12",
    "lgfxJapanGothicP_16",
    "lgfxJapanGothicP_20",
    "lgfxJapanGothicP_24",
    "lgfxJapanGothicP_28",
    "lgfxJapanGothicP_32",
    "lgfxJapanGothicP_36",
    "lgfxJapanGothicP_40",
    "efontCN_10",
    "efontCN_10_b",
    "efontCN_10_bi",
    "efontCN_10_i",
    "efontCN_12",
    "efontCN_12_b",
    "efontCN_12_bi",
    "efontCN_12_i",
    "efontCN_14",
    "efontCN_14_b",
    "efontCN_14_bi",
    "efontCN_14_i",
    "efontCN_16",
    "efontCN_16_b",
    "efontCN_16_bi",
    "efontCN_16_i",
    "efontCN_24",
    "efontCN_24_b",
    "efontCN_24_bi",
    "efontCN_24_i",
    "efontJA_10",
    "efontJA_10_b",
    "efontJA_10_bi",
    "efontJA_10_i",
    "efontJA_12",
    "efontJA_12_b",
    "efontJA_12_bi",
    "efontJA_12_i",
    "efontJA_14",
    "efontJA_14_b",
    "efontJA_14_bi",
    "efontJA_14_i",
    "efontJA_16",
    "efontJA_16_b",
    "efontJA_16_bi",
    "efontJA_16_i",
    "efontJA_24",
    "efontJA_24_b",
    "efontJA_24_bi",
    "efontJA_24_i",
    "efontKR_10",
    "efontKR_10_b",
    "efontKR_10_bi",
    "efontKR_10_i",
    "efontKR_12",
    "efontKR_12_b",
    "efontKR_12_bi",
    "efontKR_12_i",
    "efontKR_14",
    "efontKR_14_b",
    "efontKR_14_bi",
    "efontKR_14_i",
    "efontKR_16",
    "efontKR_16_b",
    "efontKR_16_bi",
    "efontKR_16_i",
    "efontKR_24",
    "efontKR_24_b",
    "efontKR_24_bi",
    "efontKR_24_i",
    "efontTW_10",
    "efontTW_10_b",
    "efontTW_10_bi",
    "efontTW_10_i",
    "efontTW_12",
    "efontTW_12_b",
    "efontTW_12_bi",
    "efontTW_12_i",
    "efontTW_14",
    "efontTW_14_b",
    "efontTW_14_bi",
    "efontTW_14_i",
    "efontTW_16",
    "efontTW_16_b",
    "efontTW_16_bi",
    "efontTW_16_i",
    "efontTW_24",
    "efontTW_24_b",
    "efontTW_24_bi",
    "efontTW_24_i",
];

const LGFX_C_HEADER_PATH: &str = "lgfx_c/lgfx_c.h";
const LGFX_C_SOURCE_PATH: &str = "lgfx_c/lgfx_c.cpp";

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> anyhow::Result<()> {
    // Rebuild if LGFX C binding is changed.
    println!("cargo:rerun-if-changed={}", LGFX_C_HEADER_PATH);
    println!("cargo:rerun-if-changed={}", LGFX_C_SOURCE_PATH);

    // Get libclang version number to construct include path.
    clang_sys::load().expect("failed to load libclang");
    let clang_version = unsafe {
        let cxstring = clang_sys::clang_getClangVersion();
        let clang_version = std::ffi::CStr::from_ptr(clang_sys::clang_getCString(cxstring)).to_string_lossy().into_owned() ;
        clang_sys::clang_disposeString(cxstring);
        clang_version  
    };
    let clang_version_pattern = regex::Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let clang_version_number = clang_version_pattern.find(clang_version.as_str()).unwrap().as_str();
    let clang_version_number_major = clang_version_number.split('.').next().unwrap();
    eprintln!("libclang version: {}, major: {}", clang_version_number, clang_version_number_major);

    let libclang_path = env::var("LIBCLANG_PATH").expect("LIBCLANG_PATH must be set");
    let libclang_include_path: PathBuf = [libclang_path.as_str(), "clang", clang_version_number_major, "include"].iter().collect();
    let bindings = bindgen::Builder::default()
        .header(LGFX_C_HEADER_PATH)
        .clang_arg("-nostdinc")
        .clang_arg(["-I", libclang_include_path.to_str().unwrap()].concat())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate LGFX bindings");
    {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());        
        bindings
            .write_to_file(out_path.join("lgfx.rs"))
            .expect("Failed to write lgfx.rs");
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let output_path: PathBuf = [out_dir.as_str(), "lgfx_fonts.rs"].iter().collect();
    let mut file = File::create(output_path)?;

    // Generate LGFX font definitions.
    writeln!(&mut file, "mod lgfx_font_raw_defs {{")?;
    writeln!(&mut file, "extern \"C\" {{")?;
    for font_name in LGFX_FONTS {
        // Font definition is in lgfx::v1::fonts namespace, thus the font definition is mangled as _ZN4lgfx2v15fonts{font name length}{font name}E
        writeln!(&mut file, "#[allow(unused)] #[link_name = \"_ZN4lgfx2v15fonts{}{}E\"] pub static {}: core::ffi::c_void;", font_name.len(), font_name, font_name)?;
    }
    writeln!(&mut file, "}}")?;
    writeln!(&mut file, "}}")?;
    for font_name in LGFX_FONTS {
        writeln!(&mut file, "#[allow(unused)] #[allow(non_upper_case_globals)] pub static {}: LgfxFont = LgfxFont{{ ptr: unsafe {{ &lgfx_font_raw_defs::{} }} }};", font_name, font_name)?;
    }

    Ok(())
}
