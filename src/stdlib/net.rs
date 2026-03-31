use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::interpreter::Value;

/// Wrap a TcpListener in an Arc<Mutex<>> so Value can be Clone
pub fn builtin_tcp_lalela(args: &[Value]) -> Value {
    let addr = match args.first() {
        Some(Value::Str(s)) => s.clone(),
        _ => { eprintln!("tcpLalela: expected a string address e.g. \"0.0.0.0:8080\""); std::process::exit(1); }
    };
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| {
        eprintln!("tcpLalela: failed to bind '{}': {}", addr, e);
        std::process::exit(1);
    });
    println!("[net] Listening on {}", addr);
    Value::Listener(Arc::new(Mutex::new(listener)))
}

/// Accept one incoming connection from a listener
pub fn builtin_tcp_amukela(args: &[Value]) -> Value {
    let listener = match args.first() {
        Some(Value::Listener(l)) => l.clone(),
        _ => { eprintln!("tcpAmukela: expected a TcpListener"); std::process::exit(1); }
    };
    let (stream, addr) = listener.lock().unwrap().accept().unwrap_or_else(|e| {
        eprintln!("tcpAmukela: accept failed: {}", e);
        std::process::exit(1);
    });
    println!("[net] Connection from {}", addr);
    Value::Stream(Arc::new(Mutex::new(stream)))
}

/// Connect to a remote TCP server
pub fn builtin_tcp_xhumana(args: &[Value]) -> Value {
    let addr = match args.first() {
        Some(Value::Str(s)) => s.clone(),
        _ => { eprintln!("tcpXhumana: expected a string address e.g. \"127.0.0.1:8080\""); std::process::exit(1); }
    };
    let stream = TcpStream::connect(&addr).unwrap_or_else(|e| {
        eprintln!("tcpXhumana: connect to '{}' failed: {}", addr, e);
        std::process::exit(1);
    });
    println!("[net] Connected to {}", addr);
    Value::Stream(Arc::new(Mutex::new(stream)))
}

/// Read one line from a stream (blocks until newline or disconnect)
pub fn builtin_tcp_funda(args: &[Value]) -> Value {
    let stream = match args.first() {
        Some(Value::Stream(s)) => s.clone(),
        _ => { eprintln!("tcpFunda: expected a TcpStream"); std::process::exit(1); }
    };
    let mut line = String::new();
    let guard = stream.lock().unwrap();
    let mut reader = BufReader::new(&*guard);
    match reader.read_line(&mut line) {
        Ok(0) => Value::Nil,   // connection closed
        Ok(_) => Value::Str(line.trim_end_matches('\n').trim_end_matches('\r').to_string()),
        Err(e) => { eprintln!("tcpFunda: read error: {}", e); Value::Nil }
    }
}

/// Write a string to a stream (appends newline)
pub fn builtin_tcp_thumela(args: &[Value]) -> Value {
    let stream = match args.first() {
        Some(Value::Stream(s)) => s.clone(),
        _ => { eprintln!("tcpThumela: expected a TcpStream as first arg"); std::process::exit(1); }
    };
    let msg = match args.get(1) {
        Some(Value::Str(s)) => s.clone(),
        Some(Value::Number(n)) => {
            if n.fract() == 0.0 { format!("{}", *n as i64) } else { format!("{}", n) }
        }
        _ => { eprintln!("tcpThumela: expected a string or number as second arg"); std::process::exit(1); }
    };
    let mut guard = stream.lock().unwrap();
    write!(guard, "{}\n", msg).unwrap_or_else(|e| {
        eprintln!("tcpThumela: write error: {}", e);
    });
    guard.flush().ok();
    Value::Nil
}

/// Close a stream
pub fn builtin_tcp_vala(args: &[Value]) -> Value {
    match args.first() {
        Some(Value::Stream(s)) => {
            let guard = s.lock().unwrap();
            guard.shutdown(std::net::Shutdown::Both).ok();
            println!("[net] Connection closed");
        }
        _ => { eprintln!("tcpVala: expected a TcpStream"); }
    }
    Value::Nil
}
