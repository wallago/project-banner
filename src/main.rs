use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    product: Option<String>,
    #[arg(long)]
    part: Option<String>,
    #[arg(long)]
    code: Option<String>,
    #[arg(long)]
    owner: Option<String>,
    #[arg(long)]
    tips: Option<Vec<String>>,
    #[arg(long, default_value_t = 40)]
    size: usize,
    #[arg(long)]
    logo: Option<String>,
}

fn main() {
    let args = Args::parse();
    let dim = "\x1b[2m"; // ANSI code for dim color (secondary)
    let reset = "\x1b[0m"; // reset formatting
    let rev = "\x1b[7m"; // revert color
    let bold = "\x1b[1m"; // bold color
    let max_width = args.size;

    let banner_text = format!(
        "{}{} Project Banner{}{}",
        rev,
        bold,
        " ".repeat((max_width + rev.len() + reset.len()) - (23),),
        reset,
    );

    let product_label = format!("{}Product:{}", dim, reset);
    let product_value = format!(
        "{}{}{}",
        bold,
        args.product.unwrap_or("none".to_string()),
        reset
    );
    let product_padding =
        " ".repeat((max_width + 4 * 4) - (product_value.len() + product_label.len()));
    let product_line = format!("{}{}{}", product_label, product_padding, product_value);

    let part_label = format!("{}Part:{}", dim, reset);
    let part_value = format!(
        "{}{}{}",
        bold,
        args.part.unwrap_or("none".to_string()),
        reset
    );
    let part_padding = " ".repeat((max_width + 4 * 4) - (part_value.len() + part_label.len()));
    let part_line = format!("{}{}{}", part_label, part_padding, part_value);

    let code_label = format!("{}Code:{}", dim, reset);
    let code_value = format!(
        "{}{}{}",
        bold,
        args.code.unwrap_or("XXX-XXXX-XXX".to_string()),
        reset
    );
    let code_padding = " ".repeat((max_width + 4 * 4) - (code_value.len() + code_label.len()));
    let code_line = format!("{}{}{}", code_label, code_padding, code_value);

    let owner_label = "(c)";
    let owner_value = format!(
        "Property of {}{}{}{} {}{}{}",
        rev,
        bold,
        args.logo.unwrap_or_default(),
        reset,
        bold,
        args.owner.unwrap_or("none".to_string()),
        reset
    );
    let owner_padding = " ".repeat(
        (max_width + 4 * 5)
            - (unicode_width::UnicodeWidthStr::width(owner_value.as_str()) + owner_label.len()),
    );
    let owner_line = format!("{}{}{}", owner_label, owner_padding, owner_value);

    let rows = vec![product_line, part_line, code_line];

    println!("┌{}┐", "─".repeat(max_width + 2));
    println!("│ {} │", banner_text);
    println!("├{}┤", "╌".repeat(max_width + 2));
    for row in &rows {
        println!("│ {} │", row);
    }
    if let Some(tips) = args.tips {
        println!("├{}┤", "╌".repeat(max_width + 2));
        println!("│ 󰌵 {}Tips: {}{} │", dim, " ".repeat(max_width - 8), reset);
        for tip in tips {
            let tip = format!("{}{}{}", dim, tip, reset);
            let tip_padding = " ".repeat((max_width + dim.len() + reset.len()) - tip.len() - 5);
            println!("│    󰁕 {}{} │", tip, tip_padding);
        }
    }
    println!("├{}┤", "╌".repeat(max_width + 2));
    println!("│ {} │", owner_line);
    println!("└{}┘", "─".repeat(max_width + 2));
}
