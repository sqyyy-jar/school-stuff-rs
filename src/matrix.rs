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

impl std::fmt::Display for Matrix2x2 {
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

impl std::fmt::Display for Matrix3x3 {
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

fn sum(arr: &[usize]) -> usize {
    let mut accumulator = 0;
    for i in arr {
        accumulator += i;
    }
    accumulator
}
