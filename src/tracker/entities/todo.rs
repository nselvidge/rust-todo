pub struct Todo {
  pub id: u32,
  pub label: String,
  pub done: bool,
}
pub fn make_todo(label: String) -> Todo {
  Todo {
    id: 1,
    label,
    done: false,
  }
}
