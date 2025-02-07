#[doc = "Register `PRO_DCACHE_MEM_SYNC0` reader"]
pub struct R(crate::R<PRO_DCACHE_MEM_SYNC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_MEM_SYNC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_MEM_SYNC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_MEM_SYNC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DCACHE_MEM_SYNC0` writer"]
pub struct W(crate::W<PRO_DCACHE_MEM_SYNC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DCACHE_MEM_SYNC0_SPEC>;
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
impl From<crate::W<PRO_DCACHE_MEM_SYNC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DCACHE_MEM_SYNC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DCACHE_MEMSYNC_ADDR` reader - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC1."]
pub type PRO_DCACHE_MEMSYNC_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_DCACHE_MEMSYNC_ADDR` writer - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC1."]
pub type PRO_DCACHE_MEMSYNC_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_DCACHE_MEM_SYNC0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC1."]
    #[inline(always)]
    pub fn pro_dcache_memsync_addr(&self) -> PRO_DCACHE_MEMSYNC_ADDR_R {
        PRO_DCACHE_MEMSYNC_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC1."]
    #[inline(always)]
    pub fn pro_dcache_memsync_addr(&mut self) -> PRO_DCACHE_MEMSYNC_ADDR_W<0> {
        PRO_DCACHE_MEMSYNC_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_mem_sync0](index.html) module"]
pub struct PRO_DCACHE_MEM_SYNC0_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_MEM_SYNC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_mem_sync0::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_MEM_SYNC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dcache_mem_sync0::W](W) writer structure"]
impl crate::Writable for PRO_DCACHE_MEM_SYNC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DCACHE_MEM_SYNC0 to value 0"]
impl crate::Resettable for PRO_DCACHE_MEM_SYNC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
