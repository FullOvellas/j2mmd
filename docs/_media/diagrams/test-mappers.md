```mermaid
flowchart LR
OrderMapper --> ProductMapper
OrderMapper --> AddressMapper
UserMapper --> CustomerMapper
CustomerMapper --> AddressMapper
StandaloneMapper
```