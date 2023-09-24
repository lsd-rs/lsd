#[cfg(windows)]
use super::windows_attributes::WindowsAttributes;
use crate::{
    color::{ColoredString, Colors},
    flags::Flags,
};

use super::Permissions;

#[derive(Clone, Debug)]
pub enum PermissionsOrAttributes {
    Permissions(Permissions),
    #[cfg(windows)]
    WindowsAttributes(WindowsAttributes),
}

impl PermissionsOrAttributes {
    pub fn render(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        match self {
            PermissionsOrAttributes::Permissions(permissions) => permissions.render(colors, flags),
            #[cfg(windows)]
            PermissionsOrAttributes::WindowsAttributes(attributes) => {
                attributes.render(colors, flags)
            }
        }
    }
}
