use std::fs;

fn main() {
    let content = fs::read_to_string("src/main.rs").expect("Failed to read main.rs");
    let lines: Vec<&str> = content.lines().collect();

    let mut enum_start = None;
    let mut enum_end = None;
    let mut layouts_start = None;
    let mut layouts_end = None;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("#[derive(Clone, Copy, PartialEq, Eq, Debug)]") {
            enum_start = Some(i);
        }
        if enum_start.is_some() && line.starts_with("struct Game {") {
            enum_end = Some(i);
            break;
        }
    }

    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("fn layout_positions(&self) -> Vec<(i32, i32, i32)> {") {
            layouts_start = Some(i);
        }
        if line.trim().starts_with("fn generate_board(&mut self) {") {
            layouts_end = Some(i);
            break;
        }
    }

    if let (Some(es), Some(ee), Some(ls), Some(le)) = (enum_start, enum_end, layouts_start, layouts_end) {
        let enum_lines = &lines[es..ee];
        let layout_lines = &lines[ls..le];

        let mut layouts_rs = String::new();

        for line in enum_lines {
            if line.trim().starts_with("fn next(&self) -> Option<Self> {") {
                layouts_rs.push_str(&line.replace("fn next", "pub fn next"));
                layouts_rs.push('\n');
            } else if line.trim().starts_with("fn name(&self) -> &'static str {") {
                layouts_rs.push_str(&line.replace("fn name", "pub fn name"));
                layouts_rs.push('\n');
            } else {
                layouts_rs.push_str(line);
                layouts_rs.push('\n');
            }
        }

        let mut in_impl = false;
        for line in layout_lines {
            if line.trim().starts_with("fn layout_positions(&self) -> Vec<(i32, i32, i32)> {") {
                layouts_rs.push_str("impl Level {\n    pub fn layout_positions(&self) -> Vec<(i32, i32, i32)> {\n");
                in_impl = true;
                continue;
            }
            if in_impl {
                if line.trim().starts_with("match self.current_level {") {
                    layouts_rs.push_str("        match self {\n");
                    continue;
                }
                if line.contains("Self::layout_") {
                    layouts_rs.push_str(&line.replace("Self::layout_", "Self::layout_"));
                    layouts_rs.push('\n');
                    continue;
                }
            }
            
            if line.trim().starts_with("fn layout_") && !line.trim().starts_with("fn layout_positions") {
                layouts_rs.push_str(&line.replace("fn layout_", "pub fn layout_"));
                layouts_rs.push('\n');
                continue;
            }
            layouts_rs.push_str(line);
            layouts_rs.push('\n');
        }
        layouts_rs.push_str("}\n");

        fs::write("src/layouts.rs", layouts_rs).expect("Failed to write layouts.rs");

        let mut new_main = String::new();
        for line in &lines[..es] {
            new_main.push_str(line);
            new_main.push('\n');
        }
        new_main.push_str("pub mod layouts;\nuse layouts::Level;\n\n");
        for line in &lines[ee..ls] {
            new_main.push_str(line);
            new_main.push('\n');
        }
        for line in &lines[le..] {
            new_main.push_str(line);
            new_main.push('\n');
        }

        fs::write("src/main.rs", new_main).expect("Failed to write main.rs");
        println!("Refactor successful.");
    } else {
        println!("Missing boundaries!");
    }
}
