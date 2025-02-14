#[doc = "Register `CONFIG4` reader"]
pub struct R(crate::R<CONFIG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG4` writer"]
pub struct W(crate::W<CONFIG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG4_SPEC>;
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
impl From<crate::W<CONFIG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_STG3_HOLD` reader - need_des"]
pub type WDT_STG3_HOLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDT_STG3_HOLD` writer - need_des"]
pub type WDT_STG3_HOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg3_hold(&self) -> WDT_STG3_HOLD_R {
        WDT_STG3_HOLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3_hold(&mut self) -> WDT_STG3_HOLD_W<0> {
        WDT_STG3_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config4](index.html) module"]
pub struct CONFIG4_SPEC;
impl crate::RegisterSpec for CONFIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config4::R](R) reader structure"]
impl crate::Readable for CONFIG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config4::W](W) writer structure"]
impl crate::Writable for CONFIG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG4 to value 0x0fff"]
impl crate::Resettable for CONFIG4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
