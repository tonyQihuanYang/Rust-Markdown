use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};

pub enum CustomError {
    UnexectedError,
}

impl<S: ScalarValue> IntoFieldError<S> for CustomError {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            CustomError::UnexectedError => FieldError::new(
                "Unexpected error",
                graphql_value!({
                    "type": "UNEXPECTED-ERROR"
                }),
            ),
        }
    }
}
