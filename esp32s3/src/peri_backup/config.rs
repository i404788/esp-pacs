#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLOW_ERR` reader - x"]
pub type FLOW_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR_MAP_MODE` reader - x"]
pub type ADDR_MAP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_MAP_MODE` writer - x"]
pub type ADDR_MAP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `BURST_LIMIT` reader - x"]
pub type BURST_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BURST_LIMIT` writer - x"]
pub type BURST_LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `TOUT_THRES` reader - x"]
pub type TOUT_THRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOUT_THRES` writer - x"]
pub type TOUT_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u16, u16, 10, O>;
#[doc = "Field `SIZE` reader - x"]
pub type SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SIZE` writer - x"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u16, u16, 10, O>;
#[doc = "Field `START` writer - x"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `TO_MEM` reader - x"]
pub type TO_MEM_R = crate::BitReader<bool>;
#[doc = "Field `TO_MEM` writer - x"]
pub type TO_MEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENA` reader - x"]
pub type ENA_R = crate::BitReader<bool>;
#[doc = "Field `ENA` writer - x"]
pub type ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - x"]
    #[inline(always)]
    pub fn flow_err(&self) -> FLOW_ERR_R {
        FLOW_ERR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - x"]
    #[inline(always)]
    pub fn addr_map_mode(&self) -> ADDR_MAP_MODE_R {
        ADDR_MAP_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - x"]
    #[inline(always)]
    pub fn burst_limit(&self) -> BURST_LIMIT_R {
        BURST_LIMIT_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:18 - x"]
    #[inline(always)]
    pub fn tout_thres(&self) -> TOUT_THRES_R {
        TOUT_THRES_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 19:28 - x"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - x"]
    #[inline(always)]
    pub fn to_mem(&self) -> TO_MEM_R {
        TO_MEM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - x"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - x"]
    #[inline(always)]
    pub fn addr_map_mode(&mut self) -> ADDR_MAP_MODE_W<3> {
        ADDR_MAP_MODE_W::new(self)
    }
    #[doc = "Bits 4:8 - x"]
    #[inline(always)]
    pub fn burst_limit(&mut self) -> BURST_LIMIT_W<4> {
        BURST_LIMIT_W::new(self)
    }
    #[doc = "Bits 9:18 - x"]
    #[inline(always)]
    pub fn tout_thres(&mut self) -> TOUT_THRES_W<9> {
        TOUT_THRES_W::new(self)
    }
    #[doc = "Bits 19:28 - x"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<19> {
        SIZE_W::new(self)
    }
    #[doc = "Bit 29 - x"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<29> {
        START_W::new(self)
    }
    #[doc = "Bit 30 - x"]
    #[inline(always)]
    pub fn to_mem(&mut self) -> TO_MEM_W<30> {
        TO_MEM_W::new(self)
    }
    #[doc = "Bit 31 - x"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W<31> {
        ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0x6480"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6480
    }
}
