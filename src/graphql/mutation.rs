#![allow(unknown_lints, redundant_closure_call)]
use juniper::{FieldResult, ID};
use diesel::{self, SaveChangesDsl};

use super::*;
use models::*;
use self::diesel::prelude::*;

macro_rules! update_object {
    ($obj:ident, $changes:ident, [ $( $key:ident ),* ]) => {
        $(if let Some(val) = $changes.$key {
            $obj.$key = val;
        })*
    }
}


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

    field register(&executor, input: RegisterInput) -> FieldResult<EditUserOutput> {
        use schema::users;
        let new_user = insert::User {
            username: &input.username,
            email: &input.email,
            admin: false
        };
        let mut user = executor.context().try(|conn| {
            diesel::insert(&new_user).into(users::table)
                .get_result::<User>(conn)
        })?;
        let user = user.set_pass(&input.password, &executor.context().conn)
            .map_err(|_| "database failure")?;
        Ok(EditUserOutput {
            client_mutation_id: input.client_mutation_id,
            user: user
        })
    }

    field edit_user(&executor, input: EditUserInput) -> FieldResult<EditUserOutput> {
        let mut user = executor.context().access_user(&input.user_id)?;
        update_object!(user, input, [
            email, firstname, lastname, name_public
        ]);
        executor.context().try(|conn| user.save_changes::<User>(conn))?;
        Ok(EditUserOutput {
            client_mutation_id: input.client_mutation_id,
            user: user
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

graphql_input_object!(
    struct RegisterInput {
        client_mutation_id: String,
        username: String,
        password: String,
        email: String,
    }
);

mutation_output!(
    struct EditUserOutput {
        user: User
    }
);

graphql_input_object!(
    struct EditUserInput {
        client_mutation_id: String,
        user_id: ID,
        email: Option<String>,
        lastname: Option<Option<String>>,
        firstname: Option<Option<String>>,
        name_public: Option<bool>
    }
);
