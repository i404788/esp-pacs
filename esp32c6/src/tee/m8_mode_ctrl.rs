#[doc = "Register `M8_MODE_CTRL` reader"]
pub struct R(crate::R<M8_MODE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M8_MODE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M8_MODE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M8_MODE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M8_MODE_CTRL` writer"]
pub struct W(crate::W<M8_MODE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M8_MODE_CTRL_SPEC>;
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
impl From<crate::W<M8_MODE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M8_MODE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M8_MODE` reader - M8 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M8_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M8_MODE` writer - M8 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M8_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M8_MODE_CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - M8 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    pub fn m8_mode(&self) -> M8_MODE_R {
        M8_MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - M8 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    #[must_use]
    pub fn m8_mode(&mut self) -> M8_MODE_W<0> {
        M8_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tee mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m8_mode_ctrl](index.html) module"]
pub struct M8_MODE_CTRL_SPEC;
impl crate::RegisterSpec for M8_MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m8_mode_ctrl::R](R) reader structure"]
impl crate::Readable for M8_MODE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m8_mode_ctrl::W](W) writer structure"]
impl crate::Writable for M8_MODE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M8_MODE_CTRL to value 0x03"]
impl crate::Resettable for M8_MODE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
