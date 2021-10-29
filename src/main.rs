struct 长方形 {
    长: u8,
    宽: u8,
    高: u8,
}


trait 计算Trait {
    fn 面积(&self) -> u8;
}

impl 计算Trait for 长方形 {
    fn 面积(&self) -> u8 {
        &self.长 * &self.宽 * &self.高
    }
}

fn print_area<T: 计算Trait>(param: T) {
    print!("{}", param.面积());
}


fn main() {
    let a = 长方形 {
        长: 2,
        宽: 3,
        高: 4,
    };

    print_area(a)
}

