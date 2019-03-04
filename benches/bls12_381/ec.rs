mod g1 {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    use pairing::{CurveProjective, Rand};
    use pairing::bls12_381::*;

    #[bench]
    fn bench_g1_mul_assign(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

        let v: Vec<(G1, Fr)> = (0..SAMPLES).map(|_| (G1::rand(&mut rng), Fr::rand(&mut rng))).collect();

        let mut count = 0;
        b.iter(|| {
            let mut tmp = v[count].0;
            tmp.mul_assign(v[count].1);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }

    #[bench]
    fn bench_g1_add_assign(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

        let v: Vec<(G1, G1)> = (0..SAMPLES).map(|_| (G1::rand(&mut rng), G1::rand(&mut rng))).collect();

        let mut count = 0;
        b.iter(|| {
            let mut tmp = v[count].0;
            tmp.add_assign(&v[count].1);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }

    #[bench]
    fn bench_g1_add_assign_mixed(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

        let v: Vec<(G1, G1Affine)> = (0..SAMPLES).map(|_| (G1::rand(&mut rng), G1::rand(&mut rng).into())).collect();

        let mut count = 0;
        b.iter(|| {
            let mut tmp = v[count].0;
            tmp.add_assign_mixed(&v[count].1);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }
}

mod g2 {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    use pairing::{CurveProjective, Rand};
    use pairing::bls12_381::*;

    #[bench]
    fn bench_g2_mul_assign(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

        let v: Vec<(G2, Fr)> = (0..SAMPLES).map(|_| (G2::rand(&mut rng), Fr::rand(&mut rng))).collect();

        let mut count = 0;
        b.iter(|| {
            let mut tmp = v[count].0;
            tmp.mul_assign(v[count].1);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }

    #[bench]
    fn bench_g2_add_assign(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

        let v: Vec<(G2, G2)> = (0..SAMPLES).map(|_| (G2::rand(&mut rng), G2::rand(&mut rng))).collect();

        let mut count = 0;
        b.iter(|| {
            let mut tmp = v[count].0;
            tmp.add_assign(&v[count].1);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }

    #[bench]
    fn bench_g2_add_assign_mixed(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(0x5dbe62598d313d76);

        let v: Vec<(G2, G2Affine)> = (0..SAMPLES).map(|_| (G2::rand(&mut rng), G2::rand(&mut rng).into())).collect();

        let mut count = 0;
        b.iter(|| {
            let mut tmp = v[count].0;
            tmp.add_assign_mixed(&v[count].1);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }
}
