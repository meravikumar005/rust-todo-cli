use std::collections::HashMap;

use clap::{ Parser, Subcommand, Args };

#[derive(Parser)]
struct Todo {
    #[command(subcommand)]
    command:Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Create(Create),
    Edit(Edit),
    Delete(Delete),
    List(List)
}
#[derive(Debug,Args)]
struct  Create {
    task:Option<String>
}
#[derive(Debug,Args)]
struct Edit {
    id:i32,
    task:String
}
#[derive(Debug,Args)]
struct  Delete {
    id:i32
}

#[derive(Debug,Args)]
struct  List {}


struct StoreTask {
    task: HashMap<i32, String>,
}

impl StoreTask {
    fn new() -> Self {
        StoreTask {
            task: HashMap::new(),
        }
    }

    fn create_task(&mut self,item: String) {
        let mut id = self.task.len().try_into().unwrap();
        id += 1;
        self.task.insert(id, item);
    }

    fn list_task(self) -> Vec<String> {
        self.task.values().cloned().collect()
    }

    fn delete_task(&mut self, id:i32) -> Vec<String> {
        self.task.remove(&id);
        self.task.values().cloned().collect()
    }

    fn edit_task(&mut self, id:i32, task:String) -> Vec<String> {
        *self.task.get_mut(&id).unwrap() = task;
        self.task.values().cloned().collect()
    }

}

fn main() {
    let mut store = StoreTask::new();
    let arg = Todo::parse();

    match arg.command {
        Some(Commands::Create(params)) => {
            let task = params.task.unwrap();
            store.create_task(task);
        },

        Some(Commands::Edit(params)) => {
            let edit = Some(params).unwrap();
            store.edit_task(edit.id, edit.task);
        },

        Some(Commands::Delete(params)) => {
            let id = Some(params).unwrap();
            store.delete_task(id.id);
        }

        Some(Commands::List(_)) => {
            let task_list = store.list_task();
            for item in task_list.iter() {
                println!("{:?}", item);
            }
        }
        None => println!("Invalid Command!")
    }
}
