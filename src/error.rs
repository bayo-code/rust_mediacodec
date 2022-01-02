#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MediaStatus {
    Ok = 0,
    ErrorInsufficientResource = 1100,
    ErrorReclaimed = 1101,
    ErrorUnknown = -10000,
    ErrorMalformed = crate::MediaStatus::ErrorUnknown as isize - 1,
    ErrorUnsupported = crate::MediaStatus::ErrorUnknown as isize - 2,
    ErrorInvalidObject = crate::MediaStatus::ErrorUnknown as isize - 3,
    ErrorInvalidParameter = crate::MediaStatus::ErrorUnknown as isize - 4,
    ErrorInvalidOperation = crate::MediaStatus::ErrorUnknown as isize - 5,
    ErrorEndOfStream = crate::MediaStatus::ErrorUnknown as isize - 6,
    ErrorIO = crate::MediaStatus::ErrorUnknown as isize - 7,
    ErrorWouldBlock = crate::MediaStatus::ErrorUnknown as isize - 8,
    DRMErrorBase = -20000,
    DRMNotProvisioned = crate::MediaStatus::DRMErrorBase as isize - 1,
    DRMResourceBusy = crate::MediaStatus::DRMErrorBase as isize - 2,
    DRMDeviceRevoked = crate::MediaStatus::DRMErrorBase as isize - 3,
    DRMShortBuffer = crate::MediaStatus::DRMErrorBase as isize - 4,
    DRMSessionNotOpened = crate::MediaStatus::DRMErrorBase as isize - 5,
    DRMTamperDetected = crate::MediaStatus::DRMErrorBase as isize - 6,
    DRMVerifyFailed = crate::MediaStatus::DRMErrorBase as isize - 7,
    DRMNeedKey = crate::MediaStatus::DRMErrorBase as isize - 8,
    DRMLicenseExpired = crate::MediaStatus::DRMErrorBase as isize - 9,
    ImgReaderErrorBase = -30000,
    ImgReaderNoBufferAvailable = crate::MediaStatus::ImgReaderErrorBase as isize - 1,
    ImgReaderMaxImagesAcquired = crate::MediaStatus::ImgReaderErrorBase as isize - 2,
    ImgReaderCannotLockImage = crate::MediaStatus::ImgReaderErrorBase as isize - 3,
    ImgReaderCannotUnlockImage = crate::MediaStatus::ImgReaderErrorBase as isize - 4,
    ImgReaderImageNotLocked = crate::MediaStatus::ImgReaderErrorBase as isize - 5,
}

impl MediaStatus {
    fn values() -> Vec<Self> {
        use crate::MediaStatus::*;
        vec![
            Ok,
            ErrorInsufficientResource,
            ErrorReclaimed,
            ErrorUnknown,
            ErrorMalformed,
            ErrorUnsupported,
            ErrorInvalidObject,
            ErrorInvalidParameter,
            ErrorInvalidOperation,
            ErrorEndOfStream,
            ErrorIO,
            ErrorWouldBlock,
            DRMErrorBase,
            DRMNotProvisioned,
            DRMResourceBusy,
            DRMDeviceRevoked,
            DRMShortBuffer,
            DRMSessionNotOpened,
            DRMTamperDetected,
            DRMVerifyFailed,
            DRMNeedKey,
            DRMLicenseExpired,
            ImgReaderErrorBase,
            ImgReaderNoBufferAvailable,
            ImgReaderMaxImagesAcquired,
            ImgReaderCannotLockImage,
            ImgReaderCannotUnlockImage,
            ImgReaderImageNotLocked,
        ]
    }
    // Makes a result based on `value`. Ok is returned with the value if the value is not one of MediaStatus error codes
    pub fn make_result(value: isize) -> Result<isize, MediaStatus> {
        let status: Result<MediaStatus, &str> = value.try_into();
        if let Ok(status) = status {
            if status.is_ok() {
                return Ok(status as isize);
            } else {
                return Err(status);
            }
        }

        // If we could not convert to a MediaStatus, we got some other value that's most likely not an error, so we return Ok
        Ok(value)
    }

    fn is_ok(&self) -> bool {
        let mut valuez = Self::values();
        // Remove the Ok. Now, the rest are errors
        valuez.remove(0);

        // If we get none, there were no errors
        return valuez.iter().find(|&&x| *self == x).is_none();
    }

    fn is_err(&self) -> bool {
        return !self.is_ok();
    }
}

impl TryFrom<isize> for MediaStatus {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        for item in Self::values() {
            if item as isize == value {
                return Ok(item);
            }
        }

        return Err("Not Found");
    }
}
