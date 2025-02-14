#[doc = "Register `ENABLE1_W1TS` writer"]
pub struct W(crate::W<ENABLE1_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE1_W1TS_SPEC>;
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
impl From<crate::W<ENABLE1_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE1_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE1_W1TS` writer - GPIO32 ~ 53 output enable set register. If the value 1 is written to a bit here, the corresponding bit in GPIO_ENABLE1_REG will be set to 1. Recommended operation: use this register to set GPIO_ENABLE1_REG."]
pub type ENABLE1_W1TS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENABLE1_W1TS_SPEC, u32, u32, 22, O>;
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 output enable set register. If the value 1 is written to a bit here, the corresponding bit in GPIO_ENABLE1_REG will be set to 1. Recommended operation: use this register to set GPIO_ENABLE1_REG."]
    #[inline(always)]
    pub fn enable1_w1ts(&mut self) -> ENABLE1_W1TS_W<0> {
        ENABLE1_W1TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO32 ~ 53 output enable bit set register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable1_w1ts](index.html) module"]
pub struct ENABLE1_W1TS_SPEC;
impl crate::RegisterSpec for ENABLE1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [enable1_w1ts::W](W) writer structure"]
impl crate::Writable for ENABLE1_W1TS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENABLE1_W1TS to value 0"]
impl crate::Resettable for ENABLE1_W1TS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
