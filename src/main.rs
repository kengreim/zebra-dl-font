#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    /// Relative path to font to upload
    input: String,

    /// Desired name of font on Zebra printer. .TTF will be appended if not included
    name: String,

    #[arg(short, long, default_value_t = String::new(), hide_default_value = true)]
    /// Name of Zebra or other ZPL-capable printer to upload font to.
    /// If omitted, the first printer that contains ZPL in its name will be used
    printer: String,
}

fn main() {
    let args = Args::parse();

    let sys_printers = printers::get_printers();

    let printer = if !args.printer.is_empty() {
        if let Some(printer) = sys_printers.iter().find(|p| p.name == args.printer) {
            printer
        } else {
            eprintln!("Could not find printer with name: {}", args.printer);
            return;
        }
    } else if let Some(printer) = sys_printers.iter().find(|p| p.name.contains("ZPL")) {
        printer
    } else {
        eprintln!("Could not find printer with ZPL in name");
        return;
    };

    if !args.name.to_ascii_uppercase().ends_with(".TTF") && args.name.contains('.') {
        eprintln!("Name must either end in .TTF or not include a period (in which case .TTF will be appended");
        return;
    }

    let cleaned_name = if args.name.to_ascii_uppercase().ends_with(".TTF") {
        args.name
    } else {
        format!("{}.TTF", args.name.to_ascii_uppercase())
    };

    let mut font_bytes = match fs::read(&args.input) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error while attempting to read font: {e}");
            return;
        }
    };

    let num_bytes = font_bytes.len();
    let mut command_bytes = format!("~DYE:{cleaned_name},B,T,{num_bytes},0,")
        .as_bytes()
        .to_vec();
    command_bytes.append(&mut font_bytes);

    if let Err(e) = printer.print(&command_bytes, None) {
        eprintln!("Error while transferring font: {e}");
    } else {
        println!(
            "Successfully transferred {} to E:{cleaned_name}",
            args.input
        );
    }
}
