#[doc = "Register `CH7CARRIER_DUTY` reader"]
pub struct R(crate::R<CH7CARRIER_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH7CARRIER_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH7CARRIER_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH7CARRIER_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH7CARRIER_DUTY` writer"]
pub struct W(crate::W<CH7CARRIER_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH7CARRIER_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CH7CARRIER_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH7CARRIER_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARRIER_LOW` reader - This register is used to configure carrier wave's low level value for channel7."]
pub type CARRIER_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CARRIER_LOW` writer - This register is used to configure carrier wave's low level value for channel7."]
pub type CARRIER_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH7CARRIER_DUTY_SPEC, u16, u16, 16, O>;
#[doc = "Field `CARRIER_HIGH` reader - This register is used to configure carrier wave's high level value for channel7."]
pub type CARRIER_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CARRIER_HIGH` writer - This register is used to configure carrier wave's high level value for channel7."]
pub type CARRIER_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH7CARRIER_DUTY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure carrier wave's low level value for channel7."]
    #[inline(always)]
    pub fn carrier_low(&self) -> CARRIER_LOW_R {
        CARRIER_LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure carrier wave's high level value for channel7."]
    #[inline(always)]
    pub fn carrier_high(&self) -> CARRIER_HIGH_R {
        CARRIER_HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure carrier wave's low level value for channel7."]
    #[inline(always)]
    pub fn carrier_low(&mut self) -> CARRIER_LOW_W<0> {
        CARRIER_LOW_W::new(self)
    }
    #[doc = "Bits 16:31 - This register is used to configure carrier wave's high level value for channel7."]
    #[inline(always)]
    pub fn carrier_high(&mut self) -> CARRIER_HIGH_W<16> {
        CARRIER_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7carrier_duty](index.html) module"]
pub struct CH7CARRIER_DUTY_SPEC;
impl crate::RegisterSpec for CH7CARRIER_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch7carrier_duty::R](R) reader structure"]
impl crate::Readable for CH7CARRIER_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch7carrier_duty::W](W) writer structure"]
impl crate::Writable for CH7CARRIER_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH7CARRIER_DUTY to value 0x0040_0040"]
impl crate::Resettable for CH7CARRIER_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0040
    }
}
