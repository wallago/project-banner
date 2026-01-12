use clap::Parser;
use crossterm::terminal::size;
use unicode_width::UnicodeWidthStr;

#[derive(Parser, Debug, Clone)]
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
    #[arg(long, default_value_t = size().unwrap().0.into())]
    size: usize,
    #[arg(long)]
    logo: Option<String>,
}

fn visible_width(s: &str) -> usize {
    let stripped = strip_ansi_codes(s);
    UnicodeWidthStr::width(stripped.as_str())
}

fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip until we hit a letter (end of ANSI sequence)
            while let Some(&next) = chars.peek() {
                chars.next();
                if next.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn padded_line(label: &str, value: &str, max_width: usize) -> String {
    let label_width = visible_width(label);
    let value_width = visible_width(value);
    let total_content = label_width + value_width;

    let padding = if total_content < max_width {
        " ".repeat(max_width - total_content)
    } else {
        String::new()
    };

    format!("{}{}{}", label, padding, value)
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in text.split_whitespace() {
        let word_width = UnicodeWidthStr::width(word);

        if current_width == 0 {
            current_line = word.to_string();
            current_width = word_width;
        } else if current_width + 1 + word_width <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
            current_width += 1 + word_width;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            current_width = word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn banner(max_width: usize, rev: &str, bold: &str, reset: &str) -> String {
    let banner_text = "Project Banner";
    let banner_padding = max_width.saturating_sub(banner_text.len());
    format!(
        "{}{}{}{}{}",
        rev,
        bold,
        banner_text,
        " ".repeat(banner_padding),
        reset
    )
}

fn product(max_width: usize, bold: &str, reset: &str, dim: &str, args: Args) -> String {
    let product_label = format!("{}Product:{}", dim, reset);
    let product_value = format!(
        "{}{}{}",
        bold,
        args.product.unwrap_or("none".to_string()),
        reset
    );
    padded_line(&product_label, &product_value, max_width)
}

fn part(max_width: usize, bold: &str, reset: &str, dim: &str, args: Args) -> String {
    let part_label = format!("{}Part:{}", dim, reset);
    let part_value = format!(
        "{}{}{}",
        bold,
        args.part.unwrap_or("none".to_string()),
        reset
    );
    padded_line(&part_label, &part_value, max_width)
}

fn code(max_width: usize, bold: &str, reset: &str, dim: &str, args: Args) -> String {
    let code_label = format!("{}Code:{}", dim, reset);
    let code_value = format!(
        "{}{}{}",
        bold,
        args.code.unwrap_or("XXX-XXXX-XXX".to_string()),
        reset
    );
    padded_line(&code_label, &code_value, max_width)
}

fn owner(max_width: usize, bold: &str, rev: &str, reset: &str, args: Args) -> String {
    let logo_part = if let Some(ref logo) = args.logo {
        format!("{}{}{}{} ", rev, bold, logo, reset)
    } else {
        String::new()
    };
    let owner_label = "(c)";
    let owner_value = format!(
        "Property of {}{}{}{}",
        logo_part,
        bold,
        args.owner.unwrap_or("none".to_string()),
        reset
    );
    padded_line(owner_label, &owner_value, max_width)
}

fn tip_lines(max_width: usize, reset: &str, dim: &str, tip: String) -> Vec<String> {
    let prefix = "   󰁕 ";
    let prefix_width = visible_width(prefix);
    let continuation_prefix = "     "; // same width, no icon
    let available_width = max_width.saturating_sub(prefix_width);

    let wrapped = wrap_text(&tip, available_width);
    wrapped
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let p = if i == 0 { prefix } else { continuation_prefix };
            let text = format!("{}{}{}", dim, line, reset);
            let line_width = visible_width(p) + visible_width(&line);
            let padding = " ".repeat(max_width.saturating_sub(line_width));
            format!("{}{}{}", p, text, padding)
        })
        .collect()
}

fn main() {
    let args = Args::parse();
    if args.size < 32 {
        return;
    }
    let max_width = args.size - 4;
    let dim = "\x1b[2m"; // ANSI code for dim color
    let reset = "\x1b[0m"; // reset formatting
    let rev = "\x1b[7m"; // revert color
    let bold = "\x1b[1m"; // bold color

    let banner = banner(max_width, rev, bold, reset);
    let product = product(max_width, bold, reset, dim, args.clone());
    let part = part(max_width, bold, reset, dim, args.clone());
    let code = code(max_width, bold, reset, dim, args.clone());
    let owner = owner(max_width, bold, rev, reset, args.clone());

    // Print banner
    println!("┌{}┐", "─".repeat(max_width + 2));
    println!("│ {} │", banner);
    println!("├{}┤", "╌".repeat(max_width + 2));

    // Print main rows
    for row in [&product, &part, &code] {
        println!("│ {} │", row);
    }

    // Print tips if present
    if let Some(tips) = args.tips {
        println!("├{}┤", "╌".repeat(max_width + 2));

        let tips_header = format!("{}Tips:{}", dim, reset);
        let tips_header_line = padded_line("󰌵 ", &tips_header, max_width);
        println!("│ {} │", tips_header_line);

        for item in tips {
            for line in tip_lines(max_width, reset, dim, item) {
                println!("│ {} │", line);
            }
        }
    }

    // Print owner line
    println!("├{}┤", "╌".repeat(max_width + 2));
    println!("│ {} │", owner);
    println!("└{}┘", "─".repeat(max_width + 2));
}
