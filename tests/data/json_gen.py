"""
This script generates a JSON file with a size of 50MB, 500MB, 1GB and 3GB.
"""
import json

three_gb_json_count = 10000000
five_hundred_mb_json_count = three_gb_json_count // 6
fifty_mb_json_count = three_gb_json_count // 60
one_gb_json_count = five_hundred_mb_json_count * 2

data = []
for i in range(one_gb_json_count):
    charge = {
        "status": "success",
        "data": {
            "order_id": "123456789",
            "customer": {
                "name": "John Doe",
                "email": "johndoe@example.com",
                "address": {
                    "street": "123 Main St",
                    "city": "San Francisco",
                    "state": "CA",
                    "zip": "94101"
                }
            },
            "items": [
                {
                    "id": "item1",
                    "name": "Product 1",
                    "price": 99.99
                },
                {
                    "id": "item2",
                    "name": "Product 2",
                    "price": 49.99
                }
            ],
            "total_amount": 149.98
        }
    }
    if i % 2 == 0:
        charge["status"] = "failed"
        charge["data"].pop("items")
    if i % 3 == 0:
        charge["metadata"] = {
            "notes": "This is a test charge"
        }
    data.append(charge)

# Write the JSON data to a file
with open("test_large_1gb_full_json_array.json", "w") as file:
    # json_text = "\n".join([json.dumps(charge) for charge in data])
    # file.write(json_text)
    json.dump(data, file)

