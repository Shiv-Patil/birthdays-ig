use crate::structs;

pub fn get_command() -> structs::command::Command {
    let alias = &["info", "?", "what"];
    structs::command::Command::new(
        "help",
        alias,
        "Get info about a particular command",
        &format!(
            "Gets information about a particular command. Takes the command as an argument.\n\
alias: {}",
            alias.join(", ")
        ),
        help_command,
    )
}

fn help_command(bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.is_empty() {
        let cmds = &mut bot
            .commands
            .iter()
            .collect::<Vec<(&String, &structs::command::Command)>>();
        cmds.sort_by(|a, b| a.0.cmp(b.0));
        let mut res = String::from("\nAvailable commands:\n\n");
        for (cmdname, cmd) in cmds {
            res.push_str(&format!("{} - {}\n", cmdname, cmd.description));
        }
        return res;
    }

    let mut res = String::new();
    for arg in args {
        if bot.commands.contains_key(arg.to_owned()) {
            res.push_str(&format!(
                "\n{}:\n{}\n",
                arg,
                bot.commands.get(arg.to_owned()).unwrap().helpstring
            ));
        } else {
            res.push_str(&format!("\n`{arg}` - No such command exists\n"));
        }
    }
    res
}
