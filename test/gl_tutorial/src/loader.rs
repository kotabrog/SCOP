use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::model::Model;

#[derive(Debug, Clone)]
pub struct Loader {
    file_path: String,
}

impl Loader {
    pub fn new(path: String) -> Self {
        Loader { file_path: path }
    }

    pub fn parse(&self, model: &mut Model) -> Result<(), String> {
        let f = File::open(&self.file_path).map_err(|e| format!("error: {}", e))?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.map_err(|e| format!("error: {}", e))?;
            let line: Vec<&str> = line.split_whitespace().collect();
            if line.is_empty() {
                continue;
            }
            match line[0] {
                "v" => self.parse_v(model, &line)?,
                "f" => self.parse_f(model, &line)?,
                _ => return Err(format!("error: parse obj: {}", line[0]))
            }
        }
        if model.is_empty() {
            return Err(format!("error: parse obj: empty"))
        }
        Ok(())
    }

    fn parse_v(&self, model: &mut Model, line: &Vec<&str>) -> Result<(), String> {
        if line.len() != 4 {
            return Err(format!("error: parse obj: {}", line.join(" ")))
        }
        let x1: f32 = line[1].parse().map_err(|_| format!("error: parse obj: {}", line[1]))?;
        let x2: f32 = line[2].parse().map_err(|_| format!("error: parse obj: {}", line[2]))?;
        let x3: f32 = line[3].parse().map_err(|_| format!("error: parse obj: {}", line[3]))?;
        model.push_vertices((x1, x2, x3).into());
        Ok(())
    }

    fn parse_f(&self, model: &mut Model, line: &Vec<&str>) -> Result<(), String> {
        if line.len() != 4 {
            return Err(format!("error: parse obj: {}", line.join(" ")))
        }
        let x1: u32 = line[1].parse().map_err(|_| format!("error: parse obj: {}", line[1]))?;
        let x2: u32 = line[2].parse().map_err(|_| format!("error: parse obj: {}", line[2]))?;
        let x3: u32 = line[3].parse().map_err(|_| format!("error: parse obj: {}", line[3]))?;
        model.push_indices(x1.saturating_sub(1));
        model.push_indices(x2.saturating_sub(1));
        model.push_indices(x3.saturating_sub(1));
        Ok(())
    }
}
