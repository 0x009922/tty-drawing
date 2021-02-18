// use std::collections::HashMap;

/// Три ряда по две колонки. Есть пиксель / нет пикселя
#[derive(Debug)]
pub struct AsciiMatrix(bool, bool, bool, bool, bool, bool);

impl AsciiMatrix {
    /// на входе - число до 64. Если будет больше - отрежется
    pub fn from_num(num: u8) -> Self {
        // if num >= 64 {
        //     panic!("received num: {}", num);
        // }

        let digits: [u8; 6] = [
            num & 0b1,
            (num & 0b10) >> 1,
            (num & 0b100) >> 2,
            (num & 0b1000) >> 3,
            (num & 0b10000) >> 4,
            (num >> 5) & 0b1,
        ];

        // разделяю число по разрядам.
        Self(
            num_to_bool(digits[0]),
            num_to_bool(digits[1]),
            num_to_bool(digits[2]),
            num_to_bool(digits[3]),
            num_to_bool(digits[4]),
            num_to_bool(digits[5]),
        )
    }

    // fn to_ascii(&self) -> char {}
}

pub fn num_to_char(num: u8) -> char {
    if num == 0 {
        // Пустота
        ' '
    } else if num % 64 == 63 {
        // Полнота
        '\u{1fb8b}'
    } else {
        // Нечто промежуточное
        // разложим наше число на биты (6 битов) - x x x x x x
        // чтобы перевести это в символ, надо помнить, что

        // список разрядных чисел в обратном порядке
        let digits: [u8; 6] = [
            (num >> 5) & 0b1,
            (num & 0b10000) >> 4,
            (num & 0b1000) >> 3,
            (num & 0b100) >> 2,
            (num & 0b10) >> 1,
            num & 0b1,
        ];

        const CODE_START: u32 = 0x1fb00;

        let code_shift = digits.iter().fold(0, |acc, digit| (acc << 1) | digit);

        // println!("shift from {} to {}", num, code_shift);

        char::from_u32(CODE_START + code_shift as u32 - 1).unwrap()

        // let code_shift =
        // ' '
        // CODE_START as char
    }
}

fn num_to_bool(num: u8) -> bool {
    num > 0
}

// /// Маппер матрицы
// pub struct AsciiMapper {
//     precomputed: HashMap<AsciiMatrix, char>
// }

// impl AsciiMapper {
//     pub fn new() -> Self {
//         // вычисляю всё заранее
//         // благо, все эти символы уложены в строгой закономерности в юникоде

//         // всего доступно
//     }

//     /// Преобразование данной матрицы к одному из BoxDrawingCharacters
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// let mapper = AsciiMapper::new();
//     /// let matrix = AsciiMatrix(true, true, false, false, false, false);
//     /// assert_eq!(mapper.to_ascii(&matrix), '🬂')
//     /// ```
//     pub fn to_ascii(&self, matrix: &AsciiMatrix) -> char {}
// }
