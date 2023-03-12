pub trait DbQuery {
    fn name(&self) -> String;

    fn raw(&self) -> String;
}
