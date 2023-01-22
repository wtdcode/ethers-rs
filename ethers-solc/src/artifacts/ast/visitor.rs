use super::{
    yul::{
        YulAssignment, YulBreak, YulContinue, YulExpression, YulExpressionStatement, YulForLoop,
        YulFunctionCall, YulFunctionDefinition, YulIdentifier, YulIf, YulLeave, YulLiteral,
        YulStatement, YulSwitch, YulVariableDeclaration,
    },
    *,
};
use eyre::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VisitError {
    #[error("{0}")]
    MsgError(String),
    #[error("")]
    Unknown,
}

pub trait Visitor {
    type Error: std::error::Error;

    fn visit_source_unit(&mut self, _source_unit: &mut SourceUnit) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_pragma_directive(
        &mut self,
        _pragma_directive: &mut PragmaDirective,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_import_directive(
        &mut self,
        _import_directive: &mut ImportDirective,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_source_unit_part(
        &mut self,
        _source_unit_part: &mut SourceUnitPart,
    ) -> Result<(), Self::Error> {
        // source_unit_part.visit(self)
        Ok(())
    }

    fn visit_using_for_directive(
        &mut self,
        _source_unit: &mut UsingForDirective,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_variable_declaration(
        &mut self,
        _source_unit: &mut VariableDeclaration,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_binary_operation(
        &mut self,
        _source_unit: &mut BinaryOperation,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_conditional(&mut self, _source_unit: &mut Conditional) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        _source_unit: &mut ElementaryTypeNameExpression,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_function_call(&mut self, _source_unit: &mut FunctionCall) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_unary_operation(
        &mut self,
        _source_unit: &mut UnaryOperation,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_tuple_expression(
        &mut self,
        _source_unit: &mut TupleExpression,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_new_expression(
        &mut self,
        _source_unit: &mut NewExpression,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_member_access(&mut self, _source_unit: &mut MemberAccess) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_literal(&mut self, _source_unit: &mut Literal) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_index_range_access(
        &mut self,
        _source_unit: &mut IndexRangeAccess,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_index_access(&mut self, _source_unit: &mut IndexAccess) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_identifier(&mut self, _identifier: &mut Identifier) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_function_call_options(
        &mut self,
        _source_unit: &mut FunctionCallOptions,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_definition(
        &mut self,
        _source_unit: &mut EnumDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_event_definition(
        &mut self,
        _source_unit: &mut EventDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_modifier_definition(
        &mut self,
        _source_unit: &mut ModifierDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_error_definition(
        &mut self,
        _source_unit: &mut ErrorDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_function_definition(
        &mut self,
        _source_unit: &mut FunctionDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_struct_definition(
        &mut self,
        _source_unit: &mut StructDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_expression(&mut self, _expression: &mut Expression) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_statement(&mut self, _statement: &mut Statement) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_contract_definition(
        &mut self,
        _contract_definition: &mut ContractDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_contract_definition_part(
        &mut self,
        _contract_definition_part: &mut ContractDefinitionPart,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_type_name(&mut self, _type_name: &mut TypeName) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_user_defined_type_definition(
        &mut self,
        _type_name: &mut UserDefinedValueTypeDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        _type_name: &mut UserDefinedValueTypeDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_user_defined_type_name_or_identifier_path(
        &mut self,
        _user_defined_type_name_or_identifier_path: &mut UserDefinedTypeNameOrIdentifierPath,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_expression_or_variable_declaration_statement(
        &mut self,
        _expression_or_variable_declaration_statement: &mut ExpressionOrVariableDeclarationStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_identifer_or_identifier_path(
        &mut self,
        _identifier_or_identifier_path: &mut IdentifierOrIdentifierPath,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_inheritance_specifier(
        &mut self,
        _inheritance_specifier: &mut InheritanceSpecifier,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_user_defined_type_name(
        &mut self,
        _inheritance_specifier: &mut UserDefinedTypeName,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_mapping(&mut self, _inheritance_specifier: &mut Mapping) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_function_type_name(
        &mut self,
        _inheritance_specifier: &mut FunctionTypeName,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_elementary_type_name(
        &mut self,
        _inheritance_specifier: &mut ElementaryTypeName,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_array_type_name(
        &mut self,
        _inheritance_specifier: &mut ArrayTypeName,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_assignment(&mut self, _assignment: &mut Assignment) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_assignment_operator(
        &mut self,
        _assignment_operator: &mut AssignmentOperator,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_binary_operator(
        &mut self,
        _binary_operator: &mut BinaryOperator,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_place_holder_statement(
        &mut self,
        _binary_operator: &mut PlaceholderStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_inline_assembly(
        &mut self,
        _binary_operator: &mut InlineAssembly,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_if_statement(
        &mut self,
        _binary_operator: &mut IfStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_for_statement(
        &mut self,
        _binary_operator: &mut ForStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_expression_statement(
        &mut self,
        _binary_operator: &mut ExpressionStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_emit_statement(
        &mut self,
        _binary_operator: &mut EmitStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_do_while_statement(
        &mut self,
        _binary_operator: &mut DoWhileStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_continue(&mut self, _binary_operator: &mut Continue) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_break(&mut self, _binary_operator: &mut Break) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_block(&mut self, _binary_operator: &mut Block) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_return(&mut self, _binary_operator: &mut Return) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_revert_statement(
        &mut self,
        _binary_operator: &mut RevertStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_try_statement(
        &mut self,
        _binary_operator: &mut TryStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_unchecked_block(
        &mut self,
        _binary_operator: &mut UncheckedBlock,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_variable_declaration_statement(
        &mut self,
        _binary_operator: &mut VariableDeclarationStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_while_statement(
        &mut self,
        _binary_operator: &mut WhileStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_identifier_path(
        &mut self,
        _binary_operator: &mut IdentifierPath,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_assignment(
        &mut self,
        _binary_operator: &mut YulAssignment,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_block(&mut self, _binary_operator: &mut YulBlock) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_break(&mut self, _binary_operator: &mut YulBreak) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_continue(
        &mut self,
        _binary_operator: &mut YulContinue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_expression_statement(
        &mut self,
        _binary_operator: &mut YulExpressionStatement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_leave(&mut self, _binary_operator: &mut YulLeave) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_function_definition(
        &mut self,
        _binary_operator: &mut YulFunctionDefinition,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_for_loop(&mut self, _binary_operator: &mut YulForLoop) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_if(&mut self, _binary_operator: &mut YulIf) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_switch(&mut self, _binary_operator: &mut YulSwitch) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_variable_declaration(
        &mut self,
        _binary_operator: &mut YulVariableDeclaration,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_function_call(
        &mut self,
        _binary_operator: &mut YulFunctionCall,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_identifier(
        &mut self,
        _binary_operator: &mut YulIdentifier,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_literal(&mut self, _binary_operator: &mut YulLiteral) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait Visitable {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor;
}

impl<T> Visitable for Vec<T>
where
    T: Visitable,
{
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        for item in self.iter_mut() {
            item.visit(v)?;
        }
        Ok(())
    }
}

/// Helper for nodes that don't need much implementation to traverse the childrens
// macro_rules! impl_visitable {
//     ($type:ty, $func:ident) => {
//         impl Visitable for $type {
//             fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
//             where
//                 V: Visitor,
//             {
//                 v.$func(self)
//             }
//         }
//     };
// }

// Writing a macro to do that is not trivial at all
impl Visitable for SourceUnitPart {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            SourceUnitPart::PragmaDirective(e) => v.visit_pragma_directive(e),
            SourceUnitPart::ImportDirective(e) => v.visit_import_directive(e),
            SourceUnitPart::UsingForDirective(e) => v.visit_using_for_directive(e),
            SourceUnitPart::VariableDeclaration(e) => v.visit_variable_declaration(e),
            SourceUnitPart::EnumDefinition(e) => v.visit_enum_definition(e),
            SourceUnitPart::ErrorDefinition(e) => v.visit_error_definition(e),
            SourceUnitPart::FunctionDefinition(e) => v.visit_function_definition(e),
            SourceUnitPart::StructDefinition(e) => v.visit_struct_definition(e),
            SourceUnitPart::UserDefinedValueTypeDefinition(e) => {
                v.visit_user_defined_type_definition(e)
            }
            SourceUnitPart::ContractDefinition(e) => v.visit_contract_definition(e),
        }
    }
}

impl Visitable for Expression {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            Expression::Assignment(e) => v.visit_assignment(e),
            Expression::BinaryOperation(e) => v.visit_binary_operation(e),
            Expression::Conditional(e) => v.visit_conditional(e),
            Expression::ElementaryTypeNameExpression(e) => {
                v.visit_elementary_type_name_expression(e)
            }
            Expression::FunctionCall(e) => v.visit_function_call(e),
            Expression::FunctionCallOptions(e) => v.visit_function_call_options(e),
            Expression::Identifier(e) => v.visit_identifier(e),
            Expression::IndexAccess(e) => v.visit_index_access(e),
            Expression::IndexRangeAccess(e) => v.visit_index_range_access(e),
            Expression::Literal(e) => v.visit_literal(e),
            Expression::MemberAccess(e) => v.visit_member_access(e),
            Expression::NewExpression(e) => v.visit_new_expression(e),
            Expression::TupleExpression(e) => v.visit_tuple_expression(e),
            Expression::UnaryOperation(e) => v.visit_unary_operation(e),
        }
    }
}

impl Visitable for Statement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            Statement::Block(e) => v.visit_block(e),
            Statement::Break(e) => v.visit_break(e),
            Statement::Continue(e) => v.visit_continue(e),
            Statement::DoWhileStatement(e) => v.visit_do_while_statement(e),
            Statement::EmitStatement(e) => v.visit_emit_statement(e),
            Statement::ExpressionStatement(e) => v.visit_expression_statement(e),
            Statement::ForStatement(e) => v.visit_for_statement(e),
            Statement::IfStatement(e) => v.visit_if_statement(e),
            Statement::InlineAssembly(e) => v.visit_inline_assembly(e),
            Statement::PlaceholderStatement(e) => v.visit_place_holder_statement(e),
            Statement::Return(e) => v.visit_return(e),
            Statement::RevertStatement(e) => v.visit_revert_statement(e),
            Statement::TryStatement(e) => v.visit_try_statement(e),
            Statement::UncheckedBlock(e) => v.visit_unchecked_block(e),
            Statement::VariableDeclarationStatement(e) => v.visit_variable_declaration_statement(e),
            Statement::WhileStatement(e) => v.visit_while_statement(e),
        }
    }
}

impl Visitable for ContractDefinitionPart {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            ContractDefinitionPart::EnumDefinition(e) => v.visit_enum_definition(e),
            ContractDefinitionPart::ErrorDefinition(e) => v.visit_error_definition(e),
            ContractDefinitionPart::EventDefinition(e) => v.visit_event_definition(e),
            ContractDefinitionPart::FunctionDefinition(e) => v.visit_function_definition(e),
            ContractDefinitionPart::ModifierDefinition(e) => v.visit_modifier_definition(e),
            ContractDefinitionPart::StructDefinition(e) => v.visit_struct_definition(e),
            ContractDefinitionPart::UserDefinedValueTypeDefinition(e) => {
                v.visit_user_defined_value_type_definition(e)
            }
            ContractDefinitionPart::UsingForDirective(e) => v.visit_using_for_directive(e),
            ContractDefinitionPart::VariableDeclaration(e) => v.visit_variable_declaration(e),
        }
    }
}

impl Visitable for TypeName {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            TypeName::ArrayTypeName(e) => v.visit_array_type_name(e),
            TypeName::ElementaryTypeName(e) => v.visit_elementary_type_name(e),
            TypeName::FunctionTypeName(e) => v.visit_function_type_name(e),
            TypeName::Mapping(e) => v.visit_mapping(e),
            TypeName::UserDefinedTypeName(e) => v.visit_user_defined_type_name(e),
        }
    }
}

impl Visitable for UserDefinedTypeNameOrIdentifierPath {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(e) => {
                v.visit_user_defined_type_name(e)
            }
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(e) => v.visit_identifier_path(e),
        }
    }
}

impl Visitable for ExpressionOrVariableDeclarationStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            ExpressionOrVariableDeclarationStatement::ExpressionStatement(e) => {
                v.visit_expression_statement(e)
            }
            ExpressionOrVariableDeclarationStatement::VariableDeclarationStatement(e) => {
                v.visit_variable_declaration_statement(e)
            }
        }
    }
}

impl Visitable for IdentifierOrIdentifierPath {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            IdentifierOrIdentifierPath::Identifier(e) => v.visit_identifier(e),
            IdentifierOrIdentifierPath::IdentifierPath(e) => v.visit_identifier_path(e),
        }
    }
}

impl Visitable for YulStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            YulStatement::YulAssignment(e) => v.visit_yul_assignment(e),
            YulStatement::YulBlock(e) => v.visit_yul_block(e),
            YulStatement::YulBreak(e) => v.visit_yul_break(e),
            YulStatement::YulContinue(e) => v.visit_yul_continue(e),
            YulStatement::YulExpressionStatement(e) => v.visit_yul_expression_statement(e),
            YulStatement::YulLeave(e) => v.visit_yul_leave(e),
            YulStatement::YulForLoop(e) => v.visit_yul_for_loop(e),
            YulStatement::YulFunctionDefinition(e) => v.visit_yul_function_definition(e),
            YulStatement::YulIf(e) => v.visit_yul_if(e),
            YulStatement::YulSwitch(e) => v.visit_yul_switch(e),
            YulStatement::YulVariableDeclaration(e) => v.visit_yul_variable_declaration(e),
        }
    }
}

impl Visitable for YulExpression {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            YulExpression::YulFunctionCall(e) => v.visit_yul_function_call(e),
            YulExpression::YulIdentifier(e) => v.visit_yul_identifier(e),
            YulExpression::YulLiteral(e) => v.visit_yul_literal(e),
        }
    }
}

impl Visitor for SourceUnit {
    type Error = VisitError;
}

/// Main entry point of the ast
impl Visitable for SourceUnit {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        v.visit_source_unit(self)?;
        self.nodes.visit(v)
    }
}

/// Implement nodes that may have sub nodes
impl Visitable for ImportDirective {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.symbol_aliases.iter_mut().try_for_each(|sa| {
            let foreign = &mut sa.foreign;
            v.visit_identifier(foreign)
        })
    }
}

impl Visitable for UsingForDirective {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        if let Some(ln) = &mut self.library_name {
            ln.visit(v)?;
        }

        self.type_name.visit(v)
    }
}

impl Visitable for VariableDeclaration {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        if let Some(overrides) = &mut self.overrides {
            overrides.visit(v)?;
        }

        if let Some(tn) = &mut self.type_name {
            return tn.visit(v)
        }

        self.value.visit(v)
    }
}

impl Visitable for ErrorDefinition {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.parameters.parameters.visit(v)
    }
}

impl Visitable for ModifierInvocation {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.arguments.visit(v)?;
        self.modifier_name.visit(v)
    }
}

impl Visitable for Block {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.statements.visit(v)
    }
}

impl Visitable for OverrideSpecifier {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        for ov in self.overrides.iter_mut() {
            ov.visit(v)?;
        }
        Ok(())
    }
}

impl<T> Visitable for Option<T>
where
    T: Visitable,
{
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.as_mut().map_or_else(|| Ok(()), |val| val.visit(v))
    }
}

impl Visitable for ParameterList {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.parameters.visit(v)
    }
}

impl Visitable for FunctionDefinition {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.body.visit(v)?;
        self.modifiers.visit(v)?;
        self.overrides.visit(v)?;
        self.parameters.visit(v)?;
        self.return_parameters.visit(v)
    }
}

impl Visitable for StructDefinition {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.members.visit(v)
    }
}

impl Visitable for UserDefinedValueTypeDefinition {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.underlying_type.visit(v)
    }
}

impl Visitable for InheritanceSpecifier {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.arguments.visit(v)?;
        self.base_name.visit(v)
    }
}

impl Visitable for ContractDefinition {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.base_contracts.visit(v)?;
        self.nodes.visit(v)
    }
}

macro_rules! impl_visitable_expression {
    ($type:ty) => {
        impl Visitable for $type {
            fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
            where
                V: Visitor,
            {
                self.lhs.visit(v)?;
                self.rhs.visit(v)
            }
        }
    };
}

impl_visitable_expression!(Assignment);
impl_visitable_expression!(BinaryOperation);

impl Visitable for Conditional {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.condition.visit(v)?;
        self.false_expression.visit(v)?;
        self.false_expression.visit(v)
    }
}

impl Visitable for FunctionCall {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.arguments.visit(v)?;
        self.expression.visit(v)
    }
}

impl Visitable for FunctionCallOptions {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.expression.visit(v)?;
        self.options.visit(v)
    }
}

impl Visitable for IndexAccess {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.base_expression.visit(v)?;
        self.index_expression.visit(v)
    }
}

impl Visitable for IndexRangeAccess {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.base_expression.visit(v)?;
        self.start_expression.visit(v)?;
        self.end_expression.visit(v)
    }
}

impl Visitable for MemberAccess {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.expression.visit(v)
    }
}

impl Visitable for NewExpression {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.type_name.visit(v)
    }
}

impl Visitable for TupleExpression {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.components.visit(v)
    }
}

impl Visitable for UnaryOperation {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.sub_expression.visit(v)
    }
}

impl Visitable for DoWhileStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.block.visit(v)?;
        self.condition.visit(v)
    }
}

impl Visitable for EmitStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.event_call.visit(v)
    }
}

impl Visitable for ExpressionStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.expression.visit(v)
    }
}

impl Visitable for BlockOrStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            BlockOrStatement::Block(e) => e.visit(v),
            BlockOrStatement::Statement(e) => e.visit(v),
        }
    }
}

impl Visitable for ForStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.body.visit(v)?;
        self.condition.visit(v)?;
        self.initialization_expression.visit(v)?;
        self.loop_expression.visit(v)
    }
}

impl Visitable for VariableDeclarationStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.declarations.visit(v)?;
        self.initial_value.visit(v)
    }
}

impl Visitable for IfStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.condition.visit(v)?;
        self.false_body.visit(v)?;
        self.true_body.visit(v)
    }
}

impl Visitable for YulBlock {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.statements.visit(v)
    }
}

impl Visitable for InlineAssembly {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.ast.visit(v)
    }
}

impl Visitable for Return {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.expression.visit(v)
    }
}

impl Visitable for RevertStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.error_call.visit(v)
    }
}

impl Visitable for TryCatchClause {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.block.visit(v)?;
        self.parameters.visit(v)
    }
}

impl Visitable for TryStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.clauses.visit(v)?;
        self.external_call.visit(v)
    }
}

impl Visitable for UncheckedBlock {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.statements.visit(v)
    }
}

impl Visitable for WhileStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        self.body.visit(v)?;
        self.condition.visit(v)
    }
}
