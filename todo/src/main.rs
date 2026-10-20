use std::rc::Rc;
slint::include_modules!(); 
use slint::Model;
fn main() {
    let todo_buffs = Rc::new(slint::VecModel::<TodoItem>::from(vec![
        TodoItem { checked: false, title: "rust demo 1".into()},
        TodoItem { checked: true,  title: "rust demo 2".into()},
    ]));

    


    let app = MainWindow::new().unwrap();
    
    app.set_todo_buffs(todo_buffs.clone().into());
    let todo_item_add = todo_buffs.clone();
    app.on_todo_added(move |text| {
        todo_item_add.push(TodoItem { checked: false, title: text });
    });

    let todo_remove = todo_buffs.clone();
    app.on_remove_done(move || {
        let mut offset = 0;
        for i in 0..todo_remove.row_count() {
            let current_index = i - offset;
            if todo_remove.row_data(current_index).unwrap().checked {
                todo_remove.remove(current_index);
                offset += 1;
            }
        }
    });
    
    app.run().unwrap();
}
