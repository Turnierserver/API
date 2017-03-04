#![allow(unknown_lints, redundant_closure_call)]
use juniper::FieldResult;

use super::Context;

use models::*;

pub struct Mutation;
graphql_object!(Mutation: Context as "Mutation" |&self| {
    field test_mutate(new_val: i64) -> FieldResult<bool> {
        Ok(new_val > 1)
    }

    field snake_oil(input: ()) -> SnakeOil { SnakeOil }

    field auth(&executor, input: AuthCredentials) -> FieldResult<AuthOutput> {
        let user = User::named(input.username, &executor.context().conn)
            .map_err(|_| "unauthorized".to_owned())?;

        if user.verify_pass(&*input.password) {
            let token = executor.context().try(|conn| user.token(conn).map(
                |token| token.hyphenated().to_string()
            ))?;
            executor.context().set_token_cookie(&*token);
            Ok(AuthOutput {
                token: token,
                client_mutation_id: input.client_mutation_id,
                user: user
            })
        } else {
            println!("invalid password");
            Err("unauthorized".to_owned())
        }
    }
});

#[derive(Debug)]
pub struct AuthOutput {
    client_mutation_id: String,
    token: String,
    user: User
}
graphql_object!(AuthOutput: Context as "AuthOutput" |&self| {
    field clientMutationId() -> &String { &self.client_mutation_id }
    field token() -> &String { &self.token }
    field user() -> &User { &self.user }
});


graphql_input_object!(
    struct AuthCredentials {
        client_mutation_id: String,
        username: String,
        password: String,
    }
);

// TODO
graphql_input_object!(struct SnakeOilInput {
    placebo: i64
});
struct SnakeOil;
graphql_object!(SnakeOil: Context |&self| {});