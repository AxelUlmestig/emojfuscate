use clap::{Parser, Subcommand, ValueEnum};
use hex;
use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::process::ExitCode;
use std::str;
use uuid::Uuid;

use emojfuscate::{
    ByteInSequence, Demojfuscate, Emojfuscate, FromEmojiError, IsEmojiRepresentation,
};
mod hex_stream;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        input: String,
        #[arg(short, long, default_value_t = DataType::Text)]
        data_type: DataType,
        #[arg(short, long)]
        line_break: bool,
    },
    Decode {
        input: String,
        #[arg(short, long, default_value_t = DataType::Text)]
        data_type: DataType,
        #[arg(short, long)]
        line_break: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum DataType {
    Text,
    UUID,
    Hexadecimal,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = match self {
            DataType::Text => "text",
            DataType::UUID => "uuid",
            DataType::Hexadecimal => "hexadecimal",
        };
        write!(f, "{}", stringified)
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let unwrapped_std_in = io::stdin().bytes().map(|b| b.unwrap());

    let mut stream = BufWriter::new(io::stdout());

    match &cli.command {
        Commands::Encode {
            line_break,
            data_type,
            input,
        } => {
            match &data_type {
                DataType::UUID => {
                    let uuid = match input.as_str() {
                        "-" => Uuid::parse_str(
                            std::str::from_utf8(&unwrapped_std_in.collect::<Vec<u8>>()).unwrap(),
                        )
                        .unwrap(),
                        uuid_string => Uuid::parse_str(uuid_string).unwrap(),
                    };

                    for emoji in uuid.emojfuscate_stream() {
                        stream.write(emoji.to_string().as_bytes()).unwrap();
                    }
                }
                DataType::Hexadecimal => match input.as_str() {
                    "-" => {
                        for emoji in
                            hex_stream::HexStream::new(unwrapped_std_in).emojfuscate_stream()
                        {
                            stream.write(emoji.to_string().as_bytes()).unwrap();
                        }
                    }
                    some_string => {
                        for emoji in
                            hex_stream::HexStream::new(some_string.bytes()).emojfuscate_stream()
                        {
                            stream.write(emoji.to_string().as_bytes()).unwrap();
                        }
                    }
                },
                DataType::Text => match input.as_str() {
                    "-" => {
                        for emoji in unwrapped_std_in.emojfuscate_stream() {
                            stream.write(emoji.to_string().as_bytes()).unwrap();
                        }
                    }
                    some_string => {
                        stream
                            .write(some_string.bytes().emojfuscate().as_bytes())
                            .unwrap();
                    }
                },
            }

            if *line_break {
                stream.write("\n".as_bytes()).unwrap();
            }
        }
        Commands::Decode {
            line_break,
            data_type,
            input,
        } => {
            match &data_type {
                DataType::UUID => {
                    let r_uuid: Result<Uuid, FromEmojiError> = match input.as_str() {
                        "-" => unwrapped_std_in.demojfuscate(),
                        some_string => some_string.bytes().demojfuscate(),
                    };

                    match r_uuid {
                        Ok(uuid) => {
                            stream
                                .write(
                                    uuid.hyphenated()
                                        .encode_lower(&mut Uuid::encode_buffer())
                                        .as_bytes(),
                                )
                                .unwrap();
                        }
                        Err(FromEmojiError::NotEnoughEmoji) => {
                            eprintln!("Not enough emoji in input to construct a UUID");
                            return ExitCode::FAILURE;
                        }
                        Err(FromEmojiError::UnexpectedInput(error_message)) => {
                            eprintln!("{}", error_message);
                            return ExitCode::FAILURE;
                        }
                        Err(FromEmojiError::InvalidUtf8) => {
                            eprintln!("The input bytes are not valid UTF-8");
                            return ExitCode::FAILURE;
                        }
                        Err(FromEmojiError::InputIsNotAnEmoji(error_message)) => {
                            eprintln!("{}", error_message);
                            return ExitCode::FAILURE;
                        }
                        Err(FromEmojiError::MissingSequenceStart) => {
                            eprintln!("Expected a sequence start emoji when decoding data with variable length");
                            return ExitCode::FAILURE;
                        }
                        Err(FromEmojiError::UnexpectedSequenceStart(err)) => {
                            eprintln!("Unexpected a sequence start emoji {}", err);
                            return ExitCode::FAILURE;
                        }
                        Err(FromEmojiError::UnexpectedSequenceEnd) => {
                            eprintln!("Unexpected a sequence end emoji when decoding data");
                            return ExitCode::FAILURE;
                        }
                    }
                }
                DataType::Hexadecimal => match input.as_str() {
                    "-" => {
                        for parsed_byte in unwrapped_std_in.demojfuscate_byte_stream() {
                            let byte = match parsed_byte {
                                Ok(ByteInSequence::Byte(b)) => b,
                                _ => continue,
                            };

                            stream.write(hex::encode(&[byte]).as_bytes()).unwrap();
                        }
                    }
                    some_string => {
                        for parsed_byte in some_string.bytes().demojfuscate_byte_stream() {
                            let byte = match parsed_byte {
                                Ok(ByteInSequence::Byte(b)) => b,
                                _ => continue,
                            };

                            stream.write(hex::encode(&[byte]).as_bytes()).unwrap();
                        }
                    }
                },
                DataType::Text => match input.as_str() {
                    "-" => {
                        for parsed_byte in unwrapped_std_in.demojfuscate_byte_stream() {
                            let byte = match parsed_byte {
                                Ok(ByteInSequence::Byte(b)) => b,
                                _ => continue,
                            };

                            stream.write(&[byte]).unwrap();
                        }
                    }
                    some_string => {
                        for parsed_byte in some_string.demojfuscate_byte_stream() {
                            let byte = match parsed_byte {
                                Ok(ByteInSequence::Byte(b)) => b,
                                _ => continue,
                            };

                            stream.write(&[byte]).unwrap();
                        }
                    }
                },
            }

            if *line_break {
                stream.write("\n".as_bytes()).unwrap();
            }
        }
    };

    io::stdout().flush().unwrap();

    return ExitCode::SUCCESS;
}
