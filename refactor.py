import os
import re

with open("src/main.rs", "r", encoding="utf-8") as f:
    lines = f.readlines()

enum_start = -1
enum_end = -1
for i, line in enumerate(lines):
    if line.startswith("#[derive(Clone, Copy, PartialEq, Eq, Debug)]"):
        enum_start = i
    if enum_start != -1 and line.startswith("struct Game {"):
        enum_end = i
        break

layouts_start = -1
layouts_end = -1
for i, line in enumerate(lines):
    if line.strip().startswith("fn layout_positions(&self) -> Vec<(i32, i32, i32)> {"):
        layouts_start = i
    if line.strip().startswith("fn generate_board(&mut self) {"):
        layouts_end = i
        break

if enum_start != -1 and layouts_end != -1:
    enum_lines = lines[enum_start:enum_end]
    layout_lines = lines[layouts_start:layouts_end]
    
    # Process layout lines to change `fn layout_` to `pub fn layout_`
    processed_layouts = []
    
    in_impl_level = False
    for line in layout_lines:
        if line.strip().startswith("fn layout_positions(&self) -> Vec<(i32, i32, i32)> {"):
            processed_layouts.append("impl Level {\n")
            processed_layouts.append("    pub fn layout_positions(&self) -> Vec<(i32, i32, i32)> {\n")
            in_impl_level = True
            continue
            
        if in_impl_level and line.strip().startswith("match self.current_level {"):
            processed_layouts.append("        match self {\n")
            continue
            
        if line.strip().startswith("fn layout_") and not line.strip().startswith("fn layout_positions"):
            processed_layouts.append(line.replace("fn layout_", "pub fn layout_"))
            continue
            
        processed_layouts.append(line)
        
    processed_layouts.append("}\n") # close impl Level
    
    # Process enum lines to add pub to methods
    processed_enum = []
    for line in enum_lines:
        if line.strip().startswith("fn next(&self) -> Option<Self> {"):
            processed_enum.append(line.replace("fn next", "pub fn next"))
        elif line.strip().startswith("fn name(&self) -> &'static str {"):
            processed_enum.append(line.replace("fn name", "pub fn name"))
        else:
            processed_enum.append(line)
            
    with open("src/layouts.rs", "w", encoding="utf-8") as f:
        f.writelines(processed_enum)
        f.writelines(processed_layouts)
        
    # Main.rs: remove these blocks
    new_main = (
        lines[:enum_start] + 
        ["pub mod layouts;\n", "use layouts::Level;\n\n"] + 
        lines[enum_end:layouts_start] + 
        lines[layouts_end:]
    )
    with open("src/main.rs", "w", encoding="utf-8") as f:
        f.writelines(new_main)
    print("Refactor complete!")
else:
    print(f"Error finding bounds. enum_start={enum_start}, enum_end={enum_end}, layouts_start={layouts_start}, layouts_end={layouts_end}")
