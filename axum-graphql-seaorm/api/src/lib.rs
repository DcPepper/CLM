use async_graphql::{
    http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    Context, Data, EmptyMutation, EmptySubscription, Object, Result, Schema,
};
use async_graphql_axum::{
    GraphQL, GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket,
};
use axum::{
    extract::{State, WebSocketUpgrade},
    http::HeaderMap,
    response::{self, IntoResponse, Response},
    routing::get,
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use tokio::net::TcpListener;

pub type TokenSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct Token(pub String);
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn root(&self, ctx: &Context<'_>) -> u32 {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        0
    }

    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
}

pub async fn on_connection_init(value: serde_json::Value) -> Result<Data> {
    #[derive(Deserialize)]
    struct Payload {
        token: String,
    }

    // Coerce the connection params into our `Payload` struct so we can
    // validate the token exists in the headers.
    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = Data::default();
        data.insert(Token(payload.token));
        Ok(data)
    } else {
        Err("Token is required".into())
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Token")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

async fn graphql_handler(
    State(schema): State<TokenSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    if let Some(token) = get_token_from_headers(&headers) {
        req = req.data(token);
    }
    schema.execute(req).await.into()
}

async fn graphql_ws_handler(
    State(schema): State<TokenSchema>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> Response {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema.clone(), protocol)
                .on_connection_init(on_connection_init)
                .serve()
        })
}

#[tokio::main]
pub async fn main() {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:postgres@localhost:5432/postgres").await;
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/ws", get(graphql_ws_handler))
        .with_state(schema);
    println!("GraphQL serving at http://localhost:8000");
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
