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

#[derive(Error, Debug, Clone)]
pub enum VisitError {
    #[error("{0}")]
    MsgError(String),
    #[error("")]
    Unknown,
}

macro_rules! impl_visitor {
    ($type:ty) => {
        paste::paste! {
        fn [<visit_ $type:snake>](&mut self, node_type: &mut $type) -> Result<(), VisitError> {
            node_type.visit(self)
        }
        }
    };
}

pub trait Visitor<D> {
    fn shared_data(&mut self) -> &D;

    fn visit_source_unit(&mut self, _source_unit: &mut SourceUnit) -> Result<(), VisitError> {
        Ok(())
    }

    impl_visitor!(PragmaDirective);
    impl_visitor!(ImportDirective);
    impl_visitor!(SourceUnitPart);
    impl_visitor!(UsingForDirective);
    impl_visitor!(VariableDeclaration);
    impl_visitor!(BinaryOperation);
    impl_visitor!(Conditional);
    impl_visitor!(ElementaryTypeName);
    impl_visitor!(ElementaryTypeNameExpression);
    impl_visitor!(FunctionCall);
    impl_visitor!(UnaryOperation);
    impl_visitor!(TupleExpression);
    impl_visitor!(NewExpression);
    impl_visitor!(MemberAccess);
    impl_visitor!(TypeDescriptions);
    impl_visitor!(Literal);
    impl_visitor!(IndexRangeAccess);
    impl_visitor!(IndexAccess);
    impl_visitor!(Identifier);
    impl_visitor!(FunctionCallOptions);
    impl_visitor!(EnumDefinition);
    impl_visitor!(EventDefinition);
    impl_visitor!(ModifierDefinition);
    impl_visitor!(ErrorDefinition);
    impl_visitor!(FunctionDefinition);
    impl_visitor!(StructDefinition);
    impl_visitor!(Expression);
    impl_visitor!(Statement);
    impl_visitor!(ContractDefinition);
    impl_visitor!(ContractDefinitionPart);
    impl_visitor!(TypeName);
    impl_visitor!(UserDefinedValueTypeDefinition);
    impl_visitor!(UserDefinedTypeNameOrIdentifierPath);
    impl_visitor!(ExpressionOrVariableDeclarationStatement);
    impl_visitor!(IdentifierOrIdentifierPath);
    impl_visitor!(InheritanceSpecifier);
    impl_visitor!(UserDefinedTypeName);
    impl_visitor!(Mapping);
    impl_visitor!(FunctionTypeName);
    impl_visitor!(ArrayTypeName);
    impl_visitor!(Assignment);
    impl_visitor!(AssignmentOperator);
    impl_visitor!(BinaryOperator);
    impl_visitor!(PlaceholderStatement);
    impl_visitor!(InlineAssembly);
    impl_visitor!(IfStatement);
    impl_visitor!(ForStatement);
    impl_visitor!(ExpressionStatement);
    impl_visitor!(EmitStatement);
    impl_visitor!(DoWhileStatement);
    impl_visitor!(Continue);
    impl_visitor!(Break);
    impl_visitor!(Block);
    impl_visitor!(Return);
    impl_visitor!(RevertStatement);
    impl_visitor!(TryStatement);
    impl_visitor!(UncheckedBlock);
    impl_visitor!(VariableDeclarationStatement);
    impl_visitor!(WhileStatement);
    impl_visitor!(IdentifierPath);
    impl_visitor!(YulAssignment);
    impl_visitor!(YulBlock);
    impl_visitor!(YulBreak);
    impl_visitor!(YulContinue);
    impl_visitor!(YulExpressionStatement);
    impl_visitor!(YulFunctionDefinition);
    impl_visitor!(YulForLoop);
    impl_visitor!(YulIf);
    impl_visitor!(YulSwitch);
    impl_visitor!(YulVariableDeclaration);
    impl_visitor!(YulFunctionCall);
    impl_visitor!(YulIdentifier);
    impl_visitor!(YulLiteral);
    impl_visitor!(YulLeave);
}

pub trait Visitable {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized;
}

/// Helper for nodes that don't need much implementation to traverse the childrens
macro_rules! empty_visitable {
    ($type:ty) => {
        impl Visitable for $type {
            fn visit<V, D>(&mut self, _: &mut V) -> Result<(), VisitError>
            where
                V: Visitor<D> + ?Sized,
            {
                Ok(())
            }
        }
    };
}

empty_visitable!(PragmaDirective);
empty_visitable!(ElementaryTypeName);
empty_visitable!(ElementaryTypeNameExpression);
empty_visitable!(EnumValue);
empty_visitable!(Literal);
empty_visitable!(Identifier);
empty_visitable!(EventDefinition);
empty_visitable!(ModifierDefinition);
empty_visitable!(UserDefinedTypeName);
empty_visitable!(Mapping);
empty_visitable!(FunctionTypeName);
empty_visitable!(AssignmentOperator);
empty_visitable!(TypeDescriptions);
empty_visitable!(BinaryOperator);
empty_visitable!(PlaceholderStatement);
empty_visitable!(Continue);
empty_visitable!(Break);
empty_visitable!(IdentifierPath);
empty_visitable!(YulBreak);
empty_visitable!(YulExpressionStatement);
empty_visitable!(YulFunctionDefinition);
empty_visitable!(YulIf);
empty_visitable!(YulSwitch);
empty_visitable!(YulVariableDeclaration);
empty_visitable!(YulIdentifier);
empty_visitable!(YulLiteral);

impl<T> Visitable for Vec<T>
where
    T: Visitable,
{
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        for item in self.iter_mut() {
            item.visit(v)?;
        }
        Ok(())
    }
}

// impl<D> Visitor<D> for SourceUnit {
//     type Error = VisitError;
// }

/// Main entry point of the ast
impl Visitable for SourceUnit {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        v.visit_source_unit(self)?;
        self.nodes.visit(v)
    }
}

impl Visitable for SourceUnitPart {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
                v.visit_user_defined_value_type_definition(e)
            }
            SourceUnitPart::ContractDefinition(e) => v.visit_contract_definition(e),
        }
    }
}

impl Visitable for Expression {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
            Statement::PlaceholderStatement(e) => v.visit_placeholder_statement(e),
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        match self {
            IdentifierOrIdentifierPath::Identifier(e) => v.visit_identifier(e),
            IdentifierOrIdentifierPath::IdentifierPath(e) => v.visit_identifier_path(e),
        }
    }
}

impl Visitable for YulStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        match self {
            YulExpression::YulFunctionCall(e) => v.visit_yul_function_call(e),
            YulExpression::YulIdentifier(e) => v.visit_yul_identifier(e),
            YulExpression::YulLiteral(e) => v.visit_yul_literal(e),
        }
    }
}

/// Implement nodes that may have sub nodes
impl Visitable for ImportDirective {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.symbol_aliases.iter_mut().try_for_each(|sa| {
            let foreign = &mut sa.foreign;
            v.visit_identifier(foreign)
        })
    }
}

impl Visitable for UsingForDirective {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.function_list.visit(v)?;
        self.library_name.visit(v)?;
        self.type_name.visit(v)
    }
}

impl Visitable for FunctionIdentifierPath {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.function.visit(v)
    }
}

impl Visitable for VariableDeclaration {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.overrides.visit(v)?;
        self.type_name.visit(v)?;
        self.value.visit(v)
    }
}

impl Visitable for EnumDefinition {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.members.visit(v)
    }
}

impl Visitable for ErrorDefinition {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.parameters.parameters.visit(v)
    }
}

impl Visitable for ModifierInvocation {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.arguments.visit(v)?;
        self.modifier_name.visit(v)
    }
}

impl Visitable for Block {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.statements.visit(v)
    }
}

impl Visitable for OverrideSpecifier {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.as_mut().map_or_else(|| Ok(()), |val| val.visit(v))
    }
}

impl Visitable for ParameterList {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.parameters.visit(v)
    }
}

impl Visitable for FunctionDefinition {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.body.visit(v)?;
        self.modifiers.visit(v)?;
        self.overrides.visit(v)?;
        self.parameters.visit(v)?;
        self.return_parameters.visit(v)
    }
}

impl Visitable for StructDefinition {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.members.visit(v)
    }
}

impl Visitable for UserDefinedValueTypeDefinition {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.underlying_type.visit(v)
    }
}

impl Visitable for ArrayTypeName {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.type_descriptions.visit(v)?;
        self.length.visit(v)
    }
}

impl Visitable for InheritanceSpecifier {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.arguments.visit(v)?;
        self.base_name.visit(v)
    }
}

impl Visitable for ContractDefinition {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.base_contracts.visit(v)?;
        self.nodes.visit(v)
    }
}

macro_rules! impl_visitable_expression {
    ($type:ty) => {
        impl Visitable for $type {
            fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
            where
                V: Visitor<D> + ?Sized,
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
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.condition.visit(v)?;
        self.false_expression.visit(v)?;
        self.false_expression.visit(v)
    }
}

impl Visitable for FunctionCall {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.arguments.visit(v)?;
        self.expression.visit(v)
    }
}

impl Visitable for FunctionCallOptions {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.expression.visit(v)?;
        self.options.visit(v)
    }
}

impl Visitable for IndexAccess {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.base_expression.visit(v)?;
        self.index_expression.visit(v)
    }
}

impl Visitable for IndexRangeAccess {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.base_expression.visit(v)?;
        self.start_expression.visit(v)?;
        self.end_expression.visit(v)
    }
}

impl Visitable for MemberAccess {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.expression.visit(v)
    }
}

impl Visitable for NewExpression {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.type_name.visit(v)
    }
}

impl Visitable for TupleExpression {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.components.visit(v)
    }
}

impl Visitable for UnaryOperation {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.sub_expression.visit(v)
    }
}

impl Visitable for DoWhileStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.block.visit(v)?;
        self.condition.visit(v)
    }
}

impl Visitable for EmitStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.event_call.visit(v)
    }
}

impl Visitable for ExpressionStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.expression.visit(v)
    }
}

impl Visitable for BlockOrStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        match self {
            BlockOrStatement::Block(e) => e.visit(v),
            BlockOrStatement::Statement(e) => e.visit(v),
        }
    }
}

impl Visitable for ForStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.body.visit(v)?;
        self.condition.visit(v)?;
        self.initialization_expression.visit(v)?;
        self.loop_expression.visit(v)
    }
}

impl Visitable for VariableDeclarationStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.declarations.visit(v)?;
        self.initial_value.visit(v)
    }
}

impl Visitable for IfStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.condition.visit(v)?;
        self.false_body.visit(v)?;
        self.true_body.visit(v)
    }
}

impl Visitable for YulBlock {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.statements.visit(v)
    }
}

impl Visitable for YulAssignment {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.value.visit(v)?;
        self.variable_names.visit(v)
    }
}

impl Visitable for YulForLoop {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.body.visit(v)?;
        self.condition.visit(v)?;
        self.post.visit(v)?;
        self.pre.visit(v)
    }
}

impl Visitable for YulFunctionCall {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.arguments.visit(v)?;
        self.function_name.visit(v)
    }
}

impl Visitable for InlineAssembly {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.ast.visit(v)
    }
}

impl Visitable for Return {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.expression.visit(v)
    }
}

impl Visitable for RevertStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.error_call.visit(v)
    }
}

impl Visitable for TryCatchClause {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.block.visit(v)?;
        self.parameters.visit(v)
    }
}

impl Visitable for TryStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.clauses.visit(v)?;
        self.external_call.visit(v)
    }
}

impl Visitable for UncheckedBlock {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.statements.visit(v)
    }
}

impl Visitable for WhileStatement {
    fn visit<V, D>(&mut self, v: &mut V) -> Result<(), VisitError>
    where
        V: Visitor<D> + ?Sized,
    {
        self.body.visit(v)?;
        self.condition.visit(v)
    }
}
