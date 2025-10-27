use maud::{html, Markup};

use crate::website::components::transaction::transaction_popup;

pub fn add_transaction() -> Markup {
    html! {
        div #"add-single-transaction" {
            (
                transaction_popup(
                    Some("New Transaction"),
                    Some("/js/add_single_transaction.js"),
                    None
                )
            )
        }
    }
}
