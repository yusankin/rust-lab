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

#[cfg(test)]
mod main_test;
