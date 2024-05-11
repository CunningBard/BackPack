

pub enum DataType {
    Int { bits: Option<u8> }, // defaults to `int` when `bits` is `None`
    Uint { bits: Option<u8> }, // defaults to `uint` when `bits` is `None`
    Float { bits: u8 },
    Bool,
    String,
    Struct { name: String },
    Pointer { to: Box<Self> },
}