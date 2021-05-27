use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Contacts {
    /// A Boolean value that indicates whether the app may access the notes stored in contacts.
    ///
    /// When your app loads one or more entries from the user’s contacts—for example, by calling the
    /// unifiedContacts(matching:keysToFetch:) method—you provide a list of keys specifying what fields to fetch.
    /// If your app links against iOS 13 or later, the app must have the com.apple.developer.contacts.notes
    /// entitlement to request the note field using CNContactNoteKey.
    /// Without the entitlement, your app receives an unauthorizedKeys error when trying to fetch notes.
    ///
    /// To add the entitlement to your app, in the Xcode property list editor, set the entitlement’s type
    /// to Boolean, and the corresponding value to YES.
    /// Before you can submit an app with this entitlement to the App Store, you must first get permission
    /// to use the entitlement.
    /// Request permission at https://developer.apple.com/contact/request/contact-note-field.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * Contacts
    #[serde(
        rename(serialize = "com.apple.developer.contacts.notes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_audio: Option<bool>,
}
