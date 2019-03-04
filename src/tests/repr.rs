use rand::{SeedableRng};
use rand_xorshift::{XorShiftRng};
use PrimeFieldRepr;

pub fn random_repr_tests<R: PrimeFieldRepr>() {
    random_encoding_tests::<R>();
    random_shl_tests::<R>();
    random_shr_tests::<R>();
}

fn random_encoding_tests<R: PrimeFieldRepr>() {
    let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

    for _ in 0..1000 {
        let r = R::rand(&mut rng);
        let mut rdecoded = R::default();

        let mut v: Vec<u8> = vec![];
        r.write_be(&mut v).unwrap();
        rdecoded.read_be(&v[0..]).unwrap();

        assert_eq!(r, rdecoded);
    }
}

fn random_shl_tests<R: PrimeFieldRepr>() {
    let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

    for _ in 0..100 {
        let r = R::rand(&mut rng);

        for shift in 0..(r.num_bits() + 1) {
            let mut r1 = r;
            let mut r2 = r;

            for _ in 0..shift {
                r1.mul2();
            }

            r2.shl(shift);

            assert_eq!(r1, r2);
        }
    }
}

fn random_shr_tests<R: PrimeFieldRepr>() {
    let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

    for _ in 0..100 {
        let r = R::rand(&mut rng);

        for shift in 0..(r.num_bits() + 1) {
            let mut r1 = r;
            let mut r2 = r;

            for _ in 0..shift {
                r1.div2();
            }

            r2.shr(shift);

            assert_eq!(r1, r2);
        }
    }
}
