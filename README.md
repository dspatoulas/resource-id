# Unique Resource Identifier (URID)

The Unique Resource Identifier (`URID`) package was to create **meaningful, human-readable** unique identifiers
as an alternative to using opaque Universally Unique Identifiers (`UUID`) and enriches the value of 
using the underlying Unique Lexicographically Sortable Identifier (`ULID`) itself.

## Advantage Over UUID

There are two primary advantages of using Resource Identifiers over UUIDs and ULIDs.

### 1. Resource Lineage

The first 4 characters of each Unique Resource Identifier (`URID`) contains the Resource for which the unique
identifier was generated. Humans can intuitive reason about the resource and the time at which the
resource was created without any other context.

#### Example

Given the URID value of `ACCT01JMYG17BTG66QFZ3EVNPY89VV`, a user instantly can derive the resource is an Account,
and can use the ULID value, the characters after the 4th character, to determine the time at which the resource
was created.

- `ACCT` - Unique Account
- `01JMYG17BTG66QFZ3EVNPY89VV` - Timestamp the Account was Created

Try it out using an [online ULID timestamp converter](https://ugai.github.io/ulid-timestamp-converter/). 
The logic can be used in application code to sync the `created_at` property when storing the account data 
into the application's datastore.

### 2. Shorter Length (30 Characters)

The `Unique Resource Identifier (URID)` was designed to be used as a primary key in the data store of an application,
which provides a significant advantage over using a `UUID` because the `ULID` can be used to bind the identifier
of a custom application resource to the time at which that resource was created.

#### Unique Lexicographically Sortable Identifier (ULID)

A Universally Unique Lexicographically Sortable Identifier (`ULID`) is a 128-bit identifier 
that's an alternative to UUIDs. ULIDs are designed to be easy to sort and human-readable. 

#### Advantages of ULID based URIDs

- **Sortable**: `ULID`s are sortable by time of creation
- **Compact**: `ULID`s are compact and easy to handle
- **URL safe**: `ULID`s are URL safe because they don't contain special characters
- **Monotonic**: `ULID`s can detect and handle the same millisecond
