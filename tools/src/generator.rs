use std::fs;

pub fn generate(category: &str, name: &str, is_bin: bool) {
    let base = format!("../codes/{}/{}", category, name);
    let src_dir = format!("{}/src", base);
    let md_path = format!("{}/README.md", base);
    let toml_path = format!("{}/Cargo.toml", base);
    let crate_name = name.to_string();

    fs::create_dir_all(&src_dir).unwrap();

    // Cargo.toml
    let cargo_toml = format!(
        "[package]
name = \"{crate_name}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
"
    );
    fs::write(&toml_path, cargo_toml).unwrap();

    // README.md
    fs::write(&md_path, format!(
        "# {}/{}\n\n## 入出力例\n\n- 入力:\n- 出力:\n\n## 設計・考慮点\n\n- [] 境界値\n- [] 実行時間\n\n## 気づき・学び\n",
        category, name
    )).unwrap();

    if is_bin {
        // main.rs
        let main_path = format!("{}/main.rs", src_dir);
        fs::write(
            &main_path,
            r#"fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod main_test;
"#,
        )
        .unwrap();

        // main_test.rs
        let test_path = format!("{}/main_test.rs", src_dir);
        fs::write(
            &test_path,
            r#"#[cfg(test)]
mod tests {
    //#[test]
    //fn sample_test() {
    //    assert_eq!(2 + 2, 4);
    //}
}
"#,
        )
        .unwrap();
    } else {
        // lib.rs
        let lib_path = format!("{}/lib.rs", src_dir);
        fs::write(&lib_path, "// 実装\n").unwrap();

        // lib_test.rs
        let test_path = format!("{}/lib_test.rs", src_dir);
        fs::write(
            &test_path,
            r#"#[cfg(test)]
mod tests {
    //#[test]
    //fn sample_test() {
    //    assert_eq!(1 + 1, 2);
    //}
}
"#,
        )
        .unwrap();
    }

    println!(
        "✅ {}/{} を作成しました（--bin: {}）",
        category, name, is_bin
    );
}
