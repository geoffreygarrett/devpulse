pub trait ToMermaid {
    fn to_mermaid(&self) -> String;
}

pub trait ToDot {
    fn to_dot(&self) -> String;
}
