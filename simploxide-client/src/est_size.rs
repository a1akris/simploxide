use simploxide_api_types::{
    GroupProfile, LinkContent, LinkOwnerSig, LinkPreview, MsgChatLink, MsgContent, Profile,
};

pub trait EstSize {
    fn est_size(&self) -> usize;
}

impl EstSize for MsgContent {
    fn est_size(&self) -> usize {
        match self {
            Self::Text { text, .. } => text.len(),
            Self::Link { text, preview, .. } => text.len() + preview.est_size(),
            Self::Image { text, image, .. } => text.len() + image.len(),
            Self::Video { text, image, .. } => text.len() + image.len(),
            Self::Voice { text, .. } => text.len(),
            Self::File { text, .. } => text.len(),
            Self::Report { text, .. } => text.len(),
            Self::Chat {
                text,
                chat_link,
                owner_sig,
                ..
            } => {
                text.len()
                    + chat_link.est_size()
                    + owner_sig.as_ref().map(EstSize::est_size).unwrap_or(0)
            }
            Self::Unknown { tag, text, .. } => tag.len() + text.len(),
            Self::Undocumented(_) => 0,
            _ => 0,
        }
    }
}

impl EstSize for LinkPreview {
    fn est_size(&self) -> usize {
        self.uri.len()
            + self.title.len()
            + self.description.len()
            + self.image.len()
            + self.content.as_ref().map(EstSize::est_size).unwrap_or(0)
    }
}

impl EstSize for LinkContent {
    fn est_size(&self) -> usize {
        match self {
            Self::Unknown { tag, .. } => tag.len(),
            _ => 0,
        }
    }
}

impl EstSize for MsgChatLink {
    fn est_size(&self) -> usize {
        match self {
            Self::Contact {
                conn_link, profile, ..
            } => conn_link.len() + profile.est_size(),
            Self::Invitation {
                inv_link, profile, ..
            } => inv_link.len() + profile.est_size(),
            Self::Group {
                conn_link,
                group_profile,
                ..
            } => conn_link.len() + group_profile.est_size(),
            Self::Undocumented(_) => 0,
            _ => 0,
        }
    }
}

impl EstSize for LinkOwnerSig {
    fn est_size(&self) -> usize {
        self.owner_id.as_deref().map(str::len).unwrap_or(0)
            + self.chat_binding.len()
            + self.owner_sig.len()
    }
}

impl EstSize for Profile {
    fn est_size(&self) -> usize {
        self.display_name.len()
            + self.full_name.len()
            + self.short_descr.as_deref().map(str::len).unwrap_or(0)
            + self.description.as_deref().map(str::len).unwrap_or(0)
            + self.image.as_deref().map(str::len).unwrap_or(0)
            + self.contact_link.as_deref().map(str::len).unwrap_or(0)
    }
}

impl EstSize for GroupProfile {
    fn est_size(&self) -> usize {
        self.display_name.len()
            + self.full_name.len()
            + self.short_descr.as_deref().map(str::len).unwrap_or(0)
            + self.description.as_deref().map(str::len).unwrap_or(0)
            + self.image.as_deref().map(str::len).unwrap_or(0)
    }
}
