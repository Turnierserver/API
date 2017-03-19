#[macro_export]
macro_rules! mutation_output {
    (
        struct $name:ident {
            $($field_name:ident : $field_type:ty $(,)* ),*
        }
    ) => {
        struct $name {
            client_mutation_id: String,
            $( $field_name: $field_type ),*
        }
        graphql_object!($name: Context |&self| {
            field clientMutationId() -> &String { &self.client_mutation_id }
            $(
                field $field_name() -> &$field_type { &self.$field_name }
            )*
        });
    };
}