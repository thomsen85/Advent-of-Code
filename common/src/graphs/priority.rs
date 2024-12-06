/// Priority for use in Min heap algorithms such as Dijkstra
#[derive(Debug)]
pub struct Priority<T, P>
where
    T: PartialEq + PartialOrd + Ord + Eq,
{
    pub value: T,
    pub data: P,
}

impl<T, P> Eq for Priority<T, P> where T: PartialEq + PartialOrd + Ord + Eq {}

impl<T, P> PartialEq for Priority<T, P>
where
    T: PartialEq + PartialOrd + Ord + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, P> Ord for Priority<T, P>
where
    T: PartialEq + PartialOrd + Ord + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

impl<T, P> PartialOrd for Priority<T, P>
where
    T: PartialEq + PartialOrd + Ord + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
