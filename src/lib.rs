struct Client<'p> {
    provider: &'p dyn Provider
}

impl Client<'_> {
    fn boolean(&self, key:&str, default:&bool) -> bool {
        return self.provider.boolean(key, default);
    }
}

trait Provider {
    fn boolean(&self, key:&str, default:&bool) -> bool;
}

struct NoOp {}

impl Provider for NoOp {
    fn boolean(&self, _key:&str, default:&bool) -> bool {
        return *default;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, NoOp};
    
    #[test]
    fn missing_uses_default() {
        let noop = &NoOp{};
        let client = Client{ provider: noop };
        assert_eq!(client.boolean("missing", &true),
                   true);
    }
}
