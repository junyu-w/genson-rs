use criterion::{black_box, criterion_group, criterion_main, Criterion};
use genson_rs::*;

fn create_test_json_str(count: u32) -> String {
    let test_json_tiny = r#"
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

    let mut test_json = String::from("[");
    for _ in 0..count {
        test_json.push_str(test_json_tiny);
        test_json.push_str(",");
    }
    test_json.pop(); // remove the last comma
    test_json.push_str("]");
    test_json
}


pub fn criterion_benchmark(c: &mut Criterion) {
    
    let test_json_tiny = create_test_json_str(1);
    c.bench_function("build json schema TINY", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_tiny.as_bytes().to_vec();
            parse_json_schema(&mut builder,  &mut object_slice, false);
            black_box(builder);
        }
    ));

    let test_json_small = create_test_json_str(100);
    c.bench_function("build json schema SMALL", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_small.as_bytes().to_vec();
            parse_json_schema(&mut builder,  &mut object_slice, false);
            black_box(builder);
        }
    ));

    let test_json_medium = create_test_json_str(1000);
    c.bench_function("build json schema MEDIUM", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_medium.as_bytes().to_vec();
            parse_json_schema(&mut builder,  &mut object_slice, false);
            black_box(builder);
        }
    ));

    let test_json_large = create_test_json_str(10000);
    c.bench_function("build json schema LARGE", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_large.as_bytes().to_vec();
            parse_json_schema(&mut builder,  &mut object_slice, false);
            black_box(builder);
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);