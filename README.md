#### Reflection Publisher-3

1. In the Observer pattern, the `notify` method in NotificationService iterates 
through all subscribers and calls their `update` method. Each Subscriber's 
`update` method sends an HTTP POST request to the subscriber's URL. This means 
if there are many subscribers, the notification process will take longer as it 
sends requests sequentially. For better performance, we could use async/parallel 
requests.

2. If we had 1 publisher and many subscribers, running the receiver app multiple 
times on different ports would simulate multiple subscribers. Each receiver 
instance would register itself with a different URL, and the publisher would 
send notifications to all of them when a product event occurs.

3. Using async programming would make the notification process more efficient. 
Instead of waiting for each HTTP request to complete before sending the next one, 
we could send all notifications concurrently. However, the current blocking 
approach is simpler to implement and debug. The trade-off is between simplicity 
and performance.