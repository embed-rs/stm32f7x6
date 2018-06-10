#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMPR2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SMPx_x`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPX_XR {
    #[doc = "3 cycles"]
    CYCLES3,
    #[doc = "15 cycles"]
    CYCLES15,
    #[doc = "28 cycles"]
    CYCLES28,
    #[doc = "56 cycles"]
    CYCLES56,
    #[doc = "84 cycles"]
    CYCLES84,
    #[doc = "112 cycles"]
    CYCLES112,
    #[doc = "144 cycles"]
    CYCLES144,
    #[doc = "480 cycles"]
    CYCLES480,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl SMPX_XR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            SMPX_XR::CYCLES3 => 0,
            SMPX_XR::CYCLES15 => 1,
            SMPX_XR::CYCLES28 => 2,
            SMPX_XR::CYCLES56 => 3,
            SMPX_XR::CYCLES84 => 4,
            SMPX_XR::CYCLES112 => 5,
            SMPX_XR::CYCLES144 => 6,
            SMPX_XR::CYCLES480 => 7,
            SMPX_XR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> SMPX_XR {
        match value {
            0 => SMPX_XR::CYCLES3,
            1 => SMPX_XR::CYCLES15,
            2 => SMPX_XR::CYCLES28,
            3 => SMPX_XR::CYCLES56,
            4 => SMPX_XR::CYCLES84,
            5 => SMPX_XR::CYCLES112,
            6 => SMPX_XR::CYCLES144,
            7 => SMPX_XR::CYCLES480,
            i => SMPX_XR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES3`"]
    #[inline]
    pub fn is_cycles3(&self) -> bool {
        *self == SMPX_XR::CYCLES3
    }
    #[doc = "Checks if the value of the field is `CYCLES15`"]
    #[inline]
    pub fn is_cycles15(&self) -> bool {
        *self == SMPX_XR::CYCLES15
    }
    #[doc = "Checks if the value of the field is `CYCLES28`"]
    #[inline]
    pub fn is_cycles28(&self) -> bool {
        *self == SMPX_XR::CYCLES28
    }
    #[doc = "Checks if the value of the field is `CYCLES56`"]
    #[inline]
    pub fn is_cycles56(&self) -> bool {
        *self == SMPX_XR::CYCLES56
    }
    #[doc = "Checks if the value of the field is `CYCLES84`"]
    #[inline]
    pub fn is_cycles84(&self) -> bool {
        *self == SMPX_XR::CYCLES84
    }
    #[doc = "Checks if the value of the field is `CYCLES112`"]
    #[inline]
    pub fn is_cycles112(&self) -> bool {
        *self == SMPX_XR::CYCLES112
    }
    #[doc = "Checks if the value of the field is `CYCLES144`"]
    #[inline]
    pub fn is_cycles144(&self) -> bool {
        *self == SMPX_XR::CYCLES144
    }
    #[doc = "Checks if the value of the field is `CYCLES480`"]
    #[inline]
    pub fn is_cycles480(&self) -> bool {
        *self == SMPX_XR::CYCLES480
    }
}
#[doc = "Values that can be written to the field `SMPx_x`"]
pub enum SMPX_XW {
    #[doc = "3 cycles"]
    CYCLES3,
    #[doc = "15 cycles"]
    CYCLES15,
    #[doc = "28 cycles"]
    CYCLES28,
    #[doc = "56 cycles"]
    CYCLES56,
    #[doc = "84 cycles"]
    CYCLES84,
    #[doc = "112 cycles"]
    CYCLES112,
    #[doc = "144 cycles"]
    CYCLES144,
    #[doc = "480 cycles"]
    CYCLES480,
}
impl SMPX_XW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            SMPX_XW::CYCLES3 => 0,
            SMPX_XW::CYCLES15 => 1,
            SMPX_XW::CYCLES28 => 2,
            SMPX_XW::CYCLES56 => 3,
            SMPX_XW::CYCLES84 => 4,
            SMPX_XW::CYCLES112 => 5,
            SMPX_XW::CYCLES144 => 6,
            SMPX_XW::CYCLES480 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPX_XW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPX_XW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPX_XW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "3 cycles"]
    #[inline]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMPX_XW::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMPX_XW::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMPX_XW::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMPX_XW::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMPX_XW::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMPX_XW::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMPX_XW::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMPX_XW::CYCLES480)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline]
    pub fn smpx_x(&self) -> SMPX_XR {
        SMPX_XR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline]
    pub fn smpx_x(&mut self) -> _SMPX_XW {
        _SMPX_XW { w: self }
    }
}
