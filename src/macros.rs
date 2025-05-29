#[macro_export]
macro_rules! impl_traits_for_label {
    ($label:ident) => {
        impl $crate::MsgLbl for $label {}
        impl $crate::ProtocolLabel for $label {}
        impl $crate::SessionType for $label {} // Assuming labels might need to be SessionTypes for HasDual
        impl $crate::HasDual for $label {
            type Dual = $label; // Labels are typically their own dual in this context
        }
    };
}
