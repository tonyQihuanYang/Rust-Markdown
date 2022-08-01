use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use mongodb::bson::{doc, Bson, Document};
use mongodb::options::FindOptions;

use super::models::markdown::{
    MarkDownId, Markdown, MarkdownGraphQl, MarkdownInput, MarkdownUpdateInput, MarkdownUpdated,
};
use super::utils::graphql_errors::CustomError;
use crate::MONGO_DB;
use futures::stream::TryStreamExt;
pub struct QueryRoot;

// TODO: Should create a handler folder for these functions...
#[juniper::graphql_object]
impl QueryRoot {
    async fn markdown_by_id(id: String) -> Result<Option<MarkdownGraphQl>, CustomError> {
        let database = MONGO_DB.get().unwrap();
        let collection = database.collection::<MarkdownGraphQl>("markdown");
        let filter = doc! {"id":id.to_owned()};
        match collection.find_one(filter, None).await {
            Ok(markdown_found) => Ok(markdown_found),
            Err(_) => Err(CustomError::UnexectedError),
        }
    }

    async fn allMarkdowns() -> FieldResult<Vec<MarkdownGraphQl>> {
        let database = MONGO_DB.get().unwrap();
        let collection = database.collection::<MarkdownGraphQl>("markdown");
        let find_options = FindOptions::builder()
            .sort(doc! { "updated_datetime": 1 })
            .build();
        let cursor = collection.find(None, find_options).await?;
        let markdowns: Vec<MarkdownGraphQl> = cursor.try_collect().await?;
        Ok(markdowns)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    async fn create_markdown(new_markdown: MarkdownInput) -> FieldResult<MarkdownGraphQl> {
        let database = MONGO_DB.get().unwrap();
        let collection = database.collection::<MarkdownGraphQl>("markdown");
        let created = MarkdownGraphQl::new(Markdown {
            title: new_markdown.title.to_owned(),
            context: new_markdown.context.to_owned(),
        });

        collection.insert_one(created.clone(), None).await?;
        Ok(created)
    }

    async fn delete_markdown(id: String) -> Result<MarkDownId, CustomError> {
        let database = MONGO_DB.get().unwrap();
        let collection = database.collection::<MarkdownGraphQl>("markdown");
        let filter = doc! {"id":id.to_owned()};
        match collection.delete_one(filter, None).await {
            Ok(_) => Ok(MarkDownId { id: id.to_owned() }),
            Err(_) => Err(CustomError::UnexectedError),
        }
    }

    async fn update_markdown(input: MarkdownUpdateInput) -> Result<MarkdownGraphQl, CustomError> {
        let database = MONGO_DB.get().unwrap();
        let collection = database.collection::<MarkdownGraphQl>("markdown");
        let filter = doc! {"id":input.id.to_owned()};

        let mut update = Document::new();
        if let Some(title) = input.title {
            update.insert("title", Bson::String(title));
        };

        if let Some(context) = input.context {
            update.insert("context", Bson::String(context));
        }

        let mut time_option = Document::new();
        time_option.insert("updated_datetime", Bson::Boolean(true));

        let mut version_option = Document::new();
        version_option.insert("version", Bson::Int32(1));

        match collection
            .find_one_and_update(
                filter,
                doc! {"$currentDate": time_option, "$inc": version_option, "$set": update},
                None,
            )
            .await
        {
            Ok(result) => match result {
                Some(updated) => Ok(updated),
                None => Err(CustomError::ItemNotFound),
            },
            Err(_) => Err(CustomError::UnexectedError),
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
