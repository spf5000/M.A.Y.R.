namespace com.mayr

structure CoffeeStoreSummary {
    @required
    id: CoffeeStoreId,

    @required
    name: String,

    avgRating: Float
}

list CoffeeStoreSummaries {
    member: CoffeeStoreSummary 
}

structure CoffeeStoreManifest {
    @required
    id: CoffeeStoreId,

    @required
    name: String,

    description: String,
    avgRating: Float
}

structure CoffeeStoreDetails {
    @required
    id: CoffeeStoreId,

    @required
    name: String,

    description: String,
    avgRating: Float
}

string CoffeeStoreId
