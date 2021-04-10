db.createCollection('CoffeeStores')
db.CoffeeStores.createIndex({"Name": -1}, {unique: true})
db.CoffeeStores.insertOne({
    "Id": "6fc64957-1611-47ba-aa32-940e0b2684c6", 
    "Name": "Starbucks", 
    "Description": 
    "McDonald's of Coffee Stores. Consistently mediocre.", 
    "AvgRating": 2.5
})
