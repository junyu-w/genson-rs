use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use genson_rs::*;

fn create_test_json_str(count: u32, in_array: bool) -> String {
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
    "#.replace("\n", "");

    let mut test_json = String::from("");
    if in_array {
        test_json.push_str("[");
    }
    for _ in 0..count {
        test_json.push_str(&test_json_tiny);
        if in_array {
            test_json.push_str(",");
        } else {
            test_json.push_str("\n");
        }
    }
    test_json.pop(); // remove the last comma, or newline
    if in_array {
        test_json.push_str("]");
    }
    test_json
}


pub fn criterion_benchmark(c: &mut Criterion) {
    
    let test_json_tiny = create_test_json_str(1, true);
    c.bench_function("build single json object schema TINY", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_tiny.as_bytes().to_vec();
            build_json_schema(&mut builder,  &mut object_slice, None);
            black_box(builder);
        }
    ));

    let test_json_small = create_test_json_str(100, true);
    c.bench_function("build single json object schema SMALL", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_small.as_bytes().to_vec();
            build_json_schema(&mut builder,  &mut object_slice, None);
            black_box(builder);
        }
    ));

    let test_json_medium = create_test_json_str(1000, true);
    c.bench_function("build single json object schema MEDIUM", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_medium.as_bytes().to_vec();
            build_json_schema(&mut builder,  &mut object_slice, None);
            black_box(builder);
        }
    ));

    let test_json_large = create_test_json_str(10000, true);
    c.bench_function("build single json object schema LARGE", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_large.as_bytes().to_vec();
            build_json_schema(&mut builder,  &mut object_slice, None);
            black_box(builder);
        }
    ));

    let test_json_tiny = create_test_json_str(1, false);
    c.bench_function("build multi json object schema TINY", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_tiny.as_bytes().to_vec();
            build_json_schema(&mut builder,  &mut object_slice, Some("\n".as_bytes()[0]));
            black_box(builder);
        }
    ));

    let test_json_small = create_test_json_str(100, false);
    c.bench_function("build multi json object schema SMALL", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_small.as_bytes().to_vec();
            build_json_schema(&mut builder,  &mut object_slice, Some("\n".as_bytes()[0]));
            black_box(builder);
        }
    ));

    let test_json_medium = create_test_json_str(1000, false);
    c.bench_function("build multi json object schema MEDIUM", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_medium.as_bytes().to_vec();
            build_json_schema(&mut builder,  &mut object_slice, Some("\n".as_bytes()[0]));
            black_box(builder);
        }
    ));

    let test_json_large = create_test_json_str(10000, false);
    c.bench_function("build multi json object schema LARGE", |b| b.iter(||
        {
            let mut builder = get_builder(None);
            let mut object_slice = test_json_large.as_bytes().to_vec();
            build_json_schema(&mut builder,  &mut object_slice, Some("\n".as_bytes()[0]));
            black_box(builder);
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);