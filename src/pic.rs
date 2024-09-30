use x86_64::instructions::port::Port;

pub struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
}

pub struct ChainedPic {
    master: Pic,
    slave: Pic,
}

impl ChainedPic {
    pub fn new(offset1: u8, offset2: u8) -> Self {
        ChainedPic {
            master: Pic {
                offset: offset1,
                command: Port::new(0x20),
                data: Port::new(0x21),
            },
            slave: Pic {
                offset: offset2,
                command: Port::new(0xA0),
                data: Port::new(0xA1),
            },
        }
    }

    pub unsafe fn disable(&mut self) {
        self.master.data.write(0xff);
        self.slave.data.write(0xff);
    }

    pub unsafe fn intialize(&mut self) {
        let mut garbage_port = Port::new(0x80);
        let mut io_wait = || garbage_port.write(0u32);

        let a1 = self.master.data.read();
        let a2 = self.slave.data.read();
        self.master.command.write(0x10 | 0x01);
        io_wait();
        self.slave.command.write(0x10 | 0x01);
        io_wait();
        self.master.data.write(self.master.offset);
        io_wait();
        self.slave.data.write(self.master.offset);
        io_wait();
        self.master.data.write(4);
        io_wait();
        self.slave.data.write(2);
        io_wait();

        self.master.data.write(0x01);
        io_wait();
        self.slave.data.write(0x01);
        io_wait();

        self.master.data.write(a1);
        self.slave.data.write(a2);
    }
}
