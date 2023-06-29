use std::ops::Range;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::{PageItem, PageItemLimit, PageLimit};
use crate::store::reducers::page_reducer;
use crate::store::LocalStore;

const PAGINATOR: u32 = 10;

#[derive(PartialEq, Properties)]
pub struct PageListProps {
    pub total: u32,
}

#[function_component]
pub fn PageList(props: &PageListProps) -> Html {
    let prev_state = use_state(|| 0);
    let (store, dispatch) = use_store::<LocalStore>();

    let onclick = {
        let dispatch = dispatch.clone();

        Callback::from(move |page| {
            page_reducer(dispatch.clone(), page);
        })
    };

    let onclick_limit = {
        let prev = prev_state.clone();
        let total = props.total;

        Callback::from(move |limit| match limit {
            PageLimit::Prev => {
                if *prev > 0 {
                    page_reducer(dispatch.clone(), (*prev - 1) * PAGINATOR);
                    prev.set(*prev - 1);
                }
            }
            PageLimit::Next => {
                if *prev < total / PAGINATOR {
                    page_reducer(dispatch.clone(), (*prev + 1) * PAGINATOR);
                    prev.set(*prev + 1);
                }
            }
        })
    };

    let start = *prev_state * PAGINATOR + 1;
    let mut end = *prev_state * PAGINATOR + PAGINATOR + 1;
    if end > props.total {
        end = props.total + 1;
    }

    html! {
        <ul class="pagination pagination-sm justify-content-end mt-2">
        if props.total > PAGINATOR {
            <PageItemLimit page={PageLimit::Prev} onclick={onclick_limit.clone()} disabled={start == 1} />
        }
        if props.total > 1 {
            {
                Range { start, end }.into_iter().map(|i| {
                    html! { <PageItem page={i} active={store.page + 1 == i} onclick={onclick.clone()} /> }}
                ).collect::<Html>()
            }
        }
        if props.total > PAGINATOR {
            <PageItemLimit page={PageLimit::Next} onclick={onclick_limit} disabled={end == props.total + 1} />
        }
        </ul>
    }
}
