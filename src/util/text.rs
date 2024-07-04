// use rust_decimal::prelude::*;

// pub fn capitalize_first_letter(s: &str) -> String {
//     let s = s.replace('_', " ");
//     let mut chars = s.chars();
//     match chars.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
//     }
// }

pub fn capitalize_and_replace(s: &str) -> String {
    s.replace('_', " ")
        .split_whitespace()
        .map(|word| {
            word.chars()
                .next()
                .map(|c| {
                    c.to_uppercase().collect::<String>() + word.get(c.len_utf8()..).unwrap_or("")
                })
                .unwrap_or_default()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// pub fn humanize_decimal(number: &Decimal) -> String {
//     if number.is_zero() {
//         return "0".to_string();
//     }

//     let mut result = String::new();
//     let mut count = 0;
//     let is_negative = number.is_sign_negative();
//     let mut num = number.abs();

//     while num >= Decimal::ONE {
//         if count % 3 == 0 && count > 0 {
//             result.insert(0, ',');
//         }
//         let digit = (&num % Decimal::from(10)).to_u32().unwrap() as u8;
//         result.insert(0, char::from_digit(digit as u32, 10).unwrap());
//         num = &num / Decimal::from(10);
//         count += 1;
//     }

//     if is_negative {
//         result.insert(0, '-');
//     }
//     result
// }
