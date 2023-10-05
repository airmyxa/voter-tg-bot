pub trait SqlQueryTr {
    fn raw(&self) -> &'static str;

    fn name(&self) -> &'static str;
}

pub struct StaticSqlQuery {
    name: &'static str,
    query: &'static str,
}

impl StaticSqlQuery {
    fn new(name: &'static str, query: &'static str) -> Self {
        StaticSqlQuery { name, query }
    }
}

impl SqlQueryTr for StaticSqlQuery {
    fn raw(&self) -> &'static str {
        return self.query;
    }

    fn name(&self) -> &'static str {
        return self.name;
    }
}
