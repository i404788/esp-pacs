#[doc = "Register `BT_LPCK_DIV_INT` reader"]
pub struct R(crate::R<BT_LPCK_DIV_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_LPCK_DIV_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_LPCK_DIV_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_LPCK_DIV_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT_LPCK_DIV_INT` writer"]
pub struct W(crate::W<BT_LPCK_DIV_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_LPCK_DIV_INT_SPEC>;
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
impl From<crate::W<BT_LPCK_DIV_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_LPCK_DIV_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BT_LPCK_DIV_NUM` reader - reg_bt_lpck_div_num"]
pub type BT_LPCK_DIV_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BT_LPCK_DIV_NUM` writer - reg_bt_lpck_div_num"]
pub type BT_LPCK_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BT_LPCK_DIV_INT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - reg_bt_lpck_div_num"]
    #[inline(always)]
    pub fn bt_lpck_div_num(&self) -> BT_LPCK_DIV_NUM_R {
        BT_LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - reg_bt_lpck_div_num"]
    #[inline(always)]
    pub fn bt_lpck_div_num(&mut self) -> BT_LPCK_DIV_NUM_W<0> {
        BT_LPCK_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock config register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_lpck_div_int](index.html) module"]
pub struct BT_LPCK_DIV_INT_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_lpck_div_int::R](R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt_lpck_div_int::W](W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT_LPCK_DIV_INT to value 0xff"]
impl crate::Resettable for BT_LPCK_DIV_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
