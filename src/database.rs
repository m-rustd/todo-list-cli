use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, Write};
use crate::utils::{check_db_file, get_db_file_path};

pub struct Record {
    pub id: u32,
    pub content: String,
}

pub struct Database {
    file: File,
}

// 解析记录
pub fn parse_record_line(line: &str) -> Record {
    let fields: Vec<&str> = line.split(',').collect();
    // 处理空行的情况
    if fields.len() == 1 {
        return Record {
            id: 0,
            content: "".to_string(),
        };
    }
    let content = fields[1..].join(",");
    Record {
        id: fields[0].parse::<u32>().unwrap(),
        content,
    }
}

impl Database {
    pub fn open() -> Database {
        // 先检查 db 文件是否存在，不存在就创建
        check_db_file().unwrap();
        // 获取db文件路径
        let db_file = get_db_file_path();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(db_file)
            .unwrap();
        Database { file }
    }

    pub fn add_record(&mut self, record: &Record) -> Result<(), io::Error> {
        let line = format!("{},{}", record.id, record.content);
        writeln!(self.file, "{}", line)
    }

    pub fn read_records(&mut self) -> Vec<Record> {
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| parse_record_line(&line))
            .collect()
    }

    pub fn remove_record(&mut self, id: u32) -> Result<(), io::Error> {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        // 根据id找出对应的行
        let line = lines.find(|(_, line)| {
            let record = parse_record_line(line.as_ref().unwrap());
            record.id == id
        });
        match line {
            Some((i, _)) => {
                // 过滤掉对应的行，这里使用的对应 api 可以查看 Rust 标准库
                let new_contents = lines
                    .filter(|(j, _)| *j != i)
                    .map(|(_, line)| line.unwrap())
                    .collect::<Vec<_>>()
                    .join("\n");
                // 这里使用了 std::io::Seek，需要导入
                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();
                // println!(" ❌ Item removed!\n");
                Ok(())
            }
            None => Err(io::Error::new(
              io::ErrorKind::Other,
              format!("No such record: {}", id),
          )),
        }
    }
}
