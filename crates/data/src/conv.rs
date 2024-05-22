use anyhow::Result;

use crate::Conn;

/// Converts an object into another using the database.
pub trait DbInto<T> {
    fn db_into(self, db: &mut Conn) -> Result<T>;
}

/// Converts an object into another using the database
/// and some arbitrary data provided via the arg.
pub trait DbIntoArg<T, A> {
    fn db_into_arg(self, db: &mut Conn, arg: A) -> Result<T>;
}

impl<T, D> DbIntoArg<T, ()> for D
where
    D: DbInto<T>,
{
    fn db_into_arg(self, db: &mut Conn, _arg: ()) -> Result<T> {
        self.db_into(db)
    }
}
