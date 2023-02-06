use crate::structs;

pub fn help_command(bot: &structs::chatbot::ChatBot, args: &[&str]) -> String {
    // Checks if there iare no arguments. If so, displays all available commands
    if args.len() == 0 {
        let mut res = String::from("\nAvailable commands:\n\n");
        for (cmdname, cmd) in &bot.commands {
            res.push_str(&format!("{} - {}\n", cmdname, cmd.description));
        }
        return res;
    }

    // displays the help string of the command passed as argument if present
    let mut res = String::new();
    for arg in args {
        if bot.commands.contains_key(arg.to_owned()) {
            res.push_str(&format!("\n{}:\n{}\n", arg, bot.commands.get(arg.to_owned()).unwrap().helpstring));
        } else {
            res.push_str(&format!("\n`{}` - No such command exists\n", arg));
        }
    }
    res
}
