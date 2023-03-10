use std::fmt::format;
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::thread;
use std::io;
use std::str;

#[derive(Clone, Debug)]
enum Command {
    Auth,
    Unknown(String),
}


impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match *self {
            Command::Auth => "AUTH",
            Command::Unknown(_) => "UNKN",
        }
    }
}

impl Command {
    pub fn new(input: Vec<u8>) -> io::Result<Self> {
        return Ok(Command::Auth)
    }
}


#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum ResultCode {
    RestartMarkerReply = 110,
    ServiceReadInXXXMinutes = 120,
    DataConnectionAlreadyOpen = 125,
    FileStatusOk = 150,
    Ok = 200,
    CommandNotImplementedSuperfluousAtThisSite = 202,
    SystemStatus = 211,
    DirectoryStatus = 212,
    FileStatus = 213,
    HelpMessage = 214,
    SystemType = 215,
    ServiceReadyForNewUser = 220,
    ServiceClosingControlConnection = 221,
    DataConnectionOpen = 225,
    ClosingDataConnection = 226,
    EnteringPassiveMode = 227,
    UserLoggedIn = 230,
    RequestedFileActionOkay = 250,
    PATHNAMECreated = 257,
    UserNameOkayNeedPassword = 331,
    NeedAccountForLogin = 332,
    RequestedFileActionPendingFurtherInformation = 350,
    ServiceNotAvailable = 421,
    CantOpenDataConnection = 425,
    ConnectionClosed = 426,
    FileBusy = 450,
    LocalErrorInProcessing = 451,
    InsufficientStorageSpace = 452,
    UnknownCommand = 500,
    InvalidParameterOrArgument = 501,
    CommandNotImplemented = 502,
    BadSequenceOfCommands = 503,
    CommandNotImplementedForThatParameter = 504,
    NotLoggedIn = 530,
    NeedAccountForStoringFiles = 532,
    FileNotFound = 550,
    PageTypeUnknown = 551,
    ExceededStorageAllocation = 552,
    FileNameNotAllowed = 553,
}


fn handle_client(mut stream: TcpStream) {
    println!("new client connected!");
}


fn send_cmd(stream: &mut TcpStream, code: ResultCode, message: &str) {
    let msg = if message.is_empty() { 
        //CommandNotImplemented = 502,
        format!("{}\r\n", code as u32)
    } else {
        format!("{} {}\r\n", code as u32, message)
    };
    println!("<==== {}", msg);
    write!(stream, "{}", msg).unwrap(); //could use ? and re-raise if this returned an optional
}


fn main() {

    let listener = TcpListener::bind("127.0.0.1:3131")
    .expect("Could not bind to this address");

    print!("Waiting for clients to connect...");

    // 'incoming' returns an iterator over the received connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move ||{
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("This stream's connection has failed");
            }
        }
    }
}
