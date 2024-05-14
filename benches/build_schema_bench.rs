use criterion::{black_box, criterion_group, criterion_main, Criterion};
use genson_rs::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let test_json = r#"
    {
        "name": "John Doe",
        "age": 43,
        "isAlive": true,
        "address": {
            "streetAddress": "123 Main St",
            "city": "Springfield",
            "state": "IL",
            "postalCode": "62701-1234"
        },
        "phoneNumbers": [
            {
                "type": "home",
                "number": "212 555-1234"
            },
            {
                "type": "office",
                "number": "646 555-4567"
            }
        ],
        "children": [],
        "spouse": null
    }
    "#;

    c.bench_function("build json schema", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let test_object = serde_json::from_str(test_json).unwrap();
            builder.add_object(&test_object);
            let schema = builder.to_json();
            black_box(schema);
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);