#![allow(unused)] // Verhindert Compiler-Warnungen für ungenutzten Code

// Definition der `MResult`-Enum mit zwei Varianten: `Ok` und `Err`
pub enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    // Erstelle eine neue `Ok`-Variante
    pub fn ok(value: T) -> Self {
        MResult::Ok(value)
    }

    // Erstelle eine neue `Err`-Variante
    pub fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Überprüfe, ob es eine `Ok`-Variante ist
    pub fn is_ok(&self) -> bool {
        matches!(self, MResult::Ok(_))
    }

    // Überprüfe, ob es eine `Err`-Variante ist
    pub fn is_err(&self) -> bool {
        matches!(self, MResult::Err(_))
    }

    // Entpacke den `Ok`-Wert, panikt, wenn es eine `Err`-Variante ist
    pub fn unwrap(self) -> T {
        match self {
            MResult::Ok(value) => value,
            MResult::Err(_) => panic!("called `unwrap()` on an `Err` value"),
        }
    }

    // Entpacke den `Err`-Wert, panikt, wenn es eine `Ok`-Variante ist
    pub fn unwrap_err(self) -> E {
        match self {
            MResult::Ok(_) => panic!("called `unwrap_err()` on an `Ok` value"),
            MResult::Err(error) => error,
        }
    }
}

// Unit-Tests für die `MResult`-Enum
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(result.is_ok());
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "called `unwrap()` on an `Err` value")]
    fn test_unwrap_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        result.unwrap();
    }

    #[test]
    fn test_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(result.is_err());
        assert!(!result.is_ok());
        assert_eq!(result.unwrap_err(), "error");
    }

    #[test]
    #[should_panic(expected = "called `unwrap_err()` on an `Ok` value")]
    fn test_unwrap_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        result.unwrap_err();
    }
}
