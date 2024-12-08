use std::{fs::read_to_string, path::Path};

pub fn part_1() -> Option<i64> {
    //     let input_path: &Path = Path::new("./src/inputs/input25.txt");
    //     let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    //     //     let input = "1=-0-2
    //     // 12111
    //     // 2=0=
    //     // 21
    //     // 2=01
    //     // 111
    //     // 20012
    //     // 112
    //     // 1=-1=
    //     // 1-12
    //     // 12
    //     // 1=
    //     // 122";

    //     // SNAFU  Decimal
    //     // 1=-0-2     1747
    //     //  12111      906
    //     //   2=0=      198
    //     //     21       11
    //     //   2=01      201
    //     //    111       31
    //     //  20012     1257
    //     //    112       32
    //     //  1=-1=      353
    //     //   1-12      107
    //     //     12        7
    //     //     1=        3
    //     //    122       37

    //     let mut sum = 0;
    //     for line in input.lines() {
    //         let snafu = Snafu::new(line.to_string());
    //         let decimal = snafu.to_decimal();
    //         sum += decimal;
    //     }

    //     let d_sum = Snafu::from_decimal(sum);
    //     // println!("sum: {:?}", d_sum);

    //     let n = 25;
    //     // println!("to_base_5({n}): {}", to_base_5(n));
    //     let n = 24;
    //     // println!("to_base_5({n}): {}", to_base_5(n));

    //     for n in 0..100 {
    //         let xx = Snafu::from_decimal(n);
    //         // println!("{n}: {:?}", xx);
    //     }

    // Some(sum)
    None
}

pub fn part_2() -> Option<i64> {
    None
}

// #[derive(Debug)]
// struct Snafu {
//     snafu: String,
// }

// impl Snafu {
//     fn new(snafu: String) -> Self {
//         Self { snafu }
//     }

//     fn to_decimal(&self) -> i64 {
//         let mut decimal = 0;
//         for (i, snafu_digit) in self.snafu.chars().rev().enumerate() {
//             let decimal_digit = match snafu_digit {
//                 '0' => 0,
//                 '1' => 1,
//                 '2' => 2,
//                 '-' => -1,
//                 '=' => -2,
//                 _ => unreachable!(),
//             };

//             decimal += decimal_digit * 5i64.pow(i as u32);
//         }
//         decimal
//     }

//     // let xx = match c {
//     //     '0' => "0",
//     //     '1' => "1",
//     //     '2' => "2",
//     //     '3' => "1=",
//     //     '4' => "1-",
//     //     _ => unreachable!(),
//     // };

//     fn from_decimal(decimal: i64) -> Self {
//         let decimal_b5 = to_base_5(decimal);
//         let s = decimal_b5.replace("3", "1-").replace("4", "1=");
//         Self::new(s)
//     }
// }

// fn to_base_5(decimal: i64) -> String {
//     let mut decimal = decimal;
//     let mut base_5 = String::new();
//     while decimal > 0 {
//         let digit = decimal % 5;
//         decimal /= 5;
//         base_5.push_str(&digit.to_string());
//     }

//     // TODO: make this more elegant
//     base_5.chars().rev().collect()
// }
