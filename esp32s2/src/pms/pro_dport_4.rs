#[doc = "Register `PRO_DPORT_4` reader"]
pub struct R(crate::R<PRO_DPORT_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DPORT_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DPORT_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DPORT_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DPORT_4` writer"]
pub struct W(crate::W<PRO_DPORT_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DPORT_4_SPEC>;
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
impl From<crate::W<PRO_DPORT_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DPORT_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_2` reader - Configure read-protection address 2."]
pub type PRO_DPORT_RESERVE_FIFO_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_2` writer - Configure read-protection address 2."]
pub type PRO_DPORT_RESERVE_FIFO_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_DPORT_4_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - Configure read-protection address 2."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_2(&self) -> PRO_DPORT_RESERVE_FIFO_2_R {
        PRO_DPORT_RESERVE_FIFO_2_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure read-protection address 2."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_2(&mut self) -> PRO_DPORT_RESERVE_FIFO_2_W<0> {
        PRO_DPORT_RESERVE_FIFO_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeriBus1 permission control register 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dport_4](index.html) module"]
pub struct PRO_DPORT_4_SPEC;
impl crate::RegisterSpec for PRO_DPORT_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dport_4::R](R) reader structure"]
impl crate::Readable for PRO_DPORT_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dport_4::W](W) writer structure"]
impl crate::Writable for PRO_DPORT_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DPORT_4 to value 0"]
impl crate::Resettable for PRO_DPORT_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}