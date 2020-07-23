use chrono::Utc;

pub fn time() -> String {
    Utc::now().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
