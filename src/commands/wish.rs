use crate::structs;
use rand::seq::IteratorRandom;

pub fn get_command() -> structs::command::Command {
    let alias = &["greet", "message"];
    structs::command::Command::new(
        "wish",
        alias,
        "Get a random birthday wish text",
        &format!(
            "Picks out a random birthday wish from a set of wishes.\n\
You can pass in the tone of wish as an argument (formal, informal, casual)\n\
alias: {}",
            alias.join(", ")
        ),
        wish_command,
    )
}

fn wish_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.is_empty() {
        return "\nPlease specify the tone of the wish as an argument.\n\
Run `help wish` for more details.\n"
            .to_string();
    }

    const FORMAL: [&str; 20] = [
        "Wishing you a joyous birthday, [Name]. May the year ahead be filled with happiness and success.",
        "On your special day, [Name], may all your dreams and wishes come true. Happy birthday.",
        "May your birthday be as wonderful as you are, [Name]. Wishing you all the best.",
        "Happy birthday, [Name]. Here's to another year of making memories and experiencing all life has to offer.",
        "On this special day, [Name], may you be surrounded by love and laughter. Happy birthday.",
        "[Name], wishing you a birthday filled with joy and happiness. May the year ahead be prosperous.",
        "Happy birthday, [Name]. May this special day bring you all the happiness you deserve.",
        "On your birthday, [Name], may all your hopes and dreams come to fruition. Wishing you a joyous celebration.",
        "[Name], wishing you a very happy birthday and all the best for the year ahead.",
        "Happy birthday, [Name]. May your day be as bright and beautiful as you are.",
        "[Name], may your birthday be a day of joy, love, and reflection. Happy birthday.",
        "Wishing you a very happy birthday, [Name]. May your day be as wonderful as you are.",
        "On this special day, [Name], may you be surrounded by all the people who care about you. Happy birthday.",
        "Happy birthday, [Name]. May you be blessed with good health, happiness, and success in the year ahead.",
        "[Name], on your birthday, may you be filled with love, laughter, and joy. Wishing you all the best.",
        "Happy birthday, [Name]. May the coming year be filled with exciting new experiences and endless opportunities.",
        "On this day, [Name], may you be surrounded by love, laughter, and good wishes. Happy birthday.",
        "[Name], happy birthday. Wishing you all the happiness and success you deserve in the year ahead.",
        "On your birthday, [Name], may all your dreams come true. Happy birthday and all the best.",
        "Happy birthday, [Name]. Wishing you a year of happiness, health, and success. May all your dreams come true."
    ];

    const INFORMAL: [&str; 20] = [
        "Sup [Name], happy bday man :)",
        "Yo [Name], wishing you a lit bday celebration <3",
        "Have a boss birthday, [Name] :) You deserve it.",
        "Yo [Name], hope your birthday is fire 🔥🎉",
        "Happy bday, [Name]! Have a wild one. :D",
        "What's up [Name]? Have an awesome birthday.",
        "[Name], birthday vibes 🎉🎂 hope it's a good one.",
        "Have a killer birthday, [Name] 🎉 Let's party.",
        "Hey [Name], happy birthday 🎂 Have a blast today.",
        "Sup [Name], sending birthday wishes your way. Have a good one. :)",
        "[Name], happy bday bro :)",
        "Yo [Name], hope your birthday is lit 🔥🎉",
        "Sup [Name], happy birthday 🎂 Have a good one.",
        "[Name], happy birthday 🎉 Let's party.",
        "Hey [Name], hope your birthday is as wild as you are. :D",
        "Happy birthday, [Name]. Have a blast 🎂🎉",
        "[Name], birthday vibes 🎉 hope it's a good one.",
        "Hey [Name], sending birthday wishes your way. Have a wild one. 🎂🎉",
        "Sup [Name], happy bday man. Let's make some memories today. :)",
        "[Name], happy birthday 🎂 Here's to another year of awesomeness.",
    ];

    const CASUAL: [&str; 20] = [
        "Hey [Name], happy birthday bud!",
        "[Name], hope your birthday is as awesome as you are.",
        "Wishing you all the best on your birthday, [Name].",
        "Happy birthday, [Name]. Let's make it a good one!",
        "Just wanted to wish you a happy birthday, [Name]. Have a great day.",
        "[Name], hope your birthday is as special as you are.",
        "Happy birthday, [Name]. Here's to another year of awesomeness.",
        "Wishing you a birthday that's just as cool as you are, [Name].",
        "Hey [Name], happy birthday! Let's make some memories today.",
        "[Name], have the best birthday ever. You deserve it.",
        "Happy birthday, [Name]! Here's to another year of living life to the fullest.",
        "Wishing you all the happiness in the world on your birthday, [Name].",
        "Hey [Name], hope your birthday is as awesome as you are.",
        "Happy birthday, [Name]. Let's make it a day to remember.",
        "Just wanted to say happy birthday, [Name]. Have a great day.",
        "[Name], wishing you a birthday that's as amazing as you are.",
        "Happy birthday, [Name]. Cheers to another year of greatness.",
        "Hey [Name], have an epic birthday. You deserve it.",
        "[Name], here's to a birthday filled with good times and great friends.",
        "Happy birthday, [Name]. Hope it's a day as special as you are.",
    ];

    let msgs = match args[0].trim().to_lowercase().as_str() {
        "formal" => &FORMAL,
        "informal" => &INFORMAL,
        "casual" => &CASUAL,
        _ => return "\nInvalid tone specified.\n".to_string(),
    };

    format!(
        "\n{}\n",
        msgs.iter().choose(&mut rand::thread_rng()).unwrap()
    )
}
