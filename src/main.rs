use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Status {
    Pending,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    status: Status,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, title: String) {
        let task = Task {
            id: self.next_id,
            title,
            status: Status::Pending,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✓ 任务已添加");
    }

    fn list(&self) {
        if self.tasks.is_empty() {
            println!("暂无任务");
            return;
        }

        for task in &self.tasks {
            let checkbox = match task.status {
                Status::Pending => "[ ]",
                Status::Done => "[✓]",
            };
            println!("{}. {} {}", task.id, checkbox, task.title);
        }
    }

    fn done(&mut self, id: usize) -> Result<(), String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or("任务不存在")?;

        task.status = Status::Done;
        println!("✓ 任务已完成");
        Ok(())
    }

    fn remove(&mut self, id: usize) -> Result<(), String> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or("任务不存在")?;

        self.tasks.remove(index);
        println!("✓ 任务已删除");
        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write("todos.json", json)?;
        Ok(())
    }

    fn load() -> io::Result<Self> {
        match fs::read_to_string("todos.json") {
            Ok(content) => {
                let tasks: Vec<Task> = serde_json::from_str(&content)?;
                let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

                Ok(TodoList { tasks, next_id })
            }
            Err(_) => Ok(TodoList::new()),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let mut todo_list = TodoList::load().expect("加载任务失败");

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("用法: todo add <任务>");
                return;
            }
            let title = args[2..].join(" ");
            todo_list.add(title);
        }
        "list" => {
            todo_list.list();
        }
        "done" => {
            if args.len() < 3 {
                println!("用法: todo done <id>");
                return;
            }
            let id: usize = args[2].parse().expect("ID 必须是数字");
            todo_list.done(id).ok();
        }
        "remove" => {
            if args.len() < 3 {
                println!("用法: todo remove <id>");
                return;
            }
            let id: usize = args[2].parse().expect("ID 必须是数字");
            todo_list.remove(id).ok();
        }
        _ => print_help(),
    }

    todo_list.save().expect("保存失败");
}

fn print_help() {
    println!("Todo CLI - 待办事项管理");
    println!("\n用法:");
    println!("  todo add <任务>    添加任务");
    println!("  todo list          列出任务");
    println!("  todo done <id>     完成任务");
    println!("  todo remove <id>   删除任务");
}
