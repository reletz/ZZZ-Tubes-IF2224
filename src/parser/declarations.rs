use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;

impl PascalParser {
    /// 1. parse_declaration_part (Entry Point Utama)
    pub(super) fn parse_declaration_part(&mut self) -> Result<DeclarationPart, SyntaxError> {
        
        let mut const_declarations: Vec<ConstantDeclaration> = Vec::new();
        let mut type_declarations: Vec<TypeDeclaration> = Vec::new();
        let mut var_declarations: Vec<VariableDeclaration> = Vec::new();
        let mut subprogram_declarations: Vec<SubprogramDeclaration> = Vec::new();

        // 1. Loop `while self.check_keyword("konstanta")`:
        while self.check_keyword("konstanta") {
            const_declarations.push(self.parse_constant_declaration_block()?);
        }
        
        // 2. Loop `while self.check_keyword("tipe")`:
        while self.check_keyword("tipe") {
            type_declarations.push(self.parse_type_declaration_block()?);
        }

        // 3. Loop `while self.check_keyword("variabel")`:
        while self.check_keyword("variabel") {
            var_declarations.push(self.parse_variable_declaration_block()?);
        }

        // 4. Loop `while self.check_keyword("prosedur") || self.check_keyword("fungsi")`:
        while self.check_keyword("prosedur") || self.check_keyword("fungsi") {
            subprogram_declarations.push(self.parse_subprogram_declaration()?);
        }

        // 5. Kembalikan `Ok(DeclarationPart { ... })`
        Ok(DeclarationPart { 
            const_declarations, 
            type_declarations, 
            var_declarations, 
            subprogram_declarations 
        })
    }

    // --- 1. CONSTANT Declarations ---

    /// Mem-parse: konstanta (ID = expr;)+
    fn parse_constant_declaration_block(&mut self) -> Result<ConstantDeclaration, SyntaxError> {
        // 1. `self.consume_keyword("konstanta", ...)`
        self.consume_keyword("konstanta", "Mengharapkan keyword 'konstanta'.")?;
        
        // 2. `let mut constants = Vec::new()`.
        let mut constants = Vec::new();

        // 3. Loop `while self.check(TokenType::Identifier)`:
        while self.check(TokenType::Identifier) {
            // a. `let name = self.consume_token(Identifier, ...).value.clone()`
            let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama konstanta.")?.value.clone();
            
            // b. `self.consume_token(RelationalOperator, "Mengharapkan '='")` (pastikan value-nya "=")
            let eq_token = self.consume_token(TokenType::RelationalOperator, "Mengharapkan '=' setelah nama konstanta.")?;
            if eq_token.value != "=" {
                return Err(self.error("Mengharapkan '=' untuk deklarasi konstanta, bukan operator relasional lain."));
            }

            // c. `let value = self.parse_expression()?`
            let value = self.parse_expression()?;
            
            // d. `self.consume_token(Semicolon, ...)`
            self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah definisi konstanta.")?;
            
            // e. `constants.push(ConstantDefinition { name, value })`
            constants.push(ConstantDefinition { name, value });
        }

        // 4. Kembalikan `Ok(ConstantDeclaration { constants })`
        Ok(ConstantDeclaration { constants })
    }

    // --- 2. TYPE Declarations ---

    /// Mem-parse: tipe (ID = type_spec;)+
    fn parse_type_declaration_block(&mut self) -> Result<TypeDeclaration, SyntaxError> {
        // 1. `self.consume_keyword("tipe", ...)`
        self.consume_keyword("tipe", "Mengharapkan keyword 'tipe'.")?;
        
        // 2. `let mut definitions = Vec::new()`.
        let mut definitions = Vec::new();

        // 3. Loop `while self.check(TokenType::Identifier)`:
        while self.check(TokenType::Identifier) {
            // a. `let name = self.consume_token(Identifier, ...).value.clone()`
            let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama tipe.")?.value.clone();

            // b. `self.consume_token(RelationalOperator, "Mengharapkan '='")` (pastikan value-nya "=")
            let eq_token = self.consume_token(TokenType::RelationalOperator, "Mengharapkan '=' setelah nama tipe.")?;
            if eq_token.value != "=" {
                return Err(self.error("Mengharapkan '=' untuk deklarasi tipe, bukan operator relasional lain."));
            }

            // c. `let type_def = self.parse_type_spec()?`
            let type_def = self.parse_type_spec()?;
            
            // d. `self.consume_token(Semicolon, ...)`
            self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah definisi tipe.")?;

            // e. `definitions.push(TypeDefinition { name, type_def })`
            definitions.push(TypeDefinition { name, type_def });
        }
        
        // 4. Kembalikan `Ok(TypeDeclaration { definitions })`
        Ok(TypeDeclaration { definitions })
    }

    // --- 3. VARIABLE Declarations ---

    /// Mem-parse: variabel (id_list : type_spec;)+
    fn parse_variable_declaration_block(&mut self) -> Result<VariableDeclaration, SyntaxError> {
        // 1. `self.consume_keyword("variabel", ...)`
        self.consume_keyword("variabel", "Mengharapkan keyword 'variabel'.")?;
        
        // 2. `let mut groups = Vec::new()`.
        let mut groups = Vec::new();

        // 3. Loop `while self.check(TokenType::Identifier)`:
        while self.check(TokenType::Identifier) {
            // a. `let identifiers = self.parse_identifier_list()?`
            let identifiers = self.parse_identifier_list()?;
            
            // b. `self.consume_token(Colon, ...)`
            self.consume_token(TokenType::Colon, "Mengharapkan ':' setelah daftar identifier variabel.")?;
            
            // c. `let var_type = self.parse_type_spec()?`
            let var_type = self.parse_type_spec()?;
            
            // d. `self.consume_token(Semicolon, ...)`
            self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah deklarasi variabel.")?;
            
            // e. `groups.push(VariableGroup { identifiers, var_type })`
            groups.push(VariableGroup { identifiers, var_type });
        }

        // 4. Kembalikan `Ok(VariableDeclaration { groups })`
        Ok(VariableDeclaration { groups })
    }

    // --- 4. SUBPROGRAM Declarations (Procedure / Function) ---

    /// Router untuk mem-parse Prosedur atau Fungsi
    fn parse_subprogram_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // 1. `if self.check_keyword("prosedur")`:
        if self.check_keyword("prosedur") {
            //    - `self.parse_procedure_declaration()`
            self.parse_procedure_declaration()
        // 2. `else if self.check_keyword("fungsi")`:
        } else if self.check_keyword("fungsi") {
            //    - `self.parse_function_declaration()`
            self.parse_function_declaration()
        // 3. `else`:
        } else {
            //    - `Err(...)`
            Err(self.error("Mengharapkan 'prosedur' atau 'fungsi'."))
        }
    }

    /// Mem-parse: prosedur ID(params); block;
    fn parse_procedure_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // 1. `self.consume_keyword("prosedur", ...)`
        self.consume_keyword("prosedur", "Mengharapkan keyword 'prosedur'.")?;
        
        // 2. `let name = self.consume_token(Identifier, ...).value.clone()`
        let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama prosedur.")?.value.clone();
        
        // 3. `let parameters = self.parse_formal_parameter_list()?` (Sesuai Revisi 3, () wajib) 
        let parameters = self.parse_formal_parameter_list()?;
        
        // 4. `self.consume_token(Semicolon, "Mengharapkan ';' setelah header prosedur")`
        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah header prosedur.")?;
        
        // 5. `let declarations = self.parse_declaration_part()?` (Panggilan REKURSIF)
        let declarations = self.parse_declaration_part()?;
        
        // 6. `let body = self.parse_compound_statement()?`
        let body = self.parse_compound_statement()?;
        
        // 7. `self.consume_token(Semicolon, "Mengharapkan ';' setelah blok prosedur")`
        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah blok prosedur.")?;
        
        // 8. Kembalikan `Ok(SubprogramDeclaration::Procedure(ProcedureDeclaration { ... }))`
        Ok(SubprogramDeclaration::Procedure(ProcedureDeclaration { 
            name, 
            parameters, 
            declarations, 
            body 
        }))
    }

    /// Mem-parse: fungsi ID(params): type; block;
    fn parse_function_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // 1. `self.consume_keyword("fungsi", ...)`
        self.consume_keyword("fungsi", "Mengharapkan keyword 'fungsi'.")?;
        
        // 2. `let name = self.consume_token(Identifier, ...).value.clone()`
        let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama fungsi.")?.value.clone();

        // 3. `let parameters = self.parse_formal_parameter_list()?` (Sesuai Revisi 3, () wajib) 
        let parameters = self.parse_formal_parameter_list()?;
        
        // 4. `self.consume_token(Colon, "Mengharapkan ':' untuk return type")`
        self.consume_token(TokenType::Colon, "Mengharapkan ':' untuk return type fungsi.")?;
        
        // 5. `let return_type = self.parse_type_spec()?`
        let return_type = self.parse_type_spec()?;
        
        // 6. `self.consume_token(Semicolon, "Mengharapkan ';' setelah header fungsi")`
        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah header fungsi.")?;
        
        // 7. `let declarations = self.parse_declaration_part()?` (Panggilan REKURSIF)
        let declarations = self.parse_declaration_part()?;
        
        // 8. `let body = self.parse_compound_statement()?`
        let body = self.parse_compound_statement()?;
        
        // 9. `self.consume_token(Semicolon, "Mengharapkan ';' setelah blok fungsi")`
        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah blok fungsi.")?;
        
        // 10. Kembalikan `Ok(SubprogramDeclaration::Function(FunctionDeclaration { ... }))`
        Ok(SubprogramDeclaration::Function(FunctionDeclaration { 
            name, 
            parameters, 
            return_type, 
            declarations, 
            body 
        }))
    }

    // --- HELPER UNTUK DECLARATIONS ---

    /// Mem-parse: '(' [param-group (';' param-group)*] ')'
    /// Sesuai Revisi 3, () wajib 
    fn parse_formal_parameter_list(&mut self) -> Result<FormalParameterList, SyntaxError> {
        // 1. `self.consume_token(LParenthesis, ...)`
        self.consume_token(TokenType::LParenthesis, "Mengharapkan '(' untuk daftar parameter.")?;
        
        // 2. `let mut parameters = Vec::new()`.
        let mut parameters = Vec::new();

        // 3. `if !self.check(RParenthesis)` (Handle list kosong `()`)
        if !self.check(TokenType::RParenthesis) {
            // 4. Loop:
            loop {
                // a. `let id_list = self.parse_identifier_list()?`
                let id_list = self.parse_identifier_list()?;
                
                // b. `self.consume_token(Colon, ...)`
                self.consume_token(TokenType::Colon, "Mengharapkan ':' setelah daftar identifier parameter.")?;
                
                // c. `let var_type = self.parse_type_spec()?`
                let var_type = self.parse_type_spec()?;
                
                // d. `parameters.push(FormalParameterGroup { identifiers: id_list, var_type })`
                parameters.push(FormalParameterGroup { identifiers: id_list, var_type });

                // e. `if !self.match_token(Semicolon)` `break;`
                if !self.match_token(&[TokenType::Semicolon]) {
                    break;
                }
            }
        }
        
        // 5. `self.consume_token(RParenthesis, ...)`
        self.consume_token(TokenType::RParenthesis, "Mengharapkan ')' setelah daftar parameter.")?;
        
        // 6. Kembalikan `Ok(FormalParameterList { parameters })`
        Ok(FormalParameterList { parameters })
    }

    /// Mem-parse: ID (',' ID)*
    fn parse_identifier_list(&mut self) -> Result<IdentifierList, SyntaxError> {
        // 1. `let mut identifiers = Vec::new()`.
        let mut identifiers = Vec::new();
        
        // 2. `identifiers.push(self.consume_token(Identifier, ...).value.clone())` (Harus ada minimal 1)
        let first_id = self.consume_token(TokenType::Identifier, "Mengharapkan identifier.")?.value.clone();
        identifiers.push(first_id);

        // 3. Loop `while self.match_token(&[TokenType::Comma])`:
        while self.match_token(&[TokenType::Comma]) {
            //    - `identifiers.push(self.consume_token(Identifier, "Mengharapkan ID setelah ','").value.clone())`
            let next_id = self.consume_token(TokenType::Identifier, "Mengharapkan identifier setelah ','.")?.value.clone();
            identifiers.push(next_id);
        }

        // 4. Kembalikan `Ok(IdentifierList { identifiers })`
        Ok(IdentifierList { identifiers })
    }

    /// Mem-parse Tipe Data (integer, real, array, subrange, dll.)
    pub(super) fn parse_type_spec(&mut self) -> Result<Type, SyntaxError> {
        // 1. `if self.match_keyword(&["integer"]) { Ok(Type::Integer) }`
        if self.match_keyword(&["integer"]) {
            return Ok(Type::Integer);
        }
        // 2. (Lakukan hal yang sama untuk `real`, `boolean`, `string`, `char`)
        if self.match_keyword(&["real"]) {
            return Ok(Type::Real);
        }
        if self.match_keyword(&["boolean"]) {
            return Ok(Type::Boolean);
        }
        if self.match_keyword(&["string"]) {
            return Ok(Type::String);
        }
        if self.match_keyword(&["char"]) {
            return Ok(Type::Char);
        }
        
        // 3. `else if self.match_keyword(&["larik"])`:
        if self.match_keyword(&["larik"]) {
            //    - Panggil `self.parse_array_type()`
            return self.parse_array_type();
        }
        
        // 4. `else if self.check(TokenType::Identifier)`:
        if self.check(TokenType::Identifier) {
            //    - `let name = self.advance().value.clone()`
            let name = self.advance().value.clone();
            //    - `Ok(Type::TypeIdentifier(name))`
            return Ok(Type::TypeIdentifier(name));
        }

        // 5. `else`: (Jika bukan keyword atau identifier, pasti subrange)
        //    - Cek `Range`: `let start = self.parse_expression()`
        //    - `if self.check(RangeOperator) { ... parse_range ... }`
        //    - Kita panggil `parse_range` yang sudah menangani `expr .. expr`
        match self.parse_range() {
            Ok(range) => Ok(Type::Subrange(range)),
            Err(_) => Err(self.error("Mengharapkan tipe data (contoh: 'integer', 'larik', 'MyType', atau '1..10')."))
        }
    }

    /// Mem-parse: 'larik' '[' <range> ']' 'dari' <type> 
    fn parse_array_type(&mut self) -> Result<Type, SyntaxError> {
        // 1. `self.consume_token(LBracket, ...)` (Keyword 'larik' sudah di-consume oleh parse_type_spec)
        self.consume_token(TokenType::LBracket, "Mengharapkan '[' setelah 'larik'.")?;
        
        // 2. `let range = self.parse_range()?`
        let range = self.parse_range()?;
        
        // 3. `self.consume_token(RBracket, ...)`
        self.consume_token(TokenType::RBracket, "Mengharapkan ']' setelah range array.")?;
        
        // 4. `self.consume_keyword("dari", ...)`
        self.consume_keyword("dari", "Mengharapkan 'dari' setelah ']' pada deklarasi array.")?;
        
        // 5. `let base_type = self.parse_type_spec()?` (Panggilan REKURSIF)
        let base_type = self.parse_type_spec()?;
        
        // 6. Kembalikan `Ok(Type::Array(ArrayType { ... }))`
        Ok(Type::Array(ArrayType { 
            range: Box::new(range), 
            base_type: Box::new(base_type) 
        }))
    }

    /// Mem-parse: <expression> '..' <expression> 
    fn parse_range(&mut self) -> Result<Range, SyntaxError> {
        // 1. `let start = self.parse_expression()?`
        let start = self.parse_expression()?;
        
        // 2. `self.consume_token(RangeOperator, "Mengharapkan '..'")`
        self.consume_token(TokenType::RangeOperator, "Mengharapkan '..' untuk mendefinisikan range.")?;
        
        // 3. `let end = self.parse_expression()?`
        let end = self.parse_expression()?;
        
        // 4. Kembalikan `Ok(Range { start: Box::new(start), end: Box::new(end) })`
        Ok(Range { 
            start: Box::new(start), 
            end: Box::new(end) 
        })
    }
}