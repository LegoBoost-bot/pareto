use pareto::Dominate;

#[derive(Dominate)]
struct TestNamed {
    a: u64, // Implements `Ord`,
    b: f64, // Implements `PartialOrd`,
    c: (),  // Tests elements which are always equal
}

#[derive(Dominate)]
struct TestUnnamed(u64, f64, ());

fn main() {
    let a = TestNamed {
        a: 3,
        b: 2.5,
        c: (),
    };

    let b = TestNamed {
        a: 5,
        b: 2.3,
        c: (),
    };

    // Check that a and b are both pareto-optimal
    assert!(!a.dominates(&b));
    assert!(!b.dominates(&a));

    let a = TestUnnamed(3, 2.5, ());
    let b = TestUnnamed(5, 2.3, ());

    // Check that a and b are both pareto-optimal
    assert!(!a.dominates(&b));
    assert!(!b.dominates(&a));

    let a = TestNamed {
        a: 5,
        b: 2.5,
        c: (),
    };

    let b = TestNamed {
        a: 6,
        b: 2.5,
        c: (),
    };

    // a should dominate b.
    assert!(a.dominates(&b));
    assert!(!b.dominates(&a));
}

#[cfg(test)]
mod test {
    use pareto::Dominate;

    #[derive(Dominate)]
    struct A(usize);

    #[test]
    fn test_equal() {
        let a = A(123);
        let b = A(123);
        assert!(a.dominates(&b));
        assert!(b.dominates(&a));
    }

    #[test]
    fn test_unequal() {
        let a = A(123);
        let b = A(321);
        assert!(a.dominates(&b));
        assert!(!b.dominates(&a));
    }

    #[derive(Dominate)]
    struct B(usize, usize);

    #[test]
    fn test_not_dominated() {
        let a = B(123, 321);
        let b = B(321, 123);
        assert!(!a.dominates(&b));
        assert!(!b.dominates(&a));
    }

    #[test]
    fn test_dominated() {
        let a = B(123, 123);
        let b = B(123, 321);
        assert!(a.dominates(&b));
        assert!(!b.dominates(&a));
    }

    #[derive(Dominate)]
    struct C {
        #[pareto_invert]
        inverted: usize,
        #[pareto_invert]
        #[pareto_invert]
        normal: usize, // Inverting twice should result in a normal comparison
    }

    #[test]
    fn test_invert() {
        let a = C {
            inverted: 3,
            normal: 4,
        };
        let b = C {
            inverted: 5,
            normal: 1,
        };
        assert!(!a.dominates(&b));
        assert!(b.dominates(&a));
    }

    #[derive(Dominate)]
    struct D {
        a: u32,
        #[pareto_ignore]
        #[allow(unused)]
        b: u32,
    }

    #[test]
    fn test_ignore() {
        let a = D { a: 30, b: u32::MAX };
        let b = D { a: 31, b: 13 };
        assert!(a.dominates(&b));
        assert!(!b.dominates(&a));
    }
}
