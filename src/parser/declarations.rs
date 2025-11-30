use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;

impl PascalParser {
    /// 1. parse_declaration_part (Entry Point Utama)
    ///    Menganalisis blok deklarasi (konstanta, tipe, variabel, subprogram)
    pub(super) fn parse_declaration_part(&mut self) -> Result<DeclarationPart, SyntaxError> {
        let mut const_declarations: Vec<ConstantDeclaration> = Vec::new();
        let mut type_declarations: Vec<TypeDeclaration> = Vec::new();
        let mut var_declarations: Vec<VariableDeclaration> = Vec::new();
        let mut subprogram_declarations: Vec<SubprogramDeclaration> = Vec::new();

        // 1. Loop `while self.check_keyword("konstanta")`:
        while self.check_keyword("konstanta") {
            const_declarations.push(self.parse_constant_declaration()?);
        }
        
        // 2. Loop `while self.check_keyword("tipe")`:
        while self.check_keyword("tipe") {
            type_declarations.push(self.parse_type_declaration()?);
        }
        
        // 3. Loop `while self.check_keyword("variabel")`:
        while self.check_keyword("variabel") {
            var_declarations.push(self.parse_variable_declaration()?);
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
            subprogram_declarations,
        })
    }

    // --- 1. CONSTANT Declarations ---

    /// Mem-parse: 'konstanta' (ID = expr;)+
    fn parse_constant_declaration(&mut self) -> Result<ConstantDeclaration, SyntaxError> {
        // 1. `let const_kw = self.consume_keyword("konstanta", ...).clone()`
        let const_kw = self.consume_keyword("konstanta", "Mengharapkan keyword 'konstanta'.")?.clone();
        
        // 2. `let mut constants = Vec::new()`.
        let mut constants = Vec::new();

        // 3. Loop `while self.check(TokenType::Identifier)`:
        while self.check(TokenType::Identifier) {
            //    a. Panggil `let def = self.parse_constant_definition()?`
            let def = self.parse_constant_definition()?;
            //    b. `constants.push(def)`
            constants.push(def);
        }
        
        // 4. Kembalikan `Ok(ConstantDeclaration { const_kw, constants })`
        Ok(ConstantDeclaration { const_kw, constants })
    }

    /// Mem-parse: ID '=' <expression> ';'
    fn parse_constant_definition(&mut self) -> Result<ConstantDefinition, SyntaxError> {
        // 1. `let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama konstanta.")?.clone()`
        let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama konstanta.")?.clone();
        
        // 2. `let equals_op = self.consume_token(TokenType::RelationalOperator, "Mengharapkan '='.")?.clone()`
        let equals_op = self.consume_token(TokenType::RelationalOperator, "Mengharapkan '='.")?.clone();
        
        // 3. `if equals_op.value != "=" { return Err(...) }`
        if equals_op.value != "=" {
            return Err(self.error(&format!("Mengharapkan '=' untuk definisi konstanta, ditemukan '{}'", equals_op.value)));
        }

        // 4. `let value = self.parse_expression()?`
        let value = self.parse_expression()?;
        
        // 5. `let semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';'.")?.clone()`
        let semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah definisi konstanta.")?.clone();
        
        // 6. Kembalikan `Ok(ConstantDefinition { name, equals_op, value, semicolon })`
        Ok(ConstantDefinition { name, equals_op, value, semicolon })
    }

    // --- 2. TYPE Declarations ---

    /// Mem-parse: 'tipe' (ID = type_spec;)+
    fn parse_type_declaration(&mut self) -> Result<TypeDeclaration, SyntaxError> {
        // 1. `let type_kw = self.consume_keyword("tipe", ...).clone()`
        let type_kw = self.consume_keyword("tipe", "Mengharapkan keyword 'tipe'.")?.clone();
        
        // 2. `let mut definitions = Vec::new()`.
        let mut definitions = Vec::new();
        
        // 3. Loop `while self.check(TokenType::Identifier)`:
        while self.check(TokenType::Identifier) {
            //    a. Panggil `let def = self.parse_type_definition()?`
            let def = self.parse_type_definition()?;
            //    b. `definitions.push(def)`
            definitions.push(def);
        }
        
        // 4. Kembalikan `Ok(TypeDeclaration { type_kw, definitions })`
        Ok(TypeDeclaration { type_kw, definitions })
    }

    /// Mem-parse: ID '=' <type-spec> ';'
    fn parse_type_definition(&mut self) -> Result<TypeDefinition, SyntaxError> {
        // 1. `let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama tipe.")?.clone()`
        let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama tipe.")?.clone();
        
        // 2. `let equals_op = self.consume_token(TokenType::RelationalOperator, "Mengharapkan '='.")?.clone()`
        let equals_op = self.consume_token(TokenType::RelationalOperator, "Mengharapkan '='.")?.clone();
        
        // 3. (Opsional tapi direkomendasikan) `if equals_op.value != "=" { return Err(...) }`
        if equals_op.value != "=" {
            return Err(self.error(&format!("Mengharapkan '=' untuk definisi tipe, ditemukan '{}'", equals_op.value)));
        }

        // 4. `let type_def = self.parse_type_spec()?`
        let type_def = self.parse_type_spec()?;
        
        // 5. `let semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';'.")?.clone()`
        let semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah definisi tipe.")?.clone();
        
        // 6. Kembalikan `Ok(TypeDefinition { name, equals_op, type_def, semicolon })`
        Ok(TypeDefinition { name, equals_op, type_def, semicolon })
    }

    // --- 3. VARIABLE Declarations ---

    /// Mem-parse: 'variabel' (id_list : type_spec;)+
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, SyntaxError> {
        // 1. `let var_kw = self.consume_keyword("variabel", ...).clone()`
        let var_kw = self.consume_keyword("variabel", "Mengharapkan keyword 'variabel'.")?.clone();
        
        // 2. `let mut groups = Vec::new()`.
        let mut groups = Vec::new();
        
        // 3. Loop `while self.check(TokenType::Identifier)`:
        while self.check(TokenType::Identifier) {
            //    a. Panggil `let group = self.parse_variable_group()?`
            let group = self.parse_variable_group()?;
            //    b. `groups.push(group)`
            groups.push(group);
        }
        
        // 4. Kembalikan `Ok(VariableDeclaration { var_kw, groups })`
        Ok(VariableDeclaration { var_kw, groups })
    }

    /// Mem-parse: <identifier-list> ':' <type-spec> ';'
    fn parse_variable_group(&mut self) -> Result<VariableGroup, SyntaxError> {
        // 1. `let identifiers = self.parse_identifier_list()?`
        let identifiers = self.parse_identifier_list()?;
        
        // 2. `let colon = self.consume_token(TokenType::Colon, "Mengharapkan ':'.")?.clone()`
        let colon = self.consume_token(TokenType::Colon, "Mengharapkan ':' setelah daftar identifier.")?.clone();
        
        // 3. `let var_type = self.parse_type_spec()?`
        let var_type = self.parse_type_spec()?;
        
        // 4. `let semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';'.")?.clone()`
        let semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah deklarasi variabel.")?.clone();
        
        // 5. Kembalikan `Ok(VariableGroup { identifiers, colon, var_type, semicolon })`
        Ok(VariableGroup { identifiers, colon, var_type, semicolon })
    }

    // --- 4. SUBPROGRAM Declarations (Procedure / Function) ---

    /// Router untuk mem-parse Prosedur atau Fungsi
    fn parse_subprogram_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // 1. `if self.check_keyword("prosedur")`:
        if self.check_keyword("prosedur") {
            //    - `Ok(self.parse_procedure_declaration()?)`
            Ok(self.parse_procedure_declaration()?)
        // 2. `else if self.check_keyword("fungsi")`:
        } else if self.check_keyword("fungsi") {
            //    - `Ok(self.parse_function_declaration()?)`
            Ok(self.parse_function_declaration()?)
        // 3. `else`:
        } else {
            //    - `Err(...)`
            Err(self.error("Mengharapkan 'prosedur' atau 'fungsi'."))
        }
    }

    /// Mem-parse: 'prosedur' ID(params); block;
    fn parse_procedure_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // 1. `let proc_kw = self.consume_keyword("prosedur", ...).clone()`
        let proc_kw = self.consume_keyword("prosedur", "Mengharapkan keyword 'prosedur'.")?.clone();
        
        // 2. `let name = self.consume_token(TokenType::Identifier, ...).clone()`
        let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama prosedur.")?.clone();
        
        // 3. `let parameters = self.parse_formal_parameter_list()?`
        let parameters = self.parse_formal_parameter_list()?;
        
        // 4. `let header_semicolon = self.consume_token(TokenType::Semicolon, ...).clone()`
        let header_semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah header prosedur.")?.clone();
        
        // 5. `let declarations = self.parse_declaration_part()?` (Panggilan REKURSIF)
        let declarations = self.parse_declaration_part()?;
        
        // 6. `let body = self.parse_compound_statement()?`
        let body = self.parse_compound_statement()?;
        
        // 7. `let block_semicolon = self.consume_token(TokenType::Semicolon, ...).clone()`
        let block_semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah blok prosedur.")?.clone();
        
        // 8. Kembalikan `Ok(SubprogramDeclaration::Procedure(ProcedureDeclaration { ... }))`
        Ok(SubprogramDeclaration::Procedure(ProcedureDeclaration {
            proc_kw,
            name,
            parameters,
            header_semicolon,
            declarations,
            body,
            block_semicolon,
        }))
    }

    /// Mem-parse: 'fungsi' ID(params): type; block;
    fn parse_function_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // 1. `let func_kw = self.consume_keyword("fungsi", ...).clone()`
        let func_kw = self.consume_keyword("fungsi", "Mengharapkan keyword 'fungsi'.")?.clone();

        // 2. `let name = self.consume_token(TokenType::Identifier, ...).clone()`
        let name = self.consume_token(TokenType::Identifier, "Mengharapkan nama fungsi.")?.clone();
        
        // 3. `let parameters = self.parse_formal_parameter_list()?`
        let parameters = self.parse_formal_parameter_list()?;
        
        // 4. `let colon = self.consume_token(TokenType::Colon, ...).clone()`
        let colon = self.consume_token(TokenType::Colon, "Mengharapkan ':' untuk tipe return fungsi.")?.clone();
        
        // 5. `let return_type = self.parse_type_spec()?`
        let return_type = self.parse_type_spec()?;
        
        // 6. `let header_semicolon = self.consume_token(TokenType::Semicolon, ...).clone()`
        let header_semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah header fungsi.")?.clone();
        
        // 7. `let declarations = self.parse_declaration_part()?` (Panggilan REKURSIF)
        let declarations = self.parse_declaration_part()?;
        
        // 8. `let body = self.parse_compound_statement()?`
        let body = self.parse_compound_statement()?;
        
        // 9. `let block_semicolon = self.consume_token(TokenType::Semicolon, ...).clone()`
        let block_semicolon = self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah blok fungsi.")?.clone();
        
        // 10. Kembalikan `Ok(SubprogramDeclaration::Function(FunctionDeclaration { ... }))`
        Ok(SubprogramDeclaration::Function(FunctionDeclaration {
            func_kw,
            name,
            parameters,
            colon,
            return_type,
            header_semicolon,
            declarations,
            body,
            block_semicolon,
        }))
    }

    // --- HELPER UNTUK DECLARATIONS ---

    /// Mem-parse: '(' [param-group (';' param-group)*] ')'
    fn parse_formal_parameter_list(&mut self) -> Result<FormalParameterList, SyntaxError> {
        // 1. `let l_paren = self.consume_token(TokenType::LParenthesis, ...).clone()`
        let l_paren = self.consume_token(TokenType::LParenthesis, "Mengharapkan '(' untuk daftar parameter.")?.clone();
        
        // 2. `let mut initial_param = None`
        let mut initial_param = None;
        
        // 3. `let mut rest = Vec::new()`
        let mut rest = Vec::new();
        
        // 4. `if !self.check(TokenType::RParenthesis)` (Handle list tidak kosong)
        if !self.check(TokenType::RParenthesis) {
            //    a. `initial_param = Some(self.parse_formal_parameter_group()?)`
            initial_param = Some(self.parse_formal_parameter_group()?);
            
            //    b. Loop `while self.check(TokenType::Semicolon)`:
            while self.check(TokenType::Semicolon) {
                //       i.   `let semi = self.advance().clone()`
                let semi = self.advance().clone();
                //       ii.  `let group = self.parse_formal_parameter_group()?`
                let group = self.parse_formal_parameter_group()?;
                //       iii. `rest.push((semi, group))`
                rest.push((semi, group));
            }
        }
        
        // 5. `let r_paren = self.consume_token(TokenType::RParenthesis, ...).clone()`
        let r_paren = self.consume_token(TokenType::RParenthesis, "Mengharapkan ')' setelah daftar parameter.")?.clone();
        
        // 6. Kembalikan `Ok(FormalParameterList { l_paren, initial_param, rest, r_paren })`
        Ok(FormalParameterList { l_paren, initial_param, rest, r_paren })
    }

    /// Mem-parse: <identifier-list> ':' <type-spec>
    /// (Dipanggil oleh `parse_formal_parameter_list`)
    fn parse_formal_parameter_group(&mut self) -> Result<FormalParameterGroup, SyntaxError> {
        // 1. Cek apakah ada keyword 'var' (pass by reference)
        let var_kw = if self.check_keyword("var") {
            Some(self.advance().clone())
        } else {
            None
        };

        // 2. `let identifiers = self.parse_identifier_list()?`
        let identifiers = self.parse_identifier_list()?;
        
        // 3. `let colon = self.consume_token(TokenType::Colon, ...).clone()`
        let colon = self.consume_token(TokenType::Colon, "Mengharapkan ':' setelah daftar identifier parameter.")?.clone();
        
        // 4. `let var_type = self.parse_type_spec()?`
        let var_type = self.parse_type_spec()?;
        
        // 5. Kembalikan `Ok(FormalParameterGroup { var_kw, identifiers, colon, var_type })`
        Ok(FormalParameterGroup { var_kw, identifiers, colon, var_type })
    }

    /// Mem-parse: ID (',' ID)*
    fn parse_identifier_list(&mut self) -> Result<IdentifierList, SyntaxError> {
        // 1. `let initial_id = self.consume_token(TokenType::Identifier, ...).clone()`
        let initial_id = self.consume_token(TokenType::Identifier, "Mengharapkan identifier.")?.clone();
        
        // 2. `let mut rest = Vec::new()`
        let mut rest = Vec::new();
        
        // 3. Loop `while self.check(TokenType::Comma)`:
        while self.check(TokenType::Comma) {
            //    a. `let comma = self.advance().clone()`
            let comma = self.advance().clone();
            //    b. `let next_id = self.consume_token(TokenType::Identifier, "Mengharapkan ID setelah ','.")?.clone()`
            let next_id = self.consume_token(TokenType::Identifier, "Mengharapkan identifier setelah ','.")?.clone();
            //    c. `rest.push((comma, next_id))`
            rest.push((comma, next_id));
        }
        
        // 4. Kembalikan `Ok(IdentifierList { initial_id, rest })`
        Ok(IdentifierList { initial_id, rest })
    }

    /// Mem-parse Tipe Data (integer, real, 'larik', subrange, dll.)
    pub(super) fn parse_type_spec(&mut self) -> Result<Type, SyntaxError> {
        // 1. `if self.check_keyword("integer") { return Ok(Type::Integer(self.advance().clone())) }`
        if self.check_keyword("integer") {
            return Ok(Type::Integer(self.advance().clone()));
        }
        // 2. Lakukan hal yang sama untuk `real`, `boolean`, `string`, `char`.
        if self.check_keyword("real") {
            return Ok(Type::Real(self.advance().clone()));
        }
        if self.check_keyword("boolean") {
            return Ok(Type::Boolean(self.advance().clone()));
        }
        if self.check_keyword("string") {
            return Ok(Type::String(self.advance().clone()));
        }
        if self.check_keyword("char") {
            return Ok(Type::Char(self.advance().clone()));
        }
        
        // 3. `else if self.check_keyword("larik")`:
        if self.check_keyword("larik") {
            //    - Panggil `self.parse_array_type()`
            return self.parse_array_type();
        }
        
        // 4. `else if self.check(TokenType::Identifier)`:
        if self.check(TokenType::Identifier) {
            //    - `return Ok(Type::TypeIdentifier(self.advance().clone()))`
            return Ok(Type::TypeIdentifier(self.advance().clone()));
        }
        
        // 5. `else`: (Jika bukan keyword atau identifier, coba parse sebagai subrange)
        //    - `let range = self.parse_range()?`
        //    - `return Ok(Type::Subrange(range))`
        let range = self.parse_range()?;
        Ok(Type::Subrange(range))
    }

    /// Mem-parse: 'larik' '[' <range> ']' 'dari' <type> 
    /// (Dipanggil oleh `parse_type_spec`)
    fn parse_array_type(&mut self) -> Result<Type, SyntaxError> {
        // 1. `let larik_kw = self.consume_keyword("larik", ...).clone()`
        let larik_kw = self.consume_keyword("larik", "Mengharapkan keyword 'larik'.")?.clone();
        
        // 2. `let l_bracket = self.consume_token(TokenType::LBracket, ...).clone()`
        let l_bracket = self.consume_token(TokenType::LBracket, "Mengharapkan '[' setelah 'larik'.")?.clone();
        
        // 3. `let range = Box::new(self.parse_range()?)`
        let index_type = Box::new(self.parse_type_spec()?);
        
        // 4. `let r_bracket = self.consume_token(TokenType::RBracket, ...).clone()`
        let r_bracket = self.consume_token(TokenType::RBracket, "Mengharapkan ']' setelah range larik.")?.clone();
        
        // 5. `let dari_kw = self.consume_keyword("dari", ...).clone()`
        let dari_kw = self.consume_keyword("dari", "Mengharapkan keyword 'dari' setelah ']' pada deklarasi larik.")?.clone();
        
        // 6. `let base_type = Box::new(self.parse_type_spec()?)` (Panggilan REKURSIF)
        let base_type = Box::new(self.parse_type_spec()?);
        
        // 7. Kembalikan `Ok(Type::Array(ArrayType { ... }))`
        Ok(Type::Array(ArrayType {
            larik_kw,
            l_bracket,
            index_type,
            r_bracket,
            dari_kw,
            base_type,
        }))
    }

    /// Mem-parse: <expression> '..' <expression> 
    /// (Dipanggil oleh `parse_type_spec` atau `parse_array_type`)
    fn parse_range(&mut self) -> Result<Range, SyntaxError> {
        // 1. `let start = Box::new(self.parse_expression()?)`
        let start = Box::new(self.parse_expression()?);
        
        // 2. `let range_op = self.consume_token(TokenType::RangeOperator, "Mengharapkan '..'.")?.clone()`
        let range_op = self.consume_token(TokenType::RangeOperator, "Mengharapkan '..' untuk range.")?.clone();
        
        // 3. `let end = Box::new(self.parse_expression()?)`
        let end = Box::new(self.parse_expression()?);
        
        // 4. Kembalikan `Ok(Range { start, range_op, end })`
        Ok(Range { start, range_op, end })
    }
}