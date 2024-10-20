use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum CliffScore {
    Score(u64),
    Overloaded,
}

impl PartialOrd for CliffScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Score(s), Self::Score(other)) => s.partial_cmp(other),
            (Self::Score(_), Self::Overloaded) => Some(Ordering::Greater),
            (Self::Overloaded, Self::Score(_)) => Some(Ordering::Less),
            (Self::Overloaded, Self::Overloaded) => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::cliff_score::CliffScore;

    #[test_case(&CliffScore::Score(10), &CliffScore::Score(100); "numeric in order")]
    #[test_case(&CliffScore::Overloaded, &CliffScore::Score(100); "overloaded less than numeric")]
    fn less_than(left: &CliffScore, right: &CliffScore) {
        assert!(left < right);
        assert!(!(right <= left));
    }

    #[test_case(&CliffScore::Score(10), &CliffScore::Score(100); "numeric in order")]
    #[test_case(&CliffScore::Score(10), &CliffScore::Score(10); "numeric equal")]
    #[test_case(&CliffScore::Overloaded, &CliffScore::Score(100); "overloaded less than numeric")]
    #[test_case(&CliffScore::Overloaded, &CliffScore::Overloaded; "both overloaded")]
    fn lte(left: &CliffScore, right: &CliffScore) {
        assert!(left <= right);
        assert!(!(right < left));
    }

    #[test_case(&CliffScore::Score(100), &CliffScore::Score(10); "numeric in reverse order")]
    #[test_case(&CliffScore::Score(100), &CliffScore::Overloaded; "numeric greater than overloaded")]
    fn greater_than(left: &CliffScore, right: &CliffScore) {
        assert!(left > right);
        assert!(!(right >= left));
    }

    #[test_case(&CliffScore::Score(10), &CliffScore::Score(10); "numeric equal")]
    #[test_case(&CliffScore::Overloaded, &CliffScore::Overloaded; "both overloaded")]
    fn equal(left: &CliffScore, right: &CliffScore) {
        assert!(left == right);
    }

    #[test_case(&CliffScore::Score(10), &CliffScore::Score(100); "numeric in order")]
    #[test_case(&CliffScore::Score(100), &CliffScore::Score(00); "numeric in reverse order")]
    #[test_case(&CliffScore::Overloaded, &CliffScore::Score(10); "overloaded and numeric")]
    #[test_case(&CliffScore::Score(10), &CliffScore::Overloaded; "numeric and overloaded")]
    fn not_equal(left: &CliffScore, right: &CliffScore) {
        assert!(left != right);
    }
}
