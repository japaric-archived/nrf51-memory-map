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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin0 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin0 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin0::Low => 0u32,
            OutWPin0::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin1 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin1 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin1::Low => 0u32,
            OutWPin1::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin2 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin2 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin2::Low => 0u32,
            OutWPin2::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin3 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin3 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin3::Low => 0u32,
            OutWPin3::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin4 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin4 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin4::Low => 0u32,
            OutWPin4::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin5 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin5 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin5::Low => 0u32,
            OutWPin5::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin6 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin6 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin6::Low => 0u32,
            OutWPin6::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin7 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin7 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin7::Low => 0u32,
            OutWPin7::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin8 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin8 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin8::Low => 0u32,
            OutWPin8::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin9 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin9 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin9::Low => 0u32,
            OutWPin9::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin10 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin10 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin10::Low => 0u32,
            OutWPin10::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin11 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin11 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin11::Low => 0u32,
            OutWPin11::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin12 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin12 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin12::Low => 0u32,
            OutWPin12::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin13 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin13 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin13::Low => 0u32,
            OutWPin13::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin14 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin14 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin14::Low => 0u32,
            OutWPin14::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin15 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin15 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin15::Low => 0u32,
            OutWPin15::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin16 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin16 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin16::Low => 0u32,
            OutWPin16::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin17 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin17 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin17::Low => 0u32,
            OutWPin17::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin18 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin18 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin18::Low => 0u32,
            OutWPin18::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin19 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin19 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin19::Low => 0u32,
            OutWPin19::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin20 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin20 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin20::Low => 0u32,
            OutWPin20::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin21 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin21 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin21::Low => 0u32,
            OutWPin21::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin22 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin22 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin22::Low => 0u32,
            OutWPin22::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin23 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin23 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin23::Low => 0u32,
            OutWPin23::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin24 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin24 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin24::Low => 0u32,
            OutWPin24::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin25 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin25 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin25::Low => 0u32,
            OutWPin25::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin26 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin26 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin26::Low => 0u32,
            OutWPin26::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin27 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin27 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin27::Low => 0u32,
            OutWPin27::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin28 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin28 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin28::Low => 0u32,
            OutWPin28::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin29 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin29 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin29::Low => 0u32,
            OutWPin29::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin30 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin30 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin30::Low => 0u32,
            OutWPin30::High => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutWPin31 {
    # [ doc = "Pin driver is low." ]
    Low,
    # [ doc = "Pin driver is high." ]
    High,
}
impl OutWPin31 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutWPin31::Low => 0u32,
            OutWPin31::High => 1u32,
        }
    }
}

impl OutW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OutW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&mut self, value: OutWPin0) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&mut self, value: OutWPin1) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&mut self, value: OutWPin2) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&mut self, value: OutWPin3) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&mut self, value: OutWPin4) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&mut self, value: OutWPin5) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&mut self, value: OutWPin6) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&mut self, value: OutWPin7) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&mut self, value: OutWPin8) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&mut self, value: OutWPin9) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&mut self, value: OutWPin10) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&mut self, value: OutWPin11) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&mut self, value: OutWPin12) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&mut self, value: OutWPin13) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&mut self, value: OutWPin14) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&mut self, value: OutWPin15) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&mut self, value: OutWPin16) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&mut self, value: OutWPin17) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&mut self, value: OutWPin18) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&mut self, value: OutWPin19) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&mut self, value: OutWPin20) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&mut self, value: OutWPin21) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&mut self, value: OutWPin22) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&mut self, value: OutWPin23) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&mut self, value: OutWPin24) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&mut self, value: OutWPin25) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&mut self, value: OutWPin26) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&mut self, value: OutWPin27) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&mut self, value: OutWPin28) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&mut self, value: OutWPin29) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&mut self, value: OutWPin30) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&mut self, value: OutWPin31) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin0 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin0 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin0::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin1 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin1 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin1::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin2 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin2 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin2::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin3 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin3 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin3::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin4 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin4 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin4::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin5 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin5 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin5::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin6 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin6 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin6::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin7 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin7 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin7::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin8 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin8 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin8::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin9 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin9 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin9::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin10 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin10 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin10::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin11 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin11 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin11::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin12 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin12 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin12::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin13 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin13 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin13::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin14 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin14 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin14::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin15 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin15 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin15::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin16 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin16 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin16::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin17 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin17 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin17::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin18 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin18 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin18::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin19 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin19 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin19::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin20 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin20 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin20::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin21 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin21 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin21::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin22 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin22 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin22::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin23 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin23 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin23::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin24 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin24 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin24::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin25 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin25 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin25::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin26 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin26 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin26::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin27 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin27 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin27::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin28 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin28 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin28::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin29 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin29 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin29::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin30 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin30 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin30::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutsetWPin31 {
    # [ doc = "Set pin driver high." ]
    Set,
}
impl OutsetWPin31 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutsetWPin31::Set => 1u32,
        }
    }
}

impl OutsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OutsetW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&mut self, value: OutsetWPin0) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&mut self, value: OutsetWPin1) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&mut self, value: OutsetWPin2) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&mut self, value: OutsetWPin3) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&mut self, value: OutsetWPin4) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&mut self, value: OutsetWPin5) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&mut self, value: OutsetWPin6) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&mut self, value: OutsetWPin7) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&mut self, value: OutsetWPin8) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&mut self, value: OutsetWPin9) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&mut self, value: OutsetWPin10) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&mut self, value: OutsetWPin11) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&mut self, value: OutsetWPin12) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&mut self, value: OutsetWPin13) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&mut self, value: OutsetWPin14) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&mut self, value: OutsetWPin15) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&mut self, value: OutsetWPin16) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&mut self, value: OutsetWPin17) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&mut self, value: OutsetWPin18) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&mut self, value: OutsetWPin19) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&mut self, value: OutsetWPin20) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&mut self, value: OutsetWPin21) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&mut self, value: OutsetWPin22) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&mut self, value: OutsetWPin23) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&mut self, value: OutsetWPin24) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&mut self, value: OutsetWPin25) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&mut self, value: OutsetWPin26) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&mut self, value: OutsetWPin27) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&mut self, value: OutsetWPin28) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&mut self, value: OutsetWPin29) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&mut self, value: OutsetWPin30) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&mut self, value: OutsetWPin31) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin0 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin0 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin0::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin1 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin1 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin1::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin2 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin2 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin2::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin3 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin3 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin3::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin4 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin4 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin4::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin5 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin5 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin5::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin6 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin6 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin6::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin7 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin7 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin7::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin8 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin8 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin8::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin9 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin9 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin9::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin10 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin10 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin10::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin11 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin11 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin11::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin12 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin12 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin12::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin13 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin13 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin13::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin14 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin14 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin14::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin15 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin15 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin15::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin16 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin16 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin16::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin17 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin17 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin17::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin18 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin18 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin18::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin19 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin19 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin19::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin20 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin20 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin20::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin21 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin21 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin21::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin22 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin22 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin22::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin23 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin23 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin23::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin24 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin24 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin24::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin25 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin25 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin25::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin26 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin26 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin26::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin27 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin27 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin27::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin28 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin28 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin28::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin29 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin29 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin29::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin30 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin30 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin30::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum OutclrWPin31 {
    # [ doc = "Set pin driver low." ]
    Clear,
}
impl OutclrWPin31 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            OutclrWPin31::Clear => 1u32,
        }
    }
}

impl OutclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OutclrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&mut self, value: OutclrWPin0) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&mut self, value: OutclrWPin1) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&mut self, value: OutclrWPin2) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&mut self, value: OutclrWPin3) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&mut self, value: OutclrWPin4) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&mut self, value: OutclrWPin5) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&mut self, value: OutclrWPin6) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&mut self, value: OutclrWPin7) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&mut self, value: OutclrWPin8) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&mut self, value: OutclrWPin9) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&mut self, value: OutclrWPin10) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&mut self, value: OutclrWPin11) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&mut self, value: OutclrWPin12) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&mut self, value: OutclrWPin13) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&mut self, value: OutclrWPin14) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&mut self, value: OutclrWPin15) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&mut self, value: OutclrWPin16) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&mut self, value: OutclrWPin17) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&mut self, value: OutclrWPin18) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&mut self, value: OutclrWPin19) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&mut self, value: OutclrWPin20) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&mut self, value: OutclrWPin21) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&mut self, value: OutclrWPin22) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&mut self, value: OutclrWPin23) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&mut self, value: OutclrWPin24) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&mut self, value: OutclrWPin25) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&mut self, value: OutclrWPin26) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&mut self, value: OutclrWPin27) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&mut self, value: OutclrWPin28) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&mut self, value: OutclrWPin29) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&mut self, value: OutclrWPin30) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&mut self, value: OutclrWPin31) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin0 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin0 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin0::Input => 0u32,
            DirWPin0::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin1 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin1 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin1::Input => 0u32,
            DirWPin1::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin2 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin2 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin2::Input => 0u32,
            DirWPin2::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin3 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin3 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin3::Input => 0u32,
            DirWPin3::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin4 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin4 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin4::Input => 0u32,
            DirWPin4::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin5 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin5 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin5::Input => 0u32,
            DirWPin5::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin6 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin6 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin6::Input => 0u32,
            DirWPin6::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin7 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin7 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin7::Input => 0u32,
            DirWPin7::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin8 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin8 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin8::Input => 0u32,
            DirWPin8::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin9 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin9 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin9::Input => 0u32,
            DirWPin9::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin10 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin10 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin10::Input => 0u32,
            DirWPin10::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin11 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin11 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin11::Input => 0u32,
            DirWPin11::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin12 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin12 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin12::Input => 0u32,
            DirWPin12::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin13 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin13 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin13::Input => 0u32,
            DirWPin13::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin14 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin14 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin14::Input => 0u32,
            DirWPin14::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin15 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin15 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin15::Input => 0u32,
            DirWPin15::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin16 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin16 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin16::Input => 0u32,
            DirWPin16::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin17 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin17 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin17::Input => 0u32,
            DirWPin17::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin18 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin18 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin18::Input => 0u32,
            DirWPin18::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin19 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin19 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin19::Input => 0u32,
            DirWPin19::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin20 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin20 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin20::Input => 0u32,
            DirWPin20::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin21 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin21 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin21::Input => 0u32,
            DirWPin21::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin22 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin22 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin22::Input => 0u32,
            DirWPin22::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin23 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin23 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin23::Input => 0u32,
            DirWPin23::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin24 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin24 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin24::Input => 0u32,
            DirWPin24::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin25 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin25 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin25::Input => 0u32,
            DirWPin25::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin26 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin26 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin26::Input => 0u32,
            DirWPin26::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin27 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin27 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin27::Input => 0u32,
            DirWPin27::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin28 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin28 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin28::Input => 0u32,
            DirWPin28::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin29 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin29 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin29::Input => 0u32,
            DirWPin29::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin30 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin30 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin30::Input => 0u32,
            DirWPin30::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirWPin31 {
    # [ doc = "Pin set as input." ]
    Input,
    # [ doc = "Pin set as output." ]
    Output,
}
impl DirWPin31 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirWPin31::Input => 0u32,
            DirWPin31::Output => 1u32,
        }
    }
}

impl DirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DirW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pin 0." ]
    pub fn pin0(&mut self, value: DirWPin0) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 1 - Pin 1." ]
    pub fn pin1(&mut self, value: DirWPin1) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Pin 2." ]
    pub fn pin2(&mut self, value: DirWPin2) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Pin 3." ]
    pub fn pin3(&mut self, value: DirWPin3) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Pin 4." ]
    pub fn pin4(&mut self, value: DirWPin4) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Pin 5." ]
    pub fn pin5(&mut self, value: DirWPin5) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Pin 6." ]
    pub fn pin6(&mut self, value: DirWPin6) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Pin 7." ]
    pub fn pin7(&mut self, value: DirWPin7) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 8 - Pin 8." ]
    pub fn pin8(&mut self, value: DirWPin8) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 9 - Pin 9." ]
    pub fn pin9(&mut self, value: DirWPin9) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Pin 10." ]
    pub fn pin10(&mut self, value: DirWPin10) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Pin 11." ]
    pub fn pin11(&mut self, value: DirWPin11) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 12 - Pin 12." ]
    pub fn pin12(&mut self, value: DirWPin12) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 13 - Pin 13." ]
    pub fn pin13(&mut self, value: DirWPin13) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Pin 14." ]
    pub fn pin14(&mut self, value: DirWPin14) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Pin 15." ]
    pub fn pin15(&mut self, value: DirWPin15) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Pin 16." ]
    pub fn pin16(&mut self, value: DirWPin16) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 17 - Pin 17." ]
    pub fn pin17(&mut self, value: DirWPin17) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 18 - Pin 18." ]
    pub fn pin18(&mut self, value: DirWPin18) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Pin 19." ]
    pub fn pin19(&mut self, value: DirWPin19) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 20 - Pin 20." ]
    pub fn pin20(&mut self, value: DirWPin20) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Pin 21." ]
    pub fn pin21(&mut self, value: DirWPin21) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Pin 22." ]
    pub fn pin22(&mut self, value: DirWPin22) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Pin 23." ]
    pub fn pin23(&mut self, value: DirWPin23) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 24 - Pin 24." ]
    pub fn pin24(&mut self, value: DirWPin24) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 25 - Pin 25." ]
    pub fn pin25(&mut self, value: DirWPin25) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Pin 26." ]
    pub fn pin26(&mut self, value: DirWPin26) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 27 - Pin 27." ]
    pub fn pin27(&mut self, value: DirWPin27) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 28 - Pin 28." ]
    pub fn pin28(&mut self, value: DirWPin28) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Pin 29." ]
    pub fn pin29(&mut self, value: DirWPin29) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 30 - Pin 30." ]
    pub fn pin30(&mut self, value: DirWPin30) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 31 - Pin 31." ]
    pub fn pin31(&mut self, value: DirWPin31) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin0 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin0 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin0::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin1 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin1 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin1::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin2 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin2 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin2::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin3 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin3 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin3::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin4 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin4 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin4::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin5 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin5 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin5::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin6 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin6 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin6::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin7 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin7 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin7::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin8 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin8 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin8::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin9 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin9 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin9::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin10 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin10 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin10::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin11 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin11 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin11::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin12 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin12 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin12::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin13 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin13 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin13::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin14 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin14 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin14::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin15 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin15 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin15::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin16 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin16 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin16::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin17 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin17 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin17::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin18 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin18 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin18::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin19 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin19 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin19::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin20 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin20 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin20::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin21 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin21 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin21::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin22 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin22 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin22::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin23 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin23 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin23::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin24 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin24 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin24::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin25 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin25 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin25::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin26 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin26 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin26::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin27 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin27 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin27::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin28 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin28 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin28::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin29 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin29 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin29::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin30 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin30 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin30::Set => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirsetWPin31 {
    # [ doc = "Set pin as output." ]
    Set,
}
impl DirsetWPin31 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirsetWPin31::Set => 1u32,
        }
    }
}

impl DirsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DirsetW { bits: 0 }
    }
    # [ doc = "Bit 0 - Set as output pin 0." ]
    pub fn pin0(&mut self, value: DirsetWPin0) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 1 - Set as output pin 1." ]
    pub fn pin1(&mut self, value: DirsetWPin1) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Set as output pin 2." ]
    pub fn pin2(&mut self, value: DirsetWPin2) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Set as output pin 3." ]
    pub fn pin3(&mut self, value: DirsetWPin3) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Set as output pin 4." ]
    pub fn pin4(&mut self, value: DirsetWPin4) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Set as output pin 5." ]
    pub fn pin5(&mut self, value: DirsetWPin5) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Set as output pin 6." ]
    pub fn pin6(&mut self, value: DirsetWPin6) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Set as output pin 7." ]
    pub fn pin7(&mut self, value: DirsetWPin7) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 8 - Set as output pin 8." ]
    pub fn pin8(&mut self, value: DirsetWPin8) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 9 - Set as output pin 9." ]
    pub fn pin9(&mut self, value: DirsetWPin9) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Set as output pin 10." ]
    pub fn pin10(&mut self, value: DirsetWPin10) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Set as output pin 11." ]
    pub fn pin11(&mut self, value: DirsetWPin11) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 12 - Set as output pin 12." ]
    pub fn pin12(&mut self, value: DirsetWPin12) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 13 - Set as output pin 13." ]
    pub fn pin13(&mut self, value: DirsetWPin13) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Set as output pin 14." ]
    pub fn pin14(&mut self, value: DirsetWPin14) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Set as output pin 15." ]
    pub fn pin15(&mut self, value: DirsetWPin15) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Set as output pin 16." ]
    pub fn pin16(&mut self, value: DirsetWPin16) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 17 - Set as output pin 17." ]
    pub fn pin17(&mut self, value: DirsetWPin17) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 18 - Set as output pin 18." ]
    pub fn pin18(&mut self, value: DirsetWPin18) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Set as output pin 19." ]
    pub fn pin19(&mut self, value: DirsetWPin19) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 20 - Set as output pin 20." ]
    pub fn pin20(&mut self, value: DirsetWPin20) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Set as output pin 21." ]
    pub fn pin21(&mut self, value: DirsetWPin21) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Set as output pin 22." ]
    pub fn pin22(&mut self, value: DirsetWPin22) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Set as output pin 23." ]
    pub fn pin23(&mut self, value: DirsetWPin23) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 24 - Set as output pin 24." ]
    pub fn pin24(&mut self, value: DirsetWPin24) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 25 - Set as output pin 25." ]
    pub fn pin25(&mut self, value: DirsetWPin25) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Set as output pin 26." ]
    pub fn pin26(&mut self, value: DirsetWPin26) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 27 - Set as output pin 27." ]
    pub fn pin27(&mut self, value: DirsetWPin27) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 28 - Set as output pin 28." ]
    pub fn pin28(&mut self, value: DirsetWPin28) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Set as output pin 29." ]
    pub fn pin29(&mut self, value: DirsetWPin29) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 30 - Set as output pin 30." ]
    pub fn pin30(&mut self, value: DirsetWPin30) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 31 - Set as output pin 31." ]
    pub fn pin31(&mut self, value: DirsetWPin31) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin0 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin0 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin0::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin1 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin1 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin1::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin2 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin2 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin2::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin3 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin3 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin3::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin4 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin4 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin4::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin5 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin5 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin5::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin6 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin6 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin6::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin7 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin7 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin7::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin8 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin8 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin8::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin9 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin9 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin9::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin10 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin10 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin10::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin11 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin11 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin11::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin12 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin12 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin12::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin13 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin13 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin13::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin14 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin14 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin14::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin15 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin15 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin15::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin16 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin16 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin16::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin17 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin17 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin17::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin18 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin18 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin18::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin19 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin19 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin19::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin20 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin20 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin20::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin21 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin21 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin21::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin22 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin22 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin22::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin23 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin23 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin23::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin24 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin24 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin24::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin25 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin25 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin25::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin26 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin26 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin26::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin27 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin27 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin27::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin28 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin28 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin28::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin29 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin29 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin29::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin30 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin30 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin30::Clear => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum DirclrWPin31 {
    # [ doc = "Set pin as input." ]
    Clear,
}
impl DirclrWPin31 {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            DirclrWPin31::Clear => 1u32,
        }
    }
}

impl DirclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DirclrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Set as input pin 0." ]
    pub fn pin0(&mut self, value: DirclrWPin0) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 1 - Set as input pin 1." ]
    pub fn pin1(&mut self, value: DirclrWPin1) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Set as input pin 2." ]
    pub fn pin2(&mut self, value: DirclrWPin2) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 2u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Set as input pin 3." ]
    pub fn pin3(&mut self, value: DirclrWPin3) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 3u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Set as input pin 4." ]
    pub fn pin4(&mut self, value: DirclrWPin4) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 4u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Set as input pin 5." ]
    pub fn pin5(&mut self, value: DirclrWPin5) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 5u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Set as input pin 6." ]
    pub fn pin6(&mut self, value: DirclrWPin6) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 6u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Set as input pin 7." ]
    pub fn pin7(&mut self, value: DirclrWPin7) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 7u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 8 - Set as input pin 8." ]
    pub fn pin8(&mut self, value: DirclrWPin8) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 8u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 9 - Set as input pin 9." ]
    pub fn pin9(&mut self, value: DirclrWPin9) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 9u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Set as input pin 10." ]
    pub fn pin10(&mut self, value: DirclrWPin10) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 10u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Set as input pin 11." ]
    pub fn pin11(&mut self, value: DirclrWPin11) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 11u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 12 - Set as input pin 12." ]
    pub fn pin12(&mut self, value: DirclrWPin12) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 12u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 13 - Set as input pin 13." ]
    pub fn pin13(&mut self, value: DirclrWPin13) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 13u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Set as input pin 14." ]
    pub fn pin14(&mut self, value: DirclrWPin14) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 14u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Set as input pin 15." ]
    pub fn pin15(&mut self, value: DirclrWPin15) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 15u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 16 - Set as input pin 16." ]
    pub fn pin16(&mut self, value: DirclrWPin16) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 16u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 17 - Set as input pin 17." ]
    pub fn pin17(&mut self, value: DirclrWPin17) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 17u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 18 - Set as input pin 18." ]
    pub fn pin18(&mut self, value: DirclrWPin18) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 18u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Set as input pin 19." ]
    pub fn pin19(&mut self, value: DirclrWPin19) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 19u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 20 - Set as input pin 20." ]
    pub fn pin20(&mut self, value: DirclrWPin20) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 20u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Set as input pin 21." ]
    pub fn pin21(&mut self, value: DirclrWPin21) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 21u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Set as input pin 22." ]
    pub fn pin22(&mut self, value: DirclrWPin22) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 22u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Set as input pin 23." ]
    pub fn pin23(&mut self, value: DirclrWPin23) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 23u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 24 - Set as input pin 24." ]
    pub fn pin24(&mut self, value: DirclrWPin24) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 24u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 25 - Set as input pin 25." ]
    pub fn pin25(&mut self, value: DirclrWPin25) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 25u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Set as input pin 26." ]
    pub fn pin26(&mut self, value: DirclrWPin26) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 26u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 27 - Set as input pin 27." ]
    pub fn pin27(&mut self, value: DirclrWPin27) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 27u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 28 - Set as input pin 28." ]
    pub fn pin28(&mut self, value: DirclrWPin28) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 28u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Set as input pin 29." ]
    pub fn pin29(&mut self, value: DirclrWPin29) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 29u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 30 - Set as input pin 30." ]
    pub fn pin30(&mut self, value: DirclrWPin30) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 30u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 31 - Set as input pin 31." ]
    pub fn pin31(&mut self, value: DirclrWPin31) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 31u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
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

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfWDir {
    # [ doc = "Configure pin as an input pin." ]
    Input,
    # [ doc = "Configure pin as an output pin." ]
    Output,
}
impl PinCnfWDir {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            PinCnfWDir::Input => 0u32,
            PinCnfWDir::Output => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfWInput {
    # [ doc = "Connect input pin." ]
    Connect,
    # [ doc = "Disconnect input pin." ]
    Disconnect,
}
impl PinCnfWInput {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            PinCnfWInput::Connect => 0u32,
            PinCnfWInput::Disconnect => 1u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfWPull {
    # [ doc = "No pull." ]
    Disabled,
    # [ doc = "Pulldown on pin." ]
    Pulldown,
    # [ doc = "Pullup on pin." ]
    Pullup,
}
impl PinCnfWPull {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            PinCnfWPull::Disabled => 0u32,
            PinCnfWPull::Pulldown => 1u32,
            PinCnfWPull::Pullup => 3u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfWDrive {
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
impl PinCnfWDrive {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            PinCnfWDrive::S0S1 => 0u32,
            PinCnfWDrive::H0S1 => 1u32,
            PinCnfWDrive::S0H1 => 2u32,
            PinCnfWDrive::H0H1 => 3u32,
            PinCnfWDrive::D0S1 => 4u32,
            PinCnfWDrive::D0H1 => 5u32,
            PinCnfWDrive::S0D1 => 6u32,
            PinCnfWDrive::H0D1 => 7u32,
        }
    }
}

# [ derive ( Clone , Copy , Eq , PartialEq ) ]
pub enum PinCnfWSense {
    # [ doc = "Disabled." ]
    Disabled,
    # [ doc = "Wakeup on high level." ]
    High,
    # [ doc = "Wakeup on low level." ]
    Low,
}
impl PinCnfWSense {
    # [ inline ( always ) ]
    fn bits(&self) -> u32 {
        match *self {
            PinCnfWSense::Disabled => 0u32,
            PinCnfWSense::High => 2u32,
            PinCnfWSense::Low => 3u32,
        }
    }
}

impl PinCnfW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PinCnfW { bits: 2 }
    }
    # [ doc = "Bit 0 - Pin direction." ]
    pub fn dir(&mut self, value: PinCnfWDir) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 0u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bit 1 - Connect or disconnect input path." ]
    pub fn input(&mut self, value: PinCnfWInput) -> &mut Self {
        const MASK: u32 = 1;
        const OFFSET: u8 = 1u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - Pull-up or -down configuration." ]
    pub fn pull(&mut self, value: PinCnfWPull) -> &mut Self {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - Drive configuration." ]
    pub fn drive(&mut self, value: PinCnfWDrive) -> &mut Self {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
    # [ doc = "Bits 16:17 - Pin sensing mechanism." ]
    pub fn sense(&mut self, value: PinCnfWSense) -> &mut Self {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= value.bits() << OFFSET;
        self
    }
}
