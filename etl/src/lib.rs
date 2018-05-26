use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input
        .iter()
        .flat_map(|(&k, vec)| {
            vec.into_iter()
                .map(|v| (v.to_lowercase(), k))
                .collect::<Vec<_>>()
        })
        .collect()
}
