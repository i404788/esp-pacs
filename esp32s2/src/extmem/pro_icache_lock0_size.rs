#[doc = "Register `PRO_ICACHE_LOCK0_SIZE` reader"]
pub struct R(crate::R<PRO_ICACHE_LOCK0_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_LOCK0_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_LOCK0_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_LOCK0_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_LOCK0_SIZE` writer"]
pub struct W(crate::W<PRO_ICACHE_LOCK0_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_LOCK0_SIZE_SPEC>;
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
impl From<crate::W<PRO_ICACHE_LOCK0_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_LOCK0_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_LOCK0_SIZE` reader - The bits are used to configure the first length of data locking, which is combined with PRO_ICACHE_LOCK0_ADDR_REG"]
pub type PRO_ICACHE_LOCK0_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRO_ICACHE_LOCK0_SIZE` writer - The bits are used to configure the first length of data locking, which is combined with PRO_ICACHE_LOCK0_ADDR_REG"]
pub type PRO_ICACHE_LOCK0_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_ICACHE_LOCK0_SIZE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to configure the first length of data locking, which is combined with PRO_ICACHE_LOCK0_ADDR_REG"]
    #[inline(always)]
    pub fn pro_icache_lock0_size(&self) -> PRO_ICACHE_LOCK0_SIZE_R {
        PRO_ICACHE_LOCK0_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The bits are used to configure the first length of data locking, which is combined with PRO_ICACHE_LOCK0_ADDR_REG"]
    #[inline(always)]
    pub fn pro_icache_lock0_size(&mut self) -> PRO_ICACHE_LOCK0_SIZE_W<0> {
        PRO_ICACHE_LOCK0_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_lock0_size](index.html) module"]
pub struct PRO_ICACHE_LOCK0_SIZE_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_LOCK0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_lock0_size::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_LOCK0_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_lock0_size::W](W) writer structure"]
impl crate::Writable for PRO_ICACHE_LOCK0_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_ICACHE_LOCK0_SIZE to value 0"]
impl crate::Resettable for PRO_ICACHE_LOCK0_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
