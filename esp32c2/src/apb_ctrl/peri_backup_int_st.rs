#[doc = "Register `PERI_BACKUP_INT_ST` reader"]
pub struct R(crate::R<PERI_BACKUP_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_BACKUP_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_BACKUP_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_BACKUP_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERI_BACKUP_DONE_INT_ST` reader - reg_peri_backup_done_int_st"]
pub type PERI_BACKUP_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `PERI_BACKUP_ERR_INT_ST` reader - reg_peri_backup_err_int_st"]
pub type PERI_BACKUP_ERR_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - reg_peri_backup_done_int_st"]
    #[inline(always)]
    pub fn peri_backup_done_int_st(&self) -> PERI_BACKUP_DONE_INT_ST_R {
        PERI_BACKUP_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_st"]
    #[inline(always)]
    pub fn peri_backup_err_int_st(&self) -> PERI_BACKUP_ERR_INT_ST_R {
        PERI_BACKUP_ERR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_st](index.html) module"]
pub struct PERI_BACKUP_INT_ST_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_backup_int_st::R](R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERI_BACKUP_INT_ST to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
