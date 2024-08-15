use std::io::{self, Read, Stderr, Stdin, Stdout};

use nix::libc;

pub struct IOHandler {
    // Don't have to tell what these are :)
    input: Stdin,
    output: Stdout,
    err: Stderr,

    // Buffer to store the any input buffering we need
    in_buf: Vec<u8>,

    // Just termios related things
    c_cflag: Option<u32>,
    c_iflag: Option<u32>,
    c_lflag: Option<u32>,
    c_oflag: Option<u32>,
}

impl IOHandler {
    pub fn new() -> IOHandler {
        IOHandler {
            input: io::stdin(),
            output: io::stdout(),
            err: io::stderr(),
            in_buf: Vec::new(),

            c_cflag: None,
            c_iflag: None,
            c_oflag: None,
            c_lflag: None,
        }
    }

    pub fn read(&mut self) -> std::io::Result<u8> {
        let mut data = vec![0u8; 1];
        match self.input.read(&mut data) {
            Ok(_) => Ok(data[0]),
            Err(err) => Err(err),
        }
    }

    pub fn disable_line_buffering(&mut self) -> io::Result<()> {
        let mut termios = core::mem::MaybeUninit::uninit();
        unsafe {
            libc::tcgetattr(0, termios.as_mut_ptr());
        }

        let mut termios = unsafe { termios.assume_init() };

        self.c_lflag = Some(termios.c_lflag);
        self.c_oflag = Some(termios.c_oflag);
        self.c_iflag = Some(termios.c_iflag);
        self.c_cflag = Some(termios.c_cflag);

        unsafe {
            libc::cfmakeraw(&mut termios);
            libc::tcsetattr(0, libc::TCSADRAIN, &termios);
        }

        Ok(())
    }

    pub fn enable_line_buffering(&self) -> io::Result<()> {
        let mut termios = core::mem::MaybeUninit::uninit();
        unsafe {
            libc::tcgetattr(0, termios.as_mut_ptr());
        }

        let mut termios = unsafe { termios.assume_init() };
        if let (Some(c_cflag), Some(c_lflag), Some(c_iflag), Some(c_oflag)) =
            (self.c_cflag, self.c_iflag, self.c_iflag, self.c_oflag)
        {
            termios.c_cflag = c_cflag;
            termios.c_lflag = c_lflag;
            termios.c_iflag = c_iflag;
            termios.c_oflag = c_oflag;
        }

        unsafe {
            libc::tcsetattr(0, libc::TCSADRAIN, &termios);
        }

        Ok(())
    }

    pub fn buf_data(&mut self, data: u8) {
        self.in_buf.push(data);
    }

    pub fn clear_buffer(&mut self) {
        self.in_buf.clear();
    }
}

impl Default for IOHandler {
    fn default() -> Self {
        Self::new()
    }
}
