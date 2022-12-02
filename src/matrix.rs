use crate::exporters::excalidraw::{Drawable, Element, ExcalidrawFile};
use std::fmt::Display;

pub mod with_result {
    use crate::exporters::excalidraw::{Drawable, Element, ExcalidrawFile};
    use std::{
        fmt::{Debug, Display},
        ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
    };

    /// A square matrix with arbitrary size and a result column
    ///
    /// **Indexing is 1-based**
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct LineMatrix<const SIZE: usize>([Line<SIZE>; SIZE]);

    impl<const SIZE: usize> LineMatrix<SIZE> {
        pub fn new(matrix: [[f64; SIZE]; SIZE], result: [f64; SIZE]) -> Self {
            let mut lines = [Line::new(matrix[0], result[0]); SIZE];
            for i in 0..SIZE {
                lines[i] = Line::new(matrix[i], result[i]);
            }
            Self(lines)
        }

        pub fn mul(mut self, line: usize, value: f64) -> Self {
            if !(1..=SIZE).contains(&line) {
                panic!("Invalid line");
            }
            self.0[line - 1] *= value;
            self
        }

        pub fn div(mut self, line: usize, value: f64) -> Self {
            if !(1..=SIZE).contains(&line) {
                panic!("Invalid line");
            }
            self.0[line - 1] /= value;
            self
        }

        pub fn add_to(mut self, line: usize, target_line: usize) -> Self {
            if !(1..=SIZE).contains(&line) {
                panic!("Invalid line");
            }
            if !(1..=SIZE).contains(&target_line) {
                panic!("Invalid target line");
            }
            self.0[target_line - 1] += self.0[line - 1];
            self
        }

        pub fn sub_to(mut self, line: usize, target_line: usize) -> Self {
            if !(1..=SIZE).contains(&line) {
                panic!("Invalid line");
            }
            if !(1..=SIZE).contains(&target_line) {
                panic!("Invalid target line");
            }
            self.0[target_line - 1] -= self.0[line - 1];
            self
        }

        pub fn mul_to(mut self, line: usize, target_line: usize, value: f64) -> Self {
            if !(1..=SIZE).contains(&line) {
                panic!("Invalid line");
            }
            if !(1..=SIZE).contains(&target_line) {
                panic!("Invalid target line");
            }
            self.0[target_line - 1] += self.0[line - 1] * value;
            self
        }

        pub fn div_to(mut self, line: usize, target_line: usize, value: f64) -> Self {
            if !(1..=SIZE).contains(&line) {
                panic!("Invalid line");
            }
            if !(1..=SIZE).contains(&target_line) {
                panic!("Invalid target line");
            }
            self.0[target_line - 1] += self.0[line - 1] / value;
            self
        }
    }

    impl<const SIZE: usize> Index<usize> for LineMatrix<SIZE> {
        type Output = Line<SIZE>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index - 1]
        }
    }

    impl<const SIZE: usize> IndexMut<usize> for LineMatrix<SIZE> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[index - 1]
        }
    }

    impl<const SIZE: usize> Debug for LineMatrix<SIZE> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&self, f)
        }
    }

    impl<const SIZE: usize> Display for LineMatrix<SIZE> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let precision = f.precision().unwrap_or(3);
            let mut col_widths = [0; SIZE];
            let mut strings = vec![Vec::with_capacity(SIZE); SIZE];
            for col in 0..SIZE {
                let mut col_width = 0;
                for row in 0..SIZE {
                    strings[row].push({
                        let res_string = format!("{:.1$}", self.0[row].0[col], precision);
                        let res = res_string
                            .as_str()
                            .trim_end_matches('0')
                            .trim_end_matches('.');
                        if res == "-0" {
                            String::from("0")
                        } else {
                            res.to_string()
                        }
                    });
                    let len = strings[row][col].len();
                    if len > col_width {
                        col_width = len;
                    }
                }
                col_widths[col] = col_width;
            }
            let mut res_strings = Vec::with_capacity(SIZE);
            let mut res_width = 0;
            for i in 0..SIZE {
                res_strings.push({
                    let res_string = format!("{:.1$}", self.0[i].1, precision);
                    let res = res_string
                        .as_str()
                        .trim_end_matches('0')
                        .trim_end_matches('.');
                    if res == "-0" {
                        String::from("0")
                    } else {
                        res.to_string()
                    }
                });
                let len = res_strings[i].len();
                if len > res_width {
                    res_width = len;
                }
            }
            let width = col_widths.iter().sum::<usize>() + res_width + 2;
            f.write_fmt(format_args!("\n┌╴{:w$}╶┐\n", "", w = width))?;
            for row in 0..SIZE {
                f.write_str("│")?;
                for col in 0..SIZE {
                    let cw = col_widths[col];
                    f.write_fmt(format_args!(" {:>cw$}", strings[row][col], cw = cw))?;
                }
                f.write_fmt(format_args!("│{:>rw$} ", res_strings[row], rw = res_width))?;
                f.write_str("│\n")?;
            }
            f.write_fmt(format_args!("└╴{:w$}╶┘", "", w = width))
        }
    }

    impl<const SIZE: usize> Drawable for LineMatrix<SIZE> {
        fn draw(&self, file: &mut ExcalidrawFile, x: i32, y: i32, locked: bool) -> (i32, i32) {
            let precision = 3;
            let scale = file.app_state.grid_size;
            file.elements.push(Element::simple_line(
                x,
                y,
                locked,
                vec![
                    [scale, 0],
                    [0, 0],
                    [0, scale * (SIZE as i32 * 2 + 1)],
                    [scale, scale * (SIZE as i32 * 2 + 1)],
                ],
            ));
            let mut col_widths = [0; SIZE];
            let mut strings = vec![Vec::with_capacity(SIZE); SIZE];
            for col in 0..SIZE {
                let mut col_width = 0;
                for row in 0..SIZE {
                    strings[row].push({
                        let res_string = format!("{:.1$}", self.0[row].0[col], precision);
                        let res = res_string
                            .as_str()
                            .trim_end_matches('0')
                            .trim_end_matches('.');
                        if res == "-0" {
                            String::from("0")
                        } else {
                            res.to_string()
                        }
                    });
                    let len = strings[row][col].len();
                    if len > col_width {
                        col_width = len;
                    }
                }
                col_widths[col] = col_width;
            }
            let mut res_strings = Vec::with_capacity(SIZE);
            let mut res_width = 0;
            for i in 0..SIZE {
                res_strings.push({
                    let res_string = format!("{:.1$}", self.0[i].1, precision);
                    let res = res_string
                        .as_str()
                        .trim_end_matches('0')
                        .trim_end_matches('.');
                    if res == "-0" {
                        String::from("0")
                    } else {
                        res.to_string()
                    }
                });
                let len = res_strings[i].len() * 10;
                if len > res_width {
                    res_width = len;
                }
            }
            let total_col_width: i32 =
                col_widths.iter().sum::<usize>() as i32 * 10 + (SIZE as i32 - 1) * scale;
            let mut current_offset = 0;
            for col in 0..SIZE {
                for row in 0..SIZE {
                    file.elements.push(Element::draw_small_monospaced_text(
                        x + scale + current_offset,
                        y + scale + scale * 2 * row as i32,
                        locked,
                        strings[row][col].clone(),
                    ));
                }
                current_offset += col_widths[col] as i32 * 10 + scale;
            }
            file.elements.push(Element::simple_line(
                x + scale * 2 + total_col_width,
                y + scale,
                locked,
                vec![[0, 0], [0, scale * (SIZE as i32 * 2 - 1)]],
            ));
            for i in 0..SIZE {
                file.elements.push(Element::draw_small_monospaced_text(
                    x + scale * 3 + total_col_width,
                    y + scale + scale * 2 * i as i32,
                    locked,
                    res_strings[i].clone(),
                ));
            }
            file.elements.push(Element::simple_line(
                x + scale * 3 + total_col_width + res_width as i32,
                y,
                locked,
                vec![
                    [0, 0],
                    [scale, 0],
                    [scale, scale * (SIZE as i32 * 2 + 1)],
                    [0, scale * (SIZE as i32 * 2 + 1)],
                ],
            ));
            (
                scale * 4 + total_col_width + res_width as i32,
                scale * (SIZE as i32 * 2 + 1),
            )
        }
    }

    #[derive(Clone, Copy)]
    pub struct Line<const SIZE: usize>([f64; SIZE], f64);

    impl<const SIZE: usize> Line<SIZE> {
        pub fn new(line: [f64; SIZE], result: f64) -> Self {
            Self(line, result)
        }
    }

    impl<const SIZE: usize> Add for Line<SIZE> {
        type Output = Self;

        fn add(mut self, rhs: Self) -> Self::Output {
            for i in 0..SIZE {
                self.0[i] += rhs.0[i];
            }
            self.1 += rhs.1;
            self
        }
    }

    impl<const SIZE: usize> Sub for Line<SIZE> {
        type Output = Self;

        fn sub(mut self, rhs: Self) -> Self::Output {
            for i in 0..SIZE {
                self.0[i] -= rhs.0[i];
            }
            self.1 -= rhs.1;
            self
        }
    }

    impl<const SIZE: usize> Mul<f64> for Line<SIZE> {
        type Output = Self;

        fn mul(mut self, rhs: f64) -> Self::Output {
            for i in 0..SIZE {
                self.0[i] *= rhs;
            }
            self.1 *= rhs;
            self
        }
    }

    impl<const SIZE: usize> Div<f64> for Line<SIZE> {
        type Output = Self;

        fn div(mut self, rhs: f64) -> Self::Output {
            for i in 0..SIZE {
                self.0[i] /= rhs;
            }
            self.1 /= rhs;
            self
        }
    }

    impl<const SIZE: usize> AddAssign for Line<SIZE> {
        fn add_assign(&mut self, rhs: Self) {
            let result = self.clone() + rhs;
            self.0 = result.0;
            self.1 = result.1;
        }
    }

    impl<const SIZE: usize> SubAssign for Line<SIZE> {
        fn sub_assign(&mut self, rhs: Self) {
            let result = self.clone() - rhs;
            self.0 = result.0;
            self.1 = result.1;
        }
    }

    impl<const SIZE: usize> MulAssign<f64> for Line<SIZE> {
        fn mul_assign(&mut self, rhs: f64) {
            let result = self.clone() * rhs;
            self.0 = result.0;
            self.1 = result.1;
        }
    }

    impl<const SIZE: usize> DivAssign<f64> for Line<SIZE> {
        fn div_assign(&mut self, rhs: f64) {
            let result = self.clone() / rhs;
            self.0 = result.0;
            self.1 = result.1;
        }
    }
}

pub struct Matrix2x2([[f64; 2]; 2], [f64; 2]);

impl Matrix2x2 {
    pub fn new(value: [[f64; 2]; 2], result: [f64; 2]) -> Self {
        Self(value, result)
    }

    pub fn mul(&mut self, row: usize, value: f64) -> &mut Self {
        self.0[row][0] *= value;
        self.0[row][1] *= value;
        self.1[row] *= value;
        self
    }

    pub fn div(&mut self, row: usize, value: f64) -> &mut Self {
        self.0[row][0] /= value;
        self.0[row][1] /= value;
        self.1[row] /= value;
        self
    }

    pub fn add_to(&mut self, row: usize, target_row: usize) -> &mut Self {
        self.0[target_row][0] += self.0[row][0];
        self.0[target_row][1] += self.0[row][1];
        self.1[target_row] += self.1[row];
        self
    }

    pub fn sub_to(&mut self, row: usize, target_row: usize) -> &mut Self {
        self.0[target_row][0] -= self.0[row][0];
        self.0[target_row][1] -= self.0[row][1];
        self.1[target_row] -= self.1[row];
        self
    }

    pub fn mul_to(&mut self, row: usize, value: f64, target_row: usize) -> &mut Self {
        self.0[target_row][0] += self.0[row][0] * value;
        self.0[target_row][1] += self.0[row][1] * value;
        self.1[target_row] += self.1[row] * value;
        self
    }

    pub fn div_to(&mut self, row: usize, value: f64, target_row: usize) -> &mut Self {
        self.0[target_row][0] += self.0[row][0] / value;
        self.0[target_row][1] += self.0[row][1] / value;
        self.1[target_row] += self.1[row] / value;
        self
    }
}

impl Display for Matrix2x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SIZE: usize = 2;
        let precision = f.precision().unwrap_or(3);
        let mut col_widths = [0; SIZE];
        let mut strings = vec![Vec::with_capacity(SIZE); SIZE];
        for col in 0..SIZE {
            let mut col_width = 0;
            for row in 0..SIZE {
                strings[row].push({
                    let res_string = format!("{:.1$}", self.0[row][col], precision);
                    let res = res_string
                        .as_str()
                        .trim_end_matches('0')
                        .trim_end_matches('.');
                    if res == "-0" {
                        String::from("0")
                    } else {
                        res.to_string()
                    }
                });
                let len = strings[row][col].len();
                if len > col_width {
                    col_width = len;
                }
            }
            col_widths[col] = col_width;
        }
        let mut res_strings = Vec::with_capacity(SIZE);
        let mut res_width = 0;
        for i in 0..SIZE {
            res_strings.push({
                let res_string = format!("{:.1$}", self.1[i], precision);
                let res = res_string
                    .as_str()
                    .trim_end_matches('0')
                    .trim_end_matches('.');
                if res == "-0" {
                    String::from("0")
                } else {
                    res.to_string()
                }
            });
            let len = res_strings[i].len();
            if len > res_width {
                res_width = len;
            }
        }
        let width = sum(&col_widths) + res_width + 2;
        f.write_fmt(format_args!("\n┌╴{:w$}╶┐\n", "", w = width))?;
        for row in 0..SIZE {
            f.write_str("│")?;
            for col in 0..SIZE {
                let cw = col_widths[col];
                f.write_fmt(format_args!(" {:>cw$}", strings[row][col], cw = cw))?;
            }
            f.write_fmt(format_args!("│{:>rw$} ", res_strings[row], rw = res_width))?;
            f.write_str("│\n")?;
        }
        f.write_fmt(format_args!("└╴{:w$}╶┘", "", w = width))
    }
}

impl Drawable for Matrix2x2 {
    fn draw(&self, file: &mut ExcalidrawFile, x: i32, y: i32, locked: bool) -> (i32, i32) {
        const SIZE: usize = 2;
        let precision = 3;
        let scale = file.app_state.grid_size;
        file.elements.push(Element::simple_line(
            x,
            y,
            locked,
            vec![
                [scale, 0],
                [0, 0],
                [0, scale * (SIZE as i32 * 2 + 1)],
                [scale, scale * (SIZE as i32 * 2 + 1)],
            ],
        ));
        let mut col_widths = [0; SIZE];
        let mut strings = vec![Vec::with_capacity(SIZE); SIZE];
        for col in 0..SIZE {
            let mut col_width = 0;
            for row in 0..SIZE {
                strings[row].push({
                    let res_string = format!("{:.1$}", self.0[row][col], precision);
                    let res = res_string
                        .as_str()
                        .trim_end_matches('0')
                        .trim_end_matches('.');
                    if res == "-0" {
                        String::from("0")
                    } else {
                        res.to_string()
                    }
                });
                let len = strings[row][col].len();
                if len > col_width {
                    col_width = len;
                }
            }
            col_widths[col] = col_width;
        }
        let mut res_strings = Vec::with_capacity(SIZE);
        let mut res_width = 0;
        for i in 0..SIZE {
            res_strings.push({
                let res_string = format!("{:.1$}", self.1[i], precision);
                let res = res_string
                    .as_str()
                    .trim_end_matches('0')
                    .trim_end_matches('.');
                if res == "-0" {
                    String::from("0")
                } else {
                    res.to_string()
                }
            });
            let len = res_strings[i].len();
            if len > res_width {
                res_width = len;
            }
        }
        let total_col_width: i32 =
            col_widths.iter().sum::<usize>() as i32 * 10 + (SIZE as i32 - 1) * scale;
        let mut current_offset = 0;
        for col in 0..SIZE {
            for row in 0..SIZE {
                file.elements.push(Element::draw_small_monospaced_text(
                    x + scale + current_offset,
                    y + scale + scale * 2 * row as i32,
                    locked,
                    strings[row][col].clone(),
                ));
            }
            current_offset += col_widths[col] as i32 * 10 + scale;
        }
        file.elements.push(Element::simple_line(
            x + scale * 2 + total_col_width,
            y + scale,
            locked,
            vec![[0, 0], [0, scale * (SIZE as i32 * 2 - 1)]],
        ));
        for i in 0..SIZE {
            file.elements.push(Element::draw_small_monospaced_text(
                x + scale * 3 + total_col_width,
                y + scale + scale * 2 * i as i32,
                locked,
                res_strings[i].clone(),
            ));
        }
        file.elements.push(Element::simple_line(
            x + scale * 3 + total_col_width + res_width as i32 * 10,
            y,
            locked,
            vec![
                [0, 0],
                [scale, 0],
                [scale, scale * (SIZE as i32 * 2 + 1)],
                [0, scale * (SIZE as i32 * 2 + 1)],
            ],
        ));
        (
            scale * 4 + total_col_width + res_width as i32,
            scale * (SIZE as i32 * 2 + 1),
        )
    }
}

pub struct Matrix3x3([[f64; 3]; 3], [f64; 3]);

impl Matrix3x3 {
    pub fn new(value: [[f64; 3]; 3], result: [f64; 3]) -> Self {
        Self(value, result)
    }

    pub fn mul(&mut self, row: usize, value: f64) -> &mut Self {
        self.0[row][0] *= value;
        self.0[row][1] *= value;
        self.0[row][2] *= value;
        self.1[row] *= value;
        self
    }

    pub fn div(&mut self, row: usize, value: f64) -> &mut Self {
        self.0[row][0] /= value;
        self.0[row][1] /= value;
        self.0[row][2] /= value;
        self.1[row] /= value;
        self
    }

    pub fn add_to(&mut self, row: usize, target_row: usize) -> &mut Self {
        self.0[target_row][0] += self.0[row][0];
        self.0[target_row][1] += self.0[row][1];
        self.0[target_row][2] += self.0[row][2];
        self.1[target_row] += self.1[row];
        self
    }

    pub fn sub_to(&mut self, row: usize, target_row: usize) -> &mut Self {
        self.0[target_row][0] -= self.0[row][0];
        self.0[target_row][1] -= self.0[row][1];
        self.0[target_row][2] -= self.0[row][2];
        self.1[target_row] -= self.1[row];
        self
    }

    pub fn mul_to(&mut self, row: usize, value: f64, target_row: usize) -> &mut Self {
        self.0[target_row][0] += self.0[row][0] * value;
        self.0[target_row][1] += self.0[row][1] * value;
        self.0[target_row][2] += self.0[row][2] * value;
        self.1[target_row] += self.1[row] * value;
        self
    }

    pub fn div_to(&mut self, row: usize, value: f64, target_row: usize) -> &mut Self {
        self.0[target_row][0] += self.0[row][0] / value;
        self.0[target_row][1] += self.0[row][1] / value;
        self.0[target_row][2] += self.0[row][2] / value;
        self.1[target_row] += self.1[row] / value;
        self
    }
}

impl Display for Matrix3x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SIZE: usize = 3;
        let precision = f.precision().unwrap_or(3);
        let mut col_widths = [0; SIZE];
        let mut strings = vec![Vec::with_capacity(SIZE); SIZE];
        for col in 0..SIZE {
            let mut col_width = 0;
            for row in 0..SIZE {
                strings[row].push({
                    let res_string = format!("{:.1$}", self.0[row][col], precision);
                    let res = res_string
                        .as_str()
                        .trim_end_matches('0')
                        .trim_end_matches('.');
                    if res == "-0" {
                        String::from("0")
                    } else {
                        res.to_string()
                    }
                });
                let len = strings[row][col].len();
                if len > col_width {
                    col_width = len;
                }
            }
            col_widths[col] = col_width;
        }
        let mut res_strings = Vec::with_capacity(SIZE);
        let mut res_width = 0;
        for i in 0..SIZE {
            res_strings.push({
                let res_string = format!("{:.1$}", self.1[i], precision);
                let res = res_string
                    .as_str()
                    .trim_end_matches('0')
                    .trim_end_matches('.');
                if res == "-0" {
                    String::from("0")
                } else {
                    res.to_string()
                }
            });
            let len = res_strings[i].len();
            if len > res_width {
                res_width = len;
            }
        }
        let width = sum(&col_widths) + res_width + 3;
        f.write_fmt(format_args!("\n┌╴{:w$}╶┐\n", "", w = width))?;
        for row in 0..SIZE {
            f.write_str("│")?;
            for col in 0..SIZE {
                let cw = col_widths[col];
                f.write_fmt(format_args!(" {:>cw$}", strings[row][col], cw = cw))?;
            }
            f.write_fmt(format_args!("│{:>rw$} ", res_strings[row], rw = res_width))?;
            f.write_str("│\n")?;
        }
        f.write_fmt(format_args!("└╴{:w$}╶┘", "", w = width))
    }
}

impl Drawable for Matrix3x3 {
    fn draw(&self, file: &mut ExcalidrawFile, x: i32, y: i32, locked: bool) -> (i32, i32) {
        const SIZE: usize = 3;
        let precision = 3;
        let scale = file.app_state.grid_size;
        file.elements.push(Element::simple_line(
            x,
            y,
            locked,
            vec![
                [scale, 0],
                [0, 0],
                [0, scale * (SIZE as i32 * 2 + 1)],
                [scale, scale * (SIZE as i32 * 2 + 1)],
            ],
        ));
        let mut col_widths = [0; SIZE];
        let mut strings = vec![Vec::with_capacity(SIZE); SIZE];
        for col in 0..SIZE {
            let mut col_width = 0;
            for row in 0..SIZE {
                strings[row].push({
                    let res_string = format!("{:.1$}", self.0[row][col], precision);
                    let res = res_string
                        .as_str()
                        .trim_end_matches('0')
                        .trim_end_matches('.');
                    if res == "-0" {
                        String::from("0")
                    } else {
                        res.to_string()
                    }
                });
                let len = strings[row][col].len();
                if len > col_width {
                    col_width = len;
                }
            }
            col_widths[col] = col_width;
        }
        let mut res_strings = Vec::with_capacity(SIZE);
        let mut res_width = 0;
        for i in 0..SIZE {
            res_strings.push({
                let res_string = format!("{:.1$}", self.1[i], precision);
                let res = res_string
                    .as_str()
                    .trim_end_matches('0')
                    .trim_end_matches('.');
                if res == "-0" {
                    String::from("0")
                } else {
                    res.to_string()
                }
            });
            let len = res_strings[i].len();
            if len > res_width {
                res_width = len;
            }
        }
        let total_col_width: i32 =
            col_widths.iter().sum::<usize>() as i32 * 10 + (SIZE as i32 - 1) * scale;
        let mut current_offset = 0;
        for col in 0..SIZE {
            for row in 0..SIZE {
                file.elements.push(Element::draw_small_monospaced_text(
                    x + scale + current_offset,
                    y + scale + scale * 2 * row as i32,
                    locked,
                    strings[row][col].clone(),
                ));
            }
            current_offset += col_widths[col] as i32 * 10 + scale;
        }
        file.elements.push(Element::simple_line(
            x + scale * 2 + total_col_width,
            y + scale,
            locked,
            vec![[0, 0], [0, scale * (SIZE as i32 * 2 - 1)]],
        ));
        for i in 0..SIZE {
            file.elements.push(Element::draw_small_monospaced_text(
                x + scale * 3 + total_col_width,
                y + scale + scale * 2 * i as i32,
                locked,
                res_strings[i].clone(),
            ));
        }
        file.elements.push(Element::simple_line(
            x + scale * 3 + total_col_width + res_width as i32 * 10,
            y,
            locked,
            vec![
                [0, 0],
                [scale, 0],
                [scale, scale * (SIZE as i32 * 2 + 1)],
                [0, scale * (SIZE as i32 * 2 + 1)],
            ],
        ));
        (
            scale * 4 + total_col_width + res_width as i32,
            scale * (SIZE as i32 * 2 + 1),
        )
    }
}

fn sum(arr: &[usize]) -> usize {
    arr.iter().sum()
    // let mut accumulator = 0;
    // for i in arr {
    //     accumulator += i;
    // }
    // accumulator
}
