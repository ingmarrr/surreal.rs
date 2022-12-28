#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
    Decimal(String),
}

impl Default for Number {
    fn default() -> Self {
        Number::Int(0)
    }
}

macro_rules! impl_number {
    (int $($t:ty),*) => {
        $(
            impl From<$t> for Number {
                fn from(value: $t) -> Self {
                    Number::Int(value as i64)
                }
            }
        )*
    };
    (float $($t:ty),*) => {
        $(
            impl From<$t> for Number {
                fn from(value: $t) -> Self {
                    Number::Float(value as f64)
                }
            }
        )*
    };
    (decimal, $($t:ty),*) => {
        $(
            impl From<$t> for Number {
                fn from(value: $t) -> Self {
                    match value.parse::<i64>() {
                        Ok(i) => Number::Int(i),
                        Err(_) => match value.parse::<f64>() {
                            Ok(f) => Number::Float(f),
                            Err(_) => Number::Decimal(value.to_string()),
                        }
                    }
                }
            }
        )*
    };
}

impl_number!(int i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_number!(float f32, f64);
impl_number!(decimal, String, &str);
