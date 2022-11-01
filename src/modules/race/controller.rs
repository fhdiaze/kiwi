use crate::modules::race::get;

pub fn handle_get() {
    let query = get::Query::new(1);
    get::handle(query);
}