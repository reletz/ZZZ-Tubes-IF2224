use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;

impl PascalParser {
    /// 1. parse_declaration_part (Entry Point Utama)
    ///    Sesuai Spek: (const)* + (type)* + (var)* + (subprogram)*
    pub(super) fn parse_declaration_part(&mut self) -> Result<DeclarationPart, SyntaxError> {
        
        let mut const_declarations: Vec<ConstantDeclaration> = Vec::new();
        let mut type_declarations: Vec<TypeDeclaration> = Vec::new();
        let mut var_declarations: Vec<VariableDeclaration> = Vec::new();
        let mut subprogram_declarations: Vec<SubprogramDeclaration> = Vec::new();

        // TODO: Implement parse_declaration_part
        // 1. Loop `while self.check_keyword("konstanta")`:
        //    - Panggil `self.parse_constant_declaration_block()`
        //    - `const_declarations.push(...)`
        // 2. Loop `while self.check_keyword("tipe")`:
        //    - Panggil `self.parse_type_declaration_block()`
        //    - `type_declarations.push(...)`
        // 3. Loop `while self.check_keyword("variabel")`:
        //    - Panggil `self.parse_variable_declaration_block()`
        //    - `var_declarations.push(...)`
        // 4. Loop `while self.check_keyword("prosedur") || self.check_keyword("fungsi")`:
        //    - Panggil `self.parse_subprogram_declaration()`
        //    - `subprogram_declarations.push(...)`
        // 5. Kembalikan `Ok(DeclarationPart { ... })`

        unimplemented!("parse_declaration_part belum diimplementasikan")
    }

    // --- 1. CONSTANT Declarations ---

    fn parse_constant_declaration_block(&mut self) -> Result<ConstantDeclaration, SyntaxError> {
        // TODO: Implement parse_constant_declaration_block
        // 1. `self.consume_keyword("konstanta", ...)`
        // 2. `let mut constants = Vec::new()`.
        // 3. Loop `while self.check(TokenType::Identifier)`:
        //    a. `let name = self.consume_token(Identifier, ...).value.clone()`
        //    b. `self.consume_token(RelationalOperator, "Mengharapkan '='")` (pastikan value-nya "=")
        //    c. `let value = self.parse_expression()?`
        //    d. `self.consume_token(Semicolon, ...)`
        //    e. `constants.push(ConstantDefinition { name, value })`
        // 4. Kembalikan `Ok(ConstantDeclaration { constants })`
        
        unimplemented!("parse_constant_declaration_block belum diimplementasikan")
    }

    // --- 2. TYPE Declarations ---

    fn parse_type_declaration_block(&mut self) -> Result<TypeDeclaration, SyntaxError> {
        // TODO: Implement parse_type_declaration_block
        // 1. `self.consume_keyword("tipe", ...)`
        // 2. `let mut definitions = Vec::new()`.
        // 3. Loop `while self.check(TokenType::Identifier)`:
        //    a. `let name = self.consume_token(Identifier, ...).value.clone()`
        //    b. `self.consume_token(RelationalOperator, "Mengharapkan '='")` (pastikan value-nya "=")
        //    c. `let type_def = self.parse_type_spec()?`
        //    d. `self.consume_token(Semicolon, ...)`
        //    e. `definitions.push(TypeDefinition { name, type_def })`
        // 4. Kembalikan `Ok(TypeDeclaration { definitions })`

        unimplemented!("parse_type_declaration_block belum diimplementasikan")
    }

    // --- 3. VARIABLE Declarations ---

    fn parse_variable_declaration_block(&mut self) -> Result<VariableDeclaration, SyntaxError> {
        // TODO: Implement parse_variable_declaration_block
        // 1. `self.consume_keyword("variabel", ...)`
        // 2. `let mut groups = Vec::new()`.
        // 3. Loop `while self.check(TokenType::Identifier)`:
        //    a. `let identifiers = self.parse_identifier_list()?`
        //    b. `self.consume_token(Colon, ...)`
        //    c. `let var_type = self.parse_type_spec()?`
        //    d. `self.consume_token(Semicolon, ...)`
        //    e. `groups.push(VariableGroup { identifiers, var_type })`
        // 4. Kembalikan `Ok(VariableDeclaration { groups })`

        unimplemented!("parse_variable_declaration_block belum diimplementasikan")
    }

    // --- 4. SUBPROGRAM Declarations (Procedure / Function) ---

    fn parse_subprogram_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // TODO: Implement parse_subprogram_declaration (Router)
        // 1. `if self.check_keyword("prosedur")`:
        //    - `self.parse_procedure_declaration()`
        // 2. `else if self.check_keyword("fungsi")`:
        //    - `self.parse_function_declaration()`
        // 3. `else`:
        //    - `Err(...)`

        unimplemented!("parse_subprogram_declaration belum diimplementasikan")
    }

    fn parse_procedure_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // TODO: Implement parse_procedure_declaration
        // 1. `self.consume_keyword("prosedur", ...)`
        // 2. `let name = self.consume_token(Identifier, ...).value.clone()`
        // 3. `let parameters = self.parse_formal_parameter_list()?` (Sesuai Revisi 3, () wajib)
        // 4. `self.consume_token(Semicolon, "Mengharapkan ';' setelah header prosedur")`
        // 5. `let declarations = self.parse_declaration_part()?` (Panggilan REKURSIF)
        // 6. `let body = self.parse_compound_statement()?`
        // 7. `self.consume_token(Semicolon, "Mengharapkan ';' setelah blok prosedur")`
        // 8. Kembalikan `Ok(SubprogramDeclaration::Procedure(ProcedureDeclaration { ... }))`
        
        unimplemented!("parse_procedure_declaration belum diimplementasikan")
    }

    fn parse_function_declaration(&mut self) -> Result<SubprogramDeclaration, SyntaxError> {
        // TODO: Implement parse_function_declaration
        // 1. `self.consume_keyword("fungsi", ...)`
        // 2. `let name = self.consume_token(Identifier, ...).value.clone()`
        // 3. `let parameters = self.parse_formal_parameter_list()?` (Sesuai Revisi 3, () wajib)
        // 4. `self.consume_token(Colon, "Mengharapkan ':' untuk return type")`
        // 5. `let return_type = self.parse_type_spec()?`
        // 6. `self.consume_token(Semicolon, "Mengharapkan ';' setelah header fungsi")`
        // 7. `let declarations = self.parse_declaration_part()?` (Panggilan REKURSIF)
        // 8. `let body = self.parse_compound_statement()?`
        // 9. `self.consume_token(Semicolon, "Mengharapkan ';' setelah blok fungsi")`
        // 10. Kembalikan `Ok(SubprogramDeclaration::Function(FunctionDeclaration { ... }))`

        unimplemented!("parse_function_declaration belum diimplementasikan")
    }

    // --- HELPER UNTUK DECLARATIONS ---

    /// Mem-parse: '(' [param-group (';' param-group)*] ')'
    /// Sesuai Revisi 3, () wajib
    fn parse_formal_parameter_list(&mut self) -> Result<FormalParameterList, SyntaxError> {
        // TODO: Implement parse_formal_parameter_list
        // 1. `self.consume_token(LParenthesis, ...)`
        // 2. `let mut parameters = Vec::new()`.
        // 3. `if !self.check(RParenthesis)` (Handle list kosong `()`)
        // 4. Loop:
        //    a. `let id_list = self.parse_identifier_list()?`
        //    b. `self.consume_token(Colon, ...)`
        //    c. `let var_type = self.parse_type_spec()?`
        //    d. `parameters.push(FormalParameterGroup { identifiers: id_list, var_type })`
        //    e. `if !self.match_token(Semicolon)` `break;`
        // 5. `self.consume_token(RParenthesis, ...)`
        // 6. Kembalikan `Ok(FormalParameterList { parameters })`
        
        unimplemented!("parse_formal_parameter_list belum diimplementasikan")
    }

    /// Mem-parse: ID (',' ID)*
    fn parse_identifier_list(&mut self) -> Result<IdentifierList, SyntaxError> {
        // TODO: Implement parse_identifier_list
        // 1. `let mut identifiers = Vec::new()`.
        // 2. `identifiers.push(self.consume_token(Identifier, ...).value.clone())` (Harus ada minimal 1)
        // 3. Loop `while self.match_token(&[TokenType::Comma])`:
        //    - `identifiers.push(self.consume_token(Identifier, "Mengharapkan ID setelah ','").value.clone())`
        // 4. Kembalikan `Ok(IdentifierList { identifiers })`
        
        unimplemented!("parse_identifier_list belum diimplementasikan")
    }

    /// Mem-parse Tipe Data (integer, real, array, subrange, dll.)
    pub(super) fn parse_type_spec(&mut self) -> Result<Type, SyntaxError> {
        // TODO: Implement parse_type_spec (Router Tipe)
        // 1. `if self.match_keyword(&["integer"]) { Ok(Type::Integer) }`
        // 2. `else if self.match_keyword(&["real"]) { Ok(Type::Real) }`
        // 3. (Lakukan hal yang sama untuk `boolean`, `string`, `char`)
        // 4. `else if self.match_keyword(&["larik"])`:
        //    - Panggil `self.parse_array_type()`
        // 5. `else if self.check(TokenType::Identifier)`:
        //    - `let name = self.advance().value.clone()`
        //    - `Ok(Type::TypeIdentifier(name))`
        // 6. `else`:
        //    - Cek `Range`: `let start = self.parse_expression()` (Ini rumit karena `parse_expression` bisa memakan ID)
        //    - (Mungkin `parse_range` harus dipanggil di sini, atau `parse_expression` harus di-refine)
        //    - `if self.check(RangeOperator) { ... parse_range ... }`
        //    - `Err(self.error("Mengharapkan tipe data."))`

        unimplemented!("parse_type_spec belum diimplementasikan")
    }

    /// Mem-parse: '[' <range> ']' 'dari' <type>
    fn parse_array_type(&mut self) -> Result<Type, SyntaxError> {
        // TODO: Implement parse_array_type
        // 1. `self.consume_token(LBracket, ...)`
        // 2. `let range = self.parse_range()?`
        // 3. `self.consume_token(RBracket, ...)`
        // 4. `self.consume_keyword("dari", ...)`
        // 5. `let base_type = self.parse_type_spec()?` (Panggilan REKURSIF)
        // 6. Kembalikan `Ok(Type::Array(ArrayType { range: Box::new(range), base_type: Box::new(base_type) }))`

        unimplemented!("parse_array_type belum diimplementasikan")
    }

    /// Mem-parse: <expression> '..' <expression>
    fn parse_range(&mut self) -> Result<Range, SyntaxError> {
        // TODO: Implement parse_range
        // 1. `let start = self.parse_expression()?`
        // 2. `self.consume_token(RangeOperator, "Mengharapkan '..'")`
        // 3. `let end = self.parse_expression()?`
        // 4. Kembalikan `Ok(Range { start: Box::new(start), end: Box::new(end) })`
        
        unimplemented!("parse_range belum diimplementasikan")
    }
}