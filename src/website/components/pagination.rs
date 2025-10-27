use css_helper::Css;
use maud::{html, Markup};

pub fn pagination(index: usize, total_pages: usize) -> Markup {
    html! {
        script { (format!(
            "
            window.totalPages = {total_pages};
            window.paginationIndex = {index};
            window.prevPaginationIndex = null;
            "))
        }
        script src="/js/pagination.js" defer {}
        (pagination_css())
        div class="pagination-container"{
            button class="page-btn prev" disabled{"<"}
            div class="page-numbers" {}
            button class="page-btn next"{">"}
        }
    }
}

fn pagination_css() -> Css {
    Css::from(
        r#"
    .pagination-container {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.4rem;
        padding: 1rem;
        user-select: none;
        font-family: system-ui, sans-serif;
    }

    .pagination-container * {}

    .page-btn {
        background-color: #0000;
        border: none;
        color: #fff;
        border-radius: 50%;
        padding: 0.4rem;
        cursor: pointer;
        transition: all 0.2s;
        width: 2rem;
        height: 2rem;
    }

    .page-btn:hover:not(:disabled) {
        background-color: rgba(255 255 255 / 0.08);
        color: white;
    }

    .page-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .page-number {
        background-color: #0000;
        border: none;
        border-radius: 100%;
        padding: 0.4rem;
        cursor: pointer;
        transition: all 0.2s;
        color: #fff;
        margin: 0.1rem;
        width: 2rem;
        height: 2rem;
    }

    .page-number.active {
        background-color: rgb(144, 202, 249);
        color: #000;
    }

    .page-number:hover:not(.active) {
        background-color: rgba(255 255 255 / 0.08);
    }

    .page-ellipsis {
        padding: 0.4rem;
    }
    "#,
    )
}
