use crate::book::BookMetadata;
use crate::stateful_list::StatefulList;

pub struct UIState {
    pub input: String,
    pub search_results: StatefulList<BookMetadata>,
}

impl Default for UIState {
    fn default() -> UIState {
        UIState {
            input: String::new(),
            search_results: StatefulList::with_items(vec![]),
        }
    }
}
