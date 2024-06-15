use clap::{Parser, Subcommand, ValueEnum};
use std::io;
use std::io::Read;
use std::io::Write;
use std::str;
use std::io::BufWriter;
use uuid::Uuid;
use hex;

use emojfuscate::to_emoji_stream::ToEmojiStream;
use emojfuscate::from_emoji_stream::{ConstructFromEmojiStream, FromEmoji};
use emojfuscate::hex_stream::HexStream;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    line_break: bool,
    // #[clap(value_enum, default_value_t=DataType::Text)]
    // data_type: DataType
}

#[derive(Subcommand)]
enum Commands {
    Encode { data_type: DataType, input: String },
    Decode { data_type: DataType, input: String },
}

#[derive(ValueEnum, Clone, Debug)]
enum DataType {
    Text,
    UUID,
    Hexadecimal
}

fn main() {
    let cli = Cli::parse();

    let unwrapped_std_in = io::stdin().bytes().map(|b| b.unwrap());

    let mut stream = BufWriter::new(io::stdout());

    match &cli.command {
        Commands::Encode { data_type, input } => {
            match &data_type {
                DataType::UUID => {
                    let uuid =
                        match input.as_str() {
                            "-" => { Uuid::parse_str(std::str::from_utf8(&unwrapped_std_in.collect::<Vec<u8>>()).unwrap()).unwrap() },
                            uuid_string => { Uuid::parse_str(uuid_string).unwrap() }
                        };

                    for emoji in uuid.to_emoji_stream() {
                        stream.write(emoji.to_string().as_bytes()).unwrap();
                    }
                },
                DataType::Hexadecimal => {
                    match input.as_str() {
                        "-" => {
                            for emoji in HexStream::new(unwrapped_std_in).to_emoji_stream() {
                                stream.write(emoji.to_string().as_bytes()).unwrap();
                            }
                        },
                        some_string => {
                            for emoji in HexStream::new(some_string.bytes()).to_emoji_stream() {
                                stream.write(emoji.to_string().as_bytes()).unwrap();
                            }
                        }
                    }
                },
                DataType::Text => {
                    match input.as_str() {
                        "-" => {
                            for emoji in unwrapped_std_in.to_emoji_stream() {
                                stream.write(emoji.to_string().as_bytes()).unwrap();
                            }
                        },
                        some_string => {
                            stream.write(some_string.bytes().to_emoji_string().as_bytes()).unwrap();
                        }
                    }
                }
            }
        },
        Commands::Decode { data_type, input } => {
            match &data_type {
                DataType::UUID => {
                    let uuid : Uuid = 
                        match input.as_str() {
                            "-" => unwrapped_std_in.from_emoji(),
                            some_string => some_string.bytes().from_emoji()
                        };

                    stream.write(uuid.hyphenated().encode_lower(&mut Uuid::encode_buffer()).as_bytes()).unwrap();
                },
                DataType::Hexadecimal => {
                    match input.as_str() {
                        "-" => {
                            for byte in unwrapped_std_in.from_emoji_stream() {
                                stream.write(hex::encode(&[byte]).as_bytes()).unwrap();
                            }
                        },
                        some_string => {
                            for byte in some_string.bytes().from_emoji_stream() {
                                stream.write(hex::encode(&[byte]).as_bytes()).unwrap();
                            }
                        }
                    }
                }
                DataType::Text => {
                    match input.as_str() {
                        "-" => {
                            for byte in unwrapped_std_in.from_emoji_stream() {
                                stream.write(&[byte]).unwrap();
                            }
                        },
                        some_string => {
                            for byte in some_string.bytes().from_emoji_stream() {
                                stream.write(&[byte]).unwrap();
                            }
                        }
                    }
                }
            }
        }
    };

    if cli.line_break {
        stream.write("\n".as_bytes()).unwrap();
    }

    io::stdout().flush().unwrap();
}

