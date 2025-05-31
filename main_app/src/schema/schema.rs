// @generated automatically by Diesel CLI.

diesel::table! {
    user (userId) {
        userId -> Integer,
        userName -> Text,
        companyId -> Integer,
        isDeleted -> Bool,
        createdDate -> Timestamp,
        modifiedDate -> Timestamp,
    }
}
