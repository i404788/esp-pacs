#[doc = "Register `DIEPTSIZ4` reader"]
pub struct R(crate::R<DIEPTSIZ4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTSIZ4` writer"]
pub struct W(crate::W<DIEPTSIZ4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ4_SPEC>;
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
impl From<crate::W<DIEPTSIZ4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERSIZE4` reader - "]
pub type D_XFERSIZE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D_XFERSIZE4` writer - "]
pub type D_XFERSIZE4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTSIZ4_SPEC, u8, u8, 7, O>;
#[doc = "Field `D_PKTCNT4` reader - "]
pub type D_PKTCNT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D_PKTCNT4` writer - "]
pub type D_PKTCNT4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTSIZ4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize4(&self) -> D_XFERSIZE4_R {
        D_XFERSIZE4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt4(&self) -> D_PKTCNT4_R {
        D_PKTCNT4_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize4(&mut self) -> D_XFERSIZE4_W<0> {
        D_XFERSIZE4_W::new(self)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt4(&mut self) -> D_PKTCNT4_W<19> {
        D_PKTCNT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz4](index.html) module"]
pub struct DIEPTSIZ4_SPEC;
impl crate::RegisterSpec for DIEPTSIZ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptsiz4::R](R) reader structure"]
impl crate::Readable for DIEPTSIZ4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptsiz4::W](W) writer structure"]
impl crate::Writable for DIEPTSIZ4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPTSIZ4 to value 0"]
impl crate::Resettable for DIEPTSIZ4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
