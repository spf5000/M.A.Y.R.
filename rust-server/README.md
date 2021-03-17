# Testing
## Manually
First run the server using cargo run. TODO: DockerCompose

### List Coffee Stores
```
curl -d "" localhost:9080/coffee/list
```

### Create Coffee Store
```
curl -d '{"CoffeeStore": {"Name": "Rosolinis", "Description": "Coffee and Pastries", "AvgRating": 4.9}}' localhost:9080/coffee/create
```


