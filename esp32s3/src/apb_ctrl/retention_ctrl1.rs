#[doc = "Register `RETENTION_CTRL1` reader"]
pub struct R(crate::R<RETENTION_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CTRL1` writer"]
pub struct W(crate::W<RETENTION_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CTRL1_SPEC>;
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
impl From<crate::W<RETENTION_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_TAG_LINK_ADDR` reader - ******* Description ***********"]
pub type RETENTION_TAG_LINK_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RETENTION_TAG_LINK_ADDR` writer - ******* Description ***********"]
pub type RETENTION_TAG_LINK_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RETENTION_CTRL1_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_tag_link_addr(&self) -> RETENTION_TAG_LINK_ADDR_R {
        RETENTION_TAG_LINK_ADDR_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_tag_link_addr(&mut self) -> RETENTION_TAG_LINK_ADDR_W<0> {
        RETENTION_TAG_LINK_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl1](index.html) module"]
pub struct RETENTION_CTRL1_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_ctrl1::R](R) reader structure"]
impl crate::Readable for RETENTION_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_ctrl1::W](W) writer structure"]
impl crate::Writable for RETENTION_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETENTION_CTRL1 to value 0"]
impl crate::Resettable for RETENTION_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
