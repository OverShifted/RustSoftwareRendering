# impl<T: Interpolate, U: Interpolate, V: Interpolate> Interpolate for (T, U) {
#     fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {
#         (
#             T::interpolate(&p0.0, &p1.0, &p2.0, weights),
#             U::interpolate(&p0.1, &p1.1, &p2.1, weights)
#         )
#     }
# }

letters = "ABCDEFGHIJ"
for i in range(1, len(letters)) :
    print("impl<", end="")
    print(", ".join([f"{c}: Interpolate" for c in letters[0:i + 1]]), end=f'> Interpolate for ({", ".join(letters[0:i + 1])}) {{\n')
    print("    fn interpolate(p0: &Self, p1: &Self, p2: &Self, weights: &Vec3) -> Self {")
    print("        (")
    for i, c in enumerate(letters[0:i + 1]):
        print(f"            {c}::interpolate(&p0.{i}, &p1.{i}, &p2.{i}, weights),")
    print("        )")
    print("    }")
    print("}\n\n")
