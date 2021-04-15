namespace com.mayr

@http(method: "POST", uri: "/coffee/create")
operation CreateCoffeeStore {
    input: CreateCoffeeRequest,
    output: CreateCoffeeResponse,
    errors: [InternalServerError]
}

structure CreateCoffeeRequest {
    @required
    coffeeStore: CoffeeStoreManifest
}

structure CreateCoffeeResponse {
    @required
    coffeeStoreDetails: CoffeeStoreDetails
}
