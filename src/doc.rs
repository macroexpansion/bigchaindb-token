use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(schemas()),
    tags(
        (name = "Token", description = "Token")
    )
)]
pub struct ApiDoc;
