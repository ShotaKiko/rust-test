use seed::prelude::*;
use seed::*;

use indexmap::IndexMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

use std::mem;

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
    //~~~~~~~~~~~~~~
    //  General
    //~~~~~~~~~~~~~~
    //ChangeText(String),
    NewTodoTitleUpdated(String),
    ClearEntireTodoList,

    //~~~~~~~~~~~~~~
    //  SingleTodo
    //~~~~~~~~~~~~~~
    CreateNewTodoItem,
    RemoveTodoItem(TodoItemId),

    //~~~~~~~~~~~~~~
    //  EditTodo
    //~~~~~~~~~~~~~~
    StartTodoEdit(TodoItemId),
    EditingTodoTitleUpdated(String),
    SaveEditingTodo,
    CancelTodoEdit,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::*;
    let data = &mut model.todo_data;

    match msg {
    //__________General_______________
        // ChangeText(new_text) => model.text_to_show = new_text,
        NewTodoTitleUpdated(name) => {
            data.new_todo_name = name
        }

        ClearEntireTodoList => {
            data.todo_list.clear()
        }

    //__________Item Specific___________
        CreateNewTodoItem => {
            data.todo_list.insert(
                TodoItemId::new_v4(), //creates uuid
                TodoItem {
                    title: mem::take(&mut data.new_todo_name), //takes from memory
                    completed: false,
                },
            );
        }

        RemoveTodoItem(todo_item_id) => {
            data.todo_list.shift_remove(&todo_item_id);
        }

    //__________Todo Edits_______________  
        StartTodoEdit(todo_item_id) => {
            if let Some(todo) = data.todo_list.get(&todo_item_id) {
                data.editing_todo_item = Some({
                    EditingTodoItem {
                        id:todo_item_id,
                        title:todo.title.clone(),
                    }
                })
            }
            
            let input = model.todo_ref.editing_todo_input.clone();
            orders.after_next_render(move |_| {
                input.get().expect("get `editing_todo_input`").select();
            });
        }
        
        EditingTodoTitleUpdated(title) => {
            if let Some(ref mut editing_todo_item) = data.editing_todo_item {
                editing_todo_item.title = title
            }
        }

        SaveEditingTodo => {
            if let Some(editing_todo_item) = data.editing_todo_item.take() {
                if let Some(todo) = data.todo_list.get_mut(&editing_todo_item.id) {
                    todo.title = editing_todo_item.title
                }
            }
        }
        
        CancelTodoEdit => {
            data.editing_todo_item = None;
        }
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
