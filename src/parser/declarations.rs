use super::parser::PascalParser;
use super::ast::*;
use crate::lexer::token_types::{TokenType};
use super::error::SyntaxError;

impl PascalParser {
    /// declaration-part -> (var-declaration | ...)
    /// Ini akan looping selama masih menemukan keyword deklarasi ('variabel')
	pub(super) fn parse_declaration_part(&mut self) -> Result<Vec<Declaration>, SyntaxError> {
        let mut declarations = Vec::new();

        loop {
            if self.check_keyword("konstanta") {
                declarations.push(self.parse_constant_declaration_block()?);
            } else if self.check_keyword("tipe") {
                declarations.push(self.parse_type_declaration_block()?);
            } else if self.check_keyword("variabel") {
                declarations.push(self.parse_variable_declaration_block()?);
            } else if self.check_keyword("prosedur") {
                declarations.push(self.parse_procedure_declaration()?);
            } else if self.check_keyword("fungsi") {
                declarations.push(self.parse_function_declaration()?);
            } else {
                break;
            }
        }
        
		Ok(declarations)
	}

    /// Parsing satu blok 'variabel' penuh
    /// var-declaration-block -> 'variabel' (var-group ;)+
    fn parse_variable_declaration_block(&mut self) -> Result<Declaration, SyntaxError> {
        self.consume_keyword("variabel", "Mengharapkan 'variabel'.")?;
        let mut groups = Vec::new();

        if !self.check(TokenType::Identifier) {
            return Err(self.error("Blok 'Variabel' harus berisi setidaknya satu grup variabel."));
        }

        groups.push(self.parse_variable_group()?);

        while self.match_token(&[TokenType::Semicolon]) {
            if !self.check(TokenType::Identifier) {
                break;
            }
            groups.push(self.parse_variable_group()?);
        }

        Ok(Declaration::Variable(VariableDeclaration { groups }))
    }

    /// Helper function untuk parsing variable group dan parameter group
    fn __parse_identifier_list(&mut self, entity_name: &str) -> Result<(Vec<String>, Type), SyntaxError> {
        let mut identifiers = Vec::new();

        let first_ident = self.consume_token(TokenType::Identifier, &format!("Mengharapkan identifier {}.", entity_name))?;
        identifiers.push(first_ident.value.clone());

        while self.match_token(&[TokenType::Comma]) {
            let next_ident = self.consume_token(TokenType::Identifier, &format!("Mengharapkan identifier {} setelah ','.", entity_name))?;
            identifiers.push(next_ident.value.clone());
        }

        self.consume_token(TokenType::Colon, &format!("Mengharapkan ':' setelah daftar identifier {}.", entity_name))?;
        let var_type = self.parse_type_spec()?;

        Ok((identifiers, var_type))
    }

    /// Parsing satu grup variabel: 'id1, id2 : tipe'
    /// var-group -> identifier-list ':' type-spec
    fn parse_variable_group(&mut self) -> Result<VariableGroup, SyntaxError> {
        let (identifiers, var_type) = self.__parse_identifier_list("variabel")?;
        Ok(VariableGroup { identifiers, var_type })
    }

    /// Parsing satu grup parameter : 'id1, id2 : tipe'
    /// parameter-group -> identifier-list ':' type-spec
    fn parse_parameter_group(&mut self) -> Result<FormalParameterGroup, SyntaxError> {
        let (identifiers, var_type) = self.__parse_identifier_list("parameter")?;
        Ok(FormalParameterGroup { identifiers, var_type })
    }

    /// Parsing daftar parameter
    fn parse_formal_parameter_list(&mut self) -> Result<Vec<FormalParameterGroup>, SyntaxError> {
        let mut params = Vec::new();

        if self.check(TokenType::RParenthesis) {
            self.advance();
            return Ok(params);
        }

        params.push(self.parse_parameter_group()?);

        while self.match_token(&[TokenType::Semicolon]) {
            params.push(self.parse_parameter_group()?);
        }

        self.consume_token(TokenType::RParenthesis, "Mengharapkan ')' setelah daftar parameter.")?;
        Ok(params)
    }

    /// Parsing satu blok 'konstanta' penuh
    /// const-declaration -> 'konstanta' (const-definition ;)+
    fn parse_constant_declaration_block(&mut self) -> Result<Declaration, SyntaxError> {
        self.consume_keyword("konstanta", "Mengharapkan 'konstanta'.")?;
        let mut constants = Vec::new();

        if !self.check(TokenType::Identifier) {
            return Err(self.error("Blok 'Konstanta' harus berisi setidaknya satu definisi konstanta."));
        }

        while self.check(TokenType::Identifier) {
            let name = self.consume_token(TokenType::Identifier, "Mengharapkan identifier konstanta.")?.value.clone();

            let op_token = self.consume_token(TokenType::RelationalOperator, "Mengharapkan '=' setelah identifier konstanta.")?;
            if op_token.value != "=" {
                return Err(self.error("Mengharapkan '=' setelah identifier konstanta."));
            }

            let value = self.parse_expression()?;

            self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah definisi konstanta.")?;

            constants.push(ConstantDefinition { name, value });
        }

        Ok(Declaration::Constant(ConstantDeclaration { constants }))
    }

    /// Parsing satu blok 'tipe' penuh
    /// type-declaration -> 'tipe' (type-definition ;)+
    fn parse_type_declaration_block(&mut self) -> Result<Declaration, SyntaxError> {
        self.consume_keyword("tipe", "Mengharapkan 'tipe'.")?;
        let mut definitions = Vec::new();

        if !self.check(TokenType::Identifier) {
            return Err(self.error("Blok 'Tipe' harus berisi setidaknya satu definisi tipe."));
        }

        while self.check(TokenType::Identifier) {
            let name = self.consume_token(TokenType::Identifier, "Mengharapkan identifier tipe.")?.value.clone();

            let op_token = self.consume_token(TokenType::RelationalOperator, "Mengharapkan '=' setelah identifier tipe.")?;
            if op_token.value != "=" {
                return Err(self.error("Mengharapkan '=' setelah identifier tipe."));
            }

            let type_def = self.parse_type_spec()?;

            self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah definisi tipe.")?;

            definitions.push(TypeDefinition { name, type_def });
        }

        Ok(Declaration::Type(TypeDeclaration { definitions }))
    }

    /// Parsing deklarasi 'prosedur' penuh
    /// procedure-declaration -> 'prosedur' identifier ( '(' formal-parameter-group ( ';' formal-parameter-group )* ')' )? ';' declaration-part 'begin' compound-statement 'end' ';'
    fn parse_procedure_declaration(&mut self) -> Result<Declaration, SyntaxError> {
        self.consume_keyword("prosedur", "Mengharapkan 'prosedur'.")?;
        let name = self.consume_token(TokenType::Identifier, "Mengharapkan identifier prosedur.")?.value.clone();

        let parameters = if self.match_token(&[TokenType::LParenthesis]) {
            self.parse_formal_parameter_list()?
        } else {
            Vec::new()
        };

        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah deklarasi parameter prosedur.")?;

        let declarations = self.parse_declaration_part()?;

        let body_stmt = self.parse_compound_statement()?;
        let body = match body_stmt {
            Statement::Compound(compound_stmt) => compound_stmt,
            _ => return Err(self.error("Mengharapkan blok 'mulai ... selesai.' sebagai badan prosedur.")),
        };

        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah deklarasi prosedur.")?;

        Ok(Declaration::Procedure(ProcedureDeclaration {
            name,
            parameters,
            declarations,
            body,
        }))
    }

    /// Parsing deklarasi 'fungsi' penuh
    /// function-declaration -> 'fungsi' identifier ( '(' formal-parameter-group ( ';' formal-parameter-group )* ')' )? ':' type-spec ';' declaration-part 'begin' compound-statement 'end' ';'
    fn parse_function_declaration(&mut self) -> Result<Declaration, SyntaxError> {
        self.consume_keyword("fungsi", "Mengharapkan 'fungsi'.")?;
        let name = self.consume_token(TokenType::Identifier, "Mengharapkan identifier fungsi.")?.value.clone();

        let parameters = if self.match_token(&[TokenType::LParenthesis]) {
            self.parse_formal_parameter_list()?
        } else {
            Vec::new()
        };

        self.consume_token(TokenType::Colon, "Mengharapkan ':' setelah deklarasi parameter fungsi.")?;
        let return_type = self.parse_type_spec()?;

        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah deklarasi tipe pengembalian fungsi.")?;

        let declarations = self.parse_declaration_part()?;

        let body_stmt = self.parse_compound_statement()?;
        let body = match body_stmt {
            Statement::Compound(compound_stmt) => compound_stmt,
            _ => return Err(self.error("Mengharapkan blok 'mulai ... selesai.' sebagai badan fungsi.")),
        };

        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah deklarasi fungsi.")?;

        Ok(Declaration::Function(FunctionDeclaration {
            name,
            parameters,
            return_type,
            declarations,
            body,
        }))
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
            let expr = self.parse_expression()?;

            if self.match_token(&[TokenType::RangeOperator]) {
                let range_end = self.parse_expression()?;
                Ok(Type::Subrange(Box::new(SubrangeType {
                    start: expr,
                    end: range_end,
                })))
            } else {
                match expr {
                    Expression::Identifier(name) => {
                        Ok(Type::TypeIdentifier(name))
                    },
                    _ => {
                        Err(self.error("Mengharapkan tipe data (integer, real, dll.), nama custom type, atau '..' untuk subrange."))
                    }
                }
            }
        }
    }
}
