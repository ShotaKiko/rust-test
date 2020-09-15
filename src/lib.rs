use seed::prelude::*;
use seed::*;

use indexmap::IndexMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;


//---------------------------------------
//               Structs
//---------------------------------------

struct Model {
    todo_data: TodoData,
    todo_ref:TodoReference,
}


type TodoItemId = Uuid; //will use uuid to generate unique identifier


#[derive(Default, Serialize, Deserialize)]
struct TodoData {
    todo_list: IndexMap<TodoItemId, TodoItem>,
    new_todo_name: String,
    editing_todo_item: Option<EditingTodoItem>
}


#[derive(Default)]
struct TodoReference {
    editing_todo_input: ElRef<HtmlInputElement>, //to avoid using Javascript's selectors
}


#[derive(Serialize, Deserialize)]
struct TodoItem {
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
struct EditingTodoItem {
    id: TodoItemId,
    title: String,
}


//---------------------------------------
//          Msg Enumerations
//---------------------------------------

#[derive(Clone)]
enum Msg {
    //ChangeText(String),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        // ChangeText(new_text) => model.text_to_show = new_text,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        input![
            attrs! {
                At::Placeholder => "Enter some text..."
            },
            // input_ev(Ev::Input, Msg::ChangeText),
        ],
        // div![&model.text_to_show]
    ]
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        todo_data: TodoData::default(),
        todo_ref: TodoReference::default()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
