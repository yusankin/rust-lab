// //リスト２−２　変数と型宣言を使い、整数を加算する
// fn main() {
//     let a = 10;
//     let b: i32 = 20;
//     let c = 30i32;
//     let d = 30_i32;
//     let e = add(add(a, b), add(c, d));

//     println!("(a+b) + (c+d) = {}", e);
// }

// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// //リスト２−３　Rustの数値リテラルと基本的な数値演算
// fn main() {
//     let twenty = 20;
//     let twenty_one: i32 = 21;
//     let twenty_two = 22i32;

//     let addition = twenty + twenty_one + twenty_two;
//     println!(
//         "{} + {} + {} = {}",
//         twenty, twenty_one, twenty_two, addition
//     );

//     let one_million: i64 = 1_000_000;
//     println!("{}", one_million.pow(2));

//     let forty_twos = [42.0, 42f32, 42.0_f32];

//     println!("{:02}", forty_twos[0]);
// }

// // リスト２−４　数値リテラルに２進数、８進数、１６進数を使う
// fn main() {
//     let three = 0b11;
//     let thirty = 0o36;
//     let three_hundred = 0x12C;

//     println!("base10: {} {} {}", three, thirty, three_hundred);
//     println!("base2: {:b} {:b} {:b}", three, thirty, three_hundred);
//     println!("base8: {:o} {:o} {:o}", three, thirty, three_hundred);
//     println!("base16: {:x} {:x} {:x}", three, thirty, three_hundred);
// }

// // リスト２−５　try_into()メソッドを使って型変換を行う
// use std::convert::TryInto;
// fn main() {
//     let a: i32 = 10;
//     let b: i16 = 100;

//     let b_ = b.try_into().unwrap();

//     if a < b_ {
//         println!("10は100より小さい。");
//     }
// }

// // リスト２−６
// use num::complex::Complex;

// fn main() {
//     let a = Complex { re: 2.1, im: -1.2 };
//     let b = Complex::new(11.1, 22.2);
//     let result = a + b;

//     println!("{}+{}i", result.re, result.im)
// }

// // リスト２−７　コンピュータがカウンタをインクリメントするスピードを試験
// use std::time::{Duration, Instant};

// fn main() {
//     let mut count = 1;
//     let time_limit = Duration::new(1, 0);
//     let start = Instant::now();
//     // 二つのInstantの減算はDurationを返す
//     while (Instant::now() - start) < time_limit {
//         count += 1;
//     }
//     println!("{}", count);
// }

// // リスト２−９　関数を定義する
// // add()は２つの定数型パラメータを受け取って１個の整数を返す
// // ２つの引数は、ローカル変数とiとjに束縛される
// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// // リスト２−１０　大きな配列への参照を作る
// fn main() {
//     let a = 42;
//     let r = &a;
//     let b = a + *r;

//     println!("a + a = {}", b);
// }

// // リスト２−１１　整数の配列から整数を探す
// fn main() {
//     let needle = 0o052;
//     let haystack = [1, 1, 2, 5, 15, 42, 52, 203, 877, 4140, 21147];

//     for item in &haystack {
//         if *item == needle {
//             println!("{}", item);
//         }
//     }
// }

// // リスト２−１２　マンデルブロ集合のレンダリング
// use num::complex::Complex;
// fn calculate_mandelbrot(
//     max_iters: usize,
//     x_min: f64,
//     x_max: f64,
//     y_min: f64,
//     y_max: f64,
//     width: usize,
//     height: usize,
// ) -> Vec<Vec<usize>> {
//     let mut rows: Vec<_> = Vec::with_capacity(height);

//     for img_y in 0..height {
//         let mut row: Vec<usize> = Vec::with_capacity(width);
//         for img_x in 0..width {
//             let x_percent = img_x as f64 / width as f64;
//             let y_percent = img_y as f64 / height as f64;
//             let cx = x_min + (x_max - x_min) * x_percent;
//             let cy = y_min + (y_max - y_min) * y_percent;
//             let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
//             row.push(escaped_at);
//         }
//         rows.push(row);
//     }
//     rows
// }

// fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
//     let mut z = Complex { re: 0.0, im: 0.0 };
//     let c = Complex::new(cx, cy);
//     for i in 0..=max_iters {
//         if z.norm() > 2.0 {
//             return i;
//         }
//         z = z * z + c;
//     }
//     return max_iters;
// }

// fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
//     for row in escape_vals {
//         let mut line = String::with_capacity(row.len());
//         for column in row {
//             let val = match column {
//                 0..=2 => " ",
//                 3..=5 => ".",
//                 6..=10 => "?",
//                 11..=30 => "*",
//                 31..=100 => "+",
//                 101..=200 => "x",
//                 201..=400 => "$",
//                 401..=700 => "#",
//                 _ => "%",
//             };
//             line.push_str(val);
//         }
//         println!("{}", line)
//     }
// }
// fn main() {
//     let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 80, 24);

//     render_mandelbrot(mandelbrot);
// }

// // リスト２−１３　明示的なライフタイム注釈をもつ関数シグネチャ
// // リスト２−１４　ライフタイムが明示的に注釈された関数のシグネチャ
// fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
//     *i + *j
// }
// fn main() {
//     let a = 10;
//     let b = 20;
//     let res = add_with_lifetimes(&a, &b);
//     println!("{}", res);
// }

// リスト２−１５　ジェネリック関数の型シグネチャ(コンパイル不可)
// 型関数Tが、山のカッコのペアと共に導入されている
// この関数は、同じ型の引数を二つ受け取り、その型の値を一つ返す
// fn add<T>(i: T, j: T) -> T {
//     i + j
// }

// // リスト２−１６　トレイト境界を持つジェネリック関数の型シグネチャ
// fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
//     i + j
// }

// // リスト２−１７　型変数とトレイト境界をもつジェネリック関数
// use std::ops::Add;
// use std::time::Duration;

// fn add<T: Add<Output = T>>(i: T, j: T) -> T {
//     i + j
// }

// fn main() {
//     let floats = add(1.2, 3.4);
//     let ints = add(10, 20);
//     let durations = add(Duration::new(5, 0), Duration::new(10, 0));
//     println!("{}", floats);
//     println!("{}", ints);
//     println!("{:?}", durations);
// }

// // リスト２−１８　数行の文字列から単純なパターンを検索
// fn main() {
//     let search_term = "picture";
//     let quote = "\
//     Every face, every shop, bedroom window, public-house, and
//     dark square is a picture feverishly turned--in search of what?
//     It is the same with books.
//     What do we seek through millions of pages?";
//     // lines()が返す引用文のイテレータで、各テキスト行が反復処理される
//     // 改行の解釈は実機OSの規約が使われる
//     for line in quote.lines() {
//         if line.contains(search_term) {
//             println!("{}", line);
//         }
//     }
// }

// // リスト２−１９　インデックス変数を手動でインクリメントする
// fn main() {
//     let search_term = "picture";
//     let quote = "\
//     Every face, every shop, bedroom window, public-house, and
//     dark square is a picture feverishly turned--in search of what?
//     It is the same with books.
//     What do we seek through millions of pages?";
//     let mut line_num: usize = 1;
//     for line in quote.lines() {
//         if line.contains(search_term) {
//             println!("{}:{}", line_num, line);
//         }
//         line_num += 1;
//     }
// }

// リスト２−２０　インデックス変数を自動でインクリメントする
fn main() {
    let search_term = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?";
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1; //行番号を求める。ステップごとに計算する必要はない。
            println!("{}:{}", line_num, line);
        }
    }
}

#[cfg(test)]
mod main_test;
