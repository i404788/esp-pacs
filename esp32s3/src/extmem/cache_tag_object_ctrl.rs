#[doc = "Register `CACHE_TAG_OBJECT_CTRL` reader"]
pub struct R(crate::R<CACHE_TAG_OBJECT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_TAG_OBJECT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_TAG_OBJECT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_TAG_OBJECT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_TAG_OBJECT_CTRL` writer"]
pub struct W(crate::W<CACHE_TAG_OBJECT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_TAG_OBJECT_CTRL_SPEC>;
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
impl From<crate::W<CACHE_TAG_OBJECT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_TAG_OBJECT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_TAG_OBJECT` reader - Set this bit to set icache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE_TAG_OBJECT_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_TAG_OBJECT` writer - Set this bit to set icache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE_TAG_OBJECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_TAG_OBJECT_CTRL_SPEC, bool, O>;
#[doc = "Field `DCACHE_TAG_OBJECT` reader - Set this bit to set dcache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type DCACHE_TAG_OBJECT_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_TAG_OBJECT` writer - Set this bit to set dcache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type DCACHE_TAG_OBJECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_TAG_OBJECT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to set icache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache_tag_object(&self) -> ICACHE_TAG_OBJECT_R {
        ICACHE_TAG_OBJECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to set dcache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn dcache_tag_object(&self) -> DCACHE_TAG_OBJECT_R {
        DCACHE_TAG_OBJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to set icache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache_tag_object(&mut self) -> ICACHE_TAG_OBJECT_W<0> {
        ICACHE_TAG_OBJECT_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to set dcache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn dcache_tag_object(&mut self) -> DCACHE_TAG_OBJECT_W<1> {
        DCACHE_TAG_OBJECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_tag_object_ctrl](index.html) module"]
pub struct CACHE_TAG_OBJECT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_TAG_OBJECT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_tag_object_ctrl::R](R) reader structure"]
impl crate::Readable for CACHE_TAG_OBJECT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_tag_object_ctrl::W](W) writer structure"]
impl crate::Writable for CACHE_TAG_OBJECT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_TAG_OBJECT_CTRL to value 0"]
impl crate::Resettable for CACHE_TAG_OBJECT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
