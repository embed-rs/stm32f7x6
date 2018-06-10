#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HIFCR {
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
#[doc = "Possible values of the field `CTCIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF7R {
    #[doc = "Clear the corresponding TCIFx flag"]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTCIF7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CTCIF7R::CLEAR => true,
            CTCIF7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTCIF7R {
        match value {
            true => CTCIF7R::CLEAR,
            i => CTCIF7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CTCIF7R::CLEAR
    }
}
#[doc = "Possible values of the field `CHTIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHTIF7R {
    #[doc = "Clear the corresponding HTIFx flag"]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CHTIF7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CHTIF7R::CLEAR => true,
            CHTIF7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHTIF7R {
        match value {
            true => CHTIF7R::CLEAR,
            i => CHTIF7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CHTIF7R::CLEAR
    }
}
#[doc = "Possible values of the field `CTEIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF7R {
    #[doc = "Clear the corresponding TEIFx flag"]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTEIF7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CTEIF7R::CLEAR => true,
            CTEIF7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTEIF7R {
        match value {
            true => CTEIF7R::CLEAR,
            i => CTEIF7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CTEIF7R::CLEAR
    }
}
#[doc = "Possible values of the field `CDMEIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDMEIF7R {
    #[doc = "Clear the corresponding DMEIFx flag"]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CDMEIF7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CDMEIF7R::CLEAR => true,
            CDMEIF7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDMEIF7R {
        match value {
            true => CDMEIF7R::CLEAR,
            i => CDMEIF7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CDMEIF7R::CLEAR
    }
}
#[doc = "Possible values of the field `CFEIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFEIF7R {
    #[doc = "Clear the corresponding CFEIFx flag"]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CFEIF7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CFEIF7R::CLEAR => true,
            CFEIF7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFEIF7R {
        match value {
            true => CFEIF7R::CLEAR,
            i => CFEIF7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CFEIF7R::CLEAR
    }
}
#[doc = "Possible values of the field `CTCIF6`"]
pub type CTCIF6R = CTCIF7R;
#[doc = "Possible values of the field `CHTIF6`"]
pub type CHTIF6R = CHTIF7R;
#[doc = "Possible values of the field `CTEIF6`"]
pub type CTEIF6R = CTEIF7R;
#[doc = "Possible values of the field `CDMEIF6`"]
pub type CDMEIF6R = CDMEIF7R;
#[doc = "Possible values of the field `CFEIF6`"]
pub type CFEIF6R = CFEIF7R;
#[doc = "Possible values of the field `CTCIF5`"]
pub type CTCIF5R = CTCIF7R;
#[doc = "Possible values of the field `CHTIF5`"]
pub type CHTIF5R = CHTIF7R;
#[doc = "Possible values of the field `CTEIF5`"]
pub type CTEIF5R = CTEIF7R;
#[doc = "Possible values of the field `CDMEIF5`"]
pub type CDMEIF5R = CDMEIF7R;
#[doc = "Possible values of the field `CFEIF5`"]
pub type CFEIF5R = CFEIF7R;
#[doc = "Possible values of the field `CTCIF4`"]
pub type CTCIF4R = CTCIF7R;
#[doc = "Possible values of the field `CHTIF4`"]
pub type CHTIF4R = CHTIF7R;
#[doc = "Possible values of the field `CTEIF4`"]
pub type CTEIF4R = CTEIF7R;
#[doc = "Possible values of the field `CDMEIF4`"]
pub type CDMEIF4R = CDMEIF7R;
#[doc = "Possible values of the field `CFEIF4`"]
pub type CFEIF4R = CFEIF7R;
#[doc = "Values that can be written to the field `CTCIF7`"]
pub enum CTCIF7W {
    #[doc = "Clear the corresponding TCIFx flag"]
    CLEAR,
}
impl CTCIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTCIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTCIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHTIF7`"]
pub enum CHTIF7W {
    #[doc = "Clear the corresponding HTIFx flag"]
    CLEAR,
}
impl CHTIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHTIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHTIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTEIF7`"]
pub enum CTEIF7W {
    #[doc = "Clear the corresponding TEIFx flag"]
    CLEAR,
}
impl CTEIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTEIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTEIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDMEIF7`"]
pub enum CDMEIF7W {
    #[doc = "Clear the corresponding DMEIFx flag"]
    CLEAR,
}
impl CDMEIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDMEIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDMEIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CDMEIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDMEIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFEIF7`"]
pub enum CFEIF7W {
    #[doc = "Clear the corresponding CFEIFx flag"]
    CLEAR,
}
impl CFEIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFEIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFEIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CFEIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFEIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF6`"]
pub type CTCIF6W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHTIF6`"]
pub type CHTIF6W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTEIF6`"]
pub type CTEIF6W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDMEIF6`"]
pub type CDMEIF6W = CDMEIF7W;
#[doc = r" Proxy"]
pub struct _CDMEIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CDMEIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDMEIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFEIF6`"]
pub type CFEIF6W = CFEIF7W;
#[doc = r" Proxy"]
pub struct _CFEIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CFEIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFEIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF5`"]
pub type CTCIF5W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHTIF5`"]
pub type CHTIF5W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTEIF5`"]
pub type CTEIF5W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDMEIF5`"]
pub type CDMEIF5W = CDMEIF7W;
#[doc = r" Proxy"]
pub struct _CDMEIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CDMEIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDMEIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFEIF5`"]
pub type CFEIF5W = CFEIF7W;
#[doc = r" Proxy"]
pub struct _CFEIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CFEIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFEIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF4`"]
pub type CTCIF4W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHTIF4`"]
pub type CHTIF4W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTEIF4`"]
pub type CTEIF4W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDMEIF4`"]
pub type CDMEIF4W = CDMEIF7W;
#[doc = r" Proxy"]
pub struct _CDMEIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CDMEIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDMEIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFEIF4`"]
pub type CFEIF4W = CFEIF7W;
#[doc = r" Proxy"]
pub struct _CFEIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CFEIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFEIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF7W::CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline]
    pub fn ctcif7(&self) -> CTCIF7R {
        CTCIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline]
    pub fn chtif7(&self) -> CHTIF7R {
        CHTIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cteif7(&self) -> CTEIF7R {
        CTEIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cdmeif7(&self) -> CDMEIF7R {
        CDMEIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cfeif7(&self) -> CFEIF7R {
        CFEIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline]
    pub fn ctcif6(&self) -> CTCIF6R {
        CTCIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline]
    pub fn chtif6(&self) -> CHTIF6R {
        CHTIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cteif6(&self) -> CTEIF6R {
        CTEIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cdmeif6(&self) -> CDMEIF6R {
        CDMEIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cfeif6(&self) -> CFEIF6R {
        CFEIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline]
    pub fn ctcif5(&self) -> CTCIF5R {
        CTCIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline]
    pub fn chtif5(&self) -> CHTIF5R {
        CHTIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cteif5(&self) -> CTEIF5R {
        CTEIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cdmeif5(&self) -> CDMEIF5R {
        CDMEIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cfeif5(&self) -> CFEIF5R {
        CFEIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline]
    pub fn ctcif4(&self) -> CTCIF4R {
        CTCIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline]
    pub fn chtif4(&self) -> CHTIF4R {
        CHTIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cteif4(&self) -> CTEIF4R {
        CTEIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cdmeif4(&self) -> CDMEIF4R {
        CDMEIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cfeif4(&self) -> CFEIF4R {
        CFEIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline]
    pub fn ctcif7(&mut self) -> _CTCIF7W {
        _CTCIF7W { w: self }
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline]
    pub fn chtif7(&mut self) -> _CHTIF7W {
        _CHTIF7W { w: self }
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cteif7(&mut self) -> _CTEIF7W {
        _CTEIF7W { w: self }
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cdmeif7(&mut self) -> _CDMEIF7W {
        _CDMEIF7W { w: self }
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cfeif7(&mut self) -> _CFEIF7W {
        _CFEIF7W { w: self }
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline]
    pub fn ctcif6(&mut self) -> _CTCIF6W {
        _CTCIF6W { w: self }
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline]
    pub fn chtif6(&mut self) -> _CHTIF6W {
        _CHTIF6W { w: self }
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cteif6(&mut self) -> _CTEIF6W {
        _CTEIF6W { w: self }
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cdmeif6(&mut self) -> _CDMEIF6W {
        _CDMEIF6W { w: self }
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cfeif6(&mut self) -> _CFEIF6W {
        _CFEIF6W { w: self }
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline]
    pub fn ctcif5(&mut self) -> _CTCIF5W {
        _CTCIF5W { w: self }
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline]
    pub fn chtif5(&mut self) -> _CHTIF5W {
        _CHTIF5W { w: self }
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cteif5(&mut self) -> _CTEIF5W {
        _CTEIF5W { w: self }
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cdmeif5(&mut self) -> _CDMEIF5W {
        _CDMEIF5W { w: self }
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cfeif5(&mut self) -> _CFEIF5W {
        _CFEIF5W { w: self }
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline]
    pub fn ctcif4(&mut self) -> _CTCIF4W {
        _CTCIF4W { w: self }
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline]
    pub fn chtif4(&mut self) -> _CHTIF4W {
        _CHTIF4W { w: self }
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cteif4(&mut self) -> _CTEIF4W {
        _CTEIF4W { w: self }
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cdmeif4(&mut self) -> _CDMEIF4W {
        _CDMEIF4W { w: self }
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline]
    pub fn cfeif4(&mut self) -> _CFEIF4W {
        _CFEIF4W { w: self }
    }
}
