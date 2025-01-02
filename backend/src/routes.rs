use axum::Json;
use serde::{Deserialize, Serialize};
use crate::services;

#[derive(Deserialize)]
pub struct MintRequest {
    pub amount: u64,
    pub wallet_address: String,
}

#[derive(Serialize)]
pub struct MintResponse {
    pub success: bool,
    pub message: String,
}

pub async fn mint_stablecoins(Json(payload): Json<MintRequest>) -> Json<MintResponse> {
    let result = services::mint_stablecoins(payload.amount, &payload.wallet_address).await;
    match result {
        Ok(_) => Json(MintResponse {
            success: true,
            message: "Minting successful!".to_string(),
        }),
        Err(err) => Json(MintResponse {
            success: false,
            message: format!("Error: {}", err),
        }),
    }
}

#[derive(Deserialize)]
pub struct RedeemRequest {
    pub amount: u64,
    pub wallet_address: String,
}

#[derive(Serialize)]
pub struct RedeemResponse {
    pub success: bool,
    pub message: String,
}

pub async fn redeem_stablecoins(Json(payload): Json<RedeemRequest>) -> Json<RedeemResponse> {
    let result = services::redeem_stablecoins(payload.amount, &payload.wallet_address).await;
    match result {
        Ok(_) => Json(RedeemResponse {
            success: true,
            message: "Redeeming successful!".to_string(),
        }),
        Err(err) => Json(RedeemResponse {
            success: false,
            message: format!("Error: {}", err),
        }),
    }
}