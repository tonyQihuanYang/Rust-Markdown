use chrono::{DateTime, Utc};
use juniper::{GraphQLInputObject, GraphQLObject};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(GraphQLObject, Debug, Serialize, Deserialize, Clone)]
#[graphql(description = "Markdown content")]
pub struct Markdown {
    pub title: String,
    pub context: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Markdown input, for create")]
pub struct MarkdownInput {
    pub title: String,
    pub context: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Markdown input, for update")]
pub struct MarkdownUpdateInput {
    pub id: String,
    pub title: Option<String>,
    pub context: Option<String>,
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize, Clone)]
#[graphql(description = "Markdown result, the markdown id")]
pub struct MarkDownId {
    pub id: String,
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize, Clone)]
#[graphql(description = "Markdown result, the markdown stored in the mongo db")]
pub struct MarkdownGraphQl {
    pub id: String,
    pub title: String,
    pub context: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_datetime: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_datetime: DateTime<Utc>,
}

impl MarkdownGraphQl {
    pub fn new(mardown: Markdown) -> Self {
        let now = Utc::now();
        Self {
            id: nanoid!(10),
            title: mardown.title,
            context: mardown.context,
            created_datetime: now,
            updated_datetime: now,
        }
    }
}
