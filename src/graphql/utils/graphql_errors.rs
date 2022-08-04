use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};

pub enum CustomError {
    UnexectedError,
    ItemNotFound,
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
            CustomError::ItemNotFound => FieldError::new(
                "Item not found",
                graphql_value!({
                    "type": "ITEM-NOT-FOUND"
                }),
            ),
        }
    }
}
