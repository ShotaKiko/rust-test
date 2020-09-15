use seed::prelude::*;
use seed::*;

use indexmap::IndexMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

use std::mem;
const ENTER_KEY: u32 = 13;
const ESC_KEY: u32 = 27;
use enclose::enc;

const STORAGE_KEY: &str = "LVT-Rust&Seed-Test"; //for storing list in localstorage

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
    LocalStorage::insert(STORAGE_KEY, &data).expect("Save to do list data to local storage.")
}

//---------------------------------------
//          View and subViews
//---------------------------------------

//________________Overall Container View_________________
fn view(model: &Model) -> impl IntoNodes<Msg> {
    let data = &model.todo_data;

    nodes![
        header_view(&data.new_todo_name),
        if data.todo_list.is_empty() {
            vec![]
        } else {
            vec![
                content_view(
                    &data.todo_list,
                    &data.editing_todo_item,
                    &model.todo_ref.editing_todo_input,
                ),
            ]
        }
    ]
}

//________________Header View_________________

fn header_view(new_todo_name: &str) -> Node<Msg> {
    header![
        C!["header"],
        h1!["To Do List"],
        input![
            C!["newTodo"],
            attrs! {
                At::AutoFocus => true.as_at_value();
                At::Placeholder => "What glorious task should we complete next?";
                At::Value => new_todo_name;
            },
        keyboard_ev(Ev::KeyDown, |keyboard_event| {
            IF!(keyboard_event.key_code() == ENTER_KEY => Msg::CreateNewTodoItem)
        }),
        input_ev(Ev::Input, Msg::NewTodoTitleUpdated)
        ],
        button![
            C!["addTodoButton"],
            ["Add New To Do"],
            ev(Ev::Click, |_| Msg::CreateNewTodoItem)
        ],
        button![
            C!["clearTodoListButton"],
            ["Clear List"],
            ev(Ev::Click, |_| Msg::ClearEntireTodoList),
        ]
    ]
}

//________________Content View__________________

fn content_view(
    todo_list: &IndexMap<TodoItemId, TodoItem>,
    editing_todo_item: &Option<EditingTodoItem>,
    editing_todo_input: &ElRef<HtmlInputElement>,
) -> Node<Msg> {
    section![
        C!["contentContainer"],
        ul![
            C!["todoList"],
            todo_list.iter().filter_map(|(todo_item_id, todo_item)| {
                let show_all = true; //could modify for showing filtered list

                IF!(show_all => todo_view(todo_item_id, todo_item, editing_todo_item, editing_todo_input))
            })
        ]
    ]
}

//________________Todo Item View__________________

#[allow(clippy::cognitive_complexity)]
fn todo_view(
    todo_item_id: &TodoItemId,
    todo_item: &TodoItem,
    editing_todo_item: &Option<EditingTodoItem>,
    editing_todo_input: &ElRef<HtmlInputElement>,
) -> Node<Msg> {
    li![
        C![
            IF!(matches!(editing_todo_item, Some(editing_todo_item) if &editing_todo_item.id == todo_item_id) => "editing"),
        ],
        label![
            ev(
                Ev::DblClick,
                enc!((todo_item_id) move |_| Msg::StartTodoEdit(todo_item_id))
            ),
            &todo_item.title
        ],
        button![
            C!["removeTodoButton"],
            ["X"],
            ev(Ev::Click, enc!((todo_item_id) move |_| Msg::RemoveTodoItem(todo_item_id)))
        ],
        match editing_todo_item {
            Some(editing_todo_item) if &editing_todo_item.id == todo_item_id => {
                input![
                    el_ref(editing_todo_input),
                    C!["editInputField"],
                    attrs! {At::Value => editing_todo_item.title},
                    ev(Ev::Blur, |_| Msg::SaveEditingTodo),
                    input_ev(Ev::Input, Msg::EditingTodoTitleUpdated),
                    keyboard_ev(Ev::KeyDown, |keyboard_event| {
                        match keyboard_event.key_code() {
                            ENTER_KEY => Some(Msg::SaveEditingTodo),
                            ESC_KEY => Some(Msg::CancelTodoEdit),
                            _ => None,
                        }
                    }),
                ]
            }
            _ => empty![],
        }
    ]
}

//---------------------------------------
//          Initialization
//---------------------------------------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        todo_data: LocalStorage::get(STORAGE_KEY).unwrap_or_default(), //defaults to Todo Default if not found
        todo_ref: TodoReference::default()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
