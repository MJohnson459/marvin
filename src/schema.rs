table! {
    documentation (id) {
        id -> Int4,
        vcs -> Varchar,
        url -> Varchar,
        version -> Varchar,
    }
}

table! {
    packages (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    releases (id) {
        id -> Int4,
        url -> Nullable<Varchar>,
        version -> Varchar,
    }
}

table! {
    repositories (name) {
        name -> Varchar,
        documentation -> Int4,
        release -> Int4,
        source -> Int4,
        status -> Nullable<Varchar>,
    }
}

table! {
    sources (id) {
        id -> Int4,
        vcs -> Varchar,
        url -> Varchar,
        version -> Varchar,
    }
}

table! {
    tags (id) {
        id -> Int4,
        release_id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

table! {
    package_releases (package_id, release_id) {
        package_id -> Int4,
        release_id -> Int4,
    }
}

joinable!(repositories -> documentation (documentation));
joinable!(repositories -> releases (release));
joinable!(repositories -> sources (source));

allow_tables_to_appear_in_same_query!(
    documentation,
    packages,
    releases,
    repositories,
    sources,
    tags,
);
