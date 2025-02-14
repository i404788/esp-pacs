#[doc = "Register `ESC_CONF2` reader"]
pub struct R(crate::R<ESC_CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESC_CONF2` writer"]
pub struct W(crate::W<ESC_CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESC_CONF2_SPEC>;
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
impl From<crate::W<ESC_CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESC_CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESC_SEQ1` reader - This register is used to define a character that need to be encoded. The default value is 0x11 that used as a flow control character."]
pub type ESC_SEQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ESC_SEQ1` writer - This register is used to define a character that need to be encoded. The default value is 0x11 that used as a flow control character."]
pub type ESC_SEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESC_CONF2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ESC_SEQ1_CHAR0` reader - This register is used to define the first character of SLIP escape sequence. The default value is 0xDB."]
pub type ESC_SEQ1_CHAR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ESC_SEQ1_CHAR0` writer - This register is used to define the first character of SLIP escape sequence. The default value is 0xDB."]
pub type ESC_SEQ1_CHAR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESC_CONF2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ESC_SEQ1_CHAR1` reader - This register is used to define the second character of SLIP escape sequence. The default value is 0xDE."]
pub type ESC_SEQ1_CHAR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ESC_SEQ1_CHAR1` writer - This register is used to define the second character of SLIP escape sequence. The default value is 0xDE."]
pub type ESC_SEQ1_CHAR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESC_CONF2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to define a character that need to be encoded. The default value is 0x11 that used as a flow control character."]
    #[inline(always)]
    pub fn esc_seq1(&self) -> ESC_SEQ1_R {
        ESC_SEQ1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register is used to define the first character of SLIP escape sequence. The default value is 0xDB."]
    #[inline(always)]
    pub fn esc_seq1_char0(&self) -> ESC_SEQ1_CHAR0_R {
        ESC_SEQ1_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register is used to define the second character of SLIP escape sequence. The default value is 0xDE."]
    #[inline(always)]
    pub fn esc_seq1_char1(&self) -> ESC_SEQ1_CHAR1_R {
        ESC_SEQ1_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to define a character that need to be encoded. The default value is 0x11 that used as a flow control character."]
    #[inline(always)]
    pub fn esc_seq1(&mut self) -> ESC_SEQ1_W<0> {
        ESC_SEQ1_W::new(self)
    }
    #[doc = "Bits 8:15 - This register is used to define the first character of SLIP escape sequence. The default value is 0xDB."]
    #[inline(always)]
    pub fn esc_seq1_char0(&mut self) -> ESC_SEQ1_CHAR0_W<8> {
        ESC_SEQ1_CHAR0_W::new(self)
    }
    #[doc = "Bits 16:23 - This register is used to define the second character of SLIP escape sequence. The default value is 0xDE."]
    #[inline(always)]
    pub fn esc_seq1_char1(&mut self) -> ESC_SEQ1_CHAR1_W<16> {
        ESC_SEQ1_CHAR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Escape sequence configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_conf2](index.html) module"]
pub struct ESC_CONF2_SPEC;
impl crate::RegisterSpec for ESC_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esc_conf2::R](R) reader structure"]
impl crate::Readable for ESC_CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esc_conf2::W](W) writer structure"]
impl crate::Writable for ESC_CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESC_CONF2 to value 0x00de_db11"]
impl crate::Resettable for ESC_CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00de_db11
    }
}
