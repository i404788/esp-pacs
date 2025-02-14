#[doc = "Register `ICACHE_AUTOLOAD_SCT0_SIZE` reader"]
pub struct R(crate::R<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_AUTOLOAD_SCT0_SIZE` writer"]
pub struct W(crate::W<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
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
impl From<crate::W<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_SCT0_SIZE` reader - The bits are used to configure the length of the first section for autoload operation. It should be combined with icache_autoload_sct0_ena."]
pub type ICACHE_AUTOLOAD_SCT0_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ICACHE_AUTOLOAD_SCT0_SIZE` writer - The bits are used to configure the length of the first section for autoload operation. It should be combined with icache_autoload_sct0_ena."]
pub type ICACHE_AUTOLOAD_SCT0_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ICACHE_AUTOLOAD_SCT0_SIZE_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:26 - The bits are used to configure the length of the first section for autoload operation. It should be combined with icache_autoload_sct0_ena."]
    #[inline(always)]
    pub fn icache_autoload_sct0_size(&self) -> ICACHE_AUTOLOAD_SCT0_SIZE_R {
        ICACHE_AUTOLOAD_SCT0_SIZE_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:26 - The bits are used to configure the length of the first section for autoload operation. It should be combined with icache_autoload_sct0_ena."]
    #[inline(always)]
    pub fn icache_autoload_sct0_size(&mut self) -> ICACHE_AUTOLOAD_SCT0_SIZE_W<0> {
        ICACHE_AUTOLOAD_SCT0_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_autoload_sct0_size](index.html) module"]
pub struct ICACHE_AUTOLOAD_SCT0_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_autoload_sct0_size::R](R) reader structure"]
impl crate::Readable for ICACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_autoload_sct0_size::W](W) writer structure"]
impl crate::Writable for ICACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_AUTOLOAD_SCT0_SIZE to value 0"]
impl crate::Resettable for ICACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
