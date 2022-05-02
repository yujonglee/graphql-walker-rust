use graphql_tools::ast::SchemaVisitor;
use graphql_tools::static_graphql::schema::{
    Document, EnumType, EnumValue, Field, InputObjectType, InputValue, InterfaceType, ObjectType,
    ScalarType, UnionType,
};

struct Visitor;

#[derive(Debug)]
struct UserContext {
    collected_object_type: Vec<String>,
    collected_scalar_type: Vec<String>,
    collected_union_type: Vec<String>,
    collected_input_type: Vec<String>,
    collected_enum_type: Vec<String>,
    collected_enum_value: Vec<String>,
    collected_interface_type: Vec<String>,
    collected_object_type_field: Vec<String>,
    collected_interface_type_field: Vec<String>,
    collected_input_type_fields: Vec<String>,
}

impl Visitor {
    fn collect_visited_info(&self, document: &Document) -> UserContext {
        let mut collected = UserContext {
            collected_object_type: Vec::new(),
            collected_interface_type: Vec::new(),
            collected_object_type_field: Vec::new(),
            collected_interface_type_field: Vec::new(),
            collected_scalar_type: Vec::new(),
            collected_union_type: Vec::new(),
            collected_enum_type: Vec::new(),
            collected_enum_value: Vec::new(),
            collected_input_type: Vec::new(),
            collected_input_type_fields: Vec::new(),
        };

        self.visit_schema_document(document, &mut collected);

        collected
    }
}

impl SchemaVisitor<UserContext> for Visitor {
    fn enter_object_type(&self, _node: &ObjectType, _visitor_context: &mut UserContext) {
        _visitor_context
            .collected_object_type
            .push(_node.name.clone());
    }

    fn enter_object_type_field(
        &self,
        _node: &Field,
        _type_: &ObjectType,
        _visitor_context: &mut UserContext,
    ) {
        let field_id = format!("{}.{}", _type_.name.as_str(), _node.name.as_str());
        _visitor_context.collected_object_type_field.push(field_id);
    }

    fn enter_interface_type(&self, _node: &InterfaceType, _visitor_context: &mut UserContext) {
        _visitor_context
            .collected_interface_type
            .push(_node.name.clone());
    }

    fn enter_interface_type_field(
        &self,
        _node: &Field,
        _type_: &InterfaceType,
        _visitor_context: &mut UserContext,
    ) {
        _visitor_context
            .collected_interface_type_field
            .push(_node.name.clone());
    }

    fn enter_scalar_type(&self, _node: &ScalarType, _visitor_context: &mut UserContext) {
        _visitor_context
            .collected_scalar_type
            .push(_node.name.clone());
    }

    fn enter_union_type(&self, _node: &UnionType, _visitor_context: &mut UserContext) {
        _visitor_context
            .collected_union_type
            .push(_node.name.clone());
    }

    fn enter_enum_type(&self, _node: &EnumType, _visitor_context: &mut UserContext) {
        _visitor_context
            .collected_enum_type
            .push(_node.name.clone());
    }

    fn enter_enum_value(
        &self,
        _node: &EnumValue,
        _enum: &EnumType,
        _visitor_context: &mut UserContext,
    ) {
        let enum_value_id = format!("{}.{}", _enum.name.as_str(), _node.name.as_str());
        _visitor_context.collected_enum_value.push(enum_value_id);
    }

    fn enter_input_object_type(&self, _node: &InputObjectType, _visitor_context: &mut UserContext) {
        _visitor_context
            .collected_input_type
            .push(_node.name.clone());
    }

    fn enter_input_object_type_field(
        &self,
        _node: &InputValue,
        _input_type: &InputObjectType,
        _visitor_context: &mut UserContext,
    ) {
        let field_id = format!("{}.{}", _input_type.name.as_str(), _node.name.as_str());
        _visitor_context.collected_input_type_fields.push(field_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphql_tools::static_graphql::schema;

    #[test]
    fn snapshot() {
        let schema = include_str!("./examples/schema.graphql");
        let schema_document = schema::parse_schema::<String>(schema).unwrap();

        let visitor = Visitor {};
        let collected = visitor.collect_visited_info(&schema_document);

        insta::assert_debug_snapshot!(collected, @r###"
        UserContext {
            collected_object_type: [
                "Book",
                "Author",
            ],
            collected_scalar_type: [],
            collected_union_type: [],
            collected_input_type: [],
            collected_enum_type: [],
            collected_enum_value: [],
            collected_interface_type: [],
            collected_object_type_field: [
                "Book.title",
                "Book.author",
                "Author.name",
                "Author.books",
            ],
            collected_interface_type_field: [],
            collected_input_type_fields: [],
        }
        "###);
    }
}
