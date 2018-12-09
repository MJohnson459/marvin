table! {
    documentation (id) {
        id -> Int4,
        package_id -> Int4,
        vcs -> Varchar,
        url -> Varchar,
        version -> Varchar,
    }
}

table! {
    packages (id) {
        id -> Int4,
        name -> Varchar,
        status -> Varchar,
    }
}

table! {
    source (id) {
        id -> Int4,
        package_id -> Int4,
        vcs -> Varchar,
        url -> Varchar,
        version -> Varchar,
    }
}

table! {
    subpackages (id) {
        id -> Int4,
        package_id -> Int4,
        name -> Varchar,
    }
}

table! {
    versions (id) {
        id -> Int4,
        package_id -> Int4,
        num -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(documentation, packages, source, subpackages, versions,);
