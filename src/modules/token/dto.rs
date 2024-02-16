use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct TokenAsset {
    #[serde(flatten)]
    pub asset: Value,
}
