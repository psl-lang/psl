use crate::{
    ast::{LiteralExpression, TokenKind},
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for LiteralExpression {
    fn produce_code(self, _ctx: &mut CodegenContext) -> String {
        let digit_only = self.value.content.replace('_', "");
        match self.value.kind {
            TokenKind::LiteralDecimal => digit_only,
            TokenKind::LiteralHexadecimal => format!("0x{digit_only}"),
            TokenKind::LiteralBinary => {
                let mut binary = String::from("0x");
                let mut buffer = 0;
                let mut nibble_len = digit_only.chars().count() % 4;
                for bit in digit_only.chars() {
                    buffer <<= 1;
                    if bit == '1' {
                        buffer |= 1;
                    }
                    nibble_len -= 1;
                    if nibble_len == 0 {
                        binary.push(to_hex_digit(buffer));
                    }
                }
                binary
            }
            _ => unreachable!(
                "Invalid TokenKind for LiteralExpression: {:?}",
                self.value.kind
            ),
        }
    }
}
impl NameResolutionPass for LiteralExpression {
    fn resolve(&self, _ctx: &mut NameResolutionContext) {}
}

impl LiteralExpression {
    pub fn infer_type(&self, _ctx: &CodegenContext) -> Result<Type, String> {
        Ok(Type::Integer)
    }
}

fn to_hex_digit(number: i32) -> char {
    match number {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => unreachable!("No matching hex digit for {number}"),
    }
}
