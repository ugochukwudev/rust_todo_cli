use std::{process, sync::atomic::{AtomicU64, self}};

#[derive(Debug)]
pub struct Task{
    task: String,
    done_status: bool, 
    id: u64, 
}

impl Task{
    fn update_status(&mut self){
        self.done_status = true; 
    }

    fn update_task(&mut self, new_name: String){
        self.task = new_name; 
    }
}

static UNIQUE_ID: AtomicU64  = AtomicU64::new(1); 

fn display_todo(todo_list: &Vec<Task>){
    if todo_list.len() < 1 {
        println!("Empty todo list"); 
        return; 
    }

    for item in todo_list{
        println!("id: {}, name: {}, done: {}", item.id, item.task, item.done_status);
    }
}

fn add_new_task(todo_list: &mut Vec<Task>, task_string: &str){

    let id_no = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);

    let task: Task = Task{
        task: task_string.into(), 
        done_status: false, 
        id: id_no, 
    };

    todo_list.push(task); 

    println!("{} added to the todo list: ", task_string); 
}

fn remove_task(todo_list: &mut Vec<Task>, id_no: u64){

    todo_list.retain(|task| task.id != id_no); 

}

fn get_task(todo_list: &mut Vec<Task>, task_id: u64) -> Result<&mut Task, &str>{

    for task in todo_list{
        if task.id == task_id{
            return Ok(task);
        }else{
            continue;
        }
    };

    return Err("Task not found in todo list"); 

}


fn display_help(){
    let help: &str = "
        Welcome to the todo_list application. 
        structure of query: 
            command [arguments] 

        supported commands: 
            add - Add a new task to the todo list, followed by a new task string. The task string should NOT be space separated. 

                usage: >add task_string

            show - Display the todo list 
                
                usage: >show

            delete - delete a task from the todo list, based on the task id provided by the user in the prompt. 

                usage: >delete task_id

            update - change the name of a task, followed by an integer number task id. 

                usage: >update task_id new_task_string 

            done - change the done status of a task from false to true, follwed by an integer number task id. 
                
                usage: >done task_id 

            exit- exit the program. 
                
                usage: >exit

            help - display this help message. 
                
                usage: >help 
        
        arguments: 
            task_id: the unique id assigned to each task. 

            task_string: the string for the task provided by the user. ";

    println!("{}", help); 
}

fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>){
    if args.len()==0{
println!("please parsa a valid command. or type help to see useful commands");
    }else {
        let command = args[0];

    match command{
        "add" => {
     let (_,rest) = args.split_at(1);
     let merged =  rest.join(" ");
            if merged.len()>0{
                let does_task_exist = todo_list.iter().find(|&task| task.task==merged);
                match does_task_exist {
                    Some(task)=> return println!("Task {:?} already exist",task.task),
                    None=> println!("creating new task")
                }
                let new_task = &merged; 
                add_new_task(todo_list, new_task); 
                display_todo(todo_list); 
            }else{
                println!("please provide a new name for the task"); 
            }
        },

        "show" =>{

            display_todo(todo_list); 

        },

        "delete" => {

            match &args[1].parse::<u64>(){
                Ok(value) =>{
                    remove_task(todo_list, *value);
                }

                Err(message) =>{
                    println!("{}", message.to_string()); 
                }
            }
        },

        "update" => {
            
            // possibility 1: id parsing error 
            match &args[1].parse::<u64>(){
                Ok(value) => {

                    // possibility 2: task getting error 
                    if let Ok(task) = get_task(todo_list, *value){

                        // possibility 3: no third argument provided. 
                        if let Some(value) = args.get(2){
                            let new_task = *value; 
                            task.update_task(new_task.into()); 
                        }else{
                            println!("no new task provided"); 
                        }

                    }else{
                        println!("task not found in todo list"); 
                    }
                },

                Err(message) => {
                    print!("{}", message); 
                }
            }
        },

        "done" => {

            match &args[1].parse::<u64>(){
                Ok(value) =>{
                    if let Ok(task) = get_task(todo_list, *value){
                        task.update_status(); 
                    }else{
                        println!("task id not found in list"); 
                    }
                }, 
                Err(message) =>{
                    println!("{}", message.to_string());
                }
            }
        },

        "exit" => {
            process::exit(0); 
        },

        "help" | _ => {
            display_help(); 
        }
        
    }
    }
    
}


pub fn run(args: Vec<&str>, todo: &mut Vec<Task>){

    parse_arguments(args, todo); 
        
}