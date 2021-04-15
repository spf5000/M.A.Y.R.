namespace com.mayr

@error("server")
structure InternalServerError {
    @required
    reason: String
}
