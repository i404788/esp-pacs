#[doc = "Register `CACHE_MMU_ACCESS_1` reader"]
pub struct R(crate::R<CACHE_MMU_ACCESS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_ACCESS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_ACCESS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_ACCESS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_MMU_ACCESS_1` writer"]
pub struct W(crate::W<CACHE_MMU_ACCESS_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_MMU_ACCESS_1_SPEC>;
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
impl From<crate::W<CACHE_MMU_ACCESS_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_MMU_ACCESS_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_MMU_RD_ACS` reader - Need add description"]
pub type PRO_MMU_RD_ACS_R = crate::BitReader<bool>;
#[doc = "Field `PRO_MMU_RD_ACS` writer - Need add description"]
pub type PRO_MMU_RD_ACS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_MMU_ACCESS_1_SPEC, bool, O>;
#[doc = "Field `PRO_MMU_WR_ACS` reader - Need add description"]
pub type PRO_MMU_WR_ACS_R = crate::BitReader<bool>;
#[doc = "Field `PRO_MMU_WR_ACS` writer - Need add description"]
pub type PRO_MMU_WR_ACS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_MMU_ACCESS_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pro_mmu_rd_acs(&self) -> PRO_MMU_RD_ACS_R {
        PRO_MMU_RD_ACS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Need add description"]
    #[inline(always)]
    pub fn pro_mmu_wr_acs(&self) -> PRO_MMU_WR_ACS_R {
        PRO_MMU_WR_ACS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pro_mmu_rd_acs(&mut self) -> PRO_MMU_RD_ACS_W<0> {
        PRO_MMU_RD_ACS_W::new(self)
    }
    #[doc = "Bit 1 - Need add description"]
    #[inline(always)]
    pub fn pro_mmu_wr_acs(&mut self) -> PRO_MMU_WR_ACS_W<1> {
        PRO_MMU_WR_ACS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_access_1](index.html) module"]
pub struct CACHE_MMU_ACCESS_1_SPEC;
impl crate::RegisterSpec for CACHE_MMU_ACCESS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_access_1::R](R) reader structure"]
impl crate::Readable for CACHE_MMU_ACCESS_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_mmu_access_1::W](W) writer structure"]
impl crate::Writable for CACHE_MMU_ACCESS_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_MMU_ACCESS_1 to value 0x03"]
impl crate::Resettable for CACHE_MMU_ACCESS_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
