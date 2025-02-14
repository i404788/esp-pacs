#[doc = "Register `TIMER0_STATUS` reader"]
pub struct R(crate::R<TIMER0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER0_VALUE` reader - current PWM timer0 counter value"]
pub type TIMER0_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER0_DIRECTION` reader - current PWM timer0 counter direction, 0: increment 1: decrement"]
pub type TIMER0_DIRECTION_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - current PWM timer0 counter value"]
    #[inline(always)]
    pub fn timer0_value(&self) -> TIMER0_VALUE_R {
        TIMER0_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - current PWM timer0 counter direction, 0: increment 1: decrement"]
    #[inline(always)]
    pub fn timer0_direction(&self) -> TIMER0_DIRECTION_R {
        TIMER0_DIRECTION_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "PWM timer0 status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_status](index.html) module"]
pub struct TIMER0_STATUS_SPEC;
impl crate::RegisterSpec for TIMER0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_status::R](R) reader structure"]
impl crate::Readable for TIMER0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER0_STATUS to value 0"]
impl crate::Resettable for TIMER0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
