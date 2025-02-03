use std::{thread, time::Duration, io::{Write,Read}};

#[cfg(feature = "socketpair")]
use socketpair as sp;

#[cfg(not(feature = "socketpair"))]
mod sp {
    use std::{io,net};
    pub type SocketpairStream = std::net::TcpStream;
    pub fn socketpair_stream() -> io::Result<(SocketpairStream, SocketpairStream)> {
	let listener =net::TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;
        let s1 = net::TcpStream::connect(addr)?;
        let (s2, s2_addr) = listener.accept()?;
        assert_eq!(s1.local_addr().unwrap(), s2_addr);
        Ok((s1, s2))
    }
}

fn reader(mut s:sp::SocketpairStream) {
    let mut buf = [0_u8; 1];
    s.read_exact(&mut buf).unwrap();
    assert_eq!(&buf[..], b"x");
}

fn writer(mut s: sp::SocketpairStream) {
    thread::sleep(Duration::new(2,0));
    s.write_all(b"z").unwrap();
}

fn interactor(mut s: sp::SocketpairStream) {
    s.write_all(b"x").unwrap();
    let mut buf = [0_u8; 1];
    s.read_exact(&mut buf).unwrap();
    assert_eq!(&buf[..], b"z");
}

fn main() {
    let (s1r, s2) = sp::socketpair_stream().unwrap();

    let s1w = s1r.try_clone().unwrap();

    let h_r = thread::spawn(move || reader(s1r));
    let h_w = thread::spawn(move || writer(s1w));
    let h2 = thread::spawn(move || interactor(s2));

    h_r.join().unwrap();
    h_w.join().unwrap();
    h2.join().unwrap();
    eprintln!("OK");
}
