use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "src/api/queries/project_by_name.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct ProjectByName;
