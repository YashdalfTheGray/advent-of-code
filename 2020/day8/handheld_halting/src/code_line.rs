pub struct CodeLine {
    pub instruction: String,
    pub offset: i32,
    pub executed: bool,
}

impl CodeLine {
    pub fn new(instruction: &str, offset: i32) -> CodeLine {
        CodeLine {
            instruction,
            offset,
            executed: false,
        }
    }
}
