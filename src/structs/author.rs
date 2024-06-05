use time::Date;
use uuid::Uuid;

use crate::structs::PersonName;

struct Author {
    id: Uuid,
    name: PersonName,
    born: Date,
}
