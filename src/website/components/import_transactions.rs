use axum::Json;
use maud::{html, Markup};
use uuid::Uuid;

use crate::{models::Transaction, website::components::transaction::transaction_popup};

pub async fn import_transactions(Json(transactions): Json<Vec<Transaction>>) -> Markup {
    let transactions: Vec<(Option<Uuid>, Transaction)> =
        transactions.into_iter().map(|t| (None, t)).collect();
    html! {
        (transaction_popup(Some("Import Transactions"), Some("/js/import_transactions.js"), Some(&transactions)))
    }
}
