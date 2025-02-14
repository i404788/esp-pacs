#[doc = "Register `REDCY_SIG1` reader"]
pub struct R(crate::R<REDCY_SIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REDCY_SIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REDCY_SIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REDCY_SIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REDCY_SIG1` writer"]
pub struct W(crate::W<REDCY_SIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REDCY_SIG1_SPEC>;
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
impl From<crate::W<REDCY_SIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REDCY_SIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDCY_SIG1` reader - ******* Description ***********"]
pub type REDCY_SIG1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REDCY_SIG1` writer - ******* Description ***********"]
pub type REDCY_SIG1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REDCY_SIG1_SPEC, u32, u32, 31, O>;
#[doc = "Field `REDCY_NANDOR` reader - ******* Description ***********"]
pub type REDCY_NANDOR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:30 - ******* Description ***********"]
    #[inline(always)]
    pub fn redcy_sig1(&self) -> REDCY_SIG1_R {
        REDCY_SIG1_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - ******* Description ***********"]
    #[inline(always)]
    pub fn redcy_nandor(&self) -> REDCY_NANDOR_R {
        REDCY_NANDOR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - ******* Description ***********"]
    #[inline(always)]
    pub fn redcy_sig1(&mut self) -> REDCY_SIG1_W<0> {
        REDCY_SIG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redcy_sig1](index.html) module"]
pub struct REDCY_SIG1_SPEC;
impl crate::RegisterSpec for REDCY_SIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [redcy_sig1::R](R) reader structure"]
impl crate::Readable for REDCY_SIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [redcy_sig1::W](W) writer structure"]
impl crate::Writable for REDCY_SIG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REDCY_SIG1 to value 0"]
impl crate::Resettable for REDCY_SIG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
