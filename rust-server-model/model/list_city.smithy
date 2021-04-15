namespace com.mayr

@http(method: "POST", uri: "/coffee/list")
operation ListCoffeeStores {
    input: ListCoffeesRequest,
    output: ListCoffeesResponse,
    errors: [InternalServerError]
}

structure ListCoffeesRequest {
    nextToken: String,
    maxItems: Integer
}

structure ListCoffeesResponse {
    @required
    coffeeStoreSummaries: CoffeeStoreSummaries,

    nextToken: String
}
