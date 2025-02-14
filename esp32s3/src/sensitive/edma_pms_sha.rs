#[doc = "Register `EDMA_PMS_SHA` reader"]
pub struct R(crate::R<EDMA_PMS_SHA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_PMS_SHA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_PMS_SHA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_PMS_SHA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_PMS_SHA` writer"]
pub struct W(crate::W<EDMA_PMS_SHA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_PMS_SHA_SPEC>;
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
impl From<crate::W<EDMA_PMS_SHA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_PMS_SHA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTR1` reader - This field is used to configure the permission of SHA accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR1` writer - This field is used to configure the permission of SHA accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EDMA_PMS_SHA_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATTR2` reader - This field is used to configure the permission of SHA accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR2` writer - This field is used to configure the permission of SHA accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EDMA_PMS_SHA_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - This field is used to configure the permission of SHA accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr1(&self) -> ATTR1_R {
        ATTR1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of SHA accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr2(&self) -> ATTR2_R {
        ATTR2_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to configure the permission of SHA accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr1(&mut self) -> ATTR1_W<0> {
        ATTR1_W::new(self)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of SHA accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr2(&mut self) -> ATTR2_W<2> {
        ATTR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA-SHA permission control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_pms_sha](index.html) module"]
pub struct EDMA_PMS_SHA_SPEC;
impl crate::RegisterSpec for EDMA_PMS_SHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_pms_sha::R](R) reader structure"]
impl crate::Readable for EDMA_PMS_SHA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_pms_sha::W](W) writer structure"]
impl crate::Writable for EDMA_PMS_SHA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EDMA_PMS_SHA to value 0x0f"]
impl crate::Resettable for EDMA_PMS_SHA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
