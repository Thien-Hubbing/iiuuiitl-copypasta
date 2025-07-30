use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Rust program to generate IIUUIITL copypasta", long_about = None)]
struct Args {
    /// The username in the copypasta
    #[arg(short, long)]
    name: String,

    /// What does the number transcend?
    #[arg(short, long)]
    transcendence: String,

    /// What number are we talking about
    #[arg(short = 'N', long)]
    number: String,

    /// Do we add an addiitonal "this is my username on" text?
    #[arg(short, long, required = false, default_missing_value = "YouTube")]
    additional: Option<String>
}

fn main() {
    let args = Args::parse();

    let base = if let Some(username_elsewhere) = args.additional {
        format!("Hello! You may know me as {} on the fictional Googology wiki, {} {}.", args.name, "but this is my username on", username_elsewhere)
    } else {
        format!("Hello! You may know me as {} on the fictional Googology wiki.", args.name)
    };

    let number = args.number;
    let transcends = args.transcendence;

    // Long ass string to avoid newlines/spaces (word wrap recommended)
    let rest_of_quote = format!("Anyway, just wanted to make sure that {number} is the final \"0 to\" video that you make (at least relating to \"{transcends} transcendence\"). I would like this to be clarified because if you're not going to highlight that {number} is unsurpassable, then don't include it in your videos, as that would be a misuse of my entry ({number}). I hope you can understand why, but aside from that, I just want to make sure that the truth is told, and that truth is that {number} is unsurpassable and eternally Trans-transcendental. Please verify this truth in any videos that you make about {number} or any videos that include it. Thanks!");

    println!("{base}\n{rest_of_quote}");
}
