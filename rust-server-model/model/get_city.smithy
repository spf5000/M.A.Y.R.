namespace com.mayr

@http(method: "POST", uri: "/coffee/get")
operation GetCoffeeStore {
    input: GetCoffeeRequest,
    output: GetCoffeeResponse,
    errors: [InternalServerError]
}

structure GetCoffeeRequest {
    @required
    coffeeStoreId: CoffeeStoreId
}

structure GetCoffeeResponse {
    @required
    coffeeStoreDetails: CoffeeStoreDetails
}
