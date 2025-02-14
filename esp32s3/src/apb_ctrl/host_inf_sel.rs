#[doc = "Register `HOST_INF_SEL` reader"]
pub struct R(crate::R<HOST_INF_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_INF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_INF_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_INF_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_INF_SEL` writer"]
pub struct W(crate::W<HOST_INF_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_INF_SEL_SPEC>;
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
impl From<crate::W<HOST_INF_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_INF_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_IO_SWAP` reader - ******* Description ***********"]
pub type PERI_IO_SWAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERI_IO_SWAP` writer - ******* Description ***********"]
pub type PERI_IO_SWAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_INF_SEL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ******* Description ***********"]
    #[inline(always)]
    pub fn peri_io_swap(&self) -> PERI_IO_SWAP_R {
        PERI_IO_SWAP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ******* Description ***********"]
    #[inline(always)]
    pub fn peri_io_swap(&mut self) -> PERI_IO_SWAP_W<0> {
        PERI_IO_SWAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_inf_sel](index.html) module"]
pub struct HOST_INF_SEL_SPEC;
impl crate::RegisterSpec for HOST_INF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_inf_sel::R](R) reader structure"]
impl crate::Readable for HOST_INF_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_inf_sel::W](W) writer structure"]
impl crate::Writable for HOST_INF_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_INF_SEL to value 0"]
impl crate::Resettable for HOST_INF_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
