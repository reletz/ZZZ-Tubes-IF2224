use super::parser::PascalParser;
use super::ast::*;
use crate::lexer::token_types::{Token, TokenType};
use super::error::SyntaxError;

impl PascalParser {
    /// declaration-part -> (var-declaration | ...)
    /// Ini akan looping selama masih menemukan keyword deklarasi ('variabel')
	pub(super) fn parse_declaration_part(&mut self) -> Result<Vec<Declaration>, SyntaxError> {
        let mut declarations = Vec::new();

        while self.check_keyword("variabel") {
            declarations.push(self.parse_variable_declaration_block()?);
        }
        // TODO: Tambah 'konstanta', 'tipe', 'prosedur', 'fungsi' di sini
        
		Ok(declarations)
	}

    /// Parsing satu blok 'variabel' penuh
    /// var-declaration-block -> 'variabel' (var-group ;)+
    fn parse_variable_declaration_block(&mut self) -> Result<Declaration, SyntaxError> {
        self.consume_keyword("variabel", "Mengharapkan 'variabel'.")?;
        let mut groups = Vec::new();

        groups.push(self.parse_variable_group()?);

        while self.match_token(&[TokenType::Semicolon]) {
            if !self.check(TokenType::Identifier) {
                break;
            }
            groups.push(self.parse_variable_group()?);
        }

        Ok(Declaration::Variable(VariableDeclaration { groups }))
    }

    /// Parsing satu grup variabel: 'id1, id2 : tipe'
    /// var-group -> identifier-list ':' type-spec
    fn parse_variable_group(&mut self) -> Result<VariableGroup, SyntaxError> {
        let mut identifiers = Vec::new();

        let first_ident = self.consume_token(TokenType::Identifier, "Mengharapkan identifier.")?;
        identifiers.push(first_ident.value.clone());

        while self.match_token(&[TokenType::Comma]) {
            let next_ident = self.consume_token(TokenType::Identifier, "Mengharapkan identifier setelah ','.")?;
            identifiers.push(next_ident.value.clone());
        }

        self.consume_token(TokenType::Colon, "Mengharapkan ':' setelah daftar identifier.")?;
        let var_type = self.parse_type_spec()?;

        Ok(VariableGroup { identifiers, var_type })
    }

    /// Parsing spesifikasi tipe data
    /// type-spec -> 'integer' | 'real' | 'boolean' | 'string' | 'char' | array-type
    fn parse_type_spec(&mut self) -> Result<Type, SyntaxError> {
        if self.match_keyword(&["integer"]) {
            Ok(Type::Integer)
        } else if self.match_keyword(&["real"]) {
            Ok(Type::Real)
        } else if self.match_keyword(&["boolean"]) {
            Ok(Type::Boolean)
        } else if self.match_keyword(&["string"]) {
            Ok(Type::String)
        } else if self.match_keyword(&["char"]) {
            Ok(Type::Char)
        } else if self.match_keyword(&["larik"]) {
            self.consume_token(TokenType::LBracket, "Mengharapkan '[' setelah 'larik'.")?;
            
            let range_start = self.parse_expression()?;
            
            self.consume_token(TokenType::RangeOperator, "Mengharapkan '..' di antara range array.")?;
            
            let range_end = self.parse_expression()?;
            
            self.consume_token(TokenType::RBracket, "Mengharapkan ']' setelah range array.")?;
            self.consume_keyword("dari", "Mengharapkan 'dari' (of) setelah range array.")?;
            
            // Rekursif: memanggil dirinya sendiri
            let base_type = self.parse_type_spec()?; 

            Ok(Type::Array(Box::new(ArrayTypeDefinition {
                range_start,
                range_end,
                base_type: Box::new(base_type),
            })))
        } else {
            Err(self.error("Mengharapkan tipe data (integer, real, larik, dll.)."))
        }
    }
}
