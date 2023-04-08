# Around

Centralized events platform for adelaide uni    

### Explanation of the platform components:

`models/user.rs` 
Represents the users who will submit or consume information about events.

`models/event.rs` 
Represents the events, including details such as the event name, date, time, location, description, (etc) and a flag indicating if free food is available.

`repositories/user_repository.rs` 
Defines the repository interface for user-related CRUD operations.

`repositories/event_repository.rs` 
Defines the repository interface for event-related CRUD operations.

`dal/user_dal.rs` 
Implements the UserRepository interface and handles data storage interactions for users.

`dal/event_dal.rs` 
Implements the EventRepository interface and handles data storage interactions for events.

`bll/user_bll.rs` 
Contains the business logic for user-related operations, such as registration, authentication, and authorization.

`bll/event_bll.rs` 
Contains the business logic for event-related operations, such as event creation, modification, and retrieval based on different criteria.

`services/user_service.rs`
Defines the API for interacting with user-related business logic, to be consumed by the client or presentation layer.

`services/event_service.rs` 
Defines the API for interacting with event-related business logic, to be consumed by the client or presentation layer.

