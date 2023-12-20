#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

impl std::fmt::Display for IntervalQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if !f.alternate() {
            match self {
                Self::Perfect => "P",
                Self::Major => "M",
                Self::Minor => "m",
                Self::Augmented => "A",
                Self::Diminished => "d",
            }
        } else {
            match self {
                Self::Perfect => "Perfect",
                Self::Major => "Major",
                Self::Minor => "Minor",
                Self::Augmented => "Augmented",
                Self::Diminished => "Diminished",
            }
        };

        write!(f, "{}", name)
    }
}

impl PartialOrd for IntervalQuality {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            IntervalQuality::Perfect => match other {
                IntervalQuality::Perfect => Some(std::cmp::Ordering::Equal),
                IntervalQuality::Major | IntervalQuality::Minor => None,
                IntervalQuality::Augmented => Some(std::cmp::Ordering::Less),
                IntervalQuality::Diminished => Some(std::cmp::Ordering::Greater),
            },
            IntervalQuality::Major => match other {
                IntervalQuality::Major => Some(std::cmp::Ordering::Equal),
                IntervalQuality::Perfect => None,
                IntervalQuality::Minor | IntervalQuality::Diminished => {
                    Some(std::cmp::Ordering::Greater)
                }
                IntervalQuality::Augmented => Some(std::cmp::Ordering::Less),
            },
            IntervalQuality::Minor => match other {
                IntervalQuality::Minor => Some(std::cmp::Ordering::Equal),
                IntervalQuality::Perfect => None,
                IntervalQuality::Major | IntervalQuality::Augmented => {
                    Some(std::cmp::Ordering::Less)
                }
                IntervalQuality::Diminished => Some(std::cmp::Ordering::Greater),
            },
            IntervalQuality::Augmented => match other {
                IntervalQuality::Augmented => Some(std::cmp::Ordering::Equal),
                IntervalQuality::Perfect | IntervalQuality::Major | IntervalQuality::Minor => {
                    Some(std::cmp::Ordering::Greater)
                }
                IntervalQuality::Diminished => Some(std::cmp::Ordering::Greater),
            },
            IntervalQuality::Diminished => match other {
                IntervalQuality::Diminished => Some(std::cmp::Ordering::Equal),
                IntervalQuality::Perfect | IntervalQuality::Major | IntervalQuality::Minor => {
                    Some(std::cmp::Ordering::Less)
                }
                IntervalQuality::Augmented => Some(std::cmp::Ordering::Less),
            },
        }
    }
}
