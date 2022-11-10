use crate::Editor;
use anyhow::Result;
use std::collections::HashMap;

type Command = Box<dyn Fn(&Editor) -> Result<()>>;

pub struct CommandList {
    bindings: HashMap<String, CommandType>,
}

impl CommandList {
    pub fn new() -> Result<CommandList> {
        Ok(CommandList {
            bindings: HashMap::new(),
        })
    }

    pub fn execute(&self, command: String, editor: &Editor) -> Result<()> {
        log::info!("command {} issued", command);
        Into::<Command>::into(self.get_command_type(command)?)(editor)
    }

    pub fn new_with_default_bindings() -> Result<CommandList> {
        use CommandType::*;
        let bindings: HashMap<String, _> = HashMap::from([("".into(), SaveFile)]);

        Ok(CommandList { bindings })
    }

    fn get_command_type(&self, command: String) -> Result<CommandType> {
        Ok(*self
            .bindings
            .get(&command)
            .unwrap_or_else(|| &CommandType::Unrecognized))
    }
}

#[derive(Clone, Copy)]
enum CommandType {
    SaveFile,
    Unrecognized,
}

impl Into<Command> for CommandType {
    fn into(self) -> Command {
        use CommandType::*;
        Box::new(match self {
            SaveFile => save_file,
            _ => |_| Ok(()),
        })
    }
}

fn save_file(_editor: &Editor) -> Result<()> {
    log::info!("saved file");
    Ok(())
}
