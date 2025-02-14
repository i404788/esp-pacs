#[doc = "Register `FUNC%s_IN_SEL_CFG` reader"]
pub struct R(crate::R<FUNC_IN_SEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_IN_SEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_IN_SEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_IN_SEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC%s_IN_SEL_CFG` writer"]
pub struct W(crate::W<FUNC_IN_SEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_IN_SEL_CFG_SPEC>;
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
impl From<crate::W<FUNC_IN_SEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_IN_SEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_SEL` reader - set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
pub type IN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN_SEL` writer - set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
pub type IN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FUNC_IN_SEL_CFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type IN_INV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type IN_INV_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNC_IN_SEL_CFG_SPEC, bool, O>;
#[doc = "Field `SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SEL_R = crate::BitReader<bool>;
#[doc = "Field `SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNC_IN_SEL_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
    #[inline(always)]
    pub fn in_sel(&self) -> IN_SEL_R {
        IN_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn in_inv_sel(&self) -> IN_INV_SEL_R {
        IN_INV_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
    #[inline(always)]
    pub fn in_sel(&mut self) -> IN_SEL_W<0> {
        IN_SEL_W::new(self)
    }
    #[doc = "Bit 5 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn in_inv_sel(&mut self) -> IN_INV_SEL_W<5> {
        IN_INV_SEL_W::new(self)
    }
    #[doc = "Bit 6 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<6> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO input function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_in_sel_cfg](index.html) module"]
pub struct FUNC_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_in_sel_cfg::R](R) reader structure"]
impl crate::Readable for FUNC_IN_SEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_in_sel_cfg::W](W) writer structure"]
impl crate::Writable for FUNC_IN_SEL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC%s_IN_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_IN_SEL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
