use std::fmt::Display;

#[derive(Debug)]
pub struct AocResult(String, Option<String>);
impl From<String> for AocResult {
    fn from(s: String) -> Self {
        Self(s, None)
    }
}
impl<T1: ToString, T2: ToString> From<(T1, T2)> for AocResult {
    fn from((s1, s2): (T1, T2)) -> Self {
        Self(s1.to_string(), Some(s2.to_string()))
    }
}
impl From<i32> for AocResult {
    fn from(s: i32) -> Self {
        s.to_string().into()
    }
}

impl Display for AocResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- Part 1 ---")?;
        write!(f, "{}", self.0)?;
        if let Some(res) = &self.1 {
            writeln!(f, "\n--- part 2 ---")?;
            write!(f, "{}", res)?;
        }
        Ok(())
    }
}
