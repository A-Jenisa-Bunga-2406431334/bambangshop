#### Reflection Publisher-3

1. In this tutorial, we use the Push model variation of Observer Pattern. 
The publisher (BambangShop) actively pushes notification data to each 
subscriber when an event occurs (product created, deleted, or promoted). 
The subscribers do not need to request data - they just receive it.

2. If we used the Pull model instead, the advantages would be that subscribers 
can request data only when they need it, reducing unnecessary data transfer. 
However, the disadvantages are: subscribers need to periodically poll the 
publisher for updates (inefficient), there would be delays in receiving 
notifications, and the publisher needs to provide additional endpoints for 
subscribers to pull data from. For our use case, Push model is better because 
we want real-time notifications.

3. If we decide to not use multi-threading in the notification process, the 
program would send HTTP requests to each subscriber sequentially. This means 
the program would have to wait for each HTTP request to complete before 
sending the next one. If there are many subscribers or if the network is slow, 
this could cause significant delays and block the main thread from handling 
other requests. With multi-threading, all notifications are sent concurrently, 
making the process much faster and non-blocking.