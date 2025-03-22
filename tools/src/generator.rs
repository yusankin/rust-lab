use std::fs;
use std::path::Path;

pub fn generate(category: &str, name: &str) {
    let base = format!("../codes/{}/{}", category, name);
    let src_dir = format!("{}/src", base);
    let md_path = format!("{}/README.md", base);
    let toml_path = format!("{}/Cargo.toml", base);
    let lib_path = format!("{}/src/lib.rs", base);
    let test_path = format!("{}/src/lib_test.rs", base);

    // ディレクトリ作成
    fs::create_dir_all(&src_dir).unwrap();

    // Cargo.toml
    fs::write(&toml_path, format!(
        "[package]
name = \"{}_{}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
", category, name)).unwrap();

    // lib.rs
    fs::write(&lib_path, "// 実装\n").unwrap();

    // lib_test.rs
    fs::write(&test_path, r#"#[cfg(test)]
mod tests {
    #[test]
    fn sample_test() {
        assert_eq!(1 + 1, 2);
    }
}
"#).unwrap();

    // README.md
    fs::write(&md_path, format!(
        "# {}/{}\n\n## 入出力例\n\n- 入力:\n- 出力:\n\n## 設計・考慮点\n\n- [] 境界値\n- [] 実行時間\n\n## 気づき・学び\n", category, name)).unwrap();

    println!("✅ {}/{} を作成しました", category, name);
}
