use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "src/api/queries/upsert_bucket.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct UpsertBucket;
