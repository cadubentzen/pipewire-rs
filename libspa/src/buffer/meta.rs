// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::fmt::Debug;

use crate::utils::Region;

/// Type of metadata attached to a buffer.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MetaType(spa_sys::spa_meta_type);

#[allow(non_upper_case_globals)]
impl MetaType {
    pub const Invalid: Self = Self(spa_sys::SPA_META_Invalid);
    /// Contains a [`MetaHeader`] with timing information.
    pub const Header: Self = Self(spa_sys::SPA_META_Header);
    /// Contains a [`MetaRegion`] with video crop information.
    pub const VideoCrop: Self = Self(spa_sys::SPA_META_VideoCrop);
    /// Contains an array of [`MetaRegion`] with video damage information.
    pub const VideoDamage: Self = Self(spa_sys::SPA_META_VideoDamage);
    /// Contains bitmap information.
    pub const Bitmap: Self = Self(spa_sys::SPA_META_Bitmap);
    /// Contains cursor information.
    pub const Cursor: Self = Self(spa_sys::SPA_META_Cursor);
    /// Contains control metadata.
    pub const Control: Self = Self(spa_sys::SPA_META_Control);
    #[cfg(libpipewire_0_3_21_or_higher)]
    /// Contains busy counter.
    pub const Busy: Self = Self(spa_sys::SPA_META_Busy);
    #[cfg(libpipewire_0_3_62_or_higher)]
    /// Contains video transform information.
    pub const VideoTransform: Self = Self(spa_sys::SPA_META_VideoTransform);
    #[cfg(libpipewire_1_0_8_or_higher)]
    /// Contains sync timeline information.
    pub const SyncTimeline: Self = Self(spa_sys::SPA_META_SyncTimeline);

    pub fn from_raw(raw: spa_sys::spa_meta_type) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> spa_sys::spa_meta_type {
        self.0
    }

    /// Returns the expected size in bytes for this metadata type.
    ///
    /// This is useful when requesting metadata in stream parameters.
    /// Returns `None` for unknown or variable-size metadata types.
    pub fn size(&self) -> Option<i32> {
        match *self {
            Self::Header => Some(std::mem::size_of::<spa_sys::spa_meta_header>() as i32),
            Self::VideoCrop => Some(std::mem::size_of::<spa_sys::spa_meta_region>() as i32),
            #[cfg(libpipewire_0_3_62_or_higher)]
            Self::VideoTransform => {
                Some(std::mem::size_of::<spa_sys::spa_meta_videotransform>() as i32)
            }
            #[cfg(libpipewire_0_3_21_or_higher)]
            Self::Busy => Some(std::mem::size_of::<spa_sys::spa_meta_busy>() as i32),
            #[cfg(libpipewire_1_0_8_or_higher)]
            Self::SyncTimeline => {
                Some(std::mem::size_of::<spa_sys::spa_meta_sync_timeline>() as i32)
            }
            // VideoDamage, Bitmap, Cursor, Control have variable sizes
            _ => None,
        }
    }
}

impl Debug for MetaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!(
            "MetaType::{}",
            match *self {
                Self::Invalid => "Invalid",
                Self::Header => "Header",
                Self::VideoCrop => "VideoCrop",
                Self::VideoDamage => "VideoDamage",
                Self::Bitmap => "Bitmap",
                Self::Cursor => "Cursor",
                Self::Control => "Control",
                #[cfg(libpipewire_0_3_21_or_higher)]
                Self::Busy => "Busy",
                #[cfg(libpipewire_0_3_62_or_higher)]
                Self::VideoTransform => "VideoTransform",
                #[cfg(libpipewire_1_0_8_or_higher)]
                Self::SyncTimeline => "SyncTimeline",
                _ => "Unknown",
            }
        );
        f.write_str(&name)
    }
}

/// Metadata attached to a buffer.
///
/// Use [`Meta::type_()`] to determine the type of metadata and then use
/// the appropriate accessor method (e.g., [`Meta::video_crop()`]) to get
/// the typed data.
#[repr(transparent)]
pub struct Meta(spa_sys::spa_meta);

impl Meta {
    pub fn as_raw(&self) -> &spa_sys::spa_meta {
        &self.0
    }

    /// Returns the type of this metadata.
    pub fn type_(&self) -> MetaType {
        MetaType::from_raw(self.0.type_)
    }

    /// Returns the size of the metadata data in bytes.
    pub fn size(&self) -> u32 {
        self.0.size
    }

    /// Returns a raw pointer to the metadata data.
    ///
    /// # Safety
    ///
    /// The caller must ensure the pointer is used correctly based on the metadata type.
    pub fn data(&self) -> *mut std::ffi::c_void {
        self.0.data
    }

    /// Returns the video crop region if this metadata is of type [`MetaType::VideoCrop`].
    ///
    /// Returns `None` if the metadata type doesn't match or if the size is insufficient.
    pub fn video_crop(&self) -> Option<&MetaRegion> {
        if self.type_() == MetaType::VideoCrop
            && self.size() >= std::mem::size_of::<spa_sys::spa_meta_region>() as u32
        {
            unsafe { Some(&*(self.0.data as *const MetaRegion)) }
        } else {
            None
        }
    }
}

impl Debug for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Meta")
            .field("type", &self.type_())
            .field("size", &self.size())
            .finish()
    }
}

/// A region metadata, used for video crop and video damage.
#[derive(Clone)]
#[repr(transparent)]
pub struct MetaRegion(spa_sys::spa_meta_region);

impl MetaRegion {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_region {
        &self.0
    }

    /// Returns the region (position and size).
    pub fn region(&self) -> &Region {
        &self.0.region
    }

    /// Returns `true` if the region is valid (has non-zero width and height).
    pub fn is_valid(&self) -> bool {
        self.0.region.size.width > 0 && self.0.region.size.height > 0
    }
}

impl Debug for MetaRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaRegion")
            .field("x", &self.0.region.position.x)
            .field("y", &self.0.region.position.y)
            .field("width", &self.0.region.size.width)
            .field("height", &self.0.region.size.height)
            .finish()
    }
}
