# [ doc = "General purpose input and output." ]
# [ repr ( C ) ]
pub struct Gpio {
    _reserved0: [u8; 1284usize],
    # [ doc = "0x504 - Write GPIO port." ]
    pub out: Out,
    # [ doc = "0x508 - Set individual bits in GPIO port." ]
    pub outset: Outset,
    # [ doc = "0x50c - Clear individual bits in GPIO port." ]
    pub outclr: Outclr,
    # [ doc = "0x510 - Read GPIO port." ]
    pub in_: In,
    # [ doc = "0x514 - Direction of GPIO pins." ]
    pub dir: Dir,
    # [ doc = "0x518 - DIR set register." ]
    pub dirset: Dirset,
    # [ doc = "0x51c - DIR clear register." ]
    pub dirclr: Dirclr,
    _reserved1: [u8; 480usize],
    # [ doc = "0x700 - Configuration of GPIO pins." ]
    pub pin_cnf0: PinCnf,
    # [ doc = "0x704 - Configuration of GPIO pins." ]
    pub pin_cnf1: PinCnf,
    # [ doc = "0x708 - Configuration of GPIO pins." ]
    pub pin_cnf2: PinCnf,
    # [ doc = "0x70c - Configuration of GPIO pins." ]
    pub pin_cnf3: PinCnf,
    # [ doc = "0x710 - Configuration of GPIO pins." ]
    pub pin_cnf4: PinCnf,
    # [ doc = "0x714 - Configuration of GPIO pins." ]
    pub pin_cnf5: PinCnf,
    # [ doc = "0x718 - Configuration of GPIO pins." ]
    pub pin_cnf6: PinCnf,
    # [ doc = "0x71c - Configuration of GPIO pins." ]
    pub pin_cnf7: PinCnf,
    # [ doc = "0x720 - Configuration of GPIO pins." ]
    pub pin_cnf8: PinCnf,
    # [ doc = "0x724 - Configuration of GPIO pins." ]
    pub pin_cnf9: PinCnf,
    # [ doc = "0x728 - Configuration of GPIO pins." ]
    pub pin_cnf10: PinCnf,
    # [ doc = "0x72c - Configuration of GPIO pins." ]
    pub pin_cnf11: PinCnf,
    # [ doc = "0x730 - Configuration of GPIO pins." ]
    pub pin_cnf12: PinCnf,
    # [ doc = "0x734 - Configuration of GPIO pins." ]
    pub pin_cnf13: PinCnf,
    # [ doc = "0x738 - Configuration of GPIO pins." ]
    pub pin_cnf14: PinCnf,
    # [ doc = "0x73c - Configuration of GPIO pins." ]
    pub pin_cnf15: PinCnf,
    # [ doc = "0x740 - Configuration of GPIO pins." ]
    pub pin_cnf16: PinCnf,
    # [ doc = "0x744 - Configuration of GPIO pins." ]
    pub pin_cnf17: PinCnf,
    # [ doc = "0x748 - Configuration of GPIO pins." ]
    pub pin_cnf18: PinCnf,
    # [ doc = "0x74c - Configuration of GPIO pins." ]
    pub pin_cnf19: PinCnf,
    # [ doc = "0x750 - Configuration of GPIO pins." ]
    pub pin_cnf20: PinCnf,
    # [ doc = "0x754 - Configuration of GPIO pins." ]
    pub pin_cnf21: PinCnf,
    # [ doc = "0x758 - Configuration of GPIO pins." ]
    pub pin_cnf22: PinCnf,
    # [ doc = "0x75c - Configuration of GPIO pins." ]
    pub pin_cnf23: PinCnf,
    # [ doc = "0x760 - Configuration of GPIO pins." ]
    pub pin_cnf24: PinCnf,
    # [ doc = "0x764 - Configuration of GPIO pins." ]
    pub pin_cnf25: PinCnf,
    # [ doc = "0x768 - Configuration of GPIO pins." ]
    pub pin_cnf26: PinCnf,
    # [ doc = "0x76c - Configuration of GPIO pins." ]
    pub pin_cnf27: PinCnf,
    # [ doc = "0x770 - Configuration of GPIO pins." ]
    pub pin_cnf28: PinCnf,
    # [ doc = "0x774 - Configuration of GPIO pins." ]
    pub pin_cnf29: PinCnf,
    # [ doc = "0x778 - Configuration of GPIO pins." ]
    pub pin_cnf30: PinCnf,
    # [ doc = "0x77c - Configuration of GPIO pins." ]
    pub pin_cnf31: PinCnf,
}

# [ repr ( C ) ]
pub struct Out {
    register: ::volatile_register::RW<u32>,
}

impl Out {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&OutR, &'w mut OutW) -> &'w mut OutW
    {
        let bits = self.register.read();
        let r = OutR { bits: bits };
        let mut w = OutW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OutR {
        OutR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OutW) -> &mut OutW
    {
        let mut w = OutW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OutR {
    bits: u32,
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin0 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin0::Low,
            1u32 => OutRPin0::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin1 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin1::Low,
            1u32 => OutRPin1::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin2 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin2::Low,
            1u32 => OutRPin2::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin3 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin3::Low,
            1u32 => OutRPin3::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin4 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin4 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin4::Low,
            1u32 => OutRPin4::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin5 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin5 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin5::Low,
            1u32 => OutRPin5::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin6 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin6 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin6::Low,
            1u32 => OutRPin6::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin7 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin7 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin7::Low,
            1u32 => OutRPin7::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin8 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin8 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin8::Low,
            1u32 => OutRPin8::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin9 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin9 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin9::Low,
            1u32 => OutRPin9::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin10 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin10 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin10::Low,
            1u32 => OutRPin10::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin11 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin11 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin11::Low,
            1u32 => OutRPin11::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin12 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin12 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin12::Low,
            1u32 => OutRPin12::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin13 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin13 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin13::Low,
            1u32 => OutRPin13::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin14 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin14 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin14::Low,
            1u32 => OutRPin14::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin15 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin15 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin15::Low,
            1u32 => OutRPin15::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin16 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin16 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin16::Low,
            1u32 => OutRPin16::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin17 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin17 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin17::Low,
            1u32 => OutRPin17::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin18 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin18 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin18::Low,
            1u32 => OutRPin18::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin19 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin19 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin19::Low,
            1u32 => OutRPin19::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin20 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin20 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin20::Low,
            1u32 => OutRPin20::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin21 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin21 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin21::Low,
            1u32 => OutRPin21::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin22 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin22 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin22::Low,
            1u32 => OutRPin22::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin23 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin23 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin23::Low,
            1u32 => OutRPin23::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin24 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin24 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin24::Low,
            1u32 => OutRPin24::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin25 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin25 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin25::Low,
            1u32 => OutRPin25::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin26 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin26 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin26::Low,
            1u32 => OutRPin26::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin27 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin27 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin27::Low,
            1u32 => OutRPin27::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin28 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin28 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin28::Low,
            1u32 => OutRPin28::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin29 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin29 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin29::Low,
            1u32 => OutRPin29::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin30 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin30 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin30::Low,
            1u32 => OutRPin30::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutRPin31 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutRPin31 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutRPin31::Low,
            1u32 => OutRPin31::High,
            _ => unreachable!(),
        }
    }
}

impl OutR {
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&self) -> OutRPin0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        OutRPin0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&self) -> OutRPin1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        OutRPin1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&self) -> OutRPin2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        OutRPin2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&self) -> OutRPin3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        OutRPin3::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&self) -> OutRPin4 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        OutRPin4::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&self) -> OutRPin5 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        OutRPin5::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&self) -> OutRPin6 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        OutRPin6::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&self) -> OutRPin7 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        OutRPin7::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&self) -> OutRPin8 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        OutRPin8::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&self) -> OutRPin9 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        OutRPin9::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&self) -> OutRPin10 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        OutRPin10::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&self) -> OutRPin11 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        OutRPin11::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&self) -> OutRPin12 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        OutRPin12::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&self) -> OutRPin13 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        OutRPin13::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&self) -> OutRPin14 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        OutRPin14::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&self) -> OutRPin15 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        OutRPin15::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&self) -> OutRPin16 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        OutRPin16::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&self) -> OutRPin17 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        OutRPin17::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&self) -> OutRPin18 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        OutRPin18::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&self) -> OutRPin19 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        OutRPin19::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&self) -> OutRPin20 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        OutRPin20::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&self) -> OutRPin21 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        OutRPin21::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&self) -> OutRPin22 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        OutRPin22::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&self) -> OutRPin23 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        OutRPin23::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&self) -> OutRPin24 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        OutRPin24::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&self) -> OutRPin25 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        OutRPin25::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&self) -> OutRPin26 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        OutRPin26::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&self) -> OutRPin27 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        OutRPin27::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&self) -> OutRPin28 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        OutRPin28::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&self) -> OutRPin29 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        OutRPin29::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&self) -> OutRPin30 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        OutRPin30::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&self) -> OutRPin31 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        OutRPin31::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OutW {
    bits: u32,
}

pub struct OutWPin0<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin0<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin1<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin1<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin2<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin2<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin3<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin3<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin4<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin4<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin5<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin5<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin6<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin6<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin7<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin7<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin8<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin8<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin9<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin9<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin10<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin10<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin11<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin11<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin12<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin12<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin13<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin13<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin14<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin14<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin15<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin15<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin16<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin16<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin17<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin17<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin18<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin18<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin19<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin19<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin20<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin20<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin21<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin21<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin22<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin22<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin23<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin23<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin24<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin24<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin25<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin25<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin26<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin26<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin27<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin27<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin28<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin28<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin29<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin29<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin30<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin30<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutWPin31<'a> {
    register: &'a mut OutW,
}

impl<'a> OutWPin31<'a> {
    pub fn low(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut OutW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl OutW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OutW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&mut self) -> OutWPin0 {
        OutWPin0 { register: self }
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&mut self) -> OutWPin1 {
        OutWPin1 { register: self }
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&mut self) -> OutWPin2 {
        OutWPin2 { register: self }
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&mut self) -> OutWPin3 {
        OutWPin3 { register: self }
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&mut self) -> OutWPin4 {
        OutWPin4 { register: self }
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&mut self) -> OutWPin5 {
        OutWPin5 { register: self }
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&mut self) -> OutWPin6 {
        OutWPin6 { register: self }
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&mut self) -> OutWPin7 {
        OutWPin7 { register: self }
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&mut self) -> OutWPin8 {
        OutWPin8 { register: self }
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&mut self) -> OutWPin9 {
        OutWPin9 { register: self }
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&mut self) -> OutWPin10 {
        OutWPin10 { register: self }
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&mut self) -> OutWPin11 {
        OutWPin11 { register: self }
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&mut self) -> OutWPin12 {
        OutWPin12 { register: self }
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&mut self) -> OutWPin13 {
        OutWPin13 { register: self }
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&mut self) -> OutWPin14 {
        OutWPin14 { register: self }
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&mut self) -> OutWPin15 {
        OutWPin15 { register: self }
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&mut self) -> OutWPin16 {
        OutWPin16 { register: self }
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&mut self) -> OutWPin17 {
        OutWPin17 { register: self }
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&mut self) -> OutWPin18 {
        OutWPin18 { register: self }
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&mut self) -> OutWPin19 {
        OutWPin19 { register: self }
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&mut self) -> OutWPin20 {
        OutWPin20 { register: self }
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&mut self) -> OutWPin21 {
        OutWPin21 { register: self }
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&mut self) -> OutWPin22 {
        OutWPin22 { register: self }
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&mut self) -> OutWPin23 {
        OutWPin23 { register: self }
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&mut self) -> OutWPin24 {
        OutWPin24 { register: self }
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&mut self) -> OutWPin25 {
        OutWPin25 { register: self }
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&mut self) -> OutWPin26 {
        OutWPin26 { register: self }
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&mut self) -> OutWPin27 {
        OutWPin27 { register: self }
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&mut self) -> OutWPin28 {
        OutWPin28 { register: self }
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&mut self) -> OutWPin29 {
        OutWPin29 { register: self }
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&mut self) -> OutWPin30 {
        OutWPin30 { register: self }
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&mut self) -> OutWPin31 {
        OutWPin31 { register: self }
    }
}

# [ repr ( C ) ]
pub struct Outset {
    register: ::volatile_register::RW<u32>,
}

impl Outset {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&OutsetR, &'w mut OutsetW) -> &'w mut OutsetW
    {
        let bits = self.register.read();
        let r = OutsetR { bits: bits };
        let mut w = OutsetW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OutsetR {
        OutsetR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OutsetW) -> &mut OutsetW
    {
        let mut w = OutsetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OutsetR {
    bits: u32,
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin0 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin0::Low,
            1u32 => OutsetRPin0::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin1 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin1::Low,
            1u32 => OutsetRPin1::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin2 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin2::Low,
            1u32 => OutsetRPin2::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin3 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin3::Low,
            1u32 => OutsetRPin3::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin4 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin4 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin4::Low,
            1u32 => OutsetRPin4::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin5 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin5 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin5::Low,
            1u32 => OutsetRPin5::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin6 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin6 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin6::Low,
            1u32 => OutsetRPin6::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin7 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin7 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin7::Low,
            1u32 => OutsetRPin7::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin8 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin8 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin8::Low,
            1u32 => OutsetRPin8::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin9 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin9 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin9::Low,
            1u32 => OutsetRPin9::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin10 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin10 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin10::Low,
            1u32 => OutsetRPin10::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin11 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin11 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin11::Low,
            1u32 => OutsetRPin11::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin12 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin12 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin12::Low,
            1u32 => OutsetRPin12::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin13 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin13 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin13::Low,
            1u32 => OutsetRPin13::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin14 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin14 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin14::Low,
            1u32 => OutsetRPin14::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin15 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin15 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin15::Low,
            1u32 => OutsetRPin15::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin16 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin16 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin16::Low,
            1u32 => OutsetRPin16::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin17 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin17 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin17::Low,
            1u32 => OutsetRPin17::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin18 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin18 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin18::Low,
            1u32 => OutsetRPin18::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin19 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin19 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin19::Low,
            1u32 => OutsetRPin19::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin20 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin20 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin20::Low,
            1u32 => OutsetRPin20::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin21 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin21 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin21::Low,
            1u32 => OutsetRPin21::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin22 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin22 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin22::Low,
            1u32 => OutsetRPin22::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin23 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin23 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin23::Low,
            1u32 => OutsetRPin23::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin24 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin24 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin24::Low,
            1u32 => OutsetRPin24::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin25 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin25 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin25::Low,
            1u32 => OutsetRPin25::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin26 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin26 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin26::Low,
            1u32 => OutsetRPin26::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin27 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin27 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin27::Low,
            1u32 => OutsetRPin27::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin28 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin28 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin28::Low,
            1u32 => OutsetRPin28::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin29 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin29 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin29::Low,
            1u32 => OutsetRPin29::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin30 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin30 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin30::Low,
            1u32 => OutsetRPin30::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetRPin31 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutsetRPin31 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutsetRPin31::Low,
            1u32 => OutsetRPin31::High,
            _ => unreachable!(),
        }
    }
}

impl OutsetR {
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&self) -> OutsetRPin0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        OutsetRPin0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&self) -> OutsetRPin1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        OutsetRPin1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&self) -> OutsetRPin2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        OutsetRPin2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&self) -> OutsetRPin3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        OutsetRPin3::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&self) -> OutsetRPin4 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        OutsetRPin4::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&self) -> OutsetRPin5 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        OutsetRPin5::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&self) -> OutsetRPin6 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        OutsetRPin6::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&self) -> OutsetRPin7 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        OutsetRPin7::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&self) -> OutsetRPin8 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        OutsetRPin8::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&self) -> OutsetRPin9 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        OutsetRPin9::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&self) -> OutsetRPin10 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        OutsetRPin10::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&self) -> OutsetRPin11 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        OutsetRPin11::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&self) -> OutsetRPin12 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        OutsetRPin12::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&self) -> OutsetRPin13 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        OutsetRPin13::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&self) -> OutsetRPin14 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        OutsetRPin14::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&self) -> OutsetRPin15 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        OutsetRPin15::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&self) -> OutsetRPin16 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        OutsetRPin16::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&self) -> OutsetRPin17 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        OutsetRPin17::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&self) -> OutsetRPin18 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        OutsetRPin18::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&self) -> OutsetRPin19 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        OutsetRPin19::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&self) -> OutsetRPin20 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        OutsetRPin20::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&self) -> OutsetRPin21 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        OutsetRPin21::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&self) -> OutsetRPin22 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        OutsetRPin22::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&self) -> OutsetRPin23 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        OutsetRPin23::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&self) -> OutsetRPin24 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        OutsetRPin24::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&self) -> OutsetRPin25 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        OutsetRPin25::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&self) -> OutsetRPin26 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        OutsetRPin26::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&self) -> OutsetRPin27 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        OutsetRPin27::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&self) -> OutsetRPin28 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        OutsetRPin28::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&self) -> OutsetRPin29 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        OutsetRPin29::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&self) -> OutsetRPin30 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        OutsetRPin30::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&self) -> OutsetRPin31 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        OutsetRPin31::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OutsetW {
    bits: u32,
}

pub struct OutsetWPin0<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin0<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin1<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin1<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin2<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin2<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin3<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin3<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin4<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin4<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin5<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin5<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin6<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin6<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin7<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin7<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin8<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin8<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin9<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin9<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin10<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin10<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin11<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin11<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin12<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin12<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin13<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin13<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin14<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin14<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin15<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin15<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin16<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin16<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin17<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin17<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin18<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin18<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin19<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin19<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin20<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin20<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin21<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin21<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin22<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin22<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin23<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin23<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin24<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin24<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin25<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin25<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin26<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin26<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin27<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin27<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin28<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin28<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin29<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin29<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin30<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin30<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutsetWPin31<'a> {
    register: &'a mut OutsetW,
}

impl<'a> OutsetWPin31<'a> {
    pub fn set(self) -> &'a mut OutsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl OutsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OutsetW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&mut self) -> OutsetWPin0 {
        OutsetWPin0 { register: self }
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&mut self) -> OutsetWPin1 {
        OutsetWPin1 { register: self }
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&mut self) -> OutsetWPin2 {
        OutsetWPin2 { register: self }
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&mut self) -> OutsetWPin3 {
        OutsetWPin3 { register: self }
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&mut self) -> OutsetWPin4 {
        OutsetWPin4 { register: self }
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&mut self) -> OutsetWPin5 {
        OutsetWPin5 { register: self }
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&mut self) -> OutsetWPin6 {
        OutsetWPin6 { register: self }
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&mut self) -> OutsetWPin7 {
        OutsetWPin7 { register: self }
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&mut self) -> OutsetWPin8 {
        OutsetWPin8 { register: self }
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&mut self) -> OutsetWPin9 {
        OutsetWPin9 { register: self }
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&mut self) -> OutsetWPin10 {
        OutsetWPin10 { register: self }
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&mut self) -> OutsetWPin11 {
        OutsetWPin11 { register: self }
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&mut self) -> OutsetWPin12 {
        OutsetWPin12 { register: self }
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&mut self) -> OutsetWPin13 {
        OutsetWPin13 { register: self }
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&mut self) -> OutsetWPin14 {
        OutsetWPin14 { register: self }
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&mut self) -> OutsetWPin15 {
        OutsetWPin15 { register: self }
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&mut self) -> OutsetWPin16 {
        OutsetWPin16 { register: self }
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&mut self) -> OutsetWPin17 {
        OutsetWPin17 { register: self }
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&mut self) -> OutsetWPin18 {
        OutsetWPin18 { register: self }
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&mut self) -> OutsetWPin19 {
        OutsetWPin19 { register: self }
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&mut self) -> OutsetWPin20 {
        OutsetWPin20 { register: self }
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&mut self) -> OutsetWPin21 {
        OutsetWPin21 { register: self }
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&mut self) -> OutsetWPin22 {
        OutsetWPin22 { register: self }
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&mut self) -> OutsetWPin23 {
        OutsetWPin23 { register: self }
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&mut self) -> OutsetWPin24 {
        OutsetWPin24 { register: self }
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&mut self) -> OutsetWPin25 {
        OutsetWPin25 { register: self }
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&mut self) -> OutsetWPin26 {
        OutsetWPin26 { register: self }
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&mut self) -> OutsetWPin27 {
        OutsetWPin27 { register: self }
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&mut self) -> OutsetWPin28 {
        OutsetWPin28 { register: self }
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&mut self) -> OutsetWPin29 {
        OutsetWPin29 { register: self }
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&mut self) -> OutsetWPin30 {
        OutsetWPin30 { register: self }
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&mut self) -> OutsetWPin31 {
        OutsetWPin31 { register: self }
    }
}

# [ repr ( C ) ]
pub struct Outclr {
    register: ::volatile_register::RW<u32>,
}

impl Outclr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&OutclrR, &'w mut OutclrW) -> &'w mut OutclrW
    {
        let bits = self.register.read();
        let r = OutclrR { bits: bits };
        let mut w = OutclrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OutclrR {
        OutclrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OutclrW) -> &mut OutclrW
    {
        let mut w = OutclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OutclrR {
    bits: u32,
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin0 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin0::Low,
            1u32 => OutclrRPin0::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin1 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin1::Low,
            1u32 => OutclrRPin1::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin2 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin2::Low,
            1u32 => OutclrRPin2::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin3 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin3::Low,
            1u32 => OutclrRPin3::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin4 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin4 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin4::Low,
            1u32 => OutclrRPin4::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin5 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin5 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin5::Low,
            1u32 => OutclrRPin5::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin6 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin6 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin6::Low,
            1u32 => OutclrRPin6::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin7 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin7 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin7::Low,
            1u32 => OutclrRPin7::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin8 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin8 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin8::Low,
            1u32 => OutclrRPin8::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin9 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin9 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin9::Low,
            1u32 => OutclrRPin9::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin10 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin10 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin10::Low,
            1u32 => OutclrRPin10::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin11 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin11 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin11::Low,
            1u32 => OutclrRPin11::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin12 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin12 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin12::Low,
            1u32 => OutclrRPin12::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin13 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin13 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin13::Low,
            1u32 => OutclrRPin13::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin14 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin14 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin14::Low,
            1u32 => OutclrRPin14::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin15 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin15 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin15::Low,
            1u32 => OutclrRPin15::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin16 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin16 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin16::Low,
            1u32 => OutclrRPin16::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin17 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin17 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin17::Low,
            1u32 => OutclrRPin17::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin18 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin18 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin18::Low,
            1u32 => OutclrRPin18::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin19 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin19 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin19::Low,
            1u32 => OutclrRPin19::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin20 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin20 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin20::Low,
            1u32 => OutclrRPin20::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin21 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin21 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin21::Low,
            1u32 => OutclrRPin21::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin22 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin22 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin22::Low,
            1u32 => OutclrRPin22::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin23 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin23 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin23::Low,
            1u32 => OutclrRPin23::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin24 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin24 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin24::Low,
            1u32 => OutclrRPin24::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin25 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin25 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin25::Low,
            1u32 => OutclrRPin25::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin26 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin26 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin26::Low,
            1u32 => OutclrRPin26::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin27 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin27 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin27::Low,
            1u32 => OutclrRPin27::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin28 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin28 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin28::Low,
            1u32 => OutclrRPin28::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin29 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin29 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin29::Low,
            1u32 => OutclrRPin29::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin30 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin30 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin30::Low,
            1u32 => OutclrRPin30::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrRPin31 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutclrRPin31 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => OutclrRPin31::Low,
            1u32 => OutclrRPin31::High,
            _ => unreachable!(),
        }
    }
}

impl OutclrR {
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&self) -> OutclrRPin0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        OutclrRPin0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&self) -> OutclrRPin1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        OutclrRPin1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&self) -> OutclrRPin2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        OutclrRPin2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&self) -> OutclrRPin3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        OutclrRPin3::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&self) -> OutclrRPin4 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        OutclrRPin4::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&self) -> OutclrRPin5 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        OutclrRPin5::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&self) -> OutclrRPin6 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        OutclrRPin6::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&self) -> OutclrRPin7 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        OutclrRPin7::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&self) -> OutclrRPin8 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        OutclrRPin8::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&self) -> OutclrRPin9 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        OutclrRPin9::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&self) -> OutclrRPin10 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        OutclrRPin10::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&self) -> OutclrRPin11 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        OutclrRPin11::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&self) -> OutclrRPin12 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        OutclrRPin12::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&self) -> OutclrRPin13 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        OutclrRPin13::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&self) -> OutclrRPin14 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        OutclrRPin14::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&self) -> OutclrRPin15 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        OutclrRPin15::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&self) -> OutclrRPin16 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        OutclrRPin16::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&self) -> OutclrRPin17 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        OutclrRPin17::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&self) -> OutclrRPin18 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        OutclrRPin18::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&self) -> OutclrRPin19 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        OutclrRPin19::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&self) -> OutclrRPin20 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        OutclrRPin20::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&self) -> OutclrRPin21 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        OutclrRPin21::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&self) -> OutclrRPin22 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        OutclrRPin22::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&self) -> OutclrRPin23 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        OutclrRPin23::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&self) -> OutclrRPin24 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        OutclrRPin24::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&self) -> OutclrRPin25 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        OutclrRPin25::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&self) -> OutclrRPin26 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        OutclrRPin26::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&self) -> OutclrRPin27 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        OutclrRPin27::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&self) -> OutclrRPin28 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        OutclrRPin28::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&self) -> OutclrRPin29 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        OutclrRPin29::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&self) -> OutclrRPin30 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        OutclrRPin30::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&self) -> OutclrRPin31 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        OutclrRPin31::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OutclrW {
    bits: u32,
}

pub struct OutclrWPin0<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin0<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin1<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin1<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin2<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin2<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin3<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin3<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin4<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin4<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin5<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin5<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin6<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin6<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin7<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin7<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin8<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin8<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin9<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin9<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin10<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin10<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin11<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin11<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin12<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin12<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin13<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin13<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin14<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin14<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin15<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin15<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin16<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin16<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin17<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin17<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin18<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin18<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin19<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin19<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin20<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin20<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin21<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin21<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin22<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin22<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin23<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin23<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin24<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin24<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin25<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin25<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin26<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin26<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin27<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin27<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin28<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin28<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin29<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin29<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin30<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin30<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct OutclrWPin31<'a> {
    register: &'a mut OutclrW,
}

impl<'a> OutclrWPin31<'a> {
    pub fn clear(self) -> &'a mut OutclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl OutclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OutclrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&mut self) -> OutclrWPin0 {
        OutclrWPin0 { register: self }
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&mut self) -> OutclrWPin1 {
        OutclrWPin1 { register: self }
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&mut self) -> OutclrWPin2 {
        OutclrWPin2 { register: self }
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&mut self) -> OutclrWPin3 {
        OutclrWPin3 { register: self }
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&mut self) -> OutclrWPin4 {
        OutclrWPin4 { register: self }
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&mut self) -> OutclrWPin5 {
        OutclrWPin5 { register: self }
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&mut self) -> OutclrWPin6 {
        OutclrWPin6 { register: self }
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&mut self) -> OutclrWPin7 {
        OutclrWPin7 { register: self }
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&mut self) -> OutclrWPin8 {
        OutclrWPin8 { register: self }
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&mut self) -> OutclrWPin9 {
        OutclrWPin9 { register: self }
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&mut self) -> OutclrWPin10 {
        OutclrWPin10 { register: self }
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&mut self) -> OutclrWPin11 {
        OutclrWPin11 { register: self }
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&mut self) -> OutclrWPin12 {
        OutclrWPin12 { register: self }
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&mut self) -> OutclrWPin13 {
        OutclrWPin13 { register: self }
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&mut self) -> OutclrWPin14 {
        OutclrWPin14 { register: self }
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&mut self) -> OutclrWPin15 {
        OutclrWPin15 { register: self }
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&mut self) -> OutclrWPin16 {
        OutclrWPin16 { register: self }
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&mut self) -> OutclrWPin17 {
        OutclrWPin17 { register: self }
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&mut self) -> OutclrWPin18 {
        OutclrWPin18 { register: self }
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&mut self) -> OutclrWPin19 {
        OutclrWPin19 { register: self }
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&mut self) -> OutclrWPin20 {
        OutclrWPin20 { register: self }
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&mut self) -> OutclrWPin21 {
        OutclrWPin21 { register: self }
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&mut self) -> OutclrWPin22 {
        OutclrWPin22 { register: self }
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&mut self) -> OutclrWPin23 {
        OutclrWPin23 { register: self }
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&mut self) -> OutclrWPin24 {
        OutclrWPin24 { register: self }
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&mut self) -> OutclrWPin25 {
        OutclrWPin25 { register: self }
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&mut self) -> OutclrWPin26 {
        OutclrWPin26 { register: self }
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&mut self) -> OutclrWPin27 {
        OutclrWPin27 { register: self }
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&mut self) -> OutclrWPin28 {
        OutclrWPin28 { register: self }
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&mut self) -> OutclrWPin29 {
        OutclrWPin29 { register: self }
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&mut self) -> OutclrWPin30 {
        OutclrWPin30 { register: self }
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&mut self) -> OutclrWPin31 {
        OutclrWPin31 { register: self }
    }
}

# [ repr ( C ) ]
pub struct In {
    register: ::volatile_register::RO<u32>,
}

impl In {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> InR {
        InR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct InR {
    bits: u32,
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin0 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin0::Low,
            1u32 => InRPin0::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin1 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin1::Low,
            1u32 => InRPin1::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin2 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin2::Low,
            1u32 => InRPin2::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin3 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin3::Low,
            1u32 => InRPin3::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin4 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin4 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin4::Low,
            1u32 => InRPin4::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin5 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin5 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin5::Low,
            1u32 => InRPin5::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin6 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin6 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin6::Low,
            1u32 => InRPin6::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin7 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin7 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin7::Low,
            1u32 => InRPin7::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin8 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin8 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin8::Low,
            1u32 => InRPin8::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin9 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin9 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin9::Low,
            1u32 => InRPin9::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin10 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin10 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin10::Low,
            1u32 => InRPin10::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin11 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin11 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin11::Low,
            1u32 => InRPin11::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin12 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin12 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin12::Low,
            1u32 => InRPin12::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin13 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin13 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin13::Low,
            1u32 => InRPin13::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin14 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin14 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin14::Low,
            1u32 => InRPin14::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin15 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin15 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin15::Low,
            1u32 => InRPin15::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin16 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin16 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin16::Low,
            1u32 => InRPin16::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin17 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin17 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin17::Low,
            1u32 => InRPin17::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin18 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin18 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin18::Low,
            1u32 => InRPin18::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin19 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin19 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin19::Low,
            1u32 => InRPin19::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin20 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin20 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin20::Low,
            1u32 => InRPin20::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin21 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin21 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin21::Low,
            1u32 => InRPin21::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin22 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin22 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin22::Low,
            1u32 => InRPin22::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin23 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin23 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin23::Low,
            1u32 => InRPin23::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin24 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin24 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin24::Low,
            1u32 => InRPin24::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin25 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin25 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin25::Low,
            1u32 => InRPin25::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin26 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin26 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin26::Low,
            1u32 => InRPin26::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin27 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin27 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin27::Low,
            1u32 => InRPin27::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin28 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin28 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin28::Low,
            1u32 => InRPin28::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin29 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin29 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin29::Low,
            1u32 => InRPin29::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin30 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin30 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin30::Low,
            1u32 => InRPin30::High,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum InRPin31 {
    # [ doc = "Pin input is low." ]
    Low,
    # [ doc = "Pin input is high." ]
    High,
}
impl InRPin31 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => InRPin31::Low,
            1u32 => InRPin31::High,
            _ => unreachable!(),
        }
    }
}

impl InR {
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&self) -> InRPin0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        InRPin0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&self) -> InRPin1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        InRPin1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&self) -> InRPin2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        InRPin2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&self) -> InRPin3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        InRPin3::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&self) -> InRPin4 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        InRPin4::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&self) -> InRPin5 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        InRPin5::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&self) -> InRPin6 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        InRPin6::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&self) -> InRPin7 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        InRPin7::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&self) -> InRPin8 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        InRPin8::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&self) -> InRPin9 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        InRPin9::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&self) -> InRPin10 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        InRPin10::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&self) -> InRPin11 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        InRPin11::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&self) -> InRPin12 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        InRPin12::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&self) -> InRPin13 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        InRPin13::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&self) -> InRPin14 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        InRPin14::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&self) -> InRPin15 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        InRPin15::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&self) -> InRPin16 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        InRPin16::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&self) -> InRPin17 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        InRPin17::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&self) -> InRPin18 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        InRPin18::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&self) -> InRPin19 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        InRPin19::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&self) -> InRPin20 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        InRPin20::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&self) -> InRPin21 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        InRPin21::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&self) -> InRPin22 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        InRPin22::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&self) -> InRPin23 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        InRPin23::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&self) -> InRPin24 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        InRPin24::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&self) -> InRPin25 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        InRPin25::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&self) -> InRPin26 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        InRPin26::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&self) -> InRPin27 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        InRPin27::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&self) -> InRPin28 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        InRPin28::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&self) -> InRPin29 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        InRPin29::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&self) -> InRPin30 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        InRPin30::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&self) -> InRPin31 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        InRPin31::from((self.bits >> OFFSET) & MASK)
    }
}

# [ repr ( C ) ]
pub struct Dir {
    register: ::volatile_register::RW<u32>,
}

impl Dir {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DirR, &'w mut DirW) -> &'w mut DirW
    {
        let bits = self.register.read();
        let r = DirR { bits: bits };
        let mut w = DirW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DirR {
        DirR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DirW) -> &mut DirW
    {
        let mut w = DirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DirR {
    bits: u32,
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin0 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin0::Input,
            1u32 => DirRPin0::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin1 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin1::Input,
            1u32 => DirRPin1::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin2 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin2::Input,
            1u32 => DirRPin2::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin3 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin3::Input,
            1u32 => DirRPin3::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin4 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin4 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin4::Input,
            1u32 => DirRPin4::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin5 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin5 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin5::Input,
            1u32 => DirRPin5::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin6 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin6 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin6::Input,
            1u32 => DirRPin6::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin7 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin7 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin7::Input,
            1u32 => DirRPin7::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin8 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin8 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin8::Input,
            1u32 => DirRPin8::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin9 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin9 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin9::Input,
            1u32 => DirRPin9::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin10 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin10 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin10::Input,
            1u32 => DirRPin10::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin11 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin11 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin11::Input,
            1u32 => DirRPin11::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin12 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin12 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin12::Input,
            1u32 => DirRPin12::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin13 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin13 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin13::Input,
            1u32 => DirRPin13::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin14 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin14 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin14::Input,
            1u32 => DirRPin14::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin15 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin15 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin15::Input,
            1u32 => DirRPin15::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin16 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin16 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin16::Input,
            1u32 => DirRPin16::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin17 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin17 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin17::Input,
            1u32 => DirRPin17::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin18 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin18 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin18::Input,
            1u32 => DirRPin18::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin19 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin19 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin19::Input,
            1u32 => DirRPin19::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin20 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin20 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin20::Input,
            1u32 => DirRPin20::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin21 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin21 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin21::Input,
            1u32 => DirRPin21::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin22 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin22 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin22::Input,
            1u32 => DirRPin22::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin23 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin23 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin23::Input,
            1u32 => DirRPin23::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin24 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin24 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin24::Input,
            1u32 => DirRPin24::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin25 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin25 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin25::Input,
            1u32 => DirRPin25::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin26 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin26 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin26::Input,
            1u32 => DirRPin26::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin27 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin27 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin27::Input,
            1u32 => DirRPin27::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin28 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin28 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin28::Input,
            1u32 => DirRPin28::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin29 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin29 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin29::Input,
            1u32 => DirRPin29::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin30 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin30 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin30::Input,
            1u32 => DirRPin30::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirRPin31 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirRPin31 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirRPin31::Input,
            1u32 => DirRPin31::Output,
            _ => unreachable!(),
        }
    }
}

impl DirR {
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&self) -> DirRPin0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        DirRPin0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&self) -> DirRPin1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        DirRPin1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&self) -> DirRPin2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        DirRPin2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&self) -> DirRPin3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        DirRPin3::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&self) -> DirRPin4 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        DirRPin4::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&self) -> DirRPin5 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        DirRPin5::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&self) -> DirRPin6 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        DirRPin6::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&self) -> DirRPin7 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        DirRPin7::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&self) -> DirRPin8 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        DirRPin8::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&self) -> DirRPin9 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        DirRPin9::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&self) -> DirRPin10 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        DirRPin10::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&self) -> DirRPin11 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        DirRPin11::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&self) -> DirRPin12 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        DirRPin12::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&self) -> DirRPin13 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        DirRPin13::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&self) -> DirRPin14 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        DirRPin14::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&self) -> DirRPin15 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        DirRPin15::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&self) -> DirRPin16 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        DirRPin16::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&self) -> DirRPin17 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        DirRPin17::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&self) -> DirRPin18 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        DirRPin18::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&self) -> DirRPin19 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        DirRPin19::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&self) -> DirRPin20 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        DirRPin20::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&self) -> DirRPin21 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        DirRPin21::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&self) -> DirRPin22 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        DirRPin22::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&self) -> DirRPin23 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        DirRPin23::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&self) -> DirRPin24 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        DirRPin24::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&self) -> DirRPin25 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        DirRPin25::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&self) -> DirRPin26 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        DirRPin26::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&self) -> DirRPin27 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        DirRPin27::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&self) -> DirRPin28 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        DirRPin28::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&self) -> DirRPin29 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        DirRPin29::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&self) -> DirRPin30 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        DirRPin30::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&self) -> DirRPin31 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        DirRPin31::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DirW {
    bits: u32,
}

pub struct DirWPin0<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin0<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin1<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin1<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin2<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin2<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin3<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin3<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin4<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin4<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin5<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin5<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin6<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin6<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin7<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin7<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin8<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin8<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin9<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin9<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin10<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin10<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin11<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin11<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin12<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin12<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin13<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin13<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin14<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin14<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin15<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin15<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin16<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin16<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin17<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin17<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin18<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin18<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin19<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin19<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin20<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin20<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin21<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin21<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin22<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin22<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin23<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin23<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin24<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin24<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin25<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin25<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin26<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin26<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin27<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin27<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin28<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin28<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin29<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin29<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin30<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin30<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirWPin31<'a> {
    register: &'a mut DirW,
}

impl<'a> DirWPin31<'a> {
    pub fn input(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut DirW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl DirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DirW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&mut self) -> DirWPin0 {
        DirWPin0 { register: self }
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&mut self) -> DirWPin1 {
        DirWPin1 { register: self }
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&mut self) -> DirWPin2 {
        DirWPin2 { register: self }
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&mut self) -> DirWPin3 {
        DirWPin3 { register: self }
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&mut self) -> DirWPin4 {
        DirWPin4 { register: self }
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&mut self) -> DirWPin5 {
        DirWPin5 { register: self }
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&mut self) -> DirWPin6 {
        DirWPin6 { register: self }
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&mut self) -> DirWPin7 {
        DirWPin7 { register: self }
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&mut self) -> DirWPin8 {
        DirWPin8 { register: self }
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&mut self) -> DirWPin9 {
        DirWPin9 { register: self }
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&mut self) -> DirWPin10 {
        DirWPin10 { register: self }
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&mut self) -> DirWPin11 {
        DirWPin11 { register: self }
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&mut self) -> DirWPin12 {
        DirWPin12 { register: self }
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&mut self) -> DirWPin13 {
        DirWPin13 { register: self }
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&mut self) -> DirWPin14 {
        DirWPin14 { register: self }
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&mut self) -> DirWPin15 {
        DirWPin15 { register: self }
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&mut self) -> DirWPin16 {
        DirWPin16 { register: self }
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&mut self) -> DirWPin17 {
        DirWPin17 { register: self }
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&mut self) -> DirWPin18 {
        DirWPin18 { register: self }
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&mut self) -> DirWPin19 {
        DirWPin19 { register: self }
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&mut self) -> DirWPin20 {
        DirWPin20 { register: self }
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&mut self) -> DirWPin21 {
        DirWPin21 { register: self }
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&mut self) -> DirWPin22 {
        DirWPin22 { register: self }
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&mut self) -> DirWPin23 {
        DirWPin23 { register: self }
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&mut self) -> DirWPin24 {
        DirWPin24 { register: self }
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&mut self) -> DirWPin25 {
        DirWPin25 { register: self }
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&mut self) -> DirWPin26 {
        DirWPin26 { register: self }
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&mut self) -> DirWPin27 {
        DirWPin27 { register: self }
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&mut self) -> DirWPin28 {
        DirWPin28 { register: self }
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&mut self) -> DirWPin29 {
        DirWPin29 { register: self }
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&mut self) -> DirWPin30 {
        DirWPin30 { register: self }
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&mut self) -> DirWPin31 {
        DirWPin31 { register: self }
    }
}

# [ repr ( C ) ]
pub struct Dirset {
    register: ::volatile_register::RW<u32>,
}

impl Dirset {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DirsetR, &'w mut DirsetW) -> &'w mut DirsetW
    {
        let bits = self.register.read();
        let r = DirsetR { bits: bits };
        let mut w = DirsetW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DirsetR {
        DirsetR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DirsetW) -> &mut DirsetW
    {
        let mut w = DirsetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DirsetR {
    bits: u32,
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin0 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin0::Input,
            1u32 => DirsetRPin0::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin1 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin1::Input,
            1u32 => DirsetRPin1::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin2 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin2::Input,
            1u32 => DirsetRPin2::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin3 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin3::Input,
            1u32 => DirsetRPin3::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin4 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin4 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin4::Input,
            1u32 => DirsetRPin4::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin5 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin5 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin5::Input,
            1u32 => DirsetRPin5::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin6 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin6 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin6::Input,
            1u32 => DirsetRPin6::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin7 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin7 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin7::Input,
            1u32 => DirsetRPin7::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin8 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin8 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin8::Input,
            1u32 => DirsetRPin8::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin9 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin9 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin9::Input,
            1u32 => DirsetRPin9::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin10 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin10 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin10::Input,
            1u32 => DirsetRPin10::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin11 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin11 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin11::Input,
            1u32 => DirsetRPin11::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin12 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin12 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin12::Input,
            1u32 => DirsetRPin12::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin13 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin13 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin13::Input,
            1u32 => DirsetRPin13::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin14 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin14 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin14::Input,
            1u32 => DirsetRPin14::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin15 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin15 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin15::Input,
            1u32 => DirsetRPin15::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin16 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin16 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin16::Input,
            1u32 => DirsetRPin16::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin17 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin17 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin17::Input,
            1u32 => DirsetRPin17::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin18 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin18 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin18::Input,
            1u32 => DirsetRPin18::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin19 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin19 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin19::Input,
            1u32 => DirsetRPin19::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin20 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin20 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin20::Input,
            1u32 => DirsetRPin20::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin21 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin21 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin21::Input,
            1u32 => DirsetRPin21::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin22 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin22 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin22::Input,
            1u32 => DirsetRPin22::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin23 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin23 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin23::Input,
            1u32 => DirsetRPin23::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin24 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin24 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin24::Input,
            1u32 => DirsetRPin24::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin25 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin25 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin25::Input,
            1u32 => DirsetRPin25::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin26 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin26 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin26::Input,
            1u32 => DirsetRPin26::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin27 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin27 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin27::Input,
            1u32 => DirsetRPin27::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin28 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin28 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin28::Input,
            1u32 => DirsetRPin28::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin29 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin29 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin29::Input,
            1u32 => DirsetRPin29::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin30 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin30 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin30::Input,
            1u32 => DirsetRPin30::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetRPin31 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirsetRPin31 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirsetRPin31::Input,
            1u32 => DirsetRPin31::Output,
            _ => unreachable!(),
        }
    }
}

impl DirsetR {
    # [ doc = "Bit 0 - Set as output pin 0." ]
    pub fn pin0(&self) -> DirsetRPin0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        DirsetRPin0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Set as output pin 1." ]
    pub fn pin1(&self) -> DirsetRPin1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        DirsetRPin1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 2 - Set as output pin 2." ]
    pub fn pin2(&self) -> DirsetRPin2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        DirsetRPin2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 3 - Set as output pin 3." ]
    pub fn pin3(&self) -> DirsetRPin3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        DirsetRPin3::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 4 - Set as output pin 4." ]
    pub fn pin4(&self) -> DirsetRPin4 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        DirsetRPin4::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 5 - Set as output pin 5." ]
    pub fn pin5(&self) -> DirsetRPin5 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        DirsetRPin5::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 6 - Set as output pin 6." ]
    pub fn pin6(&self) -> DirsetRPin6 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        DirsetRPin6::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 7 - Set as output pin 7." ]
    pub fn pin7(&self) -> DirsetRPin7 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        DirsetRPin7::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 8 - Set as output pin 8." ]
    pub fn pin8(&self) -> DirsetRPin8 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        DirsetRPin8::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 9 - Set as output pin 9." ]
    pub fn pin9(&self) -> DirsetRPin9 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        DirsetRPin9::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 10 - Set as output pin 10." ]
    pub fn pin10(&self) -> DirsetRPin10 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        DirsetRPin10::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 11 - Set as output pin 11." ]
    pub fn pin11(&self) -> DirsetRPin11 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        DirsetRPin11::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 12 - Set as output pin 12." ]
    pub fn pin12(&self) -> DirsetRPin12 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        DirsetRPin12::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 13 - Set as output pin 13." ]
    pub fn pin13(&self) -> DirsetRPin13 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        DirsetRPin13::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 14 - Set as output pin 14." ]
    pub fn pin14(&self) -> DirsetRPin14 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        DirsetRPin14::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 15 - Set as output pin 15." ]
    pub fn pin15(&self) -> DirsetRPin15 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        DirsetRPin15::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 16 - Set as output pin 16." ]
    pub fn pin16(&self) -> DirsetRPin16 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        DirsetRPin16::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Set as output pin 17." ]
    pub fn pin17(&self) -> DirsetRPin17 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        DirsetRPin17::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Set as output pin 18." ]
    pub fn pin18(&self) -> DirsetRPin18 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        DirsetRPin18::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Set as output pin 19." ]
    pub fn pin19(&self) -> DirsetRPin19 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        DirsetRPin19::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 20 - Set as output pin 20." ]
    pub fn pin20(&self) -> DirsetRPin20 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        DirsetRPin20::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 21 - Set as output pin 21." ]
    pub fn pin21(&self) -> DirsetRPin21 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        DirsetRPin21::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 22 - Set as output pin 22." ]
    pub fn pin22(&self) -> DirsetRPin22 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        DirsetRPin22::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 23 - Set as output pin 23." ]
    pub fn pin23(&self) -> DirsetRPin23 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        DirsetRPin23::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 24 - Set as output pin 24." ]
    pub fn pin24(&self) -> DirsetRPin24 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        DirsetRPin24::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 25 - Set as output pin 25." ]
    pub fn pin25(&self) -> DirsetRPin25 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        DirsetRPin25::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 26 - Set as output pin 26." ]
    pub fn pin26(&self) -> DirsetRPin26 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        DirsetRPin26::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 27 - Set as output pin 27." ]
    pub fn pin27(&self) -> DirsetRPin27 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        DirsetRPin27::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 28 - Set as output pin 28." ]
    pub fn pin28(&self) -> DirsetRPin28 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        DirsetRPin28::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 29 - Set as output pin 29." ]
    pub fn pin29(&self) -> DirsetRPin29 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        DirsetRPin29::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 30 - Set as output pin 30." ]
    pub fn pin30(&self) -> DirsetRPin30 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        DirsetRPin30::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 31 - Set as output pin 31." ]
    pub fn pin31(&self) -> DirsetRPin31 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        DirsetRPin31::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DirsetW {
    bits: u32,
}

pub struct DirsetWPin0<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin0<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin1<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin1<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin2<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin2<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin3<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin3<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin4<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin4<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin5<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin5<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin6<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin6<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin7<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin7<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin8<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin8<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin9<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin9<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin10<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin10<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin11<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin11<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin12<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin12<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin13<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin13<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin14<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin14<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin15<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin15<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin16<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin16<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin17<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin17<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin18<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin18<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin19<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin19<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin20<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin20<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin21<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin21<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin22<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin22<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin23<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin23<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin24<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin24<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin25<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin25<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin26<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin26<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin27<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin27<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin28<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin28<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin29<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin29<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin30<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin30<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirsetWPin31<'a> {
    register: &'a mut DirsetW,
}

impl<'a> DirsetWPin31<'a> {
    pub fn set(self) -> &'a mut DirsetW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl DirsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DirsetW { bits: 0 }
    }
    # [ doc = "Bit 0 - Set as output pin 0." ]
    pub fn pin0(&mut self) -> DirsetWPin0 {
        DirsetWPin0 { register: self }
    }
    # [ doc = "Bit 1 - Set as output pin 1." ]
    pub fn pin1(&mut self) -> DirsetWPin1 {
        DirsetWPin1 { register: self }
    }
    # [ doc = "Bit 2 - Set as output pin 2." ]
    pub fn pin2(&mut self) -> DirsetWPin2 {
        DirsetWPin2 { register: self }
    }
    # [ doc = "Bit 3 - Set as output pin 3." ]
    pub fn pin3(&mut self) -> DirsetWPin3 {
        DirsetWPin3 { register: self }
    }
    # [ doc = "Bit 4 - Set as output pin 4." ]
    pub fn pin4(&mut self) -> DirsetWPin4 {
        DirsetWPin4 { register: self }
    }
    # [ doc = "Bit 5 - Set as output pin 5." ]
    pub fn pin5(&mut self) -> DirsetWPin5 {
        DirsetWPin5 { register: self }
    }
    # [ doc = "Bit 6 - Set as output pin 6." ]
    pub fn pin6(&mut self) -> DirsetWPin6 {
        DirsetWPin6 { register: self }
    }
    # [ doc = "Bit 7 - Set as output pin 7." ]
    pub fn pin7(&mut self) -> DirsetWPin7 {
        DirsetWPin7 { register: self }
    }
    # [ doc = "Bit 8 - Set as output pin 8." ]
    pub fn pin8(&mut self) -> DirsetWPin8 {
        DirsetWPin8 { register: self }
    }
    # [ doc = "Bit 9 - Set as output pin 9." ]
    pub fn pin9(&mut self) -> DirsetWPin9 {
        DirsetWPin9 { register: self }
    }
    # [ doc = "Bit 10 - Set as output pin 10." ]
    pub fn pin10(&mut self) -> DirsetWPin10 {
        DirsetWPin10 { register: self }
    }
    # [ doc = "Bit 11 - Set as output pin 11." ]
    pub fn pin11(&mut self) -> DirsetWPin11 {
        DirsetWPin11 { register: self }
    }
    # [ doc = "Bit 12 - Set as output pin 12." ]
    pub fn pin12(&mut self) -> DirsetWPin12 {
        DirsetWPin12 { register: self }
    }
    # [ doc = "Bit 13 - Set as output pin 13." ]
    pub fn pin13(&mut self) -> DirsetWPin13 {
        DirsetWPin13 { register: self }
    }
    # [ doc = "Bit 14 - Set as output pin 14." ]
    pub fn pin14(&mut self) -> DirsetWPin14 {
        DirsetWPin14 { register: self }
    }
    # [ doc = "Bit 15 - Set as output pin 15." ]
    pub fn pin15(&mut self) -> DirsetWPin15 {
        DirsetWPin15 { register: self }
    }
    # [ doc = "Bit 16 - Set as output pin 16." ]
    pub fn pin16(&mut self) -> DirsetWPin16 {
        DirsetWPin16 { register: self }
    }
    # [ doc = "Bit 17 - Set as output pin 17." ]
    pub fn pin17(&mut self) -> DirsetWPin17 {
        DirsetWPin17 { register: self }
    }
    # [ doc = "Bit 18 - Set as output pin 18." ]
    pub fn pin18(&mut self) -> DirsetWPin18 {
        DirsetWPin18 { register: self }
    }
    # [ doc = "Bit 19 - Set as output pin 19." ]
    pub fn pin19(&mut self) -> DirsetWPin19 {
        DirsetWPin19 { register: self }
    }
    # [ doc = "Bit 20 - Set as output pin 20." ]
    pub fn pin20(&mut self) -> DirsetWPin20 {
        DirsetWPin20 { register: self }
    }
    # [ doc = "Bit 21 - Set as output pin 21." ]
    pub fn pin21(&mut self) -> DirsetWPin21 {
        DirsetWPin21 { register: self }
    }
    # [ doc = "Bit 22 - Set as output pin 22." ]
    pub fn pin22(&mut self) -> DirsetWPin22 {
        DirsetWPin22 { register: self }
    }
    # [ doc = "Bit 23 - Set as output pin 23." ]
    pub fn pin23(&mut self) -> DirsetWPin23 {
        DirsetWPin23 { register: self }
    }
    # [ doc = "Bit 24 - Set as output pin 24." ]
    pub fn pin24(&mut self) -> DirsetWPin24 {
        DirsetWPin24 { register: self }
    }
    # [ doc = "Bit 25 - Set as output pin 25." ]
    pub fn pin25(&mut self) -> DirsetWPin25 {
        DirsetWPin25 { register: self }
    }
    # [ doc = "Bit 26 - Set as output pin 26." ]
    pub fn pin26(&mut self) -> DirsetWPin26 {
        DirsetWPin26 { register: self }
    }
    # [ doc = "Bit 27 - Set as output pin 27." ]
    pub fn pin27(&mut self) -> DirsetWPin27 {
        DirsetWPin27 { register: self }
    }
    # [ doc = "Bit 28 - Set as output pin 28." ]
    pub fn pin28(&mut self) -> DirsetWPin28 {
        DirsetWPin28 { register: self }
    }
    # [ doc = "Bit 29 - Set as output pin 29." ]
    pub fn pin29(&mut self) -> DirsetWPin29 {
        DirsetWPin29 { register: self }
    }
    # [ doc = "Bit 30 - Set as output pin 30." ]
    pub fn pin30(&mut self) -> DirsetWPin30 {
        DirsetWPin30 { register: self }
    }
    # [ doc = "Bit 31 - Set as output pin 31." ]
    pub fn pin31(&mut self) -> DirsetWPin31 {
        DirsetWPin31 { register: self }
    }
}

# [ repr ( C ) ]
pub struct Dirclr {
    register: ::volatile_register::RW<u32>,
}

impl Dirclr {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DirclrR, &'w mut DirclrW) -> &'w mut DirclrW
    {
        let bits = self.register.read();
        let r = DirclrR { bits: bits };
        let mut w = DirclrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DirclrR {
        DirclrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DirclrW) -> &mut DirclrW
    {
        let mut w = DirclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DirclrR {
    bits: u32,
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin0 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin0 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin0::Input,
            1u32 => DirclrRPin0::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin1 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin1 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin1::Input,
            1u32 => DirclrRPin1::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin2 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin2 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin2::Input,
            1u32 => DirclrRPin2::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin3 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin3 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin3::Input,
            1u32 => DirclrRPin3::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin4 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin4 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin4::Input,
            1u32 => DirclrRPin4::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin5 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin5 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin5::Input,
            1u32 => DirclrRPin5::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin6 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin6 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin6::Input,
            1u32 => DirclrRPin6::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin7 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin7 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin7::Input,
            1u32 => DirclrRPin7::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin8 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin8 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin8::Input,
            1u32 => DirclrRPin8::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin9 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin9 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin9::Input,
            1u32 => DirclrRPin9::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin10 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin10 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin10::Input,
            1u32 => DirclrRPin10::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin11 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin11 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin11::Input,
            1u32 => DirclrRPin11::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin12 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin12 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin12::Input,
            1u32 => DirclrRPin12::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin13 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin13 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin13::Input,
            1u32 => DirclrRPin13::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin14 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin14 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin14::Input,
            1u32 => DirclrRPin14::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin15 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin15 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin15::Input,
            1u32 => DirclrRPin15::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin16 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin16 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin16::Input,
            1u32 => DirclrRPin16::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin17 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin17 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin17::Input,
            1u32 => DirclrRPin17::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin18 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin18 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin18::Input,
            1u32 => DirclrRPin18::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin19 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin19 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin19::Input,
            1u32 => DirclrRPin19::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin20 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin20 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin20::Input,
            1u32 => DirclrRPin20::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin21 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin21 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin21::Input,
            1u32 => DirclrRPin21::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin22 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin22 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin22::Input,
            1u32 => DirclrRPin22::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin23 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin23 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin23::Input,
            1u32 => DirclrRPin23::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin24 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin24 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin24::Input,
            1u32 => DirclrRPin24::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin25 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin25 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin25::Input,
            1u32 => DirclrRPin25::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin26 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin26 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin26::Input,
            1u32 => DirclrRPin26::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin27 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin27 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin27::Input,
            1u32 => DirclrRPin27::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin28 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin28 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin28::Input,
            1u32 => DirclrRPin28::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin29 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin29 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin29::Input,
            1u32 => DirclrRPin29::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin30 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin30 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin30::Input,
            1u32 => DirclrRPin30::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrRPin31 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirclrRPin31 {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => DirclrRPin31::Input,
            1u32 => DirclrRPin31::Output,
            _ => unreachable!(),
        }
    }
}

impl DirclrR {
    # [ doc = "Bit 0 - Set as input pin 0." ]
    pub fn pin0(&self) -> DirclrRPin0 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        DirclrRPin0::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Set as input pin 1." ]
    pub fn pin1(&self) -> DirclrRPin1 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        DirclrRPin1::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 2 - Set as input pin 2." ]
    pub fn pin2(&self) -> DirclrRPin2 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        DirclrRPin2::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 3 - Set as input pin 3." ]
    pub fn pin3(&self) -> DirclrRPin3 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        DirclrRPin3::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 4 - Set as input pin 4." ]
    pub fn pin4(&self) -> DirclrRPin4 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        DirclrRPin4::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 5 - Set as input pin 5." ]
    pub fn pin5(&self) -> DirclrRPin5 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        DirclrRPin5::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 6 - Set as input pin 6." ]
    pub fn pin6(&self) -> DirclrRPin6 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        DirclrRPin6::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 7 - Set as input pin 7." ]
    pub fn pin7(&self) -> DirclrRPin7 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        DirclrRPin7::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 8 - Set as input pin 8." ]
    pub fn pin8(&self) -> DirclrRPin8 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        DirclrRPin8::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 9 - Set as input pin 9." ]
    pub fn pin9(&self) -> DirclrRPin9 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        DirclrRPin9::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 10 - Set as input pin 10." ]
    pub fn pin10(&self) -> DirclrRPin10 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        DirclrRPin10::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 11 - Set as input pin 11." ]
    pub fn pin11(&self) -> DirclrRPin11 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        DirclrRPin11::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 12 - Set as input pin 12." ]
    pub fn pin12(&self) -> DirclrRPin12 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        DirclrRPin12::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 13 - Set as input pin 13." ]
    pub fn pin13(&self) -> DirclrRPin13 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        DirclrRPin13::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 14 - Set as input pin 14." ]
    pub fn pin14(&self) -> DirclrRPin14 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        DirclrRPin14::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 15 - Set as input pin 15." ]
    pub fn pin15(&self) -> DirclrRPin15 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        DirclrRPin15::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 16 - Set as input pin 16." ]
    pub fn pin16(&self) -> DirclrRPin16 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        DirclrRPin16::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 17 - Set as input pin 17." ]
    pub fn pin17(&self) -> DirclrRPin17 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        DirclrRPin17::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 18 - Set as input pin 18." ]
    pub fn pin18(&self) -> DirclrRPin18 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        DirclrRPin18::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 19 - Set as input pin 19." ]
    pub fn pin19(&self) -> DirclrRPin19 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        DirclrRPin19::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 20 - Set as input pin 20." ]
    pub fn pin20(&self) -> DirclrRPin20 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        DirclrRPin20::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 21 - Set as input pin 21." ]
    pub fn pin21(&self) -> DirclrRPin21 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        DirclrRPin21::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 22 - Set as input pin 22." ]
    pub fn pin22(&self) -> DirclrRPin22 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        DirclrRPin22::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 23 - Set as input pin 23." ]
    pub fn pin23(&self) -> DirclrRPin23 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        DirclrRPin23::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 24 - Set as input pin 24." ]
    pub fn pin24(&self) -> DirclrRPin24 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        DirclrRPin24::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 25 - Set as input pin 25." ]
    pub fn pin25(&self) -> DirclrRPin25 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        DirclrRPin25::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 26 - Set as input pin 26." ]
    pub fn pin26(&self) -> DirclrRPin26 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        DirclrRPin26::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 27 - Set as input pin 27." ]
    pub fn pin27(&self) -> DirclrRPin27 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        DirclrRPin27::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 28 - Set as input pin 28." ]
    pub fn pin28(&self) -> DirclrRPin28 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        DirclrRPin28::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 29 - Set as input pin 29." ]
    pub fn pin29(&self) -> DirclrRPin29 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        DirclrRPin29::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 30 - Set as input pin 30." ]
    pub fn pin30(&self) -> DirclrRPin30 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        DirclrRPin30::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 31 - Set as input pin 31." ]
    pub fn pin31(&self) -> DirclrRPin31 {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        DirclrRPin31::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DirclrW {
    bits: u32,
}

pub struct DirclrWPin0<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin0<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin1<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin1<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin2<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin2<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin3<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin3<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin4<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin4<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin5<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin5<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin6<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin6<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin7<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin7<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin8<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin8<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin9<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin9<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin10<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin10<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin11<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin11<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin12<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin12<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin13<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin13<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin14<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin14<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin15<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin15<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin16<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin16<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin17<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin17<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin18<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin18<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin19<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin19<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin20<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin20<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin21<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin21<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin22<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin22<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin23<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin23<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin24<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin24<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin25<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin25<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin26<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin26<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin27<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin27<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin28<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin28<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin29<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin29<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin30<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin30<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct DirclrWPin31<'a> {
    register: &'a mut DirclrW,
}

impl<'a> DirclrWPin31<'a> {
    pub fn clear(self) -> &'a mut DirclrW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

impl DirclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DirclrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Set as input pin 0." ]
    pub fn pin0(&mut self) -> DirclrWPin0 {
        DirclrWPin0 { register: self }
    }
    # [ doc = "Bit 1 - Set as input pin 1." ]
    pub fn pin1(&mut self) -> DirclrWPin1 {
        DirclrWPin1 { register: self }
    }
    # [ doc = "Bit 2 - Set as input pin 2." ]
    pub fn pin2(&mut self) -> DirclrWPin2 {
        DirclrWPin2 { register: self }
    }
    # [ doc = "Bit 3 - Set as input pin 3." ]
    pub fn pin3(&mut self) -> DirclrWPin3 {
        DirclrWPin3 { register: self }
    }
    # [ doc = "Bit 4 - Set as input pin 4." ]
    pub fn pin4(&mut self) -> DirclrWPin4 {
        DirclrWPin4 { register: self }
    }
    # [ doc = "Bit 5 - Set as input pin 5." ]
    pub fn pin5(&mut self) -> DirclrWPin5 {
        DirclrWPin5 { register: self }
    }
    # [ doc = "Bit 6 - Set as input pin 6." ]
    pub fn pin6(&mut self) -> DirclrWPin6 {
        DirclrWPin6 { register: self }
    }
    # [ doc = "Bit 7 - Set as input pin 7." ]
    pub fn pin7(&mut self) -> DirclrWPin7 {
        DirclrWPin7 { register: self }
    }
    # [ doc = "Bit 8 - Set as input pin 8." ]
    pub fn pin8(&mut self) -> DirclrWPin8 {
        DirclrWPin8 { register: self }
    }
    # [ doc = "Bit 9 - Set as input pin 9." ]
    pub fn pin9(&mut self) -> DirclrWPin9 {
        DirclrWPin9 { register: self }
    }
    # [ doc = "Bit 10 - Set as input pin 10." ]
    pub fn pin10(&mut self) -> DirclrWPin10 {
        DirclrWPin10 { register: self }
    }
    # [ doc = "Bit 11 - Set as input pin 11." ]
    pub fn pin11(&mut self) -> DirclrWPin11 {
        DirclrWPin11 { register: self }
    }
    # [ doc = "Bit 12 - Set as input pin 12." ]
    pub fn pin12(&mut self) -> DirclrWPin12 {
        DirclrWPin12 { register: self }
    }
    # [ doc = "Bit 13 - Set as input pin 13." ]
    pub fn pin13(&mut self) -> DirclrWPin13 {
        DirclrWPin13 { register: self }
    }
    # [ doc = "Bit 14 - Set as input pin 14." ]
    pub fn pin14(&mut self) -> DirclrWPin14 {
        DirclrWPin14 { register: self }
    }
    # [ doc = "Bit 15 - Set as input pin 15." ]
    pub fn pin15(&mut self) -> DirclrWPin15 {
        DirclrWPin15 { register: self }
    }
    # [ doc = "Bit 16 - Set as input pin 16." ]
    pub fn pin16(&mut self) -> DirclrWPin16 {
        DirclrWPin16 { register: self }
    }
    # [ doc = "Bit 17 - Set as input pin 17." ]
    pub fn pin17(&mut self) -> DirclrWPin17 {
        DirclrWPin17 { register: self }
    }
    # [ doc = "Bit 18 - Set as input pin 18." ]
    pub fn pin18(&mut self) -> DirclrWPin18 {
        DirclrWPin18 { register: self }
    }
    # [ doc = "Bit 19 - Set as input pin 19." ]
    pub fn pin19(&mut self) -> DirclrWPin19 {
        DirclrWPin19 { register: self }
    }
    # [ doc = "Bit 20 - Set as input pin 20." ]
    pub fn pin20(&mut self) -> DirclrWPin20 {
        DirclrWPin20 { register: self }
    }
    # [ doc = "Bit 21 - Set as input pin 21." ]
    pub fn pin21(&mut self) -> DirclrWPin21 {
        DirclrWPin21 { register: self }
    }
    # [ doc = "Bit 22 - Set as input pin 22." ]
    pub fn pin22(&mut self) -> DirclrWPin22 {
        DirclrWPin22 { register: self }
    }
    # [ doc = "Bit 23 - Set as input pin 23." ]
    pub fn pin23(&mut self) -> DirclrWPin23 {
        DirclrWPin23 { register: self }
    }
    # [ doc = "Bit 24 - Set as input pin 24." ]
    pub fn pin24(&mut self) -> DirclrWPin24 {
        DirclrWPin24 { register: self }
    }
    # [ doc = "Bit 25 - Set as input pin 25." ]
    pub fn pin25(&mut self) -> DirclrWPin25 {
        DirclrWPin25 { register: self }
    }
    # [ doc = "Bit 26 - Set as input pin 26." ]
    pub fn pin26(&mut self) -> DirclrWPin26 {
        DirclrWPin26 { register: self }
    }
    # [ doc = "Bit 27 - Set as input pin 27." ]
    pub fn pin27(&mut self) -> DirclrWPin27 {
        DirclrWPin27 { register: self }
    }
    # [ doc = "Bit 28 - Set as input pin 28." ]
    pub fn pin28(&mut self) -> DirclrWPin28 {
        DirclrWPin28 { register: self }
    }
    # [ doc = "Bit 29 - Set as input pin 29." ]
    pub fn pin29(&mut self) -> DirclrWPin29 {
        DirclrWPin29 { register: self }
    }
    # [ doc = "Bit 30 - Set as input pin 30." ]
    pub fn pin30(&mut self) -> DirclrWPin30 {
        DirclrWPin30 { register: self }
    }
    # [ doc = "Bit 31 - Set as input pin 31." ]
    pub fn pin31(&mut self) -> DirclrWPin31 {
        DirclrWPin31 { register: self }
    }
}

# [ repr ( C ) ]
pub struct PinCnf {
    register: ::volatile_register::RW<u32>,
}

impl PinCnf {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PinCnfR, &'w mut PinCnfW) -> &'w mut PinCnfW
    {
        let bits = self.register.read();
        let r = PinCnfR { bits: bits };
        let mut w = PinCnfW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PinCnfR {
        PinCnfR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PinCnfW) -> &mut PinCnfW
    {
        let mut w = PinCnfW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PinCnfR {
    bits: u32,
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfRDir {
    # [ doc = "Configure pin as an input pin." ]
    Input,
    # [ doc = "Configure pin as an output pin." ]
    Output,
}
impl PinCnfRDir {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => PinCnfRDir::Input,
            1u32 => PinCnfRDir::Output,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfRInput {
    # [ doc = "Connect input pin." ]
    Connect,
    # [ doc = "Disconnect input pin." ]
    Disconnect,
}
impl PinCnfRInput {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => PinCnfRInput::Connect,
            1u32 => PinCnfRInput::Disconnect,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfRPull {
    # [ doc = "No pull." ]
    Disabled,
    # [ doc = "Pulldown on pin." ]
    Pulldown,
    # [ doc ( hidden ) ]
    _Reserved10,
    # [ doc = "Pullup on pin." ]
    Pullup,
}
impl PinCnfRPull {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => PinCnfRPull::Disabled,
            1u32 => PinCnfRPull::Pulldown,
            2u32 => PinCnfRPull::_Reserved10,
            3u32 => PinCnfRPull::Pullup,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfRDrive {
    # [ doc = "Standard '0', Standard '1'." ]
    S0S1,
    # [ doc = "High '0', Standard '1'." ]
    H0S1,
    # [ doc = "Standard '0', High '1'." ]
    S0H1,
    # [ doc = "High '0', High '1'." ]
    H0H1,
    # [ doc = "Disconnected '0', Standard '1'." ]
    D0S1,
    # [ doc = "Disconnected '0', High '1'." ]
    D0H1,
    # [ doc = "Standard '0', Disconnected '1'." ]
    S0D1,
    # [ doc = "High '0', Disconnected '1'." ]
    H0D1,
}
impl PinCnfRDrive {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => PinCnfRDrive::S0S1,
            1u32 => PinCnfRDrive::H0S1,
            2u32 => PinCnfRDrive::S0H1,
            3u32 => PinCnfRDrive::H0H1,
            4u32 => PinCnfRDrive::D0S1,
            5u32 => PinCnfRDrive::D0H1,
            6u32 => PinCnfRDrive::S0D1,
            7u32 => PinCnfRDrive::H0D1,
            _ => unreachable!(),
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfRSense {
    # [ doc = "Disabled." ]
    Disabled,
    # [ doc ( hidden ) ]
    _Reserved1,
    # [ doc = "Wakeup on high level." ]
    High,
    # [ doc = "Wakeup on low level." ]
    Low,
}
impl PinCnfRSense {
    # [ inline ( always ) ]
    fn from(value: u32) -> Self {
        match value {
            0u32 => PinCnfRSense::Disabled,
            1u32 => PinCnfRSense::_Reserved1,
            2u32 => PinCnfRSense::High,
            3u32 => PinCnfRSense::Low,
            _ => unreachable!(),
        }
    }
}

impl PinCnfR {
    # [ doc = "Bit 0 - Pin direction." ]
    pub fn dir(&self) -> PinCnfRDir {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        PinCnfRDir::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bit 1 - Connect or disconnect input path." ]
    pub fn input(&self) -> PinCnfRInput {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        PinCnfRInput::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bits 2:3 - Pull-up or -down configuration." ]
    pub fn pull(&self) -> PinCnfRPull {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        PinCnfRPull::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bits 8:10 - Drive configuration." ]
    pub fn drive(&self) -> PinCnfRDrive {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        PinCnfRDrive::from((self.bits >> OFFSET) & MASK)
    }
    # [ doc = "Bits 16:17 - Pin sensing mechanism." ]
    pub fn sense(&self) -> PinCnfRSense {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        PinCnfRSense::from((self.bits >> OFFSET) & MASK)
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PinCnfW {
    bits: u32,
}

pub struct PinCnfWDir<'a> {
    register: &'a mut PinCnfW,
}

impl<'a> PinCnfWDir<'a> {
    pub fn input(self) -> &'a mut PinCnfW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn output(self) -> &'a mut PinCnfW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct PinCnfWInput<'a> {
    register: &'a mut PinCnfW,
}

impl<'a> PinCnfWInput<'a> {
    pub fn connect(self) -> &'a mut PinCnfW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn disconnect(self) -> &'a mut PinCnfW {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
}

pub struct PinCnfWPull<'a> {
    register: &'a mut PinCnfW,
}

impl<'a> PinCnfWPull<'a> {
    pub fn disabled(self) -> &'a mut PinCnfW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn pulldown(self) -> &'a mut PinCnfW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
    pub fn pullup(self) -> &'a mut PinCnfW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 3u32 << OFFSET;
        self.register
    }
}

pub struct PinCnfWDrive<'a> {
    register: &'a mut PinCnfW,
}

impl<'a> PinCnfWDrive<'a> {
    pub fn s0s1(self) -> &'a mut PinCnfW {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn h0s1(self) -> &'a mut PinCnfW {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 1u32 << OFFSET;
        self.register
    }
    pub fn s0h1(self) -> &'a mut PinCnfW {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 2u32 << OFFSET;
        self.register
    }
    pub fn h0h1(self) -> &'a mut PinCnfW {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 3u32 << OFFSET;
        self.register
    }
    pub fn d0s1(self) -> &'a mut PinCnfW {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 4u32 << OFFSET;
        self.register
    }
    pub fn d0h1(self) -> &'a mut PinCnfW {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 5u32 << OFFSET;
        self.register
    }
    pub fn s0d1(self) -> &'a mut PinCnfW {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 6u32 << OFFSET;
        self.register
    }
    pub fn h0d1(self) -> &'a mut PinCnfW {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 7u32 << OFFSET;
        self.register
    }
}

pub struct PinCnfWSense<'a> {
    register: &'a mut PinCnfW,
}

impl<'a> PinCnfWSense<'a> {
    pub fn disabled(self) -> &'a mut PinCnfW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 0u32 << OFFSET;
        self.register
    }
    pub fn high(self) -> &'a mut PinCnfW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 2u32 << OFFSET;
        self.register
    }
    pub fn low(self) -> &'a mut PinCnfW {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        self.register.bits &= !((MASK as u32) << OFFSET);
        self.register.bits |= 3u32 << OFFSET;
        self.register
    }
}

impl PinCnfW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PinCnfW { bits: 2 }
    }
    # [ doc = "Bit 0 - Pin direction." ]
    pub fn dir(&mut self) -> PinCnfWDir {
        PinCnfWDir { register: self }
    }
    # [ doc = "Bit 1 - Connect or disconnect input path." ]
    pub fn input(&mut self) -> PinCnfWInput {
        PinCnfWInput { register: self }
    }
    # [ doc = "Bits 2:3 - Pull-up or -down configuration." ]
    pub fn pull(&mut self) -> PinCnfWPull {
        PinCnfWPull { register: self }
    }
    # [ doc = "Bits 8:10 - Drive configuration." ]
    pub fn drive(&mut self) -> PinCnfWDrive {
        PinCnfWDrive { register: self }
    }
    # [ doc = "Bits 16:17 - Pin sensing mechanism." ]
    pub fn sense(&mut self) -> PinCnfWSense {
        PinCnfWSense { register: self }
    }
}
