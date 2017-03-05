#![allow(unknown_lints, redundant_closure_call)]
use juniper::FieldResult;

use super::Context;

use models::*;

pub struct Mutation;
graphql_object!(Mutation: Context as "Mutation" |&self| {
    field auth(&executor, input: AuthCredentials) -> FieldResult<AuthOutput> {
        if let Ok(user) = User::named(input.username, &executor.context().conn) {
            if user.verify_pass(&*input.password) {
                let token = executor.context().try(|conn| user.token(conn).map(
                    |token| token.hyphenated().to_string()
                ))?;
                executor.context().set_token_cookie(&*token);
                return Ok(AuthOutput {
                    client_mutation_id: input.client_mutation_id,
                    token: Some(token),
                    user: Some(user)
                })
            }
        }
        Ok(AuthOutput {
            client_mutation_id: input.client_mutation_id,
            token: None,
            user: None
        })
    }
});

#[derive(Debug)]
pub struct AuthOutput {
    client_mutation_id: String,
    token: Option<String>,
    user: Option<User>
}
graphql_object!(AuthOutput: Context as "AuthOutput" |&self| {
    field clientMutationId() -> &String { &self.client_mutation_id }
    field token() -> &Option<String> { &self.token }
    field user() -> &Option<User> { &self.user }
});


graphql_input_object!(
    struct AuthCredentials {
        client_mutation_id: String,
        username: String,
        password: String,
    }
);
