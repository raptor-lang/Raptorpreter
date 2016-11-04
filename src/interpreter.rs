
use std::fmt;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug, Default)]
pub struct Interpreter {
    // File data
    header: RaptorHeader,
    bytecode: Vec<u8>,
    
    // Runtime stuff
    stack: Vec<i32>,
    memory: Vec<i32>,
}

// TODO: Use more slices
impl Interpreter {
    pub fn new(mut data: Vec<u8>) -> Interpreter {
        let i = Interpreter {
            header: read_header(&data),
            bytecode: data.drain(HEADER_SIZE..).collect(),
            stack: Vec::new(),
            memory: Vec::new(),
        };
        
        i
    }

    pub fn run(&mut self) {
        debug!("Running...");
    }
}

// TODO: Move all header stuff to another module

const HEADER_SIZE: usize = 8;
const HEADER_OFFSET: usize = 0;
const MAGIC_VALUE: u32 = 0x5AB70500;

#[derive(Default)]
pub struct RaptorHeader {
    magic: u32,     // Magic number + padding ( 0x5AB70500 )
    var_count: u32,     // Number of variables
}

impl RaptorHeader {
    fn verify(&self) -> bool {
        self.magic == MAGIC_VALUE
    }
}

impl fmt::Debug for RaptorHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RaptorHeader {{
            magic: 0x{:04X}
            var_count: {}
        }}",
            self.magic,
            self.var_count
        )
    }
}

pub fn read_header(data: &Vec<u8>) -> RaptorHeader {
    if data.len() < HEADER_SIZE  {
        panic!("Invalid header size");
    }
    
    let header = read_header_impl(&data);
    if !header.verify() {
        panic!("Invalid header magic");
    }

    debug!("Header verified");
    header
}


fn read_header_impl(data: &Vec<u8>) -> RaptorHeader {
    use std::slice;
    use std::io::Read;

    let mut buffer: [u8; HEADER_SIZE] = [0u8; HEADER_SIZE];

    for i in 0..HEADER_SIZE {
        buffer[i] = data[i + HEADER_OFFSET];
    }

    let mut buffer_slice: &[u8] = &buffer;

    let mut header: RaptorHeader = Default::default();
    
    header.magic = buffer_slice.read_u32::<BigEndian>().unwrap();
    header.var_count = buffer_slice.read_u32::<BigEndian>().unwrap();
    
    print!("{:04X}",header.magic);
    debug!("Read header: {:#?}", header);
    header
}


// TODO: Add tests