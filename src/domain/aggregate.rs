#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    id: Id,
    email: Email,
    password_hash: PasswordHash,
    first_name: Option<FirstName>,
    last_name: Option<LastName>,
    display_name: Option<DisplayName>,
    bio: Option<Bio>,
    profire_image_url: Option<ProfireImageUrl>,
    country_code: CountryCode,
    language: Language,
    timezone: Timezone,
    version: Version,
}
