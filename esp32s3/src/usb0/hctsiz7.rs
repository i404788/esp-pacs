#[doc = "Register `HCTSIZ7` reader"]
pub struct R(crate::R<HCTSIZ7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCTSIZ7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCTSIZ7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCTSIZ7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCTSIZ7` writer"]
pub struct W(crate::W<HCTSIZ7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCTSIZ7_SPEC>;
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
impl From<crate::W<HCTSIZ7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCTSIZ7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERSIZE7` reader - "]
pub type H_XFERSIZE7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `H_XFERSIZE7` writer - "]
pub type H_XFERSIZE7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HCTSIZ7_SPEC, u32, u32, 19, O>;
#[doc = "Field `H_PKTCNT7` reader - "]
pub type H_PKTCNT7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `H_PKTCNT7` writer - "]
pub type H_PKTCNT7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCTSIZ7_SPEC, u16, u16, 10, O>;
#[doc = "Field `H_PID7` reader - "]
pub type H_PID7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H_PID7` writer - "]
pub type H_PID7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCTSIZ7_SPEC, u8, u8, 2, O>;
#[doc = "Field `H_DOPNG7` reader - "]
pub type H_DOPNG7_R = crate::BitReader<bool>;
#[doc = "Field `H_DOPNG7` writer - "]
pub type H_DOPNG7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCTSIZ7_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn h_xfersize7(&self) -> H_XFERSIZE7_R {
        H_XFERSIZE7_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn h_pktcnt7(&self) -> H_PKTCNT7_R {
        H_PKTCNT7_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn h_pid7(&self) -> H_PID7_R {
        H_PID7_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_dopng7(&self) -> H_DOPNG7_R {
        H_DOPNG7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn h_xfersize7(&mut self) -> H_XFERSIZE7_W<0> {
        H_XFERSIZE7_W::new(self)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn h_pktcnt7(&mut self) -> H_PKTCNT7_W<19> {
        H_PKTCNT7_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn h_pid7(&mut self) -> H_PID7_W<29> {
        H_PID7_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_dopng7(&mut self) -> H_DOPNG7_W<31> {
        H_DOPNG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz7](index.html) module"]
pub struct HCTSIZ7_SPEC;
impl crate::RegisterSpec for HCTSIZ7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hctsiz7::R](R) reader structure"]
impl crate::Readable for HCTSIZ7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hctsiz7::W](W) writer structure"]
impl crate::Writable for HCTSIZ7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCTSIZ7 to value 0"]
impl crate::Resettable for HCTSIZ7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
