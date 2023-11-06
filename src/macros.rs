///
/// Converts a `Result<T,E>` to `Result<T, AppError>`
///
/// # Example
/// ```rust
/// let data: Result<Data, DbError> = getDataFromDB()
/// let data: Result<Data, AppError> = MapErr!(DBError -> getDataFromDB())
/// // ErrorType -> is a macro syntax
/// ```
///
/// ### Or
/// ```rust
/// let data: Result<Data, DbError> = getDataFromDB()
/// let data: Result<Data, AppError> = MapErr!(getDataFromDB(), AppError::DatabaseError)
/// ```
///
///
#[macro_export]
macro_rules! MapErr {
    ($result:expr, $err_type:expr) => {{
        let mapped: Result<_, AppError> = $result.map_err(|err| $err_type(err));
        mapped
    }};
    (DBError -> $result:expr) => {{
        $result.map_err(|err| AppError::DatabaseError(err))
    }};

    (ServerErr -> $result:expr) => {{
        $result.map_err(|err| AppError::InternalError(err))
    }};

    (IoErr -> $result:expr) => {{
        $result.map_err(|err| AppError::IOError(err))
    }};
}
