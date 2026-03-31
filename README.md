#### Reflection Publisher-2

1. In MVC, Service and Repository are both part of the Model layer but have 
different responsibilities. Repository handles direct data access (CRUD 
operations to the database/storage), while Service contains business logic 
and orchestrates calls to Repository. Separating them makes the code more 
maintainable - if we change the storage mechanism, we only need to change 
the Repository without touching the Service logic.

2. If we only use the Main app without the Receiver app, we would still need 
to implement the notification mechanism but it would only notify subscribers 
that are running on the same instance. The Receiver app is needed to simulate 
real-world scenarios where subscribers are separate services. Without it, 
we cannot properly test the Observer pattern's notification mechanism across 
different services.

3. While implementing the tutorial, I noticed that using Postman made testing 
the REST endpoints much easier. Each endpoint (subscribe, unsubscribe) could 
be tested independently without needing a frontend. The DashMap data structure 
was particularly useful for thread-safe concurrent access to subscriber data, 
which is important in a web application context.