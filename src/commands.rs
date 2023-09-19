use std::io;
use crate::database::{Database, Record};

pub fn info() -> Result<(), io::Error> {
    println!("Todo is a simple todo list manager.");
    Ok(())
}

pub fn add(db: &mut Database, content: Option<String>) -> Result<(), io::Error> {
    if let Some(content) = content {
        let records = db.read_records();
        db.add_record(&Record {
            id: records.len() as u32 + 1,
            content: content.clone(),
        })?;
        println!("📝 Item added: {}", content);
        Ok(())
    } else {
        eprintln!("You need to specify the content of the todo item.");
        std::process::exit(1);
    }
}

pub fn remove(db: &mut Database, id: Option<String>) -> Result<(), io::Error> {
    if id.is_none() {
        println!("You need to specify the id of the todo item.");
        std::process::exit(1);
    }
    println!("Removing a todo item: {}", id.clone().unwrap());
    db.remove_record(id.unwrap().parse::<u32>().unwrap())?;
    println!(" ❌ Item removed!\n");
    Ok(())
}

pub fn list(db: &mut Database) -> Result<(), io::Error> {
    let records = db.read_records();
    if records.is_empty() {
        eprintln!("No records. You can add one with `todo add [content]`");
        std::process::exit(1);
    }
    for record in records {
        println!(" ⬜️ {}: {}", record.id, record.content);
    }
    Ok(())
}
