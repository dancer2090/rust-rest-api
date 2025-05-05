diesel::table! {
  users (id) {
      id -> Uuid,
      username -> Varchar,
      email -> Varchar,
  }
}