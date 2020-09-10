use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, Eq)]
pub struct InstalledApplication {
    app_used: bool,
    date: String,
    location: String,
    name: String,
    publisher: String,
    user: String,
    version: String,
}

impl Ord for InstalledApplication {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl PartialOrd for InstalledApplication {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for InstalledApplication {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for InstalledApplication {
    // String representation of this InstalledApplication, for hashing
    // and other comparisons.  For the time being I am profiling ONLY
    // the software definition which is (name, publisher, version) so
    // am dropping other fields on the floor.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.name,
            self.publisher,
            self.version,
            // self.app_used,
            // self.date,
            // self.location,
            // self.user,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Make sure that equality operator ignores date, location, user
    fn equality_ignores_irrelevant_fields() {
        let application1 = InstalledApplication {
            name: "name".to_string(),
            publisher: "publisher".to_string(),
            app_used: true,
            version: "1.2.3".to_string(),
            date: "yesterday".to_string(),
            location: "denver".to_string(),
            user: "fred".to_string(),
        };
        let application2 = InstalledApplication {
            name: "name".to_string(),
            publisher: "publisher".to_string(),
            app_used: false,
            version: "1.2.3".to_string(),
            date: "TODAY".to_string(),
            location: "SAN JOSE".to_string(),
            user: "WILMA".to_string(),
        };
        assert_eq!(application1, application2);
    }

    #[test]
    fn equality_tests_relevant_fields() {
        let application1 = InstalledApplication {
            name: "name".to_string(),
            publisher: "publisher".to_string(),
            app_used: true,
            version: "1.2.3".to_string(),
            date: "yesterday".to_string(),
            location: "denver".to_string(),
            user: "fred".to_string(),
        };
        let mut application_name = application1.clone();
        application_name.name = "name_foobar".to_string();
        let mut application_pub = application1.clone();
        application_pub.publisher = "pub_foobar".to_string();
        let mut application_version = application1.clone();
        application_version.version = "4.5.6".to_string();
        assert_ne!(application1, application_name);
        assert_ne!(application1, application_pub);
        assert_ne!(application1, application_version);
    }
}
