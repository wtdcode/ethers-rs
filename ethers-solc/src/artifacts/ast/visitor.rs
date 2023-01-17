use eyre::{Report, Result};

use super::*;

pub trait Visitor {
    type Error: std::error::Error;

    fn visit_source_unit(&mut self, _source_unit: &mut SourceUnit) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_source_unit_part(
        &mut self,
        _source_unit: &mut SourceUnitPart,
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
}

pub trait Visitable {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor;
}

macro_rules! impl_visitable {
    ($type:ty, $func:ident) => {
        impl Visitable for $type {
            fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
            where
                V: Visitor,
            {
                v.$func(self)
            }
        }
    };
}

struct VisitError {
    message: String,
}

// main entry point of the ast visit
impl Visitor for SourceUnit {
    type Error = VisitError;

    // fn visit_source_unit(&mut self, _source_unit: &mut SourceUnit) -> Result<(), Self::Error> {
    fn visit_source_unit(&mut self, _source_unit: &mut SourceUnit) -> Result<(), Self::Error> {
        Ok(())
    }
}

// impl_visitable!(SourceUnit, visit_source_unit);
impl_visitable!(SourceUnitPart, visit_source_unit_part);
impl_visitable!(Expression, visit_expression);
impl_visitable!(Statement, visit_statement);
impl_visitable!(ContractDefinition, visit_contract_definition);
impl_visitable!(ContractDefinitionPart, visit_contract_definition_part);
impl_visitable!(TypeName, visit_type_name);
impl_visitable!(
    UserDefinedTypeNameOrIdentifierPath,
    visit_user_defined_type_name_or_identifier_path
);
impl_visitable!(
    ExpressionOrVariableDeclarationStatement,
    visit_expression_or_variable_declaration_statement
);
impl_visitable!(IdentifierOrIdentifierPath, visit_identifer_or_identifier_path);
impl_visitable!(InheritanceSpecifier, visit_inheritance_specifier);
impl_visitable!(Assignment, visit_assignment);
impl_visitable!(AssignmentOperator, visit_assignment_operator);
impl_visitable!(BinaryOperator, visit_binary_operator);
