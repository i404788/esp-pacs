#[doc = "Register `DOEPTSIZ5` reader"]
pub struct R(crate::R<DOEPTSIZ5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPTSIZ5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPTSIZ5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPTSIZ5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPTSIZ5` writer"]
pub struct W(crate::W<DOEPTSIZ5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPTSIZ5_SPEC>;
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
impl From<crate::W<DOEPTSIZ5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPTSIZ5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE5` reader - "]
pub type XFERSIZE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XFERSIZE5` writer - "]
pub type XFERSIZE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ5_SPEC, u8, u8, 7, O>;
#[doc = "Field `PKTCNT5` reader - "]
pub type PKTCNT5_R = crate::BitReader<bool>;
#[doc = "Field `PKTCNT5` writer - "]
pub type PKTCNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPTSIZ5_SPEC, bool, O>;
#[doc = "Field `SUPCNT5` reader - "]
pub type SUPCNT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUPCNT5` writer - "]
pub type SUPCNT5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ5_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize5(&self) -> XFERSIZE5_R {
        XFERSIZE5_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt5(&self) -> PKTCNT5_R {
        PKTCNT5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt5(&self) -> SUPCNT5_R {
        SUPCNT5_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize5(&mut self) -> XFERSIZE5_W<0> {
        XFERSIZE5_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt5(&mut self) -> PKTCNT5_W<19> {
        PKTCNT5_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt5(&mut self) -> SUPCNT5_W<29> {
        SUPCNT5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz5](index.html) module"]
pub struct DOEPTSIZ5_SPEC;
impl crate::RegisterSpec for DOEPTSIZ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doeptsiz5::R](R) reader structure"]
impl crate::Readable for DOEPTSIZ5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doeptsiz5::W](W) writer structure"]
impl crate::Writable for DOEPTSIZ5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPTSIZ5 to value 0"]
impl crate::Resettable for DOEPTSIZ5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
