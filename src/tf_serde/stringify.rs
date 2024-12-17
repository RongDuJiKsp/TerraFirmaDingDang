use std::error::Error;

pub trait SerializedList
where
    Self: Sized,
{
    fn marshal(v: &Vec<Self>) -> String;
    fn unmarshal(string: &str) -> Result<Vec<Self>, Box<dyn Error>>;
}
