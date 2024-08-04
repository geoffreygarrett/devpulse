pub(crate) trait ToMermaid {
    fn to_mermaid(&self) -> String;
}

pub(crate) trait ToDot {
    fn to_dot(&self) -> String;
}
