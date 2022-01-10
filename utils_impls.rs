impl<A: Interpolate, B: Interpolate> Interpolate for (A, B) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
        )
    }
}


impl<A: Interpolate, B: Interpolate, C: Interpolate> Interpolate for (A, B, C) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
            C::interpolate(&p0.2, &p1.2, &p2.2, weights),
        )
    }
}


impl<A: Interpolate, B: Interpolate, C: Interpolate, D: Interpolate> Interpolate for (A, B, C, D) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
            C::interpolate(&p0.2, &p1.2, &p2.2, weights),
            D::interpolate(&p0.3, &p1.3, &p2.3, weights),
        )
    }
}


impl<A: Interpolate, B: Interpolate, C: Interpolate, D: Interpolate, E: Interpolate> Interpolate for (A, B, C, D, E) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
            C::interpolate(&p0.2, &p1.2, &p2.2, weights),
            D::interpolate(&p0.3, &p1.3, &p2.3, weights),
            E::interpolate(&p0.4, &p1.4, &p2.4, weights),
        )
    }
}


impl<A: Interpolate, B: Interpolate, C: Interpolate, D: Interpolate, E: Interpolate, F: Interpolate> Interpolate for (A, B, C, D, E, F) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
            C::interpolate(&p0.2, &p1.2, &p2.2, weights),
            D::interpolate(&p0.3, &p1.3, &p2.3, weights),
            E::interpolate(&p0.4, &p1.4, &p2.4, weights),
            F::interpolate(&p0.5, &p1.5, &p2.5, weights),
        )
    }
}


impl<A: Interpolate, B: Interpolate, C: Interpolate, D: Interpolate, E: Interpolate, F: Interpolate, G: Interpolate> Interpolate for (A, B, C, D, E, F, G) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
            C::interpolate(&p0.2, &p1.2, &p2.2, weights),
            D::interpolate(&p0.3, &p1.3, &p2.3, weights),
            E::interpolate(&p0.4, &p1.4, &p2.4, weights),
            F::interpolate(&p0.5, &p1.5, &p2.5, weights),
            G::interpolate(&p0.6, &p1.6, &p2.6, weights),
        )
    }
}


impl<A: Interpolate, B: Interpolate, C: Interpolate, D: Interpolate, E: Interpolate, F: Interpolate, G: Interpolate, H: Interpolate> Interpolate for (A, B, C, D, E, F, G, H) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
            C::interpolate(&p0.2, &p1.2, &p2.2, weights),
            D::interpolate(&p0.3, &p1.3, &p2.3, weights),
            E::interpolate(&p0.4, &p1.4, &p2.4, weights),
            F::interpolate(&p0.5, &p1.5, &p2.5, weights),
            G::interpolate(&p0.6, &p1.6, &p2.6, weights),
            H::interpolate(&p0.7, &p1.7, &p2.7, weights),
        )
    }
}


impl<A: Interpolate, B: Interpolate, C: Interpolate, D: Interpolate, E: Interpolate, F: Interpolate, G: Interpolate, H: Interpolate, I: Interpolate> Interpolate for (A, B, C, D, E, F, G, H, I) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
            C::interpolate(&p0.2, &p1.2, &p2.2, weights),
            D::interpolate(&p0.3, &p1.3, &p2.3, weights),
            E::interpolate(&p0.4, &p1.4, &p2.4, weights),
            F::interpolate(&p0.5, &p1.5, &p2.5, weights),
            G::interpolate(&p0.6, &p1.6, &p2.6, weights),
            H::interpolate(&p0.7, &p1.7, &p2.7, weights),
            I::interpolate(&p0.8, &p1.8, &p2.8, weights),
        )
    }
}


impl<A: Interpolate, B: Interpolate, C: Interpolate, D: Interpolate, E: Interpolate, F: Interpolate, G: Interpolate, H: Interpolate, I: Interpolate, J: Interpolate> Interpolate for (A, B, C, D, E, F, G, H, I, J) {
    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
        (
            A::interpolate(&p0.0, &p1.0, &p2.0, weights),
            B::interpolate(&p0.1, &p1.1, &p2.1, weights),
            C::interpolate(&p0.2, &p1.2, &p2.2, weights),
            D::interpolate(&p0.3, &p1.3, &p2.3, weights),
            E::interpolate(&p0.4, &p1.4, &p2.4, weights),
            F::interpolate(&p0.5, &p1.5, &p2.5, weights),
            G::interpolate(&p0.6, &p1.6, &p2.6, weights),
            H::interpolate(&p0.7, &p1.7, &p2.7, weights),
            I::interpolate(&p0.8, &p1.8, &p2.8, weights),
            J::interpolate(&p0.9, &p1.9, &p2.9, weights),
        )
    }
}


