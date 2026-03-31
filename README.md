#### Reflection Publisher-2

1. In MVC, Model covers both data storage and business logic. We need to 
separate Service and Repository from Model because of Single Responsibility 
Principle. Repository should only handle data access operations (CRUD), while 
Service should only handle business logic. If we put everything in Model, the 
class becomes too complex and hard to maintain. Changes in data storage would 
affect business logic and vice versa.

2. If we only use the Model without Service and Repository separation, the 
interactions between Program, Subscriber, and Notification would become very 
complex. Program model would need to directly manage Subscriber data and 
Notification logic. This means any change in one model could break other models. 
The code complexity would increase significantly because each model would need 
to know too much about other models, violating the separation of concerns principle.

3. Postman has been very helpful in testing the REST API endpoints. I used it 
to test subscribe, unsubscribe, create product, and delete product endpoints. 
Features that I find useful include: the ability to save requests in collections 
for reuse, setting request body in JSON format easily, and viewing response 
status codes and body clearly. For group projects, Postman collections can be 
shared among team members to ensure everyone tests the API consistently.