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
    use pareto::{Dominate, ParetoFront};

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

    #[derive(Dominate, Clone, Copy)]
    struct C {
        #[pareto(maximize)]
        inverted: usize,
        #[pareto(invert)]
        #[pareto(invert)]
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
        #[pareto(ignore)]
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

    #[test]
    fn test_front_1() {
        let a = C {
            inverted: usize::MAX,
            normal: 0,
        };
        let mut f = ParetoFront::new();
        f.push(a);
        for _ in 0..1000 {
            let a = C {
                inverted: rand::random(),
                normal: rand::random(),
            };
            f.push(a);
        }
        assert_eq!(f.len(), 1);
    }

    #[test]
    fn test_front_2() {
        let a = C {
            inverted: 35,
            normal: 16,
        };
        let b = C {
            inverted: 51,
            normal: 20,
        };
        let c = C {
            inverted: 34,
            normal: 16,
        };
        let d = C {
            inverted: 36,
            normal: 15,
        };
        let mut f = ParetoFront::new();
        assert!(f.push(a));
        assert!(f.push(b));
        assert!(!f.push(c));
        assert!(f.push(d));
        assert!(!f.push(d));
        assert_eq!(f.len(), 2);
    }

    #[derive(Dominate, Copy, Clone, PartialEq, Eq, Debug)]
    struct E {
        a: u32,
        #[pareto(ignore)]
        #[allow(unused)]
        b: u32,
        c: usize,
    }

    #[test]
    fn test_front_3() {
        let a = E {
            a: 28968,
            b: 0,
            c: 2,
        };
        let b = E {
            a: 28968,
            b: 0,
            c: 3,
        };

        assert!(a.dominates(&b));
        let mut f = ParetoFront::new();
        assert!(f.push(a));
        assert!(!f.push(b));
        assert_eq!(f.len(), 1);
        assert_eq!(f.iter().next(), Some(&a))
    }
}
