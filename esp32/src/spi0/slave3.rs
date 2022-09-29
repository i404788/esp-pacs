#[doc = "Register `SLAVE3` reader"]
pub struct R(crate::R<SLAVE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE3` writer"]
pub struct W(crate::W<SLAVE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE3_SPEC>;
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
impl From<crate::W<SLAVE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RDBUF_CMD_VALUE` reader - In the slave mode it is the value of read-buffer command."]
pub type SLV_RDBUF_CMD_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLV_RDBUF_CMD_VALUE` writer - In the slave mode it is the value of read-buffer command."]
pub type SLV_RDBUF_CMD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLAVE3_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLV_WRBUF_CMD_VALUE` reader - In the slave mode it is the value of write-buffer command."]
pub type SLV_WRBUF_CMD_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLV_WRBUF_CMD_VALUE` writer - In the slave mode it is the value of write-buffer command."]
pub type SLV_WRBUF_CMD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLAVE3_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLV_RDSTA_CMD_VALUE` reader - In the slave mode it is the value of read-status command."]
pub type SLV_RDSTA_CMD_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLV_RDSTA_CMD_VALUE` writer - In the slave mode it is the value of read-status command."]
pub type SLV_RDSTA_CMD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLAVE3_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLV_WRSTA_CMD_VALUE` reader - In the slave mode it is the value of write-status command."]
pub type SLV_WRSTA_CMD_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLV_WRSTA_CMD_VALUE` writer - In the slave mode it is the value of write-status command."]
pub type SLV_WRSTA_CMD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLAVE3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In the slave mode it is the value of read-buffer command."]
    #[inline(always)]
    pub fn slv_rdbuf_cmd_value(&self) -> SLV_RDBUF_CMD_VALUE_R {
        SLV_RDBUF_CMD_VALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the slave mode it is the value of write-buffer command."]
    #[inline(always)]
    pub fn slv_wrbuf_cmd_value(&self) -> SLV_WRBUF_CMD_VALUE_R {
        SLV_WRBUF_CMD_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of read-status command."]
    #[inline(always)]
    pub fn slv_rdsta_cmd_value(&self) -> SLV_RDSTA_CMD_VALUE_R {
        SLV_RDSTA_CMD_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the value of write-status command."]
    #[inline(always)]
    pub fn slv_wrsta_cmd_value(&self) -> SLV_WRSTA_CMD_VALUE_R {
        SLV_WRSTA_CMD_VALUE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In the slave mode it is the value of read-buffer command."]
    #[inline(always)]
    pub fn slv_rdbuf_cmd_value(&mut self) -> SLV_RDBUF_CMD_VALUE_W<0> {
        SLV_RDBUF_CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 8:15 - In the slave mode it is the value of write-buffer command."]
    #[inline(always)]
    pub fn slv_wrbuf_cmd_value(&mut self) -> SLV_WRBUF_CMD_VALUE_W<8> {
        SLV_WRBUF_CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of read-status command."]
    #[inline(always)]
    pub fn slv_rdsta_cmd_value(&mut self) -> SLV_RDSTA_CMD_VALUE_W<16> {
        SLV_RDSTA_CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the value of write-status command."]
    #[inline(always)]
    pub fn slv_wrsta_cmd_value(&mut self) -> SLV_WRSTA_CMD_VALUE_W<24> {
        SLV_WRSTA_CMD_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave3](index.html) module"]
pub struct SLAVE3_SPEC;
impl crate::RegisterSpec for SLAVE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave3::R](R) reader structure"]
impl crate::Readable for SLAVE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave3::W](W) writer structure"]
impl crate::Writable for SLAVE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE3 to value 0"]
impl crate::Resettable for SLAVE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}