#[doc = "Register `MOSI_DLEN` reader"]
pub struct R(crate::R<MOSI_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSI_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSI_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSI_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSI_DLEN` writer"]
pub struct W(crate::W<MOSI_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSI_DLEN_SPEC>;
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
impl From<crate::W<MOSI_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSI_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_MOSI_DBITLEN` reader - The length in bits of write-data. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type USR_MOSI_DBITLEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USR_MOSI_DBITLEN` writer - The length in bits of write-data. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type USR_MOSI_DBITLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MOSI_DLEN_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 0:22 - The length in bits of write-data. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_mosi_dbitlen(&self) -> USR_MOSI_DBITLEN_R {
        USR_MOSI_DBITLEN_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:22 - The length in bits of write-data. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_mosi_dbitlen(&mut self) -> USR_MOSI_DBITLEN_W<0> {
        USR_MOSI_DBITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MOSI length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosi_dlen](index.html) module"]
pub struct MOSI_DLEN_SPEC;
impl crate::RegisterSpec for MOSI_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mosi_dlen::R](R) reader structure"]
impl crate::Readable for MOSI_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mosi_dlen::W](W) writer structure"]
impl crate::Writable for MOSI_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOSI_DLEN to value 0"]
impl crate::Resettable for MOSI_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
