use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "src/graphql/viewer.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct Viewer;
