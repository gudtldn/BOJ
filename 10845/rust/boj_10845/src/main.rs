use std::{io::stdin, error::Error};

struct Queue {
    data: Vec<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn push(&mut self, item: i32) {
        self.data.push(item);
    }

    fn pop(&mut self) -> i32 {
        if self.data.is_empty() {
            -1
        } else {
            self.data.remove(0)
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn empty(&self) -> u8 {
        self.data.is_empty() as u8
    }

    fn front(&self) -> i32 {
        if self.data.is_empty() {
            -1
        } else {
            self.data[0]
        }
    }

    fn back(&self) -> i32 {
        if self.data.is_empty() {
            -1
        } else {
            self.data[self.data.len() - 1]
        }
    }    
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut queue = Queue::new();

    let mut input_buffer = String::new();
    stdin().read_line(&mut input_buffer)?;

    for _ in 0..input_buffer.trim().parse::<i32>()? {
        input_buffer.clear();
        stdin().read_line(&mut input_buffer)?;

        match input_buffer.trim() {
            "pop" => println!("{}", queue.pop()),
            "size" => println!("{}", queue.size()),
            "empty" => println!("{}", queue.empty()),
            "front" => println!("{}", queue.front()),
            "back" => println!("{}", queue.back()),
            _ => queue.push(input_buffer.trim().split(' ').nth(1).unwrap().parse::<i32>()?),
        }
    }

    Ok(())
}
