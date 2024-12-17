trait SerializedList {
    fn marshal(v: Vec<Self>) -> String;
    fn unmarshal(string: &str) -> Vec<Self>;
}
