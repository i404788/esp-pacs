#[doc = "Register `M_PRIME` reader"]
pub struct R(crate::R<M_PRIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M_PRIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M_PRIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M_PRIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M_PRIME` writer"]
pub struct W(crate::W<M_PRIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M_PRIME_SPEC>;
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
impl From<crate::W<M_PRIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M_PRIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_PRIME` reader - Stores M'"]
pub type M_PRIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `M_PRIME` writer - Stores M'"]
pub type M_PRIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M_PRIME_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Stores M'"]
    #[inline(always)]
    pub fn m_prime(&self) -> M_PRIME_R {
        M_PRIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores M'"]
    #[inline(always)]
    pub fn m_prime(&mut self) -> M_PRIME_W<0> {
        M_PRIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSA M' register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m_prime](index.html) module"]
pub struct M_PRIME_SPEC;
impl crate::RegisterSpec for M_PRIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m_prime::R](R) reader structure"]
impl crate::Readable for M_PRIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m_prime::W](W) writer structure"]
impl crate::Writable for M_PRIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M_PRIME to value 0"]
impl crate::Resettable for M_PRIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
