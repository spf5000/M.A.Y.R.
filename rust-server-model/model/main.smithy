namespace com.mayr

use aws.protocols#restJson1

@restJson1
service MayrService {
    version: "2021-04-11",
    resources: [CoffeeStore],
    operations: [
        ListCoffeeStores,
        GetCoffeeStore
    ]
}

resource CoffeeStore {
    identifiers: { coffeeStoreId: CoffeeStoreId },
    create: CreateCoffeeStore,
}